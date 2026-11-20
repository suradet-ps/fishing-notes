use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use wasm_bindgen::JsCast;
use web_sys::{Document, Storage, window};

thread_local! {
  static SCROLL_POSITIONS: RefCell<HashMap<String, f64>> = RefCell::new(HashMap::new());
  static PENDING_RESTORE: Cell<Option<f64>> = const { Cell::new(None) };
}

/// Remember the current page's scroll position for Back/Forward restoration.
pub fn save_scroll_position() {
  let Some(window) = window() else {
    return;
  };
  let path = window.location().pathname().unwrap_or_default();
  let y = window.scroll_y().unwrap_or(0.0);
  SCROLL_POSITIONS.with(|positions| {
    positions.borrow_mut().insert(path, y);
  });
}

/// Capture the target page's last position when a history traversal starts.
pub fn mark_history_navigation() {
  let Some(window) = window() else {
    return;
  };
  let path = window.location().pathname().unwrap_or_default();
  let y = SCROLL_POSITIONS.with(|positions| positions.borrow().get(&path).copied());
  PENDING_RESTORE.set(y);
}

/// Restore a pending history position, or start a new page at the top.
pub fn restore_or_top() {
  let Some(window) = window() else {
    return;
  };
  if let Some(y) = PENDING_RESTORE.take() {
    window.scroll_to_with_x_and_y(0.0, y);
  } else {
    window.scroll_to_with_x_and_y(0.0, 0.0);
  }
}

/// A fragment navigation already chose its target; drop any pending restore.
pub fn clear_pending_restore() {
  PENDING_RESTORE.set(None);
}

/// Fallback for a popstate that happens before the new view has mounted.
pub fn restore_after_history_navigation() {
  if let Some(y) = PENDING_RESTORE.take()
    && let Some(window) = window()
  {
    window.scroll_to_with_x_and_y(0.0, y);
  }
}

/// Let the app own scroll restoration so SPA views restore the position of
/// their own history entry instead of the browser guessing from the document.
pub fn disable_browser_scroll_restoration() {
  if let Some(window) = window()
    && let Ok(history) = window.history()
  {
    let _ = history.set_scroll_restoration(web_sys::ScrollRestoration::Manual);
  }
}

/// Progress through the article, from 0 at its top to 1 when its bottom
/// reaches the viewport bottom. Values outside that range are clamped.
pub fn reading_progress(
  scroll_y: f64,
  article_top: f64,
  article_bottom: f64,
  viewport: f64,
) -> f64 {
  let start = article_top;
  let end = (article_bottom - viewport).max(start);
  if end <= start {
    return if scroll_y >= start { 1.0 } else { 0.0 };
  }
  ((scroll_y - start) / (end - start)).clamp(0.0, 1.0)
}

/// Update the CSS variable consumed by the article reading-progress bar.
pub fn update_reading_progress() {
  let Some(window) = window() else {
    return;
  };
  let Some(document) = window.document() else {
    return;
  };
  let scroll_y = window.scroll_y().unwrap_or(0.0);
  let viewport = window
    .inner_height()
    .ok()
    .and_then(|value| value.as_f64())
    .unwrap_or(0.0);
  let progress = document
    .query_selector(".article")
    .ok()
    .flatten()
    .map(|article| {
      let rect = article.get_bounding_client_rect();
      reading_progress(
        scroll_y,
        rect.top() + scroll_y,
        rect.bottom() + scroll_y,
        viewport,
      )
    })
    .unwrap_or(0.0);
  if let Some(root) = document
    .document_element()
    .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok())
  {
    let _ = root
      .style()
      .set_property("--reading-progress", &format!("{progress:.4}"));
  }
}

/// Detect the router base path at runtime.
///
/// On GitHub Pages the app is served from `/<repo>/`, so the router needs a
/// matching base. Locally (and on any custom domain) the base is empty.
pub fn detect_base() -> String {
  let path = window()
    .and_then(|w| w.location().pathname().ok())
    .unwrap_or_default();
  if path.starts_with("/fishing-notes") {
    "/fishing-notes".to_string()
  } else {
    String::new()
  }
}

/// Prefix an app-internal path with the router base.
///
/// `leptos_router`'s `<A>` leaves absolute hrefs untouched, so links must
/// already include the base for GitHub Pages project sites. The click handler
/// also ignores anchors outside the base path, which otherwise causes a full
/// page load at the domain root and a 404.
pub fn href(path: &str) -> String {
  let base = detect_base();
  if base.is_empty() {
    path.to_string()
  } else {
    format!("{base}{path}")
  }
}

/// Prefix fingerprinted content-asset URLs in rendered post HTML with the
/// GitHub Pages base path.
///
/// Markdown images are rewritten at build time to `/assets/<hashed-name>`.
/// That works at the domain root, but a project site is served from
/// `/fishing-notes/`, so the browser would request `/assets/...` and get a 404.
/// Only rewrite signed asset URLs; external images and unrelated markup are
/// left untouched.
pub fn prefix_asset_urls(html: &str) -> String {
  let base = detect_base();
  if base.is_empty() {
    return html.to_string();
  }
  html.replace("src=\"/assets/", &format!("src=\"{base}/assets/"))
}

fn local_storage() -> Option<Storage> {
  window()?.local_storage().ok().flatten()
}

const THEME_KEY: &str = "fishing-notes:theme";

/// Read the persisted theme preference, falling back to the OS setting.
pub fn load_theme() -> Theme {
  if let Some(storage) = local_storage()
    && let Ok(Some(v)) = storage.get_item(THEME_KEY)
  {
    return match v.as_str() {
      "dark" => Theme::Dark,
      "light" => Theme::Light,
      _ => system_theme(),
    };
  }
  system_theme()
}

pub fn save_theme(theme: Theme) {
  if let Some(storage) = local_storage() {
    let _ = storage.set_item(THEME_KEY, theme.as_str());
  }
}

fn system_theme() -> Theme {
  let dark = window()
    .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok())
    .flatten()
    .map(|m| m.matches())
    .unwrap_or(false);
  if dark { Theme::Dark } else { Theme::Light }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
  Light,
  Dark,
}

impl Theme {
  pub fn as_str(&self) -> &'static str {
    match self {
      Theme::Light => "light",
      Theme::Dark => "dark",
    }
  }

  pub fn toggle(&self) -> Theme {
    match self {
      Theme::Light => Theme::Dark,
      Theme::Dark => Theme::Light,
    }
  }
}

/// Format an ISO `YYYY-MM-DD` date into a friendly Thai string (พ.ศ.).
pub fn format_date(iso: &str) -> String {
  let parts: Vec<&str> = iso.split('-').collect();
  if parts.len() != 3 {
    return iso.to_string();
  }
  let months = [
    "ม.ค.",
    "ก.พ.",
    "มี.ค.",
    "เม.ย.",
    "พ.ค.",
    "มิ.ย.",
    "ก.ค.",
    "ส.ค.",
    "ก.ย.",
    "ต.ค.",
    "พ.ย.",
    "ธ.ค.",
  ];
  let year: i32 = parts[0].parse().unwrap_or(0);
  let month: usize = parts[1].parse().unwrap_or(0);
  let day = parts[2].trim_start_matches('0');
  if (1..=12).contains(&month) && year > 0 {
    format!("{} {} {}", day, months[month - 1], year + 543)
  } else {
    iso.to_string()
  }
}

/// Apply the active theme to the document root element.
pub fn apply_theme(theme: Theme) {
  if let Some(doc) = window().and_then(|w| w.document()) {
    let _ = doc.document_element().map(|el| {
      let _ = el.set_attribute("data-theme", theme.as_str());
    });
  }
}

#[allow(dead_code)]
pub fn document() -> Option<Document> {
  window().and_then(|w| w.document())
}

#[cfg(test)]
mod tests {
  use super::reading_progress;

  #[test]
  fn progress_tracks_the_article_bounds() {
    assert_eq!(reading_progress(0.0, 100.0, 1000.0, 500.0), 0.0);
    assert_eq!(reading_progress(300.0, 100.0, 1000.0, 500.0), 0.5);
    assert_eq!(reading_progress(500.0, 100.0, 1000.0, 500.0), 1.0);
    assert_eq!(reading_progress(900.0, 100.0, 1000.0, 500.0), 1.0);
  }

  #[test]
  fn short_articles_report_complete_once_reached() {
    assert_eq!(reading_progress(0.0, 100.0, 300.0, 500.0), 0.0);
    assert_eq!(reading_progress(100.0, 100.0, 300.0, 500.0), 1.0);
  }
}
