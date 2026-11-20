//! Site-wide constants, shared by the UI (`fishing_notes::site`), the RSS build
//! and the sitemap/robots generation (`build.rs` includes this file via
//! `#[path]`). One definition, no drift.

pub const TITLE: &str = "Fishing Notes";
pub const TAGLINE: &str = "บันทึกการตกปลาเหยื่อปลอมจากคลิปญี่ปุ่น";
pub const DESCRIPTION: &str = "สรุปเทคนิคตกปลาเหยื่อปลอม ซีบาส แบส อีจิ้ง และเท็นยะ จากคลิปโปรญี่ปุ่น";
pub const AUTHOR: &str = "suradet-ps";
pub const GITHUB_URL: &str = "https://github.com/suradet-ps/fishing-notes";
pub const SITE_URL: &str = "https://suradet-ps.github.io/fishing-notes";

/// The absolute URL to put in `og:image` / `twitter:image` for a cover.
///
/// Inline data URLs are not useful to crawlers, so they fall back to the
/// site-wide card. Protocol-relative URLs are normalized to HTTPS. Local
/// fingerprinted `/assets/...` paths get the deployment prefix.
pub fn share_image_url(src: &str) -> Option<String> {
  if src.starts_with("data:") {
    None
  } else if src.starts_with("http") {
    Some(src.to_string())
  } else if let Some(rest) = src.strip_prefix("//") {
    Some(format!("https://{rest}"))
  } else {
    Some(format!("{SITE_URL}{src}"))
  }
}

#[cfg(test)]
mod tests {
  use super::{SITE_URL, share_image_url};

  #[test]
  fn share_image_skips_inline_data_urls() {
    assert_eq!(share_image_url("data:image/png;base64,AAAA"), None);
  }

  #[test]
  fn share_image_keeps_absolute_and_normalizes_protocol_relative() {
    assert_eq!(
      share_image_url("https://cdn.example.org/cover.jpg").as_deref(),
      Some("https://cdn.example.org/cover.jpg")
    );
    assert_eq!(
      share_image_url("//cdn.example.org/cover.jpg").as_deref(),
      Some("https://cdn.example.org/cover.jpg")
    );
  }

  #[test]
  fn share_image_prefixes_local_assets() {
    assert_eq!(
      share_image_url("/assets/cover.jpg"),
      Some(format!("{SITE_URL}/assets/cover.jpg"))
    );
  }
}
