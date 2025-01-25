use assets::cards::{Language, OverlordCard};
use base64::{engine::general_purpose::STANDARD as Base64Engine, Engine as _};
use futures::StreamExt;
use image::{DynamicImage, ImageFormat, ImageReader};
use leptos::{leptos_dom::logging, prelude::*};
use send_wrapper::SendWrapper;
use std::{collections::HashMap, io::Cursor};

fn convert_to_grey_image(image_buffer: &[u8]) -> DynamicImage {
    ImageReader::new(Cursor::new(image_buffer))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
}

fn convert_to_array_of_u8(input: &str) -> Vec<u8> {
    // Parse the input string as JSON
    match serde_json::from_str::<serde_json::Value>(input) {
        Ok(value) => {
            if let Some(content) = value["content"].as_array() {
                // Convert the JSON array into a Vec<u8>
                content
                    .iter()
                    .filter_map(|v| v.as_u64().map(|n| n as u8))
                    .collect()
            } else {
                vec![] // Return an empty Vec if "content" is not found
            }
        }
        Err(err) => {
            eprintln!("Failed to parse JSON: {:?}", err);
            vec![]
        }
    }
}

fn convert_to_image(json_input: &str) -> DynamicImage {
    convert_to_grey_image(&convert_to_array_of_u8(json_input))
}

fn encode_image_to_base64(image: DynamicImage) -> String {
    let mut buffer = Cursor::new(Vec::new());
    image
        .write_to(&mut buffer, ImageFormat::Png)
        .expect("Failed to write image to buffer");

    let base64 = Base64Engine.encode(buffer.into_inner());
    format!("data:image/png;base64,{}", base64)
}

fn convert_to_str(json_input: &str) -> String {
    let arr = convert_to_array_of_u8(json_input);
    //leptos::logging::log!("{:?}", arr);
    String::from_utf8(arr).expect("Invalid UTF-8 sequence")
}

#[component]
fn GameRoomImage(
    /// Image source
    #[prop(into)]
    src: String,
) -> impl IntoView {
    let game_room_image = {
        let mut source = SendWrapper::new(
            gloo_net::eventsource::futures::EventSource::new(src.as_str())
                .expect("couldn't connect to SSE stream"),
        );
        let signal = ReadSignal::from_stream_unsync(source.subscribe("message").unwrap().map(
            |subscription| {
                match subscription {
                    Ok(subscription) => encode_image_to_base64(convert_to_image(
                        subscription
                            .1
                            .data()
                            .as_string()
                            .expect("expected string value")
                            .as_str(),
                    )),
                    Err(_) => "0".to_string(),
                }
            },
        ));

        on_cleanup(move || source.take().close());
        signal
    };

    view! {
        <div>
            <img src={move || game_room_image.get().unwrap_or_default()} alt="Game room" />
        </div>
    }
}

#[component]
fn OptionalGameRoomImage(
    /// Image source
    #[prop(into)]
    src: String,
) -> impl IntoView {
    let (show_image, set_show_image) = signal(false);

    view! {
        <button
            on:click=move |_| {
                *set_show_image.write() = !show_image.get();
            }
        >
            "Bild anzeigen / verstecken"
        </button>
        <Show
            when=move || { show_image.get() }
            fallback=|| view! { <div></div> }
        >
            <GameRoomImage src=src.clone()/>
        </Show>
    }
}

#[component]
fn CardSelector(
    /// overlord cards
    #[prop(into)]
    keywords_to_ol_cards: HashMap<String, OverlordCard>,
) -> impl IntoView {
    let (value, set_value) = signal(Some("".to_string()));
    provide_context(value);
    view! {
      <select
        on:change:target=move |ev| {
          set_value.set(Some(ev.target().value()));
        }
        prop:value=move || value.get()
      >
      {keywords_to_ol_cards.iter().map(|(ol_keyword, card)| view! { cx,
                <option value={ol_keyword.clone()}>{card.name.clone()}</option>
            }).collect::<Vec<_>>()}
      </select>
      <OverlordCard keywords_to_ol_cards />
    }
}

#[component]
fn CardListener(
    /// overlord cards
    #[prop(into)]
    keywords_to_ol_cards: HashMap<String, OverlordCard>,
    /// detected cards source
    src: String,
) -> impl IntoView {
    let detected_overlord_card_string = {
        let mut source = SendWrapper::new(
            gloo_net::eventsource::futures::EventSource::new(src.as_str())
                .expect("couldn't connect to SSE stream"),
        );
        let signal = ReadSignal::from_stream_unsync(source.subscribe("message").unwrap().map(
            |subscription| {
                match subscription {
                    Ok(subscription) => convert_to_str(
                        subscription
                            .1
                            .data()
                            .as_string()
                            .unwrap_or("no card".to_string())
                            .as_str(),
                    ),
                    Err(_) => "".to_string(),
                }
            },
        ));

        on_cleanup(move || source.take().close());
        signal
    };

    provide_context(detected_overlord_card_string);

    view! {
        <OverlordCard keywords_to_ol_cards />
        <p>{move || detected_overlord_card_string.get().unwrap_or_default()}</p>
    }
}

#[component]
fn OverlordCard(
    /// overlord cards
    #[prop(into)]
    keywords_to_ol_cards: HashMap<String, OverlordCard>,
) -> impl IntoView {
    fn convert_to_overlord_card_keyword(maybe_ol_keyword: Option<String>) -> String {
        match maybe_ol_keyword {
            Some(keyword) => match keyword.strip_prefix("overlordcard/") {
                Some(value) => {
                    leptos::logging::log!("got {}", value);
                    value.to_string()
                }
                None => "".to_string(),
            },
            None => "".to_string(),
        }
    }

    let overlord_keyword = use_context::<ReadSignal<Option<String>>>()
        .expect("to have found the overlord card keyword signal provided");
    let overlord_card = move || {
        keywords_to_ol_cards
            .get(&convert_to_overlord_card_keyword(overlord_keyword.get()))
            .cloned()
    };
    view! {
        <div>
        { move || {
            if let Some(card) = overlord_card() {
                view! {
                    <div>
                        <h2>{card.translate_name(Language::De)}</h2>
                        <h3>Effekt</h3>
                        <p>{card.translate_effect(Language::De)}</p>
                        <h3>Overlord Taktik</h3>
                        <p>{card.translate_overlord_tactic(Language::De)}</p>
                        <h3>Heldentaktik</h3>
                        <p>{card.translate_heroes_tactic(Language::De)}</p>
                    </div>
                }
            }
            else {
                view! {
                    <div>
                        <h2>{Some("".to_string())}</h2>
                        <h3>Effekt</h3>
                        <p>{Some("".to_string())}</p>
                        <h3>Overlord Taktik</h3>
                        <p>{Some("".to_string())}</p>
                        <h3>Heldentaktik</h3>
                        <p>{Some("".to_string())}</p>
                    </div>
                }
            }
        }}
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let overlord_cards_json_file = include_bytes!("../../assets/overlord_cards.json");
    let keywords_to_ol_cards: HashMap<String, OverlordCard> =
        serde_json::from_slice(overlord_cards_json_file).expect("Invalid JSON");
    //leptos::logging::log!("{:?}", keywords_to_ol_cards);
    view! {
        <div>
            //<LogViewer url=String::from("http://0.0.0.0.:3030/Q_SHORT_LOG") />
            <OptionalGameRoomImage src=String::from("http://127.0.0.1:3030/Q_GAME_ROOM_FEED") />
            //<CardSelector keywords_to_ol_cards />
            <CardListener keywords_to_ol_cards src=String::from("http://127.0.0.1:3030/Q_DETECTED_OL_CARDS") />
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
}
