use std::fmt;
use serde_json;

/// Model for events appearing along the process.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    timestamp: f64,
    intensity: f64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<Event>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jump: Option<f64>
}

impl Event {
    pub fn new(timestamp: f64, intensity: f64) -> Event {
        Event {
            timestamp,
            intensity,
            children: vec!(),
            jump: None
        }
    }

    pub fn add_child(&mut self, par: Event) {
        self.children.push(par);
    }

    pub fn jump(mut self, jump: f64) -> Self {
        self.jump.get_or_insert(jump);
        self
    }

    pub fn get_jump(&self) -> f64 {
        self.jump.unwrap()
    }

    pub fn get_timestamp(&self) -> f64 {
        self.timestamp
    }

    pub fn get_intensity(&self) -> f64 {
        self.intensity
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serial_string = serde_json::to_string_pretty(self).unwrap();

        write!(f, "{}", serial_string)
    }
}
