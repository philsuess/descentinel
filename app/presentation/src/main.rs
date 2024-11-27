use leptos::*;
use rand::Rng;
use thaw::*;

// thaw components at https://thaw-85fsrigp0-thaw.vercel.app/components/button

#[component]
fn GameRoomImage(
    /// Image source
    #[prop(into)]
    src: ReadSignal<String>,
) -> impl IntoView {
    view! {
        <Image src=src />
    }
}

#[component]
fn App() -> impl IntoView {
    let mut rng = rand::thread_rng();
    let (text, set_text) = create_signal(String::from("http://0.0.0.0:3030/Q_GAME_ROOM_FEED"));

    view! {
        <button
            on:click=move |_| {
                set_text.set([String::from("http://0.0.0.0:3030/Q_GAME_ROOM_FEED"),rng.gen::<u8>().to_string()].join("?"));
            }
        >
            "Refresh image"
        </button>
        <GameRoomImage src=text />
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
}
