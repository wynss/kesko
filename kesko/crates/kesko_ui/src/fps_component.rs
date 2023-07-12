use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_egui::{
    egui,
    egui::{
        plot::{Legend, Line, PlotPoints},
        Color32,
    },
    EguiContexts,
};

const HISTORY_LEN: usize = 20;

#[derive(Component)]
pub(crate) struct FPSComponent {
    fps_ave: f32,
    fps_ave_history: VecDeque<f32>,
    fps_history: Vec<f32>,
    open: bool,
}

impl Default for FPSComponent {
    fn default() -> Self {
        Self {
            fps_ave: 0.0,
            fps_ave_history: VecDeque::with_capacity(HISTORY_LEN),
            fps_history: Vec::new(),
            open: false,
        }
    }
}

impl FPSComponent {
    pub(crate) fn update_system(
        mut spawn_event_reader: EventReader<FPSComponentEvent>,
        mut comp: Query<&mut Self>,
    ) {
        let mut comp = comp.get_single_mut().unwrap();

        for event in spawn_event_reader.iter() {
            match event {
                FPSComponentEvent::Open => comp.open = true,
                FPSComponentEvent::FPSData { ave, history } => {
                    comp.fps_ave = *ave;
                    comp.fps_history = history.clone();
                    comp.fps_ave_history.push_back(*ave);

                    if comp.fps_ave_history.len() > HISTORY_LEN {
                        comp.fps_ave_history.pop_front();
                    }
                }
            }
        }
    }

    pub(crate) fn show_and_send_system(mut egui_context: EguiContexts, mut comp: Query<&mut Self>) {
        let mut comp = comp.get_single_mut().unwrap();
        comp.show(egui_context.ctx_mut());
    }

    fn show(&mut self, ctx: &bevy_egui::egui::Context) {
        let Self {
            fps_ave,
            fps_ave_history,
            fps_history,
            open,
        } = self;

        egui::Window::new("FPS")
            .open(open)
            .default_size([400.0, 120.0])
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(format!("Ave: {:.1} fps", fps_ave));

                let plot = egui::plot::Plot::new("fps-plot")
                    .legend(Legend::default())
                    .allow_drag(false)
                    .allow_scroll(false)
                    .allow_zoom(false)
                    .show_axes([false, true]);

                plot.show(ui, |ui| {
                    ui.line(
                        Line::new(PlotPoints::from_ys_f32(fps_history.as_slice()))
                            .color(Color32::from_rgb(100, 200, 100))
                            .name("FPS"),
                    );
                    ui.line(
                        Line::new(PlotPoints::from_ys_f32(fps_ave_history.make_contiguous()))
                            .color(Color32::from_rgb(100, 100, 200))
                            .name("FPS Ave"),
                    );
                });
            });
    }
}

#[derive(Event)]
pub enum FPSComponentEvent {
    /// will open the component
    Open,
    /// send fps to uppate the component
    FPSData { ave: f32, history: Vec<f32> },
}
