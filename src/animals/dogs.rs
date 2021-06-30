use crate::animals::animal::Animal;

pub trait Bark {
     fn bark(&self) -> i32;
}

impl Bark for Animal {
     fn bark(&self) -> i32 {
        return self.age + 1;
    }
}