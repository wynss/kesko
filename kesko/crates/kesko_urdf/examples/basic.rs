use kesko_urdf::RawRobot;

fn main() {
    let urdf_str = r#"
    <robot name="MyRobot">
        <link name="link1">
            <inertial>
                <mass value="1.0"/>
                <origin rpy="0 0 0" xyz="0 0 0"/>
                <inertia ixx="1.0" ixy="0.0" ixz="0.0" iyy="1.0" iyz="0.0" izz="1.0"/>
            </inertial>
            <visual>
                <origin rpy="0 0 0" xyz="0 0 0"/>
                <geometry>
                    <box size="1 1 1"/>
                </geometry>
                <material name="red">
                    <color rgba="1 0 0 1"/>
                </material>
            </visual>
        </link>

        <link name="link2">
        </link>
        
        <joint name="joint1" type="fixed">
            <parent link="link1"/>
            <child link="link2"/>
        </joint>
    </robot>
    "#;

    let robot = RawRobot::from_urdf_str(urdf_str);

    println!("{}", robot);
}
