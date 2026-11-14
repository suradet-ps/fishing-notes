use web_sys::{Document, Storage, window};

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
