use leptos::*;
use thaw::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Image src="http://0.0.0.0:3030/Q_GAME_ROOM_FEED"/>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
}
