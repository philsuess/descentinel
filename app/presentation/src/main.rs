use base64::{engine::general_purpose::STANDARD as Base64Engine, Engine as _};
use futures::StreamExt;
use image::{DynamicImage, ImageFormat, ImageReader};
use leptos::{prelude::*, tachys::view};
use presentation::{read_overlord_cards_from_file, OverlordCard};
use send_wrapper::SendWrapper;
use serde::Deserialize;
use std::{collections::HashMap, hash::Hash, io::Cursor};

// thaw components at https://thaw-85fsrigp0-thaw.vercel.app/components/button

/*#[component]
fn LogViewer(
    /// Log source
    #[prop(into)]
    url: String,
) -> impl IntoView {
    let cumulated_log = String::from("");
    let (log_message, set_log_message) = signal(String::from("no log messages yet"));

    let resource = Resource::new(
        || (),
        move |_| {
            let url = url.clone(); // Clone the URL for use inside the async block
            async move {
                // Perform a GET request to the provided URL
                match reqwest::get(&url).await {
                    Ok(response) => {
                        if response.status().is_success() {
                            response
                                .text()
                                .await
                                .unwrap_or_else(|_| "Failed to read response".into())
                        } else {
                            format!("Request failed with status: {}", response.status())
                        }
                    }
                    Err(err) => format!("Failed to make request: {}", err),
                }
            }
        },
    );

    Effect::new(move |_| {
        if let Some(latest_log) = resource.get() {
            let cumulated_log = [cumulated_log.clone(), latest_log.clone()].join("\n");
            set_log_message.set(cumulated_log.clone());
            resource.refetch();
        }
    });

    view! {<p>{move || log_message.get()}</p>}
}*/

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
fn CardSelector(
    /// overlord cards
    #[prop(into)]
    keywords_to_ol_cards: HashMap<String, OverlordCard>,
) -> impl IntoView {
    let kw_to_ol_cards_clone = keywords_to_ol_cards.clone();
    let (value, set_value) = signal("doom".to_string());
    let overlord_card = move || kw_to_ol_cards_clone[value.get().as_str()].clone();
    provide_context(value);
    view! {
      <select
        on:change:target=move |ev| {
          set_value.set(ev.target().value());
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
fn OverlordCard(
    /// overlord cards
    #[prop(into)]
    keywords_to_ol_cards: HashMap<String, OverlordCard>,
) -> impl IntoView {
    let overlord_keyword = use_context::<ReadSignal<String>>()
        .expect("to have found the overlord card keyword signal provided");
    let overlord_card = move || keywords_to_ol_cards.get(&overlord_keyword.get()).cloned();
    view! {
        <div>
        { move || {
            if let Some(card) = overlord_card() {
                view! {
                    <div>
                        <h2>{card.name}</h2>
                        <p>{card.effect}</p>
                    </div>
                }
            }
            else {
                view! {
                    <div>
                        <h2>{"card not found".to_string()}</h2>
                        <p>{"".to_string()}</p>
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
    let v: HashMap<String, OverlordCard> =
        serde_json::from_slice(overlord_cards_json_file).expect("Invalid JSON");
    leptos::logging::log!("{:?}", v);
    view! {
        //<LogViewer url=String::from("http://0.0.0.0.:3030/Q_SHORT_LOG") />
        //<GameRoomImage src=String::from("http://127.0.0.1:3030/Q_GAME_ROOM_FEED") />
        <CardSelector keywords_to_ol_cards=v/>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
}
