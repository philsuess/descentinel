import { createSignal, createResource } from "solid-js";
import { decode } from "msgpack-lite";
import { OverlordCard } from "./components/overlord_card";

const stringedVectorToUint8 = (stringedVector: any) => {
    if (typeof stringedVector === 'string' || stringedVector instanceof String) {
        console.log("vec: ", stringedVector);
        const arrayWithoutBrackets = stringedVector.split("[")[1].split("]")[0];
        return new Uint8Array(arrayWithoutBrackets.split(",").map(Number));
    }

    return new Uint8Array([0]);
};

const fetchCardAsBytes = async () =>
    await (await fetch("http://raspberrypi.local:3030/descentinel/detected_ol_card", { mode: 'cors' })).text();

export function DisplayLatestOverlordCard() {
    const [latestCard, setLatestCard] = createSignal("");
    const [cardId] = createResource(latestCard, fetchCardAsBytes);

    setInterval(() => setLatestCard(Date.now().toString()), 1000);

    const convertUint8BytesToString = () => {
        console.log("array: ", latestCard());
        return decode(stringedVectorToUint8(cardId()));
    };

    return <OverlordCard overlord_card_id={convertUint8BytesToString()} />
};
