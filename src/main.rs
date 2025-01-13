enum Phone {
    Vowel(Vowel),
    Consonant(Consonant),
}

impl Phone {
    fn is_vowel(&self) -> bool {
        match self {
            Phone::Vowel(_) => true,
            _ => false,
        }
    }

    fn is_consonant(&self) -> bool {
        match self {
            Phone::Vowel(_) => false,
            _ => true,
        }
    } 
}

struct Vector2 {
    x: f32,
    y: f32,
}

struct Vowel {
    pos: Vector2,
    rounded: bool,
}

struct Consonant {
    place: ArtPlace,
    manner: Manner,
}

enum ArtPlace {
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    Postalveolar,
    Velar,
    Palatal,
    Glottal,
}

enum Manner {
    Plosive,
    Fricative,
    Nasal,
    Glide,
    Liquid,
}

fn main() {

}
