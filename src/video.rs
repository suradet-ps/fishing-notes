//! Short inline video clips for articles.
//!
//! Authors write a fenced `video` block:
//!
//! ```text
//! src: assets/clip.mp4
//! poster: assets/still.jpg
//! title: Two-finger grip
//! caption: The index and middle fingers pinch the reel seat.
//! muted: true
//! ```
//!
//! The markdown renderer turns it into a native `<video controls>` figure,
//! while the build script validates both the clip and its poster against the
//! staged content assets.

/// A parsed `video` directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directive {
  pub src: String,
  pub poster: Option<String>,
  pub title: Option<String>,
  pub caption: Option<String>,
  /// Start muted so an inline technique clip never surprises the reader with
  /// audio; the native controls still allow unmuting.
  pub muted: bool,
}

impl Directive {
  /// Parse the body of a fenced `video` block. The first bare line is treated
  /// as `src`; all other values use `key: value` lines.
  pub fn parse(source: &str) -> Option<Self> {
    let mut src = None;
    let mut poster = None;
    let mut title = None;
    let mut caption = None;
    let mut muted = false;

    for line in source.lines() {
      let line = line.trim();
      if line.is_empty() || line.starts_with('#') {
        continue;
      }
      let (key, value) = match line.split_once(':') {
        Some((key, value)) => (key.trim().to_ascii_lowercase(), value.trim()),
        None if src.is_none() && !line.contains(char::is_whitespace) => ("src".to_string(), line),
        None => continue,
      };
      match key.as_str() {
        "src" => src = Some(value.to_string()),
        "poster" => poster = Some(value.to_string()),
        "title" => title = Some(value.to_string()),
        "caption" => caption = Some(value.to_string()),
        "muted" => muted = truthy(value),
        _ => {}
      }
    }

    Some(Self {
      src: src?,
      poster,
      title,
      caption,
      muted,
    })
  }
}

fn truthy(value: &str) -> bool {
  matches!(
    value.trim().to_ascii_lowercase().as_str(),
    "true" | "yes" | "1" | "on"
  )
}

/// Collect every content-asset reference used by `video` blocks, without
/// pulling the markdown renderer into the build script.
pub fn asset_references(markdown: &str) -> Vec<String> {
  let mut references = Vec::new();
  let mut in_video = false;
  let mut body = String::new();

  for line in markdown.lines() {
    let trimmed = line.trim_start();
    if !in_video {
      if trimmed.starts_with("```video") || trimmed.starts_with("~~~video") {
        in_video = true;
        body.clear();
      }
      continue;
    }

    if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
      if let Some(directive) = Directive::parse(&body) {
        for reference in [Some(&directive.src), directive.poster.as_ref()]
          .into_iter()
          .flatten()
        {
          if let Some(key) = crate::fingerprint::normalize_ref(reference) {
            references.push(key);
          }
        }
      }
      in_video = false;
      continue;
    }

    body.push_str(line);
    body.push('\n');
  }

  references.sort();
  references.dedup();
  references
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_directive_keys_and_bare_source() {
    let directive = Directive::parse(
      "assets/clip.mp4\nposter: assets/still.jpg\ntitle: Grip\ncaption: Two fingers\nmuted: true",
    )
    .expect("valid directive");
    assert_eq!(directive.src, "assets/clip.mp4");
    assert_eq!(directive.poster.as_deref(), Some("assets/still.jpg"));
    assert_eq!(directive.title.as_deref(), Some("Grip"));
    assert_eq!(directive.caption.as_deref(), Some("Two fingers"));
    assert!(directive.muted);
  }

  #[test]
  fn requires_a_source() {
    assert_eq!(Directive::parse("poster: assets/still.jpg"), None);
  }

  #[test]
  fn collects_only_video_block_assets() {
    let markdown = r#"
```rust
let x = "assets/not-a-video.mp4";
```

```video
src: assets/clip.mp4
poster: assets/still.jpg
```

```video
assets/other.mp4
```
"#;
    assert_eq!(
      asset_references(markdown),
      vec!["assets/clip.mp4", "assets/other.mp4", "assets/still.jpg"]
    );
  }
}
