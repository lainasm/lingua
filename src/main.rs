mod vector2;
mod phones;
use std::borrow::BorrowMut;

use vector2::Vector2;
use phones::{Consonant, Phone, Voicing};

#[derive(Debug)]
struct Word {
    pub phones: Vec<Phone>,
}

impl Word {
    pub fn from(str: &str) -> Word {
        let mut phones = Vec::new();

        for i in 0..str.chars().count() {
            if str.chars().nth(i + 1) == Some(':') {
                phones.push(Phone::from_long(str.chars().nth(i).unwrap()));
            } else if str.chars().nth(i) != Some(':') {
                phones.push(Phone::from(str.chars().nth(i).unwrap()));
            }
        }

        Word {
            phones
        }
    }

    pub fn transcription(&self) -> String {
        self.phones.iter().map(|p| if p.is_long() {
            p.transcription().to_string() + ":"
        } else {
            p.transcription().to_string()
        }).collect()
    }

    pub fn replace(&mut self, a: Phone, b: Phone) {
        for i in 0..self.phones.len() {
            if self.phones[i] == a {
                self.phones[i] = b;
            }
        }
    }
    
    pub fn replace_sequence(&mut self, a: Phone, b: &[Phone]) {
        let mut phones = Vec::new();

        for i in 0..self.phones.len() {
            if self.phones[i] == a {
                for new_phone in b {
                    phones.push(*new_phone);
                }
            } else {
                phones.push(self.phones[i]);
            }
        }

        self.phones = phones;
    }

    pub fn replace_before(&mut self, a: Phone, b: Phone, before: &[Phone]) {
        for i in 0..self.phones.len() {
            if self.phones[i] == a {
                let mut can_replace = true;
                for (j, before_phoneme) in before.iter().enumerate() {
                    if i + j + 1 < self.phones.len() {
                        if self.phones[i + j + 1] != *before_phoneme {
                            can_replace = false;
                        }
                    } else {
                        can_replace = false;
                    }
                }

                if can_replace {
                    self.phones[i] = b;
                }
            }
        }
    }

    pub fn replace_before_front(&mut self, a: Phone, b: Phone) {
        for i in 0..self.phones.len() {
            if self.phones[i] == a {
                if i + 1 < self.phones.len() {
                    if self.phones[i + 1].is_front_vowel() {
                        self.phones[i] = b;
                    }
                }
            }
        }
    }

    pub fn replace_sequence_initial(&mut self, a: Phone, b: &[Phone]) {
        let mut new_phones = Vec::new();
        if self.phones[0] == a {
            for added in b {
                new_phones.push(*added);
            }
        } else {
            new_phones.push(self.phones[0]);
        }

        for i in 1..self.phones.len() {
            new_phones.push(self.phones[i]);
        }

        self.phones = new_phones;
    }

    pub fn transform(&mut self) {
        self.replace(Phone::from('ɧ'), Phone::from('s'));
        self.replace_sequence(Phone::from('ʈ'), &[Phone::from('r'), Phone::from('t')]);
        self.replace_sequence(Phone::from('ɖ'), &[Phone::from('r'), Phone::from('d')]);
        self.replace_sequence(Phone::from('ɳ'), &[Phone::from('r'), Phone::from('n')]);
        self.replace_sequence(Phone::from('ʂ'), &[Phone::from('r'), Phone::from('s')]);
        self.replace_sequence(Phone::from('ɭ'), &[Phone::from('r'), Phone::from('l')]);

        if self.phones.contains(&Phone::from('l')) || self.phones.contains(&Phone::from('r')) {
            self.replace(Phone::from('e'), Phone::from('i'));
            self.replace(Phone::from('ø'), Phone::from('y'));
            self.replace(Phone::from('o'), Phone::from('u'));
        }

        self.replace_before(Phone::from_long('e'), Phone::from('e'), &[Phone::from('r'), Phone::from('d')]);
        self.replace_before(Phone::from('o'), Phone::from('a'), &[Phone::from('n'), Phone::from('g')]);
        self.replace_before(Phone::from('o'), Phone::from('a'), &[Phone::from('r'), Phone::from('d')]);
        self.replace_before(Phone::from('o'), Phone::from('a'), &[Phone::from('l'), Phone::from('d')]);
        self.replace_before(Phone::from_long('o'), Phone::from('a'), &[Phone::from('r'), Phone::from('d')]);

        self.replace(Phone::from_long('a'), Phone::from('a'));

        self.replace_before_front(Phone::from('ɕ'), Phone::from('k'));
        self.replace_before_front(Phone::from('j'), Phone::from('g'));

        self.replace(Phone::from_long('o'), Phone::from_long('a'));
        self.replace(Phone::from_long('u'), Phone::from_long('o'));
        self.replace(Phone::from_long('ʉ'), Phone::from_long('u'));

        self.replace(Phone::from('t'), Phone::from('θ'));
        self.replace(Phone::from('d'), Phone::from('ð'));
        self.replace(Phone::from('g'), Phone::from('ɣ'));

        self.replace_sequence_initial(Phone::from('r'), &[Phone::from('h'), Phone::from('r')]);
        self.replace_sequence_initial(Phone::from('n'), &[Phone::from('h'), Phone::from('n')]);
        self.replace_sequence_initial(Phone::from('l'), &[Phone::from('h'), Phone::from('l')]);
    }
}

fn main() {
    let mut words = vec![
        Word::from("go:ɖ"),
        Word::from("ba:ka"),
        Word::from("riŋ"),
        Word::from("følja"),
        Word::from("dolde"),
        Word::from("det"),
        Word::from("ɕedja"),
        Word::from("dʉ:"),
        Word::from("ska:"),
        Word::from("fo:gel"),
    ];

    println!("{:#?}", words.iter().map(|w| w.transcription()).collect::<Vec<_>>());
    for w in &mut words {
        w.transform();
    }
    println!("{:#?}", words.iter().map(|w| w.transcription()).collect::<Vec<_>>());
}



