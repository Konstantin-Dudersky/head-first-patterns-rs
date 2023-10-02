use crate::fly_behavior;

pub trait Duck<F>
where
    F: fly_behavior::FlyBehavior,
{
    fn new(fly: F);
    fn display(&self);

    fn perform_fly(&self) {
        println!("");
    }
}

pub struct MallardDuck;

impl Duck<fly_behavior::FlyWithWIngs> for MallardDuck {
    fn display(&self) {
        println!("I'm a real Mallard duck");
    }

    fn new(fly: fly_behavior::FlyWithWIngs) {
        todo!()
    }
}
