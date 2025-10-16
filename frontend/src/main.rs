use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use urlencoding::encode;
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Track {
    id: String,
    title: String,
    artist: String,
    #[serde(default)]
    primary_genre: String,
    #[serde(default)]
    year: Option<i32>,
   
}

// dev: call backend on 127.0.0.1:8080; prod: same-origin
fn api_base() -> String {
    // Replace with your Cloud Run URL after deployment
    let window = web_sys::window().expect("window");
    let loc = window.location();
    let host = loc.host().unwrap_or_default();
    
    if host.contains("localhost") || host.contains("127.0.0.1") {
        "http://127.0.0.1:8081".to_string()
    } else {
        // YOUR CLOUD RUN URL HERE
        "https://spotify-api-885145268827.us-central1.run.app".to_string()
    }
}
//fn api_base() -> String {
  //  let window = web_sys::window().expect("window");
    //let loc = window.location();
    //let host = loc.host().unwrap_or_default();
    //if host.contains("localhost") || host.contains("127.0.0.1") {
      //  "http://127.0.0.1:8080".to_string()
   // } else {
     //   loc.origin().unwrap_or_else(|_| String::from(""))
   // }
//}

// cute music-note icon for list items
fn note_icon() -> Html {
    html! {
      <svg class="note" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
        <path d="M19 3v11.55A4 4 0 1 1 17 18V8.28l-8 2.13v6.14A4 4 0 1 1 7 18V6l12-3z"/>
      </svg>
    }
}

#[function_component(App)]
fn app() -> Html {
    // single search box like your mock
    let query   = use_state(|| String::new());
    let results = use_state(|| Vec::<Track>::new());
    let loading = use_state(|| false);
    let error   = use_state(|| Option::<String>::None);

    // input handler
    let on_query = {
        let query = query.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            query.set(input.value());
        })
    };

    // fetch handler
    let on_fetch = {
        let query   = query.clone();
        let results = results.clone();
        let loading = loading.clone();
        let error   = error.clone();

        Callback::from(move |_| {
            let q = (*query).clone();

            let results_h = results.clone();
            let loading_h = loading.clone();
            let error_h   = error.clone();

            loading_h.set(true);
            error_h.set(None);
            results_h.set(vec![]);

            spawn_local(async move {
                let base = api_base();
                // send q as both artist and genre; backend will score it
                let url = format!(
                    "{}/recommendations?artist={}&genre={}&limit={}",
                    base, encode(&q), encode(&q), 20u32
                );

                match Request::get(&url).send().await {
                    Ok(r) if r.ok() => match r.json::<Vec<Track>>().await {
                        Ok(list) => { results_h.set(list); loading_h.set(false); }
                        Err(e)   => { error_h.set(Some(format!("Failed to parse response: {e:?}"))); loading_h.set(false); }
                    },
                    Ok(r) => { error_h.set(Some(format!("HTTP {}", r.status()))); loading_h.set(false); }
                    Err(e) => { error_h.set(Some(format!("Network error: {e:?}"))); loading_h.set(false); }
                }
            });
        })
    };

    html! {
      <div class="wrap">
        <h1 class="title">{ "Song" }<br/>{ "Recommendations" }</h1>

        <div class="search">
          <input
            class="search-input"
            type="text"
            placeholder="Favorite artist or genre"
            value={(*query).clone()}
            oninput={on_query}
          />
          <button class="search-btn" onclick={on_fetch}>{ "Search" }</button>
        </div>

        if *loading {
          <p>{ "Fetching" }<span class="loading"></span></p>
        }
        if let Some(msg) = &*error {
          <p class="error">{ format!("Error: {msg}") }</p>
        }

        if !results.is_empty() {
          <h2 class="section">{ "Recommended songs" }</h2>
        }

        <ul class="list">
          { for results.iter().map(|t| html!{
            <li class="card">
              <div class="icon">{ note_icon() }</div>
              <div class="item">
                <div class="song">{ &t.title }</div>
                <div class="artist">{ &t.artist }</div>
              </div>
            </li>
          })}
        </ul>
      </div>
    }
}

fn main() {
    #[cfg(debug_assertions)]
    {
        wasm_logger::init(wasm_logger::Config::default());
        log::info!("Rustify frontend starting…");
    }
    yew::Renderer::<App>::new().render();
}
