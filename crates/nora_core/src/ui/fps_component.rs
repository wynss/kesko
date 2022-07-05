use bevy_egui::egui::{self, plot::{Legend, Line, Values}, Color32};

pub(crate) struct FPSComponent {
    ave_fps: f32,
    open: bool
}

impl Default for FPSComponent {
    fn default() -> Self {
        Self {
            ave_fps: 0.0,
            open: false
        }
    } 
}

impl super::UIComponent for FPSComponent {

    type InEvent = ();
    type OutEvent = ();

    fn handle_event(&mut self, _event: &Self::InEvent) {
        
    }

    fn show(&mut self, ctx: &bevy_egui::egui::Context) -> Option<Self::OutEvent> {
        
        let Self {
            ave_fps,
            open
        } = self;

        egui::Window::new("FPS")
            .open(open)
            .default_size([200.0, 70.0])
            .resizable(true)
            .show(ctx, |ui| {

                ui.ctx().request_repaint();

                ui.label(format!("Ave: {} fps", 10.0));

                let plot = egui::plot::Plot::new("fps-plot").legend(Legend::default());
                plot.show(ui, |ui| {
                    ui.line(Line::new(Values::from_parametric_callback(
                        move |x| (x, x),
                        0.0..=10.0,
                        10
                    )).color(Color32::from_rgb(100, 200, 100)).name("FPS"));
                });
        });

        None
    }

    fn remove(&self) -> bool {
        false
    }

    fn toggle_open(&mut self) {
        self.open = !self.open;
    }

}

impl super::UIComponentName for FPSComponent {
    fn name() -> &'static str {
        "fps-component"
    }
}