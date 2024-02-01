use std::io;

fn main() {
    let mut todo_list: Vec<String> = Vec::new();
    let mut finished_list: Vec<String> = Vec::new();

    loop {
        // println!("Please enter a command:");
        // println!("addtodo - Create new todo");
        // println!("todolist - View all todos");
        // println!("tododone - Mark todo as finished");
        // println!("done - View finished todos");
        // println!("exit - Exit");

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        let command: String = command.trim().to_string();

        match command.as_str() {
            "addtodo" => {
                println!("Enter the new todo:");
                let mut new_todo = String::new();
                io::stdin().read_line(&mut new_todo).expect("Failed to read line");
                todo_list.push(new_todo.trim().to_string());
                println!("New todo created!");
            }
            "todolist" => {
                println!("All todos:");
                for (index, task) in todo_list.iter().enumerate() {
                    println!("{}: {}", index + 1, task);
                }
            }
            "tododone" => {
                println!("All todos:");
                for (index, task) in todo_list.iter().enumerate() {
                    println!("{}: {}", index + 1, task);
                }
                println!("\n\n\n");
                println!("Enter the ID of the todo to mark as finished:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: usize = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if id == 0 || id > todo_list.len() {
                    println!("Invalid ID");
                    continue;
                }
                let finished_todo = todo_list.remove(id - 1);
                finished_list.push(finished_todo);
                println!("Todo marked as finished!");
            }
            "done" => {
                println!("Finished todos:");
                for (index, task) in finished_list.iter().enumerate() {
                    println!("{}: {}", index + 1, task);
                }
            }
            "exit" => {
                break;
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}