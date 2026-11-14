use leptos::prelude::*;
use leptos_meta::Title;

use crate::icons::FishMark;
use fishing_notes::site;

/// About page.
#[component]
pub fn About() -> impl IntoView {
  view! {
      <div class="container about-page">
          <Title text=format!("เกี่ยวกับ · {}", site::TITLE)/>
          <article class="article">
              <header class="article-header">
                  <p class="back-link">"เกี่ยวกับบล็อกนี้"</p>
                  <h1 class="article-title">"อ่านน้ำให้เป็น แล้วปลาจะตามมา"</h1>
              </header>
              <div class="prose">
                  <p>
                      <strong>{site::TITLE}</strong>
                      " คือบันทึกการตกปลาเหยื่อปลอมที่สรุปจากคลิปของโปรญี่ปุ่น "
                      "ทั้งซีบาส แบส อีจิ้ง และเท็นยะ ตั้งแต่พื้นฐานอุปกรณ์ไปจนถึงเทคนิคหน้างาน "
                      <FishMark/>
                  </p>
                  <p>
                      "ทุกบทความเป็นไฟล์ Markdown ธรรมดาใน "
                      <code>"content/posts/"</code>
                      " เขียนเพิ่มแล้วเว็บจะเก็บให้อัตโนมัติ ไม่มีฐานข้อมูล ไม่มีหน้าแอดมิน "
                      "และไม่ต้องแก้โค้ดต่อบทความ"
                  </p>
                  <h2>"ทำไมต้อง Rust + Leptos"</h2>
                  <p>
                      "เว็บนี้สร้างด้วย "
                      <strong>"Rust"</strong>
                      " และ "
                      <strong>"Leptos"</strong>
                      " (client-side rendering) แล้วคอมไพล์เป็น WebAssembly ผ่าน Trunk "
                      "ตัวเว็บจึงเบา เร็ว และไม่ต้องพึ่ง JavaScript จากภายนอก "
                      "ส่วนเนื้อหาถูกฝังตอน build ทำให้เปิดอ่านได้ทันที"
                  </p>
                  <h2>"แหล่งที่มา"</h2>
                  <p>
                      "แต่ละบทความอ้างอิงคลิปต้นฉบับไว้ในกล่องสรุปด้านบนและท้ายบทความ "
                      "โดยเก็บทั้งชื่อคลิป ช่อง และลิงก์ YouTube เพื่อให้กลับไปดูต้นฉบับได้ "
                      "บางช่วงที่คำบรรยายอัตโนมัติคลุมเครือจะถูกทำเครื่องหมายว่า "
                      <strong>{"เคล็ดลับเสริม"}</strong>
                      " เพื่อแยกความรู้ทั่วไปออกจากเนื้อหาที่อยู่ในคลิป"
                  </p>
                  <p>
                      "ซอร์สโค้ดของเว็บอยู่ที่ "
                      <a href=site::GITHUB_URL target="_blank" rel="noreferrer">
                          {site::GITHUB_URL}
                      </a>
                      "."
                  </p>
              </div>
          </article>
      </div>
  }
}
