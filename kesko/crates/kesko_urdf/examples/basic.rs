use kesko_urdf::Robot;

fn main() {
    let urdf_str = r#"
    <robot name="MyRobot">
        <link name="link1">
        </link>
        <link name="link2">
        </link>
    </robot>
    "#;

    let robot = Robot::from_urdf_str(urdf_str);

    println!("Robot name: {}", robot.name);
}
