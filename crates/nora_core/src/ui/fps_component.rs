use bevy_egui::egui::{self, plot::{Legend, Line, Values}, Color32};

pub(crate) struct FPSComponent {
    open: bool
}

impl Default for FPSComponent {
    fn default() -> Self {
        Self {
            open: false
        }
    } 
}

impl super::UIComponent for FPSComponent {

    fn show(&mut self, ctx: &bevy_egui::egui::Context) -> Option<super::event::UIEvent> {
        
        let Self {
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