use serde::Serialize;

use kesko_physics::multibody::MultiBodyState;


#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) struct Response {
    pub(crate) multibody_states: Option<Vec<MultiBodyState>>
}
impl Response {
    pub(crate) fn new() -> Self {
        Self {
            multibody_states: None
        }
    }
}
