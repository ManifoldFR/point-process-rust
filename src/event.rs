use std::fmt;
use serde_json;

/// Model for events appearing along the process.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    timestamp: f64,
    intensity: f64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<Event>
}

impl Event {
    pub fn new(timestamp: f64, intensity: f64) -> Event {
        Event {
            timestamp,
            intensity,
            children: vec!()
        }
    }

    pub fn add_child(&mut self, par: Event) {
        self.children.push(par);
    }

    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }

    pub fn intensity(&self) -> f64 {
        self.intensity
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serial_string = serde_json::to_string_pretty(self).unwrap();

        write!(f, "{}", serial_string)
    }
}
