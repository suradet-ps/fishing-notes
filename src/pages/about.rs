use leptos::prelude::*;
use leptos_meta::Title;

use fishing_notes::assets;
use fishing_notes::site;

fn photo(reference: &str) -> String {
  assets::resolve(reference)
    .map(crate::util::href)
    .unwrap_or_default()
}

/// About page: the editorial promise, the source method, and the tech notes.
#[component]
pub fn About() -> impl IntoView {
  let night = photo("assets/seabass-20h-night-bridge.jpg");
  let pier = photo("assets/eging-basics-pier.jpg");
  let hands = photo("assets/murata-spin-overhead.jpg");

  view! {
      <div class="about-page">
          <Title text=format!("เกี่ยวกับ · {}", site::TITLE)/>

          <section class="container about-hero">
              <p class="eyebrow">"ABOUT THE JOURNAL"</p>
              <h1 class="about-title">
                  <span class="hero-title-line">"อ่านน้ำให้เป็น"</span>
                  <span class="hero-title-line hero-title-accent">"แล้วปลาจะตามมา"</span>
              </h1>
              <p class="about-lede">
                  {site::TITLE}
                  " คือบันทึกการตกปลาเหยื่อปลอมที่สรุปจากคลิปของโปรญี่ปุ่น "
                  "ทั้งซีบาส แบส อีจิ้ง และเท็นยะ ตั้งแต่พื้นฐานอุปกรณ์ไปจนถึงเทคนิคหน้างาน"
              </p>
          </section>

          <section class="container about-collage" aria-label="ภาพจากคลิปต้นฉบับ">
              <figure class="about-photo about-photo-wide">
                  <img src=night alt="บรรยากาศตกปลาใต้สะพานตอนกลางคืน" loading="eager" decoding="async"/>
              </figure>
              <figure class="about-photo">
                  <img src=pier alt="ท่าเรือในคลิปอีจิ้ง" loading="lazy" decoding="async"/>
              </figure>
              <figure class="about-photo">
                  <img src=hands alt="มือและรอกในคลาสแคสต์ของมุราตะ" loading="lazy" decoding="async"/>
              </figure>
          </section>

          <section class="container about-layout">
              <aside class="about-side">
                  <p class="eyebrow">"ในบันทึกนี้"</p>
                  <ul class="about-index">
                      <li><span>"01"</span>" สรุปจากคลิปจริง"</li>
                      <li><span>"02"</span>" ภาพจากต้นฉบับ"</li>
                      <li><span>"03"</span>" ภาษาไทยอ่านง่าย"</li>
                      <li><span>"04"</span>" ใช้ได้หน้างาน"</li>
                  </ul>
              </aside>

              <div class="prose about-prose">
                  <h2>"วิธีอ่านบันทึกนี้"</h2>
                  <p>
                      "แต่ละบทความเริ่มจากสิ่งที่คลิปสอนจริง ไม่ใช่ทฤษฎีลอย ๆ "
                      "แล้วเรียบเรียงเป็นเรื่องเล่าที่พาไปตามลำดับความคิดของโปร "
                      "พร้อมภาพตัดจากคลิปเพื่อให้เห็นมือ จังหวะคัน และบรรยากาศของหมาย"
                  </p>
                  <p>
                      "ทุกบทความเป็นไฟล์ Markdown ธรรมดาใน "
                      <code>"content/posts/"</code>
                      " เขียนเพิ่มแล้วเว็บจะเก็บให้อัตโนมัติ ไม่มีฐานข้อมูล "
                      "และไม่ต้องแก้โค้ดต่อบทความ"
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
                      "ภาพประกอบทุกภาพตัดจากคลิปต้นฉบับของช่อง เพื่อใช้ประกอบคำอธิบายเท่านั้น "
                      "เครดิตและลิงก์ของแต่ละคลิปอยู่ในบทความนั้น ๆ"
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
                  <p>
                      "ซอร์สโค้ดของเว็บอยู่ที่ "
                      <a href=site::GITHUB_URL target="_blank" rel="noreferrer">
                          {site::GITHUB_URL}
                      </a>
                      "."
                  </p>
              </div>
          </section>
      </div>
  }
}
