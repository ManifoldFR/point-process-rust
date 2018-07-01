use std::fmt;
use serde_json;

/// Model for events appearing along the process.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub timestamp: f64,
    intensity: Option<f64>,
    children: Vec<Event>
}

impl Event {
    pub fn new(timestamp: f64) -> Event {
        Event {
            timestamp,
            intensity: None,
            children: vec!()
        }
    }

    pub fn add_intensity(&mut self, intensity: f64) {
        self.intensity.get_or_insert(intensity);
    }

    pub fn add_child(&mut self, par: Event) {
        self.children.push(par);
    }

    pub fn intensity(&self) -> Result<f64,String> {
        self.intensity.ok_or(String::from("Event has no associated intensity."))
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serial_string = serde_json::to_string_pretty(self).unwrap();

        write!(f, "{}", serial_string)
    }
}
