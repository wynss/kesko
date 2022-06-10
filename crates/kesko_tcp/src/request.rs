use serde::{Serialize, Deserialize};
use serde_json::Value;


#[derive(Default)]
pub(crate) struct HttpRequest {
    json: Option<String>
}

impl HttpRequest {
    pub(crate) fn parse(request_str: String) -> Result<Self, String> {

        for line in request_str.lines() {
            if line.starts_with("{") {

                return Ok(Self {json: Some(line.to_owned())});

            }
        }

        Err("Failed to parse http request".to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum SimAction {
    Close,
    GetState,
    Restart,
    None
}

impl From<&str> for SimAction {
    fn from(s: &str) -> Self {
        match s {
            "close" => Self::Close,
            "get_state" => Self::GetState,
            "restart" => Self::Restart,
            &_ => Self::None
        } 
    }
}

#[derive(Debug)]
pub(crate) struct SimRequest {
    pub(crate) action: SimAction
}

impl SimRequest {
    pub(crate) fn from_http_request(req: &HttpRequest) -> Option<Self> {

        match &req.json {
            Some(json) => {

                let json: Value = serde_json::from_str(json.as_str()).unwrap();
                let action: SimAction = json["action"].as_str().unwrap().into();
                Some(Self {
                    action
                })
            },
            None => None
        }
    }
}