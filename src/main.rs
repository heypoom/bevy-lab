use bevy::prelude::*;

fn greeter() {
    println!("hello world!");
}

fn main() {
    App::new().add_system(greeter).run();
}
