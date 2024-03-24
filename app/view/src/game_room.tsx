import { createSignal, createEffect } from "solid-js";

export function DisplayGameRoom() {
    const [latestImageBytes, setLatestImageBytes] = createSignal(new Uint8Array())

    setInterval(async () => {
        //const url = "https://images.pexels.com/photos/3361739/pexels-photo-3361739.jpeg?auto=compress&cs=tinysrgb&dpr=2&w=500";
        const url = "http://localhost:3030/descentinel/game_room_image";
        //const url = "http://localhost:3030/descentinel/log";
        fetch(url, { mode: 'cors' }).then(
            (response) => {
                const reader = response.body?.getReader();
                reader?.read().then(
                    (readResult) => {
                        const bytes = readResult.value;
                        //console.log("got ", bytes);
                        if (bytes) {
                            setLatestImageBytes(bytes);
                        }
                    }
                )
            }
        );
    }, 4000);

    const convertUint8BytesToBase64 = () => {
        let recovered_array = latestImageBytes().reduce((data, byte) => data + String.fromCharCode(byte), '').toString().slice(1, -1).split(",").map((intAsString) => parseInt(intAsString));
        console.log(recovered_array);
        return btoa(recovered_array.reduce((data, byte) => data + String.fromCharCode(byte), '').toString());
        //return btoa(String.fromCharCode(...new Uint8Array(latestImageBytes())))
        //console.log(bin2string(latestImageBytes()));
        //return btoa(bin2string(latestImageBytes()))
        return btoa(latestImageBytes().reduce((data, byte) => data + String.fromCharCode(byte), '').toString());
    };
    const convertToImage = () => {
        //console.log(latestImageBytes());
        /*const blob = new Blob([latestImageBytes()],{ type: 'image/png' });
        console.log(latestImageBytes());
        const content = new Uint8Array([137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 5, 0, 0, 0, 5, 8, 6, 0, 0, 0, 141, 111, 38, 229, 0, 0, 0, 28, 73, 68, 65, 84, 8, 215, 99, 248, 255, 255, 63, 195, 127, 6, 32, 5, 195, 32, 18, 132, 208, 49, 241, 130, 88, 205, 4, 0, 14, 245, 53, 203, 209, 142, 14, 31, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130]);
        return URL.createObjectURL(new Blob([content], { type: 'image/png' }));
        return URL.createObjectURL(blob);*/
 
        return "data:image/jpeg;base64,".concat(convertUint8BytesToBase64());
    };

    return <div><img src={convertToImage()} /></div>
};

// https://www.solidjs.com/tutorial/introduction_derived

