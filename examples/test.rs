use bevy::prelude::*;


fn main() {
    let v1 = Vec3::new(3.0, 4.0, 5.0);
    let v2 = Vec3::Z;
    println!("{}", v1 * v2);
}