use bevy::prelude::*;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(my_system)
        .run();
}

fn my_system() {
    println!("Hello");
}