use std::fmt::{Display, Formatter};
use chrono::{Datelike, DateTime, Utc};
use colored::Colorize;


fn main() {

    let mut vec = Vec::new();
    for i in 0..10{
        let mut task = Task::new(i, "Task".to_string(), "Description".to_string(), Some(Utc::now()+chrono::Duration::days(i as i64)));
        /*if i%2 == 0{
            task.mark_done();
        }*/
        vec.push(task);

    }

    for task in vec.iter(){
        /*if task.deadline.is_some_and(|d| d < (Utc::now()+chrono::Duration::days(5))) && !task.done {
            println!("{}",task.to_string().as_str().red());
        }*/
        let msg = match (task.deadline, task.done){
            (Some(d),false) => match (d - Utc::now()).num_days() {
                x if x < 0 => task.to_string().as_str().red(),
                0 => task.to_string().as_str().truecolor(255, 100, 0),
                1 => task.to_string().as_str().yellow(),
                _ => task.to_string().as_str().green(),
            },
            (None,false) =>task.to_string().as_str().green(),
            (_,true) => task.to_string().as_str().truecolor(80,80,80).strikethrough(),
        };
        println!("{}", msg);
    }
}

struct Task{
    id: u32,
    name: String,
    description: String,
    done: bool,
    deadline: Option<DateTime<Utc>>
}

impl Task{
    fn new(id: u32 ,name: String, description: String,deadline: Option<DateTime<Utc>>) -> Task{
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
}

impl Display for Task{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.deadline{
            Some(d) => write!(f, "Task: {} - {} - {} - {}/{}/{}", self.id, self.name, self.description, d.day(), d.month(), d.year()),
            None => write!(f, "Task: {} - {} - {}", self.id, self.name, self.description),
        }
    }
}