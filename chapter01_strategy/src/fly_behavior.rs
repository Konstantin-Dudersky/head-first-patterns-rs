// pub type FlyBehavior = fn() -> ();

// pub fn fly_with_wings() -> () {
//     println!("I'm flying!");
// }

// pub fn fly_no_way() -> () {
//     println!("I can't fly!");
// }

pub trait FlyBehavior {
    fn fly();
}

pub struct FlyWithWIngs;

impl FlyBehavior for FlyWithWIngs {
    fn fly() {
        todo!()
    }
}
