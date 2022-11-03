use std::{
    collections::BTreeMap,
    ops::RangeInclusive
};

use bevy::prelude::*;
use bevy_egui::{egui::{self, Ui}, EguiContext};

use kesko_object_interaction::event::SelectEvent;
use kesko_physics::{
    multibody::MultibodyRoot,
    joint::{
        JointType, 
        JointMotorEvent,
        MotorCommand, revolute::RevoluteJoint, prismatic::PrismaticJoint, spherical::SphericalJoint, KeskoAxis
    },
    event::PhysicRequestEvent,
    rapier_extern::rapier::prelude as rapier
};

use kesko_core::interaction::multibody_selection::MultibodySelectionEvent;
use kesko_models::ControlDescription;


// Joint data to store relevant information and values to display and control the joints
#[derive(Debug)]
struct JointData {
    name: String,
    joint_type: JointType,
    val_axis_1: rapier::Real,
    val_axis_2: rapier::Real,
    val_axis_3: rapier::Real,
    current_val_x: rapier::Real,
    current_val_y: rapier::Real,
    current_val_z: rapier::Real,
    limits: Option<Vec2>
}


/// UI Component that handles showing multibody information and also controlling their joints
#[derive(Component, Default)]
pub(crate) struct MultibodyUIComponent {
    ui_open: bool,
    multibody_root: Option<Entity>,
    multibody_name: Option<String>,
    multibody_joints: Option<BTreeMap<Entity, JointData>>,
    control_description: Option<String>
}

impl MultibodyUIComponent {

    pub(crate) fn show_system(
        mut multibody_select_event_reader: EventReader<MultibodySelectionEvent>,
        mut physic_request_event_writer: EventWriter<PhysicRequestEvent>,
        mut select_event_writer: EventWriter<SelectEvent>,
        mut joint_motor_event_writer: EventWriter<JointMotorEvent>,
        mut egui_context: ResMut<EguiContext>,
        mut comp: Query<&mut Self>,
        body_query: Query<(&MultibodyRoot, Option<&ControlDescription>)>,
        revolute_joints: Query<&RevoluteJoint>,
        prismatic_joints: Query<&PrismaticJoint>,
        spherical_joints: Query<&SphericalJoint>,
    ) {

        // get a reference to 'self'
        let mut comp = comp.get_single_mut().expect("We should have a component");

        // update if we have a multibody selected or not
        comp.parse_events(&body_query, &mut multibody_select_event_reader);

        // We have a body selected -> update joint data
        if let Some(root_entity) = comp.multibody_root {
            match body_query.get(root_entity) {
                Err(_) => {
                    comp.reset();
                }
                Ok((root, _)) =>{
                    if comp.multibody_joints.is_none() {
                        // Build map with relevant joint data to be displayed
                        let mut joints = BTreeMap::<Entity, JointData>::new();
                        for (name, entity) in root.child_map.iter() {

                            if let Ok(joint) = revolute_joints.get(*entity) {
                                joints.insert(*entity, JointData {
                                    name: name.clone(),
                                    joint_type: JointType::Revolute,
                                    val_axis_1: 0.0,
                                    val_axis_2: 0.0,
                                    val_axis_3: 0.0,
                                    current_val_x: 0.0,
                                    current_val_y: 0.0,
                                    current_val_z: 0.0,
                                    limits: joint.limits,
                                });
                            } 
                            else if let Ok(joint) = prismatic_joints.get(*entity) {
                                joints.insert(*entity, JointData {
                                    name: name.clone(),
                                    joint_type: JointType::Prismatic,
                                    val_axis_1: 0.0,
                                    val_axis_2: 0.0,
                                    val_axis_3: 0.0,
                                    current_val_x: 0.0,
                                    current_val_y: 0.0,
                                    current_val_z: 0.0,
                                    limits: joint.limits
                                });

                            }
                            else if let Ok(_) = spherical_joints.get(*entity) {
                                joints.insert(*entity, JointData {
                                    name: name.clone(),
                                    joint_type: JointType::Spherical,
                                    val_axis_1: 0.0,
                                    val_axis_2: 0.0,
                                    val_axis_3: 0.0,
                                    current_val_x: 0.0,
                                    current_val_y: 0.0,
                                    current_val_z: 0.0,
                                    limits: None
                                });

                            } 
                            else {

                            }
                        }
                        comp.multibody_joints = Some(joints);

                    } else if let Some(joints) = &mut comp.multibody_joints {
                        // just update the component data with current joint values
                        for (joint_entity, joint_data) in joints.iter_mut() {
                            if let Ok(joint) = revolute_joints.get(*joint_entity) {
                                let val = Self::smooth_joint_val(joint.rotation().to_degrees());
                                match joint.axis {
                                    KeskoAxis::X => joint_data.current_val_x = val,
                                    KeskoAxis::Y => joint_data.current_val_y = val,
                                    KeskoAxis::Z => joint_data.current_val_z = val,
                                    _ => {}
                                }
                            }
                            else if let Ok(joint) = prismatic_joints.get(*joint_entity) {
                                let val = joint.position();
                                match joint.axis {
                                    KeskoAxis::X => joint_data.current_val_x = val,
                                    KeskoAxis::Y => joint_data.current_val_y = val,
                                    KeskoAxis::Z => joint_data.current_val_z = val,
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // run ui logic
        comp.show(
            egui_context.ctx_mut(), 
            &mut select_event_writer, 
            &mut physic_request_event_writer,
            &mut joint_motor_event_writer
        );

    }

    /// read possible events that a multibody has been selected/deselected and update component state
    fn parse_events(&mut self, body_query: &Query<(&MultibodyRoot, Option<&ControlDescription>)>, multibody_select_event_reader: &mut EventReader<MultibodySelectionEvent>) {
        for event in multibody_select_event_reader.iter() {
            match event {
                MultibodySelectionEvent::Selected(root_entity) => {
                    self.ui_open = true;
                    self.multibody_root = Some(*root_entity);
                    if let Ok((root, control_desc)) = body_query.get(*root_entity) {
                        self.multibody_name = Some(root.name.clone());
                        if let Some(control_desc) = control_desc {
                            self.control_description = Some(control_desc.0.clone());
                        } else {
                            self.control_description = None;
                        }
                    }
                    self.multibody_joints = None;
                },
                MultibodySelectionEvent::Deselected(root_entity) => {
                    // make sure we are only closing when the correct entity is deselected
                    if Some(*root_entity) == self.multibody_root {
                        self.reset();
                    }
                }
            }
        }
    }

    /// UI logic
    fn show(
        &mut self, 
        ctx: &egui::Context, 
        select_event_writer: &mut EventWriter<SelectEvent>,
        physic_request_event_writer: &mut EventWriter<PhysicRequestEvent>,
        joint_motor_event_writer: &mut EventWriter<JointMotorEvent>
    ) {
        let Self {
            ui_open,
            multibody_root,
            multibody_name,
            multibody_joints,
            control_description
        } = self;

        egui::Window::new("Multibody")
            .open(ui_open)
            .resizable(true)
            .hscroll(false)
            .vscroll(true)
            .title_bar(true)
            .show(ctx, |ui| {

                ui.vertical(|ui| {

                    // display name
                    if let Some(name) = &multibody_name {
                        ui.heading("Name");
                        ui.label(format!("{}", name));
                        ui.separator();
                    }

                    // display joints and controls
                    if let Some(joints) = multibody_joints {
                        ui.vertical(|ui| {
                            ui.heading("Joints");
                            egui::Grid::new("joint_grid")
                            .num_columns(7)
                            .spacing([20.0, 4.0])
                            .striped(true)
                            .show(ui, |ui| {

                                // headers
                                ui.label("Name");
                                ui.label("Type");
                                ui.label("X");
                                ui.label("Y");
                                ui.label("Z");
                                ui.label("Set value");
                                ui.end_row();

                                // joint rows
                                for (joint_entity, joint_data) in joints.iter_mut() {

                                    ui.label(joint_data.name.clone());
                                    ui.label(format!("{:?}", joint_data.joint_type));
                                    ui.label(format!("{:.2}", joint_data.current_val_x));
                                    ui.label(format!("{:.2}", joint_data.current_val_y));
                                    ui.label(format!("{:.2}", joint_data.current_val_z));

                                    match joint_data.joint_type {
                                        JointType::Revolute => {
                                            Self::revolute_slider(ui, joint_entity, joint_data, joint_motor_event_writer);
                                        },
                                        JointType::Spherical => {
                                            // TODO: Spherical joints seems to be broken in rapier at the moment, implement when fixed
                                            // Self::spherical_sliders(ui, joint_entity, joint_data, joint_motor_event_writer);
                                        },
                                        JointType::Prismatic => { 
                                            Self::prismatic_slider(ui, joint_entity, joint_data, joint_motor_event_writer);
                                        }
                                        _ => {}
                                    }
                                    ui.end_row();
                                }
                            });
                        });
                        ui.separator();
                    }
                    
                    // display name
                    if let Some(control_description) = control_description {
                        ui.heading("Control");
                        ui.label(format!("{}", control_description));
                        ui.separator();
                    }
                    
                    ui.horizontal(|ui| {
                        if ui.button("Close And Deselect").clicked() {
                            if let Some(entity) = multibody_root {
                                select_event_writer.send(SelectEvent::Deselect(*entity))
                            }
                        }

                        if ui.button("Despawn").clicked() {
                            if let Some(entity) = multibody_root {
                                select_event_writer.send(SelectEvent::Deselect(*entity));
                                physic_request_event_writer.send(PhysicRequestEvent::DespawnBody(entity.to_bits()));
                            }
                        }

                    });
                });
            });
    }
    
    /// Add slider for a revolute joint
    fn revolute_slider(ui: &mut Ui, joint_entity: &Entity, joint_data: &mut JointData, joint_motor_event_writer: &mut EventWriter<JointMotorEvent>) {
        if ui.add(egui::Slider::new(&mut joint_data.val_axis_1, Self::get_slider_range(joint_data.limits, &joint_data.joint_type)).suffix("°").step_by(0.1)).changed() {
            // send motor action if the value has changed
            joint_motor_event_writer.send(JointMotorEvent {
                entity: *joint_entity,
                action: MotorCommand::PositionRevolute { position: joint_data.val_axis_1.to_radians(), stiffness: None, damping: None}
            });
        }
    }

    fn prismatic_slider(ui: &mut Ui, joint_entity: &Entity, joint_data: &mut JointData, joint_motor_event_writer: &mut EventWriter<JointMotorEvent>) {
        if ui.add(egui::Slider::new(&mut joint_data.val_axis_1, Self::get_slider_range(joint_data.limits, &joint_data.joint_type)).suffix("m").step_by(0.01)).changed() {
            // send motor action if the value has changed
            joint_motor_event_writer.send(JointMotorEvent {
                entity: *joint_entity,
                action: MotorCommand::PositionPrismatic { position: joint_data.val_axis_1, stiffness: None, damping: None}
            });
        }
    }

    /// Add sliders for a spherical joints to control x, y and z axis and send motor actions
    fn spherical_sliders(ui: &mut Ui, joint_entity: &Entity, joint_data: &mut JointData, joint_motor_event_writer: &mut EventWriter<JointMotorEvent>) {
        ui.horizontal(|ui| {
            // X
            if ui.add(egui::Slider::new(&mut joint_data.val_axis_1, Self::get_slider_range(joint_data.limits, &joint_data.joint_type)).suffix("°").text("X")).changed() {
                joint_motor_event_writer.send(JointMotorEvent {
                    entity: *joint_entity,
                    action: MotorCommand::PositionSpherical { 
                        position: joint_data.val_axis_1.to_radians(),
                        axis: KeskoAxis::AngX
                    }
                });
            }
            // Y
            if ui.add(egui::Slider::new(&mut joint_data.val_axis_2, Self::get_slider_range(joint_data.limits, &joint_data.joint_type)).suffix("°").text("Y")).changed() {
                joint_motor_event_writer.send(JointMotorEvent {
                    entity: *joint_entity,
                    action: MotorCommand::PositionSpherical { 
                        position: joint_data.val_axis_2.to_radians(),
                        axis: KeskoAxis::AngY
                    }
                });
            }
            // Z
            if ui.add(egui::Slider::new(&mut joint_data.val_axis_3, Self::get_slider_range(joint_data.limits, &joint_data.joint_type)).suffix("°").text("Z")).changed() {
                joint_motor_event_writer.send(JointMotorEvent {
                    entity: *joint_entity,
                    action: MotorCommand::PositionSpherical { 
                        position: joint_data.val_axis_3.to_radians(),
                        axis: KeskoAxis::AngZ
                    }
                });
            }
        });
    }

    /// Create a slider range from limits
    fn get_slider_range(limits: Option<Vec2>, joint_type: &JointType) -> RangeInclusive<rapier::Real> {

        match joint_type {
            &JointType::Revolute => {
                match limits {
                    Some(limits) => RangeInclusive::<rapier::Real>::new(limits.x.to_degrees() as rapier::Real, limits.y.to_degrees() as rapier::Real),
                    None => RangeInclusive::<rapier::Real>::new(-180.0, 180.0)
                }
            },
            &JointType::Prismatic => {
                match limits {
                    Some(limits) => RangeInclusive::<rapier::Real>::new(limits.x as rapier::Real, limits.y as rapier::Real),
                    None => RangeInclusive::<rapier::Real>::new(-1.0, 1.0)
                }
            }
            _ => {
                RangeInclusive::<rapier::Real>::new(-180.0, 180.0)
            }
        }
    }

    /// reset component setting values to default
    fn reset(&mut self) {
        self.ui_open = false;
        self.multibody_root = None;
        self.multibody_name = None;
        self.multibody_joints = None;
        self.control_description = None;
    }

    /// to smoothen the displayed joint values since they can often be -0.0/0.0 which
    /// result in a very erratic UI behavior
    fn smooth_joint_val(val: rapier::Real) -> rapier::Real {
        if val.abs() < 1e-2 && val.is_sign_negative(){
            return 0.0
        }
        val
    }
}
