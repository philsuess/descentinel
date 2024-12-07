use leptos::*;
use rand::Rng;
use thaw::*;

// thaw components at https://thaw-85fsrigp0-thaw.vercel.app/components/button

#[component]
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
}

#[component]
fn App() -> impl IntoView {
    let game_room_feed_url = String::from("http://0.0.0.0:3030/Q_GAME_ROOM_FEED");

    view! {
        //<GameRoomImage src=game_room_feed_url />
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
}
