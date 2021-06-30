

pub struct Animal {
    // how old is the pet
    pub age: i32
}

impl Animal {
    pub fn new(age: i32) -> Animal {
        Animal { age }
    }
}