use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

use fishing_notes::site;

/// 404 / post-not-found fallback.
#[component]
pub fn NotFound() -> impl IntoView {
  view! {
      <div class="container notfound">
          <Title text=format!("ไม่พบหน้า · {}", site::TITLE)/>
          <div class="notfound-inner">
              <p class="notfound-code">"404"</p>
              <h1 class="notfound-title">"ไม่พบหน้าที่ต้องการ"</h1>
              <p class="notfound-sub">
                  "บทความอาจถูกย้าย หรือลิงก์อาจไม่ถูกต้อง"
              </p>
              <A href="/">
                  <span class="btn-primary">"← กลับไปหน้าบทความ"</span>
              </A>
          </div>
      </div>
  }
}
