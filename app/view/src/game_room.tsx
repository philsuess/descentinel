import { createSignal, createEffect } from "solid-js";

export function DisplayGameRoom() {
    const [latestImageBytes, setLatestImageBytes] = createSignal(new Uint8Array())

    setInterval(async () => {
        //const url = "https://images.pexels.com/photos/3361739/pexels-photo-3361739.jpeg?auto=compress&cs=tinysrgb&dpr=2&w=500";
        const url = "http://localhost:3030/descentinel/game_room_image";
        fetch(url, { mode: 'cors' }).then(
            (response) => {
                const reader = response.body?.getReader();
                reader?.read().then(
                    (readResult) => {
                        const bytes = readResult.value;
                        console.log("got ", bytes);
                        if (bytes) {
                            setLatestImageBytes(bytes);
                        }
                    }
                )
            }
        );
    }, 4000);

    const convertUint8BytesToBase64 = () => {
        return btoa(
            latestImageBytes().reduce((data, byte) => data + String.fromCharCode(byte), '')
        );
    };
    const convertToImage = () => {
        return "data:image/png;base64,".concat(convertUint8BytesToBase64());
    };

    return <div><img src={convertToImage()} /></div>
};

// https://www.solidjs.com/tutorial/introduction_derived

