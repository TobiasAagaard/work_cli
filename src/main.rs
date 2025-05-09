use inquire::Select;

fn main() {
    loop {
        let options: Vec<&'static str> = vec![
            "Start tracking time",
            "Add new task",
            "View tasks",
            "Exit",
        ];

        let ans: Result<&'static str, inquire::InquireError> = Select::new("What do you want to do?", options).prompt();
     
        match ans {
            Ok("Start tracking time") => start_tracking(),
            Ok("Add new task") => add_task(),
            Ok("View tasks") => view_tasks(),
            Ok("Exit") => break,
            Ok(_) => println!("Invalid selection."),
            Err(_) => {
                println!("Something went wrong with the prompt.");
                break;
            }
        }
    }
}

fn start_tracking() {
    println!("Started tracking time...");
}

fn add_task() {
    println!("Add a new task...");
}

fn view_tasks() {
    println!("Viewing tasks...");
}
