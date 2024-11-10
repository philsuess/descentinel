import { createSignal, createResource } from "solid-js";
import { decode } from "msgpack-lite";

const stringedVectorToUint8 = (stringedVector: any) => {
    if (typeof stringedVector === 'string' || stringedVector instanceof String) {
        const arrayWithoutBrackets = stringedVector.split("[")[1].split("]")[0];
        return new Uint8Array(arrayWithoutBrackets.split(",").map(Number));
    }

    return new Uint8Array([0]);
};

const fetchImageAsBase64String = async () =>
    //await (await fetch("http://raspberrypi.local:3030/descentinel/game_room_image", { mode: 'cors' })).text();
    await (await fetch("http://localhost:3031/descentinel/Q_GAME_ROOM_FEED", { mode: 'cors' })).json();

export function DisplayLatestGameRoomImage() {
    const [imageAsBase64String, setImageAsBase64String] = createSignal("");
    const [imageAsBase64] = createResource(imageAsBase64String, fetchImageAsBase64String);

    setInterval(() => setImageAsBase64String(Date.now().toString()), 500);

    const convertUint8BytesToString = () => {
        //return decode(stringedVectorToUint8(imageAsBase64()));
        return imageAsBase64();
    };

    return <div><img src={"data:image/jpeg;base64,".concat(convertUint8BytesToString())} /></div>
};
