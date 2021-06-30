mod animals;

use animals::animal::Animal;
use animals::dogs::Bark;

fn main() {
    println!("Hello, world!");

    let pet = Animal::new(12);
    println!("the pet is {} years old", pet.age);
    println!("the pet has a {} bark score", pet.bark());
}
