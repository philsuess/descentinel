use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct OverlordCard {
    pub name: String,
    pub card_type: OverlordCardType,
    pub threat_cost: u32,
    pub discard_value: u32,
    pub treachery_cost: Option<u32>,
    pub treachery_colour: Option<TreacheryColour>,
    pub effect: String,
    pub overlord_tactic: Option<String>,
    pub heroes_tactic: Option<String>,
    pub translations: Vec<OverlordCardTranslation>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Trap {
    Generic,
    #[serde(alias = "Trap (Space)")]
    Space,
    #[serde(alias = "Trap (Chest)")]
    Chest,
    #[serde(alias = "Trap (Door)")]
    Door,
    #[serde(alias = "Trap (Treasure)")]
    Treasure,
}

#[derive(Debug, Clone)]
pub enum OverlordCardType {
    Event,
    Power,
    Spawn,
    Trap(Trap),
}

#[derive(Deserialize, Debug, Clone)]
pub enum TreacheryColour {
    #[serde(alias = "purple")]
    Purple,
    #[serde(alias = "green")]
    Green,
    #[serde(alias = "red")]
    Red,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Language {
    #[serde(alias = "de")]
    De,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OverlordCardTranslation {
    pub language: Language,
    pub name: String,
    pub effect: String,
    #[serde(alias = "Overlord_tactic")]
    pub overlord_tactic: Option<String>,
    #[serde(alias = "Heroes_tactic")]
    pub heroes_tactic: Option<String>,
}

impl OverlordCard {
    pub fn translate(&self, language: Language) -> Option<OverlordCardTranslation> {
        let _ = language;
        self.translations
            .iter()
            .find(|&translation| matches!(&translation.language, language))
            .cloned()
    }

    pub fn translate_name(&self, language: Language) -> Option<String> {
        let translation = self.translate(language)?;
        Some(translation.name)
    }

    pub fn translate_effect(&self, language: Language) -> Option<String> {
        let translation = self.translate(language)?;
        Some(translation.effect)
    }

    pub fn translate_overlord_tactic(&self, language: Language) -> Option<String> {
        let translation = self.translate(language)?;
        translation.overlord_tactic
    }

    pub fn translate_heroes_tactic(&self, language: Language) -> Option<String> {
        let translation = self.translate(language)?;
        translation.heroes_tactic
    }
}

impl<'de> Deserialize<'de> for OverlordCard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize, Debug, Clone)]
        struct OverlordCardRaw {
            pub name: String,
            #[serde(alias = "type")]
            pub card_type: String,
            pub threat_cost: u32,
            pub discard_value: u32,
            pub treachery_cost: Option<u32>,
            pub treachery_colour: Option<TreacheryColour>,
            pub effect: String,
            #[serde(alias = "Overlord_tactic")]
            pub overlord_tactic: Option<String>,
            #[serde(alias = "Heroes_tactic")]
            pub heroes_tactic: Option<String>,
            pub translations: Vec<OverlordCardTranslation>,
        }

        let raw = OverlordCardRaw::deserialize(deserializer)?;

        let card_type = match raw.card_type.as_str() {
            "Power" => OverlordCardType::Power,
            "Event" => OverlordCardType::Event,
            "Spawn" => OverlordCardType::Spawn,
            "Trap" => OverlordCardType::Trap(Trap::Generic),
            "Trap (Chest)" => OverlordCardType::Trap(Trap::Chest),
            "Trap (Door)" => OverlordCardType::Trap(Trap::Door),
            "Trap (Space)" => OverlordCardType::Trap(Trap::Space),
            "Trap (Treasure)" => OverlordCardType::Trap(Trap::Treasure),
            other => {
                return Err(serde::de::Error::unknown_variant(
                    other,
                    &[
                        "Power",
                        "Event",
                        "Spawn",
                        "Trap",
                        "Trap (Chest)",
                        "Trap (Door)",
                        "Trap (Space)",
                        "Trap (Treasure)",
                    ],
                ))
            }
        };
        Ok(OverlordCard {
            name: raw.name,
            card_type,
            threat_cost: raw.threat_cost,
            discard_value: raw.discard_value,
            treachery_cost: raw.treachery_cost,
            treachery_colour: raw.treachery_colour,
            effect: raw.effect,
            overlord_tactic: raw.overlord_tactic,
            heroes_tactic: raw.heroes_tactic,
            translations: raw.translations,
        })
    }
}

pub fn read_overlord_cards_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, OverlordCard>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let cards = serde_json::from_reader(reader)?;
    Ok(cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_all_overlord_cards_works() {
        let overlord_cards_json_file =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/overlord_cards.json");
        let overlord_cards = read_overlord_cards_from_file(overlord_cards_json_file).unwrap();
        assert!(overlord_cards.contains_key("aim"));
        assert!(overlord_cards.contains_key("trapmaster_treachery"));
        let trapmaster_treachery = overlord_cards["trapmaster_treachery"].clone();
        assert_eq!(trapmaster_treachery.name, "Trapmaster");
        assert!(trapmaster_treachery.treachery_cost.is_some_and(|c| c == 1));
        assert!(trapmaster_treachery
            .treachery_colour
            .is_some_and(|c| matches!(c, TreacheryColour::Purple)));
    }

    #[test]
    fn basic_overlord_card_parsing_works() {
        let aim = r#"
        {
            "name": "Aim",
            "type": "Event",
            "threat_cost": 2,
            "discard_value": 1,
            "effect": "Play immediately after declaring an attack but before rolling any dice. Your attack is an aimed attack.",
            "Overlord_tactic": "This low cost card is useful when you are attacking a weak hero and want to make sure to deal the final blow. It is also useful to counter the dodge ready action.",
            "Heroes_tactic": "None. Pray that the re-roll is worst than the first roll.",
            "translations": [
                {
                    "language": "de",
                    "name": "Zielen",
                    "effect": "Spiele die Karte nachdem Du einen Angriff deklariert hast aber bevor Du würfelst. Der Angriff ist ein gezielter Angriff.",
                    "Overlord_tactic": "Diese kostengünstige Karte ist nützlich, wenn Du einen schwachen Helden angreifen und fertig machen möchtest. Es ist auch nützlich, um der Ausweichaktion entgegenzuwirken.",
                    "Heroes_tactic": "Keine. Betet, dass der Wiederholungswurf schlechter ist als der erste Wurf."
                }
            ]
        }"#;
        let as_card: OverlordCard = serde_json::from_str(aim).unwrap();
        assert_eq!(as_card.name, "Aim");
        matches!(as_card.card_type, OverlordCardType::Event);
        assert_eq!(as_card.threat_cost, 2);
        assert_eq!(as_card.discard_value, 1);
        assert!(as_card.effect.contains("Play immediately"));
        assert!(as_card
            .overlord_tactic
            .is_some_and(|t| t.contains("This low")));
        assert!(as_card
            .heroes_tactic
            .is_some_and(|t| t.contains("None. Pray")));
        assert_eq!(as_card.translations.len(), 1);
        assert_eq!(as_card.translations[0].name, "Zielen");
        matches!(as_card.translations[0].language, Language::De);
        assert!(as_card.translations[0].effect.contains("Spiele die"));
        assert!(as_card.translations[0]
            .overlord_tactic
            .clone()
            .is_some_and(|t| t.contains("Diese kostengün")));
        assert!(as_card.translations[0]
            .heroes_tactic
            .clone()
            .is_some_and(|t| t.contains("Keine.")))
    }
}
