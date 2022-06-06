use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

const FONT_PATH: &str = "fonts/Roboto-Regular.ttf";

#[derive(Component)]
struct FPSText;

#[derive(Default)]
pub struct FPSScreenPlugin;
impl Plugin for FPSScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup)
            .add_system(Self::update_fps_system);
    }
}

impl FPSScreenPlugin {
    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            text: Text::with_section(
                "FPS: ".to_string(),
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 20.0,
                    color: Color::BLACK,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        }).insert(FPSText);
    }

    fn update_fps_system(
        diagnostic: Res<Diagnostics>,
        mut query: Query<&mut Text, With<FPSText>>
    ) {
        for mut text in query.iter_mut() {
            if let Some(fps) = diagnostic.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    text.sections[0].value = format!("FPS: {:.1}", average);
                }
            }
        }
    }
}