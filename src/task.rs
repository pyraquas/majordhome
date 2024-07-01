use chrono::{Datelike, DateTime, Utc};
use std::fmt::{Display, Formatter};

pub struct Task{
    id: u32,
    name: String,
    description: String,
    done: bool,
    deadline: Option<DateTime<Utc>>
}

impl Task{
    pub(crate) fn new(id: u32, name: String, description: String, deadline: Option<DateTime<Utc>>) -> Task{
        Task{
            id,
            name,
            description,
            done: false,
            deadline
        }
    }

    fn mark_done(&mut self){
        self.done = true;
    }

    pub fn is_done(&self) -> bool{
        self.done
    }

    pub fn deadline(&self) -> Option<DateTime<Utc>>{
        self.deadline
    }
}

impl Display for Task{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.deadline{
            Some(d) => write!(f, "Task: {} - {} - {} - {}/{}/{}", self.id, self.name, self.description, d.day(), d.month(), d.year()),
            None => write!(f, "Task: {} - {} - {}", self.id, self.name, self.description),
        }
    }
}
