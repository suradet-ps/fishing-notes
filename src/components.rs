use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::icons::{ArrowIcon, FishMark, MoonIcon, SunIcon};
use crate::util::{Theme, apply_theme, load_theme, save_theme};
use fishing_notes::content::Post;
use fishing_notes::frontmatter::Lang;
use fishing_notes::site;

/// Tags that describe the whole library rather than a specific subject; cards
/// prefer the first specific tag so every card says something useful.
const GENERIC_TAGS: [&str; 3] = ["เทคนิค", "เหยื่อปลอม", "โปรญี่ปุ่น"];

fn post_card_description(description: &str) -> String {
  if description.is_empty() {
    site::DESCRIPTION.to_string()
  } else {
    description.to_string()
  }
}

/// The most specific tag on a post, used as the card's kicker.
pub fn primary_tag(post: &Post) -> String {
  post
    .meta
    .tags
    .iter()
    .find(|tag| !GENERIC_TAGS.contains(&tag.as_str()))
    .cloned()
    .unwrap_or_else(|| "บทความ".to_string())
}

/// Replace ASCII hyphens with non-breaking hyphens for display, so titles
/// like `1-2-3` never break after a dash. A few recurring Thai compounds and
/// names are also glued with word joiners: Thai has no spaces, and the
/// browser's dictionary will otherwise split a display title inside a name.
/// The underlying content is unchanged.
pub fn display_title(title: &str) -> String {
  const KEEP_TOGETHER: [&str; 8] = [
    "บทเรียน",
    "มุราตะ",
    "นามิกิ",
    "มิยาซาวะ",
    "ฮิโรเซ",
    "โอโนะ",
    "ยูกิ",
    "โคทาโร่",
  ];

  let mut out = title.replace('-', "\u{2011}");
  for phrase in KEEP_TOGETHER {
    let joined = phrase
      .chars()
      .map(|c| c.to_string())
      .collect::<Vec<_>>()
      .join("\u{2060}");
    out = out.replace(phrase, &joined);
  }
  out
}

/// A resolved, base-aware cover URL.
pub fn cover_src(post: &Post) -> Option<String> {
  post.cover.as_ref().map(|cover| {
    if cover.src.starts_with("http") || cover.src.starts_with("data:") {
      cover.src.clone()
    } else {
      crate::util::href(&cover.src)
    }
  })
}

/// Cover alt text, falling back to the post title.
pub fn cover_alt(post: &Post) -> String {
  post
    .cover
    .as_ref()
    .map(|cover| cover.alt.clone())
    .unwrap_or_else(|| post.meta.title.clone())
}

/// Shared theme state, provided near the root of the app.
#[derive(Clone, Copy)]
pub struct ThemeContext {
  pub theme: ReadSignal<Theme>,
  pub set_theme: WriteSignal<Theme>,
}

/// Wraps the app, initialises the theme and keeps `<html data-theme>` in sync.
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
  let initial = load_theme();
  let (theme, set_theme) = signal(initial);
  apply_theme(initial);
  provide_context(ThemeContext { theme, set_theme });

  Effect::new(move |_| {
    let t = theme.get();
    apply_theme(t);
    save_theme(t);
  });

  view! { {children()} }
}

/// Access the current theme context.
pub fn use_theme() -> ThemeContext {
  expect_context::<ThemeContext>()
}

/// Light/dark mode toggle button.
#[component]
pub fn ThemeToggle() -> impl IntoView {
  let ThemeContext { theme, set_theme } = use_theme();
  view! {
      <button
          class="theme-toggle"
          type="button"
          aria-label="Toggle light / dark mode"
          title="Toggle light / dark mode"
          on:click=move |_| set_theme.set(theme.get().toggle())
      >
          <span class="theme-toggle-icon">
              {move || match theme.get() {
                  Theme::Light => Either::Left(view! { <MoonIcon/> }),
                  Theme::Dark => Either::Right(view! { <SunIcon/> }),
              }}
          </span>
      </button>
  }
}

/// Site header with branding and primary navigation.
#[component]
pub fn Nav() -> impl IntoView {
  view! {
      <header class="site-header">
          <a class="skip-link" href="#main">"Skip to content"</a>
          <div class="container nav-inner">
              <A href=crate::util::href("/")>
                  <span class="brand">
                      <span class="brand-mark"><FishMark/></span>
                      <span class="brand-text">
                          <span class="brand-name">{site::TITLE}</span>
                          <span class="brand-sub">"FIELD NOTES FROM JAPAN"</span>
                      </span>
                  </span>
              </A>
              <nav class="nav-links" aria-label="หลัก">
                  <A href=crate::util::href("/")>
                      <span class="nav-link">"บทความ"</span>
                  </A>
                  <A href=crate::util::href("/about")>
                      <span class="nav-link">"เกี่ยวกับ"</span>
                  </A>
                  <a href=crate::util::href("/rss.xml") target="_blank" rel="noreferrer">
                      <span class="nav-link">"RSS"</span>
                  </a>
                  <ThemeToggle/>
              </nav>
          </div>
      </header>
  }
}

/// Site footer.
#[component]
pub fn Footer() -> impl IntoView {
  let year = js_sys::Date::new_0().get_full_year();
  view! {
      <footer class="site-footer">
          <div class="container footer-grid">
              <div class="footer-brand">
                  <span class="footer-mark"><FishMark/></span>
                  <p class="footer-title">{site::TITLE}</p>
                  <p class="footer-tagline">{site::TAGLINE}</p>
              </div>
              <div class="footer-col">
                  <p class="footer-label">"อ่าน"</p>
                  <A href=crate::util::href("/")><span>"บทความทั้งหมด"</span></A>
                  <A href=crate::util::href("/about")><span>"เกี่ยวกับบล็อก"</span></A>
                  <a href=crate::util::href("/rss.xml") target="_blank" rel="noreferrer">"RSS"</a>
              </div>
              <div class="footer-col">
                  <p class="footer-label">"ต้นทาง"</p>
                  <a href=site::GITHUB_URL target="_blank" rel="noreferrer">"GitHub"</a>
                  <span>"ภาพจากคลิปต้นฉบับ"</span>
                  <span>"เรียบเรียงเป็นภาษาไทย"</span>
              </div>
          </div>
          <div class="container footer-base">
              <p>{site::AUTHOR}" · © "{year}</p>
              <p>"บันทึกการตกปลาเหยื่อปลอมจากคลิปโปรญี่ปุ่น"</p>
          </div>
      </footer>
  }
}

/// A clickable tag used for filtering on the home page.
#[component]
pub fn TagChip(
  tag: String,
  active: bool,
  #[prop(default = 0)] count: usize,
  #[prop(optional)] on_click: Option<Callback<String>>,
) -> impl IntoView {
  let tag_for_click = tag.clone();
  view! {
      <button
          class=move || if active { "tag-chip active" } else { "tag-chip" }
          type="button"
          aria-pressed=if active { "true" } else { "false" }
          on:click=move |_| {
              if let Some(cb) = &on_click {
                  cb.run(tag_for_click.clone());
              }
          }
      >
          <span>{tag}</span>
          {(count > 0).then(|| view! { <span class="tag-count">{count}</span> })}
      </button>
  }
}

/// A small badge marking a post whose language differs from the site default.
#[component]
pub fn LangBadge(lang: Lang) -> impl IntoView {
  (!matches!(lang, Lang::Th)).then(|| {
    view! {
        <span class="lang-badge" lang=lang.code() aria-label=lang.label()>
            {lang.badge()}
        </span>
    }
  })
}

/// The lead story in the home hero: one large cover with a gradient overlay.
#[component]
pub fn FeatureStory(post: Post) -> impl IntoView {
  let href = crate::util::href(&format!("/post/{}", post.slug));
  let cover = cover_src(&post);
  let alt = cover_alt(&post);
  let tag = primary_tag(&post);
  let title = display_title(&post.meta.title);
  let description = display_title(&post_card_description(&post.meta.description));
  let reading = post.reading_time;
  let date = crate::util::format_date(&post.meta.date);

  view! {
      <A href=href>
          <article class="feature-story">
              <div class="feature-media">
                  {cover.map(|src| view! {
                      <img src=src alt=alt.clone() loading="eager" decoding="async"/>
                  })}
                  <span class="feature-badge">"เรื่องเด่น"</span>
              </div>
              <div class="feature-body">
                  <div class="feature-kicker">
                      <span>{tag}</span>
                      <span class="dot">"·"</span>
                      <span>{format!("{reading} นาทีอ่าน")}</span>
                  </div>
                  <h2 class="feature-title">{title}</h2>
                  <p class="feature-desc">{description}</p>
                  <div class="feature-foot">
                      <span class="feature-date">{date}</span>
                      <span class="feature-more">"อ่านเรื่องนี้ " <ArrowIcon/></span>
                  </div>
              </div>
          </article>
      </A>
  }
}

/// A single post preview card in the library grid.
#[component]
pub fn PostCard(post: Post) -> impl IntoView {
  let href = crate::util::href(&format!("/post/{}", post.slug));
  let date = crate::util::format_date(&post.meta.date);
  let description = display_title(&post_card_description(&post.meta.description));
  let tag = primary_tag(&post);
  let cover = cover_src(&post);
  let alt = cover_alt(&post);
  let reading = post.reading_time;
  let lang = post.meta.lang;
  let title = display_title(&post.meta.title);

  view! {
    <A href=href>
        <article class="story-card">
            <div class="story-card-media">
                {cover.map(|src| view! {
                    <img src=src alt=alt.clone() loading="lazy" decoding="async"/>
                })}
            </div>
            <div class="story-card-body">
                <div class="story-card-kicker">
                    <LangBadge lang=lang/>
                    <span class="card-tag">{tag}</span>
                    <span class="dot">"·"</span>
                    <span>{format!("{reading} นาที")}</span>
                </div>
                <h3 class="story-card-title">{title}</h3>
                <p class="story-card-desc">{description}</p>
                <div class="story-card-foot">
                    <time>{date}</time>
                    <span class="card-more">"อ่านต่อ " <ArrowIcon/></span>
                </div>
            </div>
        </article>
    </A>
  }
}

#[cfg(test)]
mod tests {
  use super::{post_card_description, site};

  #[test]
  fn post_card_uses_site_description_when_post_description_is_empty() {
    assert_eq!(post_card_description("").as_str(), site::DESCRIPTION);
  }

  #[test]
  fn post_card_keeps_a_present_post_description() {
    assert_eq!(post_card_description("A post summary."), "A post summary.");
  }

  #[test]
  fn display_title_never_breaks_dashes_or_common_thai_names() {
    let joined = |phrase: &str| {
      phrase
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("\u{2060}")
    };
    let title = super::display_title("บทเรียน 1-2-3 จากมุราตะ");
    assert!(title.contains('\u{2011}'), "dash must be non-breaking");
    assert!(title.contains(&joined("บทเรียน")), "compound must be joined");
    assert!(title.contains(&joined("มุราตะ")), "name must be joined");
  }
}
