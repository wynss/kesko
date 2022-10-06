use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use kesko_models::Model;



#[derive(Deserialize, Serialize, Debug)]
pub(crate) enum SimAction {
    Close,
    GetState,
    Restart,
    SpawnModel {
        model: Model,
        position: Vec3,
        color: Color
    },
    None
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SimHttpRequest {
    pub(crate) actions: Vec<SimAction>
}

impl SimHttpRequest {
    pub(crate) fn parse(request_str: String) -> Result<String, String> {
        for line in request_str.lines() {
            if line.starts_with("{") {
                return Ok(line.to_owned());
            }
        }
        Err("Failed to parse http request".to_owned())
    }

    pub(crate) fn from_http_str(req: String) -> Result<SimHttpRequest, String> {
        match Self::parse(req) {
            Ok(json) =>{
                match serde_json::from_str::<SimHttpRequest>(json.as_str()) {
                    Ok(req) => Ok(req),
                    Err(e) => Err(format!("Failed to convert http request to SimHttpRequest: {}", e))
                }
            }
            Err(e) => Err(format!("{}", e))
        }
    }
}
