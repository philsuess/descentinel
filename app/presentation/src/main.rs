use leptos::*;
use reqwest::Client;
use wasm_bindgen::JsCast;
use web_sys::{Blob, Url};

#[component]
fn image_display(cx: Scope) -> impl IntoView {
    let (image_url, set_image_url) = create_signal(cx, None::<String>);

    // Fetch image on mount
    create_effect(cx, move |_| {
        spawn_local(async move {
            if let Ok(image_bytes) = fetch_image_bytes().await {
                // Convert byte array into Blob and then to an Object URL
                let array = js_sys::Uint8Array::from(image_bytes.as_slice());
                let blob = Blob::new_with_u8_array_sequence(&array).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();

                // Set the image URL in the signal
                set_image_url(Some(url));

                // Clean up the Object URL when it's no longer needed
                on_cleanup(cx, move || {
                    Url::revoke_object_url(&url).unwrap();
                });
            }
        });
    });

    view! { cx,
        <div>
            {move || {
                if let Some(url) = image_url.get() {
                    view! { cx, <img src=url alt="Fetched from server" /> }
                } else {
                    view! { cx, <p>"Loading image..."</p> }
                }
            }}
        </div>
    }
}

// Fetches the image bytes from the server as Vec<u8>
async fn fetch_image_bytes() -> Result<Vec<u8>, reqwest::Error> {
    let client = Client::new();
    let response = client.get("http://127.0.0.1:3030/image").send().await?;

    response.json().await
}

fn main() {
    mount_to_body(|cx| view! { cx, <ImageDisplay /> });
}
