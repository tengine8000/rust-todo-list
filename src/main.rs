mod todo;
use todo::todo::Todo;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("Too less arguments passed…");
    }
    let action = args[1].clone();
    let task = args[2].clone();

    let mut todo = Todo::new().expect(
        "Error in ToDo list creation",
    );
    if action == "add" {
        todo.insert(task);
        match todo.save() {
            Ok(_) => println!("Task added"),
            Err(e) => println!("Error : {}", e),
        }
    } else if action == "start" {
        match todo.start(&task) {
            None => println!("'{}' not present in ToDo list", task),
            Some(_) => match todo.save() {
                Ok(_) => println!("Task started"),
                Err(e) => println!("Error : {}", e),
            },
        }
    } else if action == "done" {
        match todo.done(&task) {
            None => println!("'{}' not present in ToDo list", task),
            Some(_) => match todo.save() {
                Ok(_) => println!("Task Done"),
                Err(e) => println!("Error : {}", e),
            },
        }
    }
}
