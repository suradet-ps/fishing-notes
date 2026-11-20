mod components;
mod icons;
mod pages;
mod util;

use fishing_notes::content::load_posts;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use std::borrow::Cow;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

use components::{Footer, Nav, ThemeProvider};
use pages::{about::About, home::Home, not_found::NotFound, post::Post};
use util::detect_base;

pub fn main() {
  console_error_panic_hook::set_once();
  mount_to_body(App);
}

/// Root component: meta context, theme provider, router and layout.
#[component]
fn App() -> impl IntoView {
  provide_meta_context();

  let posts = load_posts();
  provide_context(posts);

  Effect::new(move |_| {
    let window = web_sys::window().expect("no window");
    util::disable_browser_scroll_restoration();
    util::update_reading_progress();
    util::save_scroll_position();

    let update = Closure::wrap(Box::new(|| {
      util::update_reading_progress();
      util::save_scroll_position();
    }) as Box<dyn FnMut()>);
    let _ = window.add_event_listener_with_callback("scroll", update.as_ref().unchecked_ref());
    let _ = window.add_event_listener_with_callback("resize", update.as_ref().unchecked_ref());
    update.forget();

    let window_for_popstate = window.clone();
    let popstate = Closure::wrap(Box::new(move |_e: web_sys::Event| {
      util::mark_history_navigation();
      let window = window_for_popstate.clone();
      let restore = Closure::once_into_js(util::restore_after_history_navigation);
      let _ =
        window.set_timeout_with_callback_and_timeout_and_arguments_0(restore.unchecked_ref(), 150);
    }) as Box<dyn FnMut(web_sys::Event)>);
    let _ = window.add_event_listener_with_callback("popstate", popstate.as_ref().unchecked_ref());
    popstate.forget();
  });

  let base = detect_base();

  view! {
      <ThemeProvider>
          <Router base=Cow::Owned(base)>
              <Nav/>
              <main id="main" class="main">
                  <Routes fallback=|| view! { <NotFound/> }>
                      <Route path=path!("/") view=Home/>
                      <Route path=path!("/about") view=About/>
                      <Route path=path!("/post/:slug") view=Post/>
                  </Routes>
              </main>
              <Footer/>
          </Router>
      </ThemeProvider>
  }
}
