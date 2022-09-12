use std::{
    collections::HashMap,
    ops::RangeInclusive
};

use bevy::prelude::*;
use bevy_egui::{egui::{self, Ui}, EguiContext};

use nora_object_interaction::event::SelectEvent;
use nora_physics::{
    multibody::MultibodyRoot,
    joint::{
        Joint,
        JointAxis,
        JointLimits,
        JointType, 
        JointMotorEvent,
        MotorAction
    },
};

use crate::interaction::multibody_selection::MultibodySelectionEvent;


// Joint data to store relevant information and values to display and control the joints
#[derive(Debug)]
struct JointData {
    entity: Entity,
    joint_type: JointType,
    axis_val_1: f32,
    axis_val_2: Option<f32>,
    axis_val_3: Option<f32>,
    limits: Option<JointLimits<f32>>
}


/// UI Component that handles showing multibody information and also controlling their joints
#[derive(Component, Default)]
pub(crate) struct MultibodyUIComponent {
    ui_open: bool,
    multibody_root: Option<Entity>,
    multibody_name: Option<String>,
    multibody_joints: Option<HashMap<String, JointData>>,
}

impl MultibodyUIComponent {

    pub(crate) fn show_system(
        mut multibody_select_event_reader: EventReader<MultibodySelectionEvent>,
        mut select_event_writer: EventWriter<SelectEvent>,
        mut joint_motor_event_writer: EventWriter<JointMotorEvent>,
        mut egui_context: ResMut<EguiContext>,
        mut comp: Query<&mut Self>,
        body_query: Query<&MultibodyRoot>,
        joint_query: Query<&Joint>
    ) {

        // get a reference to 'self'
        let mut comp = comp.get_single_mut().expect("We should have a component");

        // read possible events that a multibody has been selected/deselected
        for event in multibody_select_event_reader.iter() {
            match event {
                MultibodySelectionEvent::Selected(root_entity) => {

                    comp.ui_open = true;
                    comp.multibody_root = Some(*root_entity);

                    if let Ok(root) = body_query.get(*root_entity) {

                        comp.multibody_name = Some(root.name.clone());

                        // Build map with relevant joint data to be displayed
                        let mut joints = HashMap::<String, JointData>::new();
                        for (name, entity) in root.joints.iter() {
                            if let Ok(joint) = joint_query.get(*entity) {

                                // check which type of joint we are dealing with and add it
                                let joint_type = joint.get_type();
                                match joint_type {
                                    JointType::Revolute => {
                                        joints.insert(name.clone(), JointData {
                                            entity: *entity,
                                            joint_type, 
                                            axis_val_1: 0.0,
                                            axis_val_2: None,
                                            axis_val_3: None,
                                            limits: joint.get_limits()
                                        });
                                    },
                                    JointType::Spherical => {
                                        joints.insert(name.clone(), JointData {
                                            entity: *entity,
                                            joint_type, 
                                            axis_val_1: 0.0,
                                            axis_val_2: Some(0.0),
                                            axis_val_3: Some(0.0),
                                            limits: None
                                        });
                                    },
                                    JointType::Prismatic => {
                                        todo!();
                                    }
                                    _ => {}
                                }
                            }
                        }

                        comp.multibody_joints = Some(joints);
                    }
                },
                MultibodySelectionEvent::Deselected(root_entity) => {
                    // make sure we are only closing when the correct entity is deselected
                    if Some(*root_entity) == comp.multibody_root {
                        comp.ui_open = false;
                        comp.multibody_root = None;
                        comp.multibody_name = None;
                        comp.multibody_joints = None;
                    }
                }
            }
        }
        
        // run ui logic
        comp.show(egui_context.ctx_mut(), &mut select_event_writer, &mut joint_motor_event_writer);

    }

    /// Add slider for a revolute joint
    fn revolute_slider(ui: &mut Ui, joint_data: &mut JointData, joint_motor_event_writer: &mut EventWriter<JointMotorEvent>) {

        if ui.add(egui::Slider::new(&mut joint_data.axis_val_1, Self::get_slider_range(joint_data.limits)).suffix("°").step_by(0.1)).changed() {
            // send motor action if the value has changed
            joint_motor_event_writer.send(JointMotorEvent {
                entity: joint_data.entity,
                action: MotorAction::PositionRevolute { 
                    position: joint_data.axis_val_1.to_radians(), 
                    damping: 0.1, 
                    stiffness: 1.0 
                }
            });
        }
    }

    /// Add sliders for a spherical joints
    fn spherical_sliders(ui: &mut Ui, joint_data: &mut JointData, joint_motor_event_writer: &mut EventWriter<JointMotorEvent>) {
        ui.horizontal(|ui| {
            // X
            if ui.add(egui::Slider::new(&mut joint_data.axis_val_1, Self::get_slider_range(joint_data.limits)).suffix("°").text("X")).changed() {
                joint_motor_event_writer.send(JointMotorEvent {
                    entity: joint_data.entity,
                    action: MotorAction::PositionSpherical { 
                        position: joint_data.axis_val_1.to_radians(),
                        axis: JointAxis::AngX,
                        damping: 0.1, 
                        stiffness: 1.0 
                    }
                });
            }
            // Y
            if ui.add(egui::Slider::new(&mut joint_data.axis_val_2.unwrap(), Self::get_slider_range(joint_data.limits)).suffix("°").text("Y")).changed() {
                joint_motor_event_writer.send(JointMotorEvent {
                    entity: joint_data.entity,
                    action: MotorAction::PositionSpherical { 
                        position: joint_data.axis_val_2.unwrap().to_radians(),
                        axis: JointAxis::AngY,
                        damping: 0.1, 
                        stiffness: 1.0 
                    }
                });
            }
            // Z
            if ui.add(egui::Slider::new(&mut joint_data.axis_val_3.unwrap(), Self::get_slider_range(joint_data.limits)).suffix("°").text("Z")).changed() {
                joint_motor_event_writer.send(JointMotorEvent {
                    entity: joint_data.entity,
                    action: MotorAction::PositionSpherical { 
                        position: joint_data.axis_val_3.unwrap().to_radians(),
                        axis: JointAxis::AngZ,
                        damping: 0.1, 
                        stiffness: 1.0 
                    }
                });
            }
        });
    }

    /// Create a slider range from limits
    fn get_slider_range(limits: Option<JointLimits<f32>>) -> RangeInclusive<f32> {
        match limits {
            Some(limits) => RangeInclusive::<f32>::new(limits.min, limits.max),
            None => RangeInclusive::<f32>::new(-180.0, 180.0)
        }
    }

    /// UI logic
    fn show(
        &mut self, 
        ctx: &egui::Context, 
        select_event_writer: &mut EventWriter<SelectEvent>, 
        joint_motor_event_writer: &mut EventWriter<JointMotorEvent>
    ) {
        let Self {
            ui_open,
            multibody_root,
            multibody_name,
            multibody_joints,
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
                            .num_columns(3)
                            .spacing([40.0, 4.0])
                            .striped(true)
                            .show(ui, |ui| {
                                for (joint_name, joint_data) in joints.iter_mut() {

                                    ui.label(joint_name);
                                    ui.label(format!("{:?}", joint_data.joint_type));

                                    match joint_data.joint_type {
                                        JointType::Revolute => {
                                            Self::revolute_slider(ui, joint_data, joint_motor_event_writer);
                                        },
                                        JointType::Spherical => {
                                            Self::spherical_sliders(ui, joint_data, joint_motor_event_writer);
                                        }
                                        _ => {}
                                    }
                                    ui.end_row();
                                }
                            });
                        });
                        ui.separator();
                    }

                    // close button
                    if ui.button("Close And Deselect").clicked() {
                        if let Some(entity) = multibody_root {
                            select_event_writer.send(SelectEvent::Deselect(*entity))
                        }
                    }

                });
            });
    }
}
