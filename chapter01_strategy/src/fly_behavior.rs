pub type FlyBehavior = fn() -> ();

pub fn FlyWithWings() -> () {
    println!("I'm flying!");
}

pub fn FlyNoWay() -> () {
    println!("I can't fly!");
}
