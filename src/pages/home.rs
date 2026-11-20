use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::{FeatureStory, PostCard, TagChip};
use crate::icons::{ArrowIcon, FishMark, SearchIcon};
use fishing_notes::content::Post;
use fishing_notes::site;

/// Topics that make a useful entry point into the library. Tags outside this
/// list remain searchable; the topic bar stays short on purpose.
const TOPIC_TAGS: [&str; 10] = [
  "ซีบาส",
  "แบส",
  "อีจิ้ง",
  "การตีเหยื่อ",
  "ชายฝั่ง",
  "ตกปลาทะเล",
  "อุปกรณ์",
  "มือใหม่",
  "กระดี่เหล็ก",
  "สปูน",
];

fn topic_counts(posts: &[Post]) -> Vec<(String, usize)> {
  TOPIC_TAGS
    .iter()
    .filter_map(|topic| {
      let count = posts
        .iter()
        .filter(|p| p.meta.tags.iter().any(|t| t == topic))
        .count();
      (count > 0).then(|| (topic.to_string(), count))
    })
    .collect()
}

/// Search text copied from a rendered card can contain display-only word
/// joiners and non-breaking hyphens. Normalize both sides of the comparison.
fn normalize_search(text: &str) -> String {
  text
    .replace('\u{2011}', "-")
    .replace('\u{2060}', "")
    .to_lowercase()
}

/// Home page: a magazine cover story, a quiet topic index, and the library.
#[component]
pub fn Home() -> impl IntoView {
  let posts = expect_context::<Vec<Post>>();
  let total = posts.len();
  let featured = posts.first().cloned();
  let topics = topic_counts(&posts);
  let topic_count = topics.len();

  let (query, set_query) = signal(String::new());
  let (active_tag, set_active_tag) = signal(None::<String>);
  let search_ref = NodeRef::<leptos::html::Input>::new();

  let browsing = Memo::new(move |_| query.get().trim().is_empty() && active_tag.get().is_none());

  let filtered = Memo::new({
    let posts = posts.clone();
    move |_| {
      let q = normalize_search(query.get().trim());
      let tag = active_tag.get();
      let all = posts
        .iter()
        .filter(|p| {
          let matches_tag = tag
            .as_ref()
            .map(|t| p.meta.tags.iter().any(|x| x == t))
            .unwrap_or(true);
          let matches_q = q.is_empty()
            || normalize_search(&p.meta.title).contains(&q)
            || normalize_search(&p.meta.description).contains(&q)
            || p.meta.tags.iter().any(|t| normalize_search(t).contains(&q));
          matches_tag && matches_q
        })
        .cloned()
        .collect::<Vec<_>>();

      // The lead story is already on the page; the grid starts after it while
      // the reader is browsing. Search/filter results show every match.
      if browsing.get() {
        all.into_iter().skip(1).collect()
      } else {
        all
      }
    }
  });

  let filtered_grid = filtered;
  let filtered_count = filtered;
  let filtered_show = filtered;

  view! {
      <div class="home">
          <Title text=format!("{} · บันทึกการตกปลา", site::TITLE)/>

          <Show when=move || browsing.get()>
              <section class="home-hero">
                  <div class="container home-hero-inner">
                      <div class="hero-copy">
                          <p class="eyebrow">"FIELD NOTES · FROM JAPAN"</p>
                          <h1 class="hero-title">
                              <span class="hero-title-line">"อ่านน้ำให้เป็น"</span>
                              <span class="hero-title-line hero-title-accent">"แล้วปลาจะตามมา"</span>
                          </h1>
                          <p class="hero-lede">{site::DESCRIPTION}</p>
                          <p class="hero-lede hero-lede-quiet">
                              "ทุกบทความสรุปจากคลิปของโปรญี่ปุ่น พร้อมภาพจริงจากต้นฉบับ "
                              "ให้เห็นจังหวะมือ จังหวะคัน และบรรยากาศหน้างาน"
                          </p>
                          <div class="hero-actions">
                              <a class="btn-primary" href="#library" rel="external">
                                  "เริ่มอ่านบทความ " <ArrowIcon/>
                              </a>
                              <span class="hero-note">"ภาพประกอบจากคลิปต้นฉบับ"</span>
                          </div>
                          <div class="hero-facts">
                              <div class="hero-fact">
                                  <strong>{total}</strong>
                                  <span>"บทความ"</span>
                              </div>
                              <div class="hero-fact">
                                  <strong>{topic_count}</strong>
                                  <span>"หัวข้อให้สำรวจ"</span>
                              </div>
                              <div class="hero-fact hero-fact-mark" aria-hidden="true">
                                  <FishMark/>
                              </div>
                          </div>
                      </div>
                      {featured.clone().map(|post| view! { <FeatureStory post=post/> })}
                  </div>
              </section>
          </Show>

          <section class="section library" id="library" tabindex="-1">
              <div class="container">
                  <header class="section-head library-head">
                      <div>
                          <p class="eyebrow">"THE LIBRARY"</p>
                          <h2 class="section-title">"คลังบทความ"</h2>
                          <p class="section-sub">"เลือกอ่านตามหัวข้อ หรือค้นหาสิ่งที่กำลังสนใจ"</p>
                      </div>
                      <div class="search-box">
                          <span class="search-icon"><SearchIcon/></span>
                          <input
                              type="search"
                              class="search-input"
                              aria-label="ค้นหาบทความ"
                              placeholder="ค้นหา เช่น ซีบาส, อีจิ้ง, การตีเหยื่อ…"
                              node_ref=search_ref
                              prop:value=move || query.get()
                              on:input=move |ev| set_query.set(event_target_value(&ev))
                          />
                      </div>
                  </header>

                  <div class="topic-bar">
                      {move || {
                          let active_tag = active_tag;
                          let set_active_tag = set_active_tag;
                          let mut chips = vec![
                              view! {
                                  <TagChip
                                      tag="ทั้งหมด".to_string()
                                      active=active_tag.get().is_none()
                                      on_click=Callback::new({
                                          let set = set_active_tag;
                                          move |_| set.set(None)
                                      })
                                  />
                              },
                          ];
                          for (topic, count) in &topics {
                              let t2 = topic.clone();
                              let set = set_active_tag;
                              chips.push(
                                  view! {
                                      <TagChip
                                          tag=t2.clone()
                                          count=*count
                                          active=active_tag.get().as_ref() == Some(&t2)
                                          on_click=Callback::new(move |_| set.set(Some(t2.clone())))
                                      />
                                  },
                              );
                          }
                          chips
                      }}
                  </div>

                  <div class="library-meta">
                      <p class="result-count" aria-live="polite">
                          {move || {
                              let n = filtered_count.get().len();
                              if browsing.get() {
                                  format!("{total} บทความ · ภาพจากคลิปต้นฉบับ")
                              } else {
                                  format!("พบ {n} บทความ")
                              }
                          }}
                      </p>
                      <Show when=move || !browsing.get()>
                          <button
                              class="clear-filter"
                              type="button"
                              on:click=move |_| {
                                  set_query.set(String::new());
                                  set_active_tag.set(None);
                                  if let Some(input) = search_ref.get() {
                                      let _ = input.focus();
                                  }
                              }
                          >
                              "ล้างตัวกรอง"
                          </button>
                      </Show>
                  </div>

                  <div class="story-grid">
                      {move || {
                          filtered_grid
                              .get()
                              .into_iter()
                              .map(|p| view! { <PostCard post=p/> })
                              .collect::<Vec<_>>()
                      }}
                  </div>

                  <Show when=move || {
                      if browsing.get() {
                          total == 0
                      } else {
                          filtered_show.get().is_empty()
                      }
                  }>
                      <div class="empty-state">
                          <span class="empty-mark"><FishMark/></span>
                          <p class="empty-title">"ยังไม่พบเรื่องที่ตรงกับคำค้น"</p>
                          <p class="empty-sub">"ลองใช้คำสั้นลง หรือเลือกหัวข้อด้านบน"</p>
                      </div>
                  </Show>
              </div>
          </section>
      </div>
  }
}

#[cfg(test)]
mod tests {
  use super::normalize_search;

  #[test]
  fn search_normalizes_display_only_characters() {
    assert_eq!(
      normalize_search("บท\u{2060}เรียน 1\u{2011}2\u{2011}3"),
      "บทเรียน 1-2-3"
    );
  }
}
