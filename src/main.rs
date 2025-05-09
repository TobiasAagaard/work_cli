use inquire::Select;

fn main() {
    let options = vec![
        "Start tracking time",
        "Add new task",
        "View tasks",
        "Exit",
    ];

    let ans = Select::new("What do you want to do?", options)
        .prompt();

    match ans {
        Ok(choice) => println!("You chose: {}", choice),
        Err(err) => println!("Error: {}", err),
    }
}
