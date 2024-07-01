use std::fmt::Display;
use chrono::{Datelike, Utc};
use colored::Colorize;
use task::Task;

mod task;


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
        let msg = match (task.deadline(), task.is_done()){
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
