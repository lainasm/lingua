use crate::Vector2;

use std::sync::LazyLock;
use std::collections::HashMap;

static VOWEL_CHART: [(Vector2, char, bool); 28] = [
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

static CONSONANT_CHART: LazyLock<HashMap<(ArtPlace, Manner, Voicing), char>> = LazyLock::new(|| {
    let chart = HashMap::from(
        [
        ((ArtPlace::Bilabial, Manner::Nasal, Voicing::Voiceless), 'm'),
        ((ArtPlace::Bilabial, Manner::Nasal, Voicing::Voiced), 'm'),
        ((ArtPlace::Labiodental, Manner::Nasal, Voicing::Voiceless), 'ɱ'),
        ((ArtPlace::Labiodental, Manner::Nasal, Voicing::Voiced), 'ɱ'),
        ((ArtPlace::Dental, Manner::Nasal, Voicing::Voiceless), 'n'),
        ((ArtPlace::Dental, Manner::Nasal, Voicing::Voiced), 'n'),
        ((ArtPlace::Retroflex, Manner::Nasal, Voicing::Voiceless), 'ɳ'),
        ((ArtPlace::Retroflex, Manner::Nasal, Voicing::Voiced), 'ɳ'),
        ((ArtPlace::Palatal, Manner::Nasal, Voicing::Voiceless), 'ɲ'),
        ((ArtPlace::Palatal, Manner::Nasal, Voicing::Voiced), 'ɲ'),
        ((ArtPlace::Velar, Manner::Nasal, Voicing::Voiceless), 'ŋ'),
        ((ArtPlace::Velar, Manner::Nasal, Voicing::Voiced), 'ŋ'),
        ((ArtPlace::Bilabial, Manner::Plosive, Voicing::Voiceless), 'p'),
        ((ArtPlace::Bilabial, Manner::Plosive, Voicing::Voiced), 'b'),
        ((ArtPlace::Labiodental, Manner::Plosive, Voicing::Voiceless), 'p'),
        ((ArtPlace::Labiodental, Manner::Plosive, Voicing::Voiced), 'b'),
        ((ArtPlace::Dental, Manner::Plosive, Voicing::Voiceless), 't'),
        ((ArtPlace::Dental, Manner::Plosive, Voicing::Voiced), 'd'),
        ((ArtPlace::Retroflex, Manner::Plosive, Voicing::Voiceless), 'ʈ'),
        ((ArtPlace::Retroflex, Manner::Plosive, Voicing::Voiced), 'ɖ'),
        ((ArtPlace::Palatal, Manner::Plosive, Voicing::Voiceless), 'c'),
        ((ArtPlace::Palatal, Manner::Plosive, Voicing::Voiced), 'ɟ'),
        ((ArtPlace::Velar, Manner::Plosive, Voicing::Voiceless), 'k'),
        ((ArtPlace::Velar, Manner::Plosive, Voicing::Voiced), 'g'),
        ((ArtPlace::Glottal, Manner::Plosive, Voicing::Voiceless), 'ʔ'),
        ((ArtPlace::Glottal, Manner::Plosive, Voicing::Voiced), 'ʔ'),
        ((ArtPlace::Dental, Manner::Sibilant, Voicing::Voiceless), 's'),
        ((ArtPlace::Dental, Manner::Sibilant, Voicing::Voiced), 'z'),
        ((ArtPlace::Retroflex, Manner::Sibilant, Voicing::Voiceless), 'ʂ'),
        ((ArtPlace::Retroflex, Manner::Sibilant, Voicing::Voiced), 'ʐ'),
        ((ArtPlace::Palatal, Manner::Sibilant, Voicing::Voiceless), 'ɕ'),
        ((ArtPlace::Palatal, Manner::Sibilant, Voicing::Voiced), 'ʑ'),
        ((ArtPlace::Bilabial, Manner::Fricative, Voicing::Voiceless), 'ɸ'),
        ((ArtPlace::Bilabial, Manner::Fricative, Voicing::Voiced), 'β'),
        ((ArtPlace::Labiodental, Manner::Fricative, Voicing::Voiceless), 'f'),
        ((ArtPlace::Labiodental, Manner::Fricative, Voicing::Voiced), 'v'),
        ((ArtPlace::Dental, Manner::Fricative, Voicing::Voiceless), 'θ'),
        ((ArtPlace::Dental, Manner::Fricative, Voicing::Voiced), 'ð'),
        ((ArtPlace::Retroflex, Manner::Fricative, Voicing::Voiceless), 'ɻ'),
        ((ArtPlace::Retroflex, Manner::Fricative, Voicing::Voiced), 'ɻ'),
        ((ArtPlace::Palatal, Manner::Fricative, Voicing::Voiceless), 'ç'),
        ((ArtPlace::Palatal, Manner::Fricative, Voicing::Voiced), 'ʝ'),
        ((ArtPlace::Velar, Manner::Fricative, Voicing::Voiceless), 'x'),
        ((ArtPlace::Velar, Manner::Fricative, Voicing::Voiced), 'ɣ'),
        ((ArtPlace::Glottal, Manner::Fricative, Voicing::Voiceless), 'h'),
        ((ArtPlace::Glottal, Manner::Fricative, Voicing::Voiced), 'ɦ'),
        ((ArtPlace::Labiodental, Manner::Approximant, Voicing::Voiceless), 'ʋ'),
        ((ArtPlace::Labiodental, Manner::Approximant, Voicing::Voiced), 'ʋ'),
        ((ArtPlace::Dental, Manner::Approximant, Voicing::Voiceless), 'ɹ'),
        ((ArtPlace::Dental, Manner::Approximant, Voicing::Voiced), 'ɹ'),
        ((ArtPlace::Retroflex, Manner::Approximant, Voicing::Voiceless), 'ɻ'),
        ((ArtPlace::Retroflex, Manner::Approximant, Voicing::Voiced), 'ɻ'),
        ((ArtPlace::Palatal, Manner::Approximant, Voicing::Voiceless), 'j'),
        ((ArtPlace::Palatal, Manner::Approximant, Voicing::Voiced), 'j'),
        ((ArtPlace::Velar, Manner::Approximant, Voicing::Voiceless), 'ɰ'),
        ((ArtPlace::Velar, Manner::Approximant, Voicing::Voiced), 'ɰ'),
        ((ArtPlace::Labiodental, Manner::Tap, Voicing::Voiceless), 'ⱱ'),
        ((ArtPlace::Labiodental, Manner::Tap, Voicing::Voiced), 'ⱱ'),
        ((ArtPlace::Dental, Manner::Tap, Voicing::Voiceless), 'ɾ'),
        ((ArtPlace::Dental, Manner::Tap, Voicing::Voiced), 'ɾ'),
        ((ArtPlace::Retroflex, Manner::Tap, Voicing::Voiceless), 'ɽ'),
        ((ArtPlace::Retroflex, Manner::Tap, Voicing::Voiced), 'ɽ'),
        ((ArtPlace::Bilabial, Manner::Trill, Voicing::Voiceless), 'ʙ'),
        ((ArtPlace::Bilabial, Manner::Trill, Voicing::Voiced), 'ʙ'),
        ((ArtPlace::Dental, Manner::Trill, Voicing::Voiceless), 'r'),
        ((ArtPlace::Dental, Manner::Trill, Voicing::Voiced), 'r'),
        ((ArtPlace::Retroflex, Manner::Trill, Voicing::Voiceless), 'ɽ'),
        ((ArtPlace::Retroflex, Manner::Trill, Voicing::Voiced), 'ɽ'),
        ((ArtPlace::Dental, Manner::Lateral, Voicing::Voiceless), 'l'),
        ((ArtPlace::Dental, Manner::Lateral, Voicing::Voiced), 'l'),
        ]
    );

    chart
});

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

    pub fn transcription(&self) -> char {
        match self {
            Phone::Vowel(x) => x.transcription(),
            Phone::Consonant(x) => x.transcription(),
        }
    }

    pub fn from(c: char) -> Phone {
        if let Some((a, b, c)) = VOWEL_CHART.iter().find(|(_, c2, _)| c == *c2) {
            Phone::Vowel(Vowel {
                pos: *a,
                rounded: *c,
            })
        } else {
            let ((a, m, v), _) = CONSONANT_CHART.iter().find(|((_, _, _), c2)| c == **c2).unwrap();

            Phone::Consonant(Consonant {
                place: *a,
                manner: *m,
                voiced: *v,
            })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vowel {
    pos: Vector2,
    rounded: bool,
}

impl Vowel {
    pub fn transcription(&self) -> char {
        VOWEL_CHART.iter().filter(|(_, _, r)| *r == self.rounded).min_by(|(p, _, _), (p2, _, _)| {
            let dist1 = self.pos.dist(*p);
            let dist2 = self.pos.dist(*p2);

            dist1.partial_cmp(&dist2).unwrap()
        }).unwrap().1
    }
}

impl Consonant {
    pub fn transcription(&self) -> char {
        *CONSONANT_CHART.get(&(self.place, self.manner, self.voiced)).unwrap_or(&'0')
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Consonant {
    pub place: ArtPlace,
    pub manner: Manner,
    pub voiced: Voicing,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Voicing {
    Voiceless,
    Voiced,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum ArtPlace {
    Bilabial,
    Labiodental,
    Dental,
    Retroflex,
    Velar,
    Palatal,
    Glottal,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Manner {
    Plosive,
    Fricative,
    Nasal,
    Sibilant,
    Approximant,
    Tap,
    Trill,
    Lateral,
}
