use bevy::prelude::*;

use crate::models::Model;


/// Event meant for propagate UI actions
#[derive(Debug)]
pub(crate) enum UIEvent {
    OpenSpawnWindow,
    OpenFPSWindow,
    SpawnModel {
        model: Model,
        transform: Transform,
        color: Color
    }
}
