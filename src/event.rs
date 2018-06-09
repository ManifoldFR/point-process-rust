use std::fmt;

#[derive(Clone, Debug)]
pub struct Event {
    timestamp: f64,
    author: Option<String>,
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
            author: None,
            children: vec!()
        }
    }

    pub fn add_child(&mut self, par: Event) {
        self.children.push(par);
    }
}