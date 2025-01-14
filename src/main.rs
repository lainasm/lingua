mod vector2;
mod phones;
use vector2::Vector2;
use phones::Phone;

fn main() {
    let phone = Phone::from('u');
    println!("{:#?}", phone);
}
