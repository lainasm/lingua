use crate::Vector2;

const VOWEL_CHART: [(Vector2, char, bool); 28] = [
    (Vector2 { x: 0f32, y: 0f32 }, 'i', false),
    (Vector2 { x: 0f32, y: 0f32 }, 'y', true),
    (Vector2 { x: 1f32, y: 0f32 }, 'ɯ', false),
    (Vector2 { x: 1f32, y: 0f32 }, 'u', true),
    (Vector2 { x: 0.5f32, y: 0f32 }, 'ɨ', false),
    (Vector2 { x: 0.5f32, y: 0f32 }, 'ʉ', true),

    (Vector2 { x: 0.17f32, y: 0.166f32 }, 'ɪ', false),
    (Vector2 { x: 0.17f32, y: 0.166f32 }, 'ʏ', true),

    (Vector2 { x: 0.5f32 * 0.333f32, y: 0.333f32 }, 'e', false),
    (Vector2 { x: 0.5f32 * 0.333f32, y: 0.333f32 }, 'ø', true),
    (Vector2 { x: (0.5f32 * 0.333f32 + 1f32) / 2f32, y: 0.333f32 }, 'ɘ', false),
    (Vector2 { x: (0.5f32 * 0.333f32 + 1f32) / 2f32, y: 0.333f32 }, 'ɵ', true),
    (Vector2 { x: 1f32, y: 0.333f32 }, 'ɤ', false),
    (Vector2 { x: 1f32, y: 0.333f32 }, 'o', true),

    (Vector2 { x: (0.5f32 * 0.5f32 + 1f32) / 2f32, y: 0.5f32 }, 'ə', false),

    (Vector2 { x: 0.5f32 * 0.666f32, y: 0.666f32 }, 'ɛ', false),
    (Vector2 { x: 0.5f32 * 0.666f32, y: 0.666f32 }, 'œ', true),
    (Vector2 { x: (0.5f32 * 0.666f32 + 1f32) / 2f32, y: 0.666f32 }, 'ɜ', false),
    (Vector2 { x: (0.5f32 * 0.666f32 + 1f32) / 2f32, y: 0.666f32 }, 'ɞ', true),
    (Vector2 { x: 1f32, y: 0.666f32 }, 'ʌ', false),
    (Vector2 { x: 1f32, y: 0.666f32 }, 'ɔ', true),

    (Vector2 { x: 0.5f32 * 0.833f32, y: 0.833f32 }, 'æ', false),
    (Vector2 { x: (0.5f32 * 0.833f32 + 1f32) / 2f32, y: 0.833f32 }, 'ɐ', false),

    (Vector2 { x: 0.5f32, y: 1f32 }, 'a', false),
    (Vector2 { x: 0.5f32, y: 1f32 }, 'ɶ', true),
    (Vector2 { x: (0.5f32 + 1f32) / 2f32, y: 1f32 }, 'ä', false),
    (Vector2 { x: 1f32, y: 1f32 }, 'ɑ', false),
    (Vector2 { x: 1f32, y: 1f32 }, 'ɒ', true),
];

pub struct Morpheme {
    pub phones: Vec<Phone>,
}

#[derive(Debug, Clone, Copy)]
pub enum Phone {
    Vowel(Vowel),
    Consonant(Consonant),
}

impl Phone {
    pub fn is_vowel(&self) -> bool {
        match self {
            Phone::Vowel(_) => true,
            Phone::Consonant(_) => false,
        }
    }

    pub fn is_consonant(&self) -> bool {
        match self {
            Phone::Vowel(_) => false,
            Phone::Consonant(_) => true,
        }
    } 

    pub fn from(c: char) -> Phone {
        if let Some((a, b, c)) = VOWEL_CHART.iter().find(|(_, c2, _)| c == *c2) {
            Phone::Vowel(Vowel {
                pos: *a,
                rounded: *c,
            })
        } else {
            Phone::Consonant(Consonant {
                place: ArtPlace::Bilabial,
                manner: Manner::Plosive,
            })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vowel {
    pos: Vector2,
    rounded: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Consonant {
    place: ArtPlace,
    manner: Manner,
}

#[derive(Debug, Clone, Copy)]
pub enum ArtPlace {
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    Postalveolar,
    Velar,
    Palatal,
    Glottal,
}

#[derive(Debug, Clone, Copy)]
pub enum Manner {
    Plosive,
    Fricative,
    Nasal,
    Glide,
    Liquid,
}
