import { Show } from 'solid-js';
import _cards from '../assets/overlord_cards.json';

type Trap = "Trap" | "Trap (Chest)" | "Trap (Door)";
type CardType = Trap | "Event" | "Power" | "Spawn";
type TreacheryColor = "Purple";
type Language = "de";

interface OverlordCardTranslation {
    language: Language;
    name: string;
    effect: string;
    Overlord_tactic?: string;
    Heroes_tactic?: string;
}

type Translations = {
    [index: number]: OverlordCardTranslation;
}

interface OverlordCard {
    name: string;
    type: CardType;
    threat_cost: string | number;
    discard_value: number;
    treachery_cost?: number | string;
    treachery_colour?: string;
    effect: string;
    Overlord_tactic?: string;
    Heroes_tactic?: string;
    translations: Translations;
}

type OverlordMap = {
    [ol_key: string]: OverlordCard;
}

const cards = _cards as OverlordMap;

interface OverlordCardProps {
    overlord_card_id: string,
}

export function OverlordCard(props: OverlordCardProps) {
    console.log("looking up card ", props.overlord_card_id);
    const card = cards[props.overlord_card_id];
    console.log(card)
    if (card === undefined) {
        return <div>Card {props.overlord_card_id} not found</div>
    }
    const german = card.translations[0];

    return <div class="ol-card">
        <p class="ol-name">{german.name}</p>
        <p class="ol-threat-cost">Kosten: {card.threat_cost}</p>
        <p class="ol-discard-value">Abwurfwert: {card.discard_value}</p>
        <p class="ol-effect">Effekt: {german.effect}</p>
        <Show
            when={german.Overlord_tactic !== undefined}>
            <p class="ol-ol-tactics">Overlord Taktik: {german.Overlord_tactic}</p>
        </Show>
        <Show
            when={german.Heroes_tactic !== undefined}>
            <p class="ol-ol-tactics">Helden Taktik: {german.Heroes_tactic}</p>
        </Show>
    </div>
};

