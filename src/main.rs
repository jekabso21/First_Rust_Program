use std::io;


fn main() {
    let mut todo_list: Vec<String> = Vec::new();
    let mut finished_list: Vec<String> = Vec::new();

    loop {
        print!("{}[2J", 27 as char);
        println!("Please choose an option:");
        println!("1. Create new todo");
        println!("2. View all todos");
        println!("3. View finished todos");
        println!("4. Mark todo as finished");
        println!("5. Exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => {
                print!("{}[2J", 27 as char);
                println!("Enter the new todo:");
                let mut new_todo = String::new();
                io::stdin().read_line(&mut new_todo).expect("Failed to read line");
                todo_list.push(new_todo.trim().to_string());
                println!("New todo created!");
            }
            2 => {
                loop {
                    print!("{}[2J", 27 as char);
                    println!("All todos:");
                    for (index, task) in todo_list.iter().enumerate() {
                        println!("{}: {}", index + 1, task);
                    }
                    println!("Enter 0 to return to the main menu");
                    let mut option = String::new();
                    io::stdin().read_line(&mut option).expect("Failed to read line");
                    let option: u32 = match option.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };
                    if option == 0 {
                        break;
                    }
                }
            }
            3 => {
                loop {
                    print!("{}[2J", 27 as char);
                    println!("Finished todos:");
                    for (index, task) in finished_list.iter().enumerate() {
                        println!("{}: {}", index + 1, task);
                    }
                    println!("Enter 0 to return to the main menu");
                    let mut option = String::new();
                    io::stdin().read_line(&mut option).expect("Failed to read line");
                    let option: u32 = match option.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };
                    if option == 0 {
                        break;
                    }
                }
            }
            4 => {
                print!("{}[2J", 27 as char);
                println!("All todos:");
                    for (index, task) in todo_list.iter().enumerate() {
                        println!("{}: {}", index + 1, task);
                    }
                println!("\n \n");
                println!("Enter the index of the todo to mark as finished:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).expect("Failed to read line");
                let index: usize = match index_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if index >= 1 && index <= todo_list.len() {
                    let finished_todo = todo_list.remove(index - 1);
                    finished_list.push(finished_todo);
                    println!("Todo marked as finished!");
                } else {
                    println!("Invalid index!");
                }
            }
            5 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option!");
            }
        }
    }
}
