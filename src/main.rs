use std::io::{self, Read, Write};
use std::fs::{File, OpenOptions};

#[derive(Debug, serde::Serialize, serde::Deserialize)]

struct TodoItem {
    id: u32,
    description: String,
    completed: bool
}

impl TodoItem {
    
    fn new(id: u32, description: &str) -> TodoItem {
        return TodoItem {
            id,
            description: String::from(description),
            completed: false
        };
    }
}

fn load_todo_list_from_file(filename: &str) -> io::Result<Vec<TodoItem>> {
    
    let mut file = File::open(filename).expect("Could not open file");

    let mut contents = String::new();
    
    file.read_to_string(&mut contents).expect("Could not read file");
    
    let todo_list: Vec<TodoItem> = serde_json::from_str(&contents).expect("Error deserializing data");
    
    Ok(todo_list)
}

fn save_todo_list(filename: &str, todo_list: &[TodoItem]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename)?;

    let json_data = serde_json::to_string_pretty(todo_list)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

fn mark_todo_as_completed(todo_list: &mut Vec<TodoItem>) {
    println!("Please enter the todo item id you want to mark as complete: \n");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Did not receive anything, please try again"); 

    let todo_item_id: u32 = input.trim_end().parse().expect("Please enter an integer value");

    let mut flag = false;

    for item in todo_list.iter_mut() {
        if item.id == todo_item_id {
            item.completed = true;
            flag = true;
            break;
        }
    }

    if flag == false {
        println!("Todo item with id: {} not present.\n", todo_item_id);
    }
    else {
        println!("You successfully completed todo item with id: {}.\n", todo_item_id);
    }
}

fn add_todo_item(todo_list: &mut Vec<TodoItem>) {
    println!("Please enter your todo: ");
    let mut todo_description = String::new();
    io::stdin().read_line(&mut todo_description).expect("Did not receive anything, please try again");

    let new_id = todo_list.len() as u32 + 1;

    let new_todo_item = TodoItem::new(new_id, todo_description.trim_end());
    
    todo_list.push(new_todo_item);

    println!("Todo item addedd successfully!\n");

}

fn display_todo_list(todo_list: &[TodoItem]) {
    println!("Welcome! Please review your Todo List for Today\n");

    for (index, item) in todo_list.iter().enumerate() {
        println!("{}. {}: Completed: {}\n", index + 1, item.description, item.completed);
    }

}

fn main() {
    const TODO_FILE: &str = "todo_list.json";

    let mut current_todo_list = match  load_todo_list_from_file(TODO_FILE){
        Ok(todo_list) => todo_list,
        Err(_) => Vec::new()
    };

    loop {
        println!("Please choose your ACTION!\n");
        println!("Enter 1: for creating a new Todo item\n");
        println!("Enter 2: for marking a Todo as completed\n");
        println!("Enter 3: for all the todo items\n");
        println!("Enter 4: for Exit\n");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Did not receive any input, Please try again");
        
        match input.trim_end() {
            "1" => add_todo_item(&mut current_todo_list),
            "2" => mark_todo_as_completed(&mut current_todo_list),
            "3" => display_todo_list(&current_todo_list),
            "4" => break,
            _ => println!("Please enter a valid action number\n")
        }
    }
    
    if let Err(err) = save_todo_list(TODO_FILE, &current_todo_list) {
        eprintln!("Error saving todo list: {}", err);
    }
}
