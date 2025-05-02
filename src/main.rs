mod vector2;
mod phones;
use vector2::Vector2;
use phones::{Consonant, Phone};

#[derive(Debug)]
struct Word {
    phones: Vec<Phone>,
}

impl Word {
    fn from(str: &str) -> Word {
        Word {
            phones: str.chars().map(|c| Phone::from(c)).collect(),
        }
    }

    fn transcription(&self) -> String {
        self.phones.iter().map(|p| p.transcription()).collect()
    }
}

fn main() {
    let phone = Phone::from('u');
    println!("{:#?}", phone);

    let word = Word::from("aieou");
    println!("{word:#?}");
    println!("{}", word.transcription());
    let t = Consonant {
        manner: phones::Manner::Plosive,
        place: phones::ArtPlace::Velar,
        voiced: phones::Voicing::Voiced,
    };

    println!("{}", t.transcription());
}
