use crate::fly_behavior;

pub trait Duck {
    fn display(&self);
}

pub struct MallardDuck;

impl Duck for MallardDuck {
    fn display(&self) {
        println!("I'm a real Mallard duck");
    }
}
