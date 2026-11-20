use crate::assets;
use crate::frontmatter::{self, Frontmatter, Slug};
use crate::markdown::{self, Heading};
use include_dir::{Dir, File, include_dir};

/// Embedded posts directory. Drop a new `content/posts/<slug>.md` file in and
/// it is picked up automatically at compile time - no code changes required.
///
/// Only `posts/` is embedded: `content/assets/**` is fingerprinted into
/// `dist/assets/` by `build.rs` (see `src/assets.rs`) and must never inflate
/// the WASM binary.
static POSTS_DIR: Dir = include_dir!("content/posts");

/// A display cover for a post.
#[derive(Debug, Clone, PartialEq)]
pub struct Cover {
  /// Already-resolved public URL (`/assets/...`).
  pub src: String,
  pub alt: String,
  /// Caption recovered from the first body figure when it became the cover.
  pub caption: Option<String>,
}

/// A fully processed blog post, ready to render.
#[derive(Debug, Clone, PartialEq)]
pub struct Post {
  pub slug: Slug,
  pub meta: Frontmatter,
  /// Raw markdown body (kept for RSS / future use).
  pub body: String,
  /// Rendered HTML body.
  pub html: String,
  /// Cover image used by cards and the article header.
  pub cover: Option<Cover>,
  /// `h2` sections, for the article table of contents.
  pub headings: Vec<Heading>,
  /// Estimated reading time in minutes.
  pub reading_time: usize,
}

/// Load, parse and sort every published post from the embedded content tree.
pub fn load_posts() -> Vec<Post> {
  let mut posts = Vec::new();

  // include_dir's `files()` is non-recursive, so collect every file in the
  // tree (including nested category directories) first.
  let mut files = Vec::new();
  collect_files(&POSTS_DIR, &mut files);

  for file in files {
    let path = file.path();
    let is_markdown = path
      .extension()
      .and_then(|e| e.to_str())
      .map(|e| e.eq_ignore_ascii_case("md"))
      .unwrap_or(false);

    if is_markdown
      && let Some(raw) = file.contents_utf8()
      && let Some(post) = parse_post(raw, path)
      && !post.meta.draft
    {
      posts.push(post);
    }
  }

  posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
  posts
}

/// Recursively gather every file beneath a directory.
fn collect_files(dir: &Dir<'static>, out: &mut Vec<&File<'static>>) {
  for file in dir.files() {
    out.push(file);
  }
  for sub in dir.dirs() {
    collect_files(sub, out);
  }
}

/// Parse a single markdown file into a [`Post`].
///
/// Uses the shared [`frontmatter::parse`] so the in-app render and the RSS
/// feed (built in `build.rs`) can never disagree about a post. Errors cannot
/// reach runtime: `build.rs` already fails the build on any malformed post.
fn parse_post(raw: &str, path: &std::path::Path) -> Option<Post> {
  let parsed = frontmatter::parse(raw).ok()?;

  let stem = path.file_stem().map(|s| s.to_string_lossy());
  let slug = frontmatter::derive_slug(&parsed.meta, stem.as_deref()).ok()?;

  let document = markdown::render_document(&parsed.body);
  let mut html = document.html;
  let mut headings = document.headings;

  let first_figure = markdown::first_image(&parsed.body).map(|(dest, alt)| {
    let src = assets::resolve(&dest).unwrap_or(&dest).to_string();
    (src, alt)
  });
  let mut cover = resolve_cover(&parsed.meta, &slug, first_figure.as_ref());

  // When the cover *is* the first body figure, avoid showing the same still
  // twice: lift the figure (and its caption) into the article header and
  // renumber the remaining figures.
  if let (Some(cover_ref), Some((first_src, _))) = (cover.as_ref(), first_figure.as_ref())
    && &cover_ref.src == first_src
  {
    let (stripped, caption) = strip_leading_figure(&html, first_src);
    html = stripped;
    if let Some(cover) = cover.as_mut() {
      cover.caption = caption.map(|text| strip_caption_label(&text));
    }
  }

  // Keep TOC anchors honest if a cover-stripping change ever edits headings.
  headings.retain(|h| !h.text.is_empty());

  // Thai does not use spaces between words, so count non-whitespace characters
  // and use a comfortable ~450 characters per minute for the reading estimate.
  let chars = parsed.body.chars().filter(|c| !c.is_whitespace()).count();
  let reading_time = ((chars as f64 / 450.0).ceil() as usize).max(1);

  Some(Post {
    slug,
    meta: parsed.meta,
    body: parsed.body,
    html,
    cover,
    headings,
    reading_time,
  })
}

/// Pick a cover in editorial priority order: an explicit frontmatter cover,
/// a generated `<slug>-cover.jpg`, then the first Markdown image.
fn resolve_cover(
  meta: &Frontmatter,
  slug: &Slug,
  first_figure: Option<&(String, String)>,
) -> Option<Cover> {
  if let Some(reference) = &meta.cover {
    let src = assets::resolve(reference).unwrap_or(reference).to_string();
    return Some(Cover {
      src,
      alt: meta.title.clone(),
      caption: None,
    });
  }

  let convention = format!("assets/{slug}-cover.jpg");
  if let Some(src) = assets::resolve(&convention) {
    return Some(Cover {
      src: src.to_string(),
      alt: meta.title.clone(),
      caption: None,
    });
  }

  first_figure.map(|(src, alt)| Cover {
    src: src.clone(),
    alt: if alt.is_empty() {
      meta.title.clone()
    } else {
      alt.clone()
    },
    caption: None,
  })
}

/// Remove the leading image paragraph and its caption when the image has been
/// promoted to the article cover. Caption numbers after it are shifted down so
/// the remaining figures still read 1, 2, 3...
fn strip_leading_figure(html: &str, cover_src: &str) -> (String, Option<String>) {
  let src_attr = format!("src=\"{cover_src}\"");
  let Some(src_pos) = html.find(&src_attr) else {
    return (html.to_string(), None);
  };
  let Some(img_pos) = html[..src_pos].rfind("<img") else {
    return (html.to_string(), None);
  };
  let Some(p_start) = html[..img_pos].rfind("<p>") else {
    return (html.to_string(), None);
  };
  // Only strip a paragraph that exists solely to hold the image.
  if !html[p_start + 3..img_pos].trim().is_empty() {
    return (html.to_string(), None);
  }
  let Some(close) = html[img_pos..].find("</p>") else {
    return (html.to_string(), None);
  };
  let p_end = img_pos + close + 4;
  let Some(img_end) = html[img_pos..].find('>').map(|i| img_pos + i + 1) else {
    return (html.to_string(), None);
  };
  if img_end > p_end - 4
    || !html[p_start + 3..img_pos].trim().is_empty()
    || !html[img_end..p_end - 4].trim().is_empty()
  {
    return (html.to_string(), None);
  }

  let mut out = String::with_capacity(html.len());
  out.push_str(&html[..p_start]);
  out.push_str(&html[p_end..]);

  let mut caption = None;
  let tail = &out[p_start..];
  let skip = tail.len() - tail.trim_start().len();
  let caption_pos = p_start + skip;
  let caption_open = "<p class=\"prose-caption\">";
  if out[caption_pos..].starts_with(caption_open) {
    let inner_start = caption_pos + caption_open.len();
    if let Some(cap_close) = out[inner_start..].find("</p>") {
      let cap_end = inner_start + cap_close;
      caption = Some(markdown::plain_text(&out[inner_start..cap_end]));
      out.replace_range(caption_pos..cap_end + 4, "");
    }
  }

  let removed_numbered = caption
    .as_deref()
    .is_some_and(|text| text.contains("ภาพที่ "));
  if removed_numbered {
    out = renumber_captions(&out, -1);
  }
  (out, caption)
}

/// Shift caption numbers inside `<p class="prose-caption">` blocks only.
fn renumber_captions(html: &str, delta: i32) -> String {
  const OPEN: &str = "<p class=\"prose-caption\">";
  let mut out = String::with_capacity(html.len());
  let mut rest = html;
  while let Some(pos) = rest.find(OPEN) {
    out.push_str(&rest[..pos]);
    out.push_str(OPEN);
    let after = &rest[pos + OPEN.len()..];
    let Some(end) = after.find("</p>") else {
      out.push_str(after);
      return out;
    };
    out.push_str(&renumber_caption_labels(&after[..end], delta));
    out.push_str("</p>");
    rest = &after[end + 4..];
  }
  out.push_str(rest);
  out
}

/// Shift every `ภาพที่ N:` label in one caption, never below 1.
fn renumber_caption_labels(html: &str, delta: i32) -> String {
  const MARKER: &str = "ภาพที่ ";
  let mut out = String::with_capacity(html.len());
  let mut rest = html;
  while let Some(pos) = rest.find(MARKER) {
    out.push_str(&rest[..pos]);
    let after = &rest[pos + MARKER.len()..];
    let digits = after.chars().take_while(|c| c.is_ascii_digit()).count();
    if digits == 0 || !after[digits..].starts_with(':') {
      out.push_str(MARKER);
      rest = after;
      continue;
    }
    let number = after[..digits].parse::<i32>().unwrap_or(1);
    out.push_str(MARKER);
    out.push_str(&(number + delta).max(1).to_string());
    rest = &after[digits..];
  }
  out.push_str(rest);
  out
}

/// `ภาพที่ 1: คำอธิบาย` -> `คำอธิบาย` for the hero caption.
fn strip_caption_label(caption: &str) -> String {
  caption
    .strip_prefix("ภาพที่ ")
    .and_then(|rest| rest.split_once(':'))
    .map(|(_, text)| text.trim().to_string())
    .unwrap_or_else(|| caption.to_string())
}

/// Collect the unique, sorted list of tags across all posts.
pub fn all_tags(posts: &[Post]) -> Vec<String> {
  let mut tags: Vec<String> = posts
    .iter()
    .flat_map(|p| p.meta.tags.iter().cloned())
    .collect::<std::collections::BTreeSet<_>>()
    .into_iter()
    .collect();
  tags.sort();
  tags
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn loads_embedded_posts() {
    let posts = load_posts();
    assert!(
      posts.len() >= 2,
      "expected at least 2 embedded posts, found {}",
      posts.len()
    );
    assert!(posts.iter().any(|p| p.slug == "seabass-bachi-nuke"));
    assert!(posts.iter().any(|p| p.slug == "sinking-pencil"));
    // newest first
    assert!(posts[0].meta.date >= posts[1].meta.date);
  }

  #[test]
  fn embedded_slugs_are_unique() {
    let posts = load_posts();
    let mut seen = std::collections::HashSet::new();
    for post in &posts {
      assert!(
        seen.insert(post.slug.as_str()),
        "duplicate post slug `{}` - two posts would overwrite the same dist/post/<slug>/ page",
        post.slug
      );
    }
  }

  #[test]
  fn every_post_has_a_cover_image() {
    for post in load_posts() {
      let cover = post
        .cover
        .as_ref()
        .unwrap_or_else(|| panic!("{} has no cover", post.slug));
      assert!(
        cover.src.starts_with("/assets/"),
        "{} cover is not a resolved asset: {}",
        post.slug,
        cover.src
      );
      assert!(
        !cover.alt.trim().is_empty(),
        "{} cover has no alt",
        post.slug
      );
    }
  }

  #[test]
  fn every_post_has_a_table_of_contents() {
    for post in load_posts() {
      assert!(
        !post.headings.is_empty(),
        "{} has no h2 sections to put in its TOC",
        post.slug
      );
      for (i, heading) in post.headings.iter().enumerate() {
        assert_eq!(heading.id, format!("section-{}", i + 1));
        assert!(!heading.text.trim().is_empty());
      }
    }
  }

  #[test]
  fn strips_the_leading_figure_and_renumbers_later_captions() {
    let html = concat!(
      "<p>Intro</p>\n",
      "<p><img src=\"/assets/a.jpg\" alt=\"a\" /></p>\n",
      "<p class=\"prose-caption\"><strong>ภาพที่ 1:</strong> first</p>\n",
      "<p><img src=\"/assets/b.jpg\" alt=\"b\" /></p>\n",
      "<p class=\"prose-caption\"><strong>ภาพที่ 2:</strong> second</p>"
    );
    let (stripped, caption) = strip_leading_figure(html, "/assets/a.jpg");
    assert_eq!(caption.as_deref(), Some("ภาพที่ 1: first"));
    assert!(!stripped.contains("/assets/a.jpg"));
    assert!(!stripped.contains("first"));
    assert!(stripped.contains("ภาพที่ 1:</strong> second"));
    assert!(!stripped.contains("ภาพที่ 2:"));
  }

  #[test]
  fn strips_a_figure_whose_img_has_loading_attributes() {
    let html = concat!(
      "<p><img loading=\"lazy\" decoding=\"async\" src=\"/assets/a.jpg\" alt=\"a\" /></p>\n",
      "<p class=\"prose-caption\"><strong>ภาพที่ 1:</strong> first</p>"
    );
    let (stripped, caption) = strip_leading_figure(html, "/assets/a.jpg");
    assert!(stripped.trim().is_empty(), "{stripped:?}");
    assert_eq!(caption.as_deref(), Some("ภาพที่ 1: first"));
  }

  #[test]
  fn cover_stripping_leaves_a_non_matching_first_figure_alone() {
    let html = "<p><img src=\"/assets/a.jpg\" alt=\"a\" /></p>";
    let (stripped, caption) = strip_leading_figure(html, "/assets/other.jpg");
    assert_eq!(stripped, html);
    assert_eq!(caption, None);
  }

  #[test]
  fn cover_stripping_preserves_a_mixed_image_paragraph() {
    let html = "<p><img src=\"/assets/a.jpg\" alt=\"a\" /> Important instructions.</p>";
    let (stripped, caption) = strip_leading_figure(html, "/assets/a.jpg");
    assert_eq!(stripped, html);
    assert_eq!(caption, None);
  }

  #[test]
  fn unnumbered_caption_does_not_shift_remaining_numbers() {
    let html = concat!(
      "<p><img src=\"/assets/a.jpg\" alt=\"a\" /></p>\n",
      "<p class=\"prose-caption\">no number</p>\n",
      "<p class=\"prose-caption\"><strong>ภาพที่ 2:</strong> second</p>"
    );
    let (stripped, caption) = strip_leading_figure(html, "/assets/a.jpg");
    assert_eq!(caption.as_deref(), Some("no number"));
    assert!(stripped.contains("ภาพที่ 2:</strong> second"), "{stripped}");
  }

  #[test]
  fn renumber_only_caption_paragraphs() {
    let html = concat!(
      "<p>ภาพที่ 2: prose stays put</p>",
      "<p class=\"prose-caption\"><strong>ภาพที่ 2:</strong> caption moves</p>",
      "<code>ภาพที่ 2: code stays put</code>"
    );
    let out = renumber_captions(html, -1);
    assert!(out.contains("<p>ภาพที่ 2: prose stays put</p>"), "{out}");
    assert!(
      out.contains("<p class=\"prose-caption\"><strong>ภาพที่ 1:</strong> caption moves</p>"),
      "{out}"
    );
    assert!(out.contains("<code>ภาพที่ 2: code stays put</code>"), "{out}");
  }

  #[test]
  fn caption_number_never_drops_below_one() {
    let html = "<p class=\"prose-caption\"><strong>ภาพที่ 1:</strong> x</p>";
    assert_eq!(renumber_captions(html, -1), html);
  }
}
