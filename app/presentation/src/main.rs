use leptos::prelude::*;
/*use rand::Rng;
use thaw::*;*/

// thaw components at https://thaw-85fsrigp0-thaw.vercel.app/components/button

#[component]
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
}

/*#[component]
fn GameRoomImage(
    /// Image source
    #[prop(into)]
    src: String,
) -> impl IntoView {
    let (image_src, set_image_src) = create_signal(src.clone());

    let resource = create_resource(
        || (),
        |_| async {
            gloo::timers::future::TimeoutFuture::new(50).await;
        },
    );

    create_effect(move |prev| {
        resource.track();
        let mut rng = rand::thread_rng();
        if prev.is_some() {
            set_image_src.set([src.clone(), rng.gen::<u16>().to_string()].join("?"));
            resource.refetch();
        }
    });

    view! {
        <Image src=MaybeSignal::from(image_src) />
    }
}*/

#[component]
fn App() -> impl IntoView {
    let mut rng = rand::thread_rng();
    let (text, set_text) = create_signal(String::from("http://0.0.0.0:3030/Q_GAME_ROOM_FEED"));

    view! {
        <LogViewer url=String::from("http://0.0.0.0.:3030/Q_SHORT_LOG") />
        //<GameRoomImage src=String::from("http://0.0.0.0:3030/Q_GAME_ROOM_FEED") />
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
    mount_to_body(|| view! { <App /> });
}
