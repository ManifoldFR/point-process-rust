use std::fmt;

/// Model for events appearing along the process.
#[derive(Clone, Debug)]
pub struct Event {
    pub timestamp: f64,
    intensity: Option<f64>,
    children: Vec<Event>
}


impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        if self.children.len() > 0 {
            write!(f, "Event({},{})", self.timestamp, self.children.iter().fold(
                String::new(),
                |acc, ev| acc + &ev.to_string()
            ))
        } else {
            write!(f, "Event({})", self.timestamp)
        }
    }
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

    pub fn intensity(&self) -> Result<f64,&'static str> {
        self.intensity.ok_or("No intensity here.")
    }
}