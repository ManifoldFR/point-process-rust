extern crate rand;

mod models;

use models::Event;


fn main() {
    let ev = Event::new(1.0, "Donny");
    println!("Hello, world!");
    println!("data: {}", ev);

    let mut ev2 = Event::new(2.0, "Bill");
    ev2.add_child(ev);

    println!("data: {}", ev2);
}
