use {
    serde::{Deserialize, Serialize},
    serde_json,
    std::{
        collections::HashMap,
        fs::File,
        io,
        io::{Read, Write},
        path::Path,
        process::ExitStatus,
    },
};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
enum State {
    Done,
    Undone,
}

// Sanity keeper
fn clear_terminal() -> ExitStatus {
    std::process::Command::new("clear").status().unwrap()
}

// Task Manager
fn print_tasks(tasks: HashMap<String, State>) {
    println!("Task list:");
    for (k, v) in tasks {
        println!(
            "Task: {}, State: {}",
            k,
            match v {
                State::Done => "Done",
                State::Undone => "Undone",
            }
        );
    }
}

fn new_task(
    tasks: HashMap<String, State>,
    task_name: String,
) -> Result<HashMap<String, State>, String> {
    let mut updated_tasks = tasks.clone();
    if !updated_tasks.contains_key(&task_name) {
        updated_tasks.insert(task_name, State::Undone);
        Ok(updated_tasks)
    } else {
        Err("This task already exists".to_string())
    }
}

fn remove_task(
    tasks: HashMap<String, State>,
    task_name: String,
) -> Result<HashMap<String, State>, String> {
    let mut updated_tasks = tasks.clone();
    if updated_tasks.contains_key(&task_name) {
        updated_tasks.remove(&task_name);
        Ok(updated_tasks)
    } else {
        Err("This task doesn't exist".to_string())
    }
}

fn alternate_state(
    tasks: HashMap<String, State>,
    task_name: String,
) -> Result<HashMap<String, State>, String> {
    let mut updated_tasks = tasks.clone();
    if updated_tasks.contains_key(&task_name) {
        if let Some(state) = updated_tasks.get_mut(&task_name) {
            if state == &mut State::Undone {
                *state = State::Done;
            } else {
                *state = State::Undone;
            }
            Ok(updated_tasks)
        } else {
            Err("Something went wrong".to_string())
        }
    } else {
        Err("This task doesn't exist".to_string())
    }
}

// File manager
fn save_tasks(file_path: &Path, tasks: HashMap<String, State>) {
    let serialized = serde_json::to_string(&tasks).unwrap();
    let mut file = File::create(file_path).expect("Can't create file");
    file.write_all(serialized.as_bytes()).unwrap();
}

fn load_tasks(file_path: &Path) -> HashMap<String, State> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            let mut file = File::create(file_path).expect("Can't create file");
            let default_tasks = HashMap::from([(String::from("Welcome"), State::Undone)]);
            let serialized = serde_json::to_string(&default_tasks).unwrap();
            file.write_all(serialized.as_bytes()).unwrap();
            return default_tasks;
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let deserialized: HashMap<String, State> = serde_json::from_str(&contents).unwrap();
    deserialized
}

// Program function
fn main() {
    let file_path = Path::new("tasks.json");
    let mut tasks: HashMap<String, State> = load_tasks(file_path);
    let mut input = String::new();
    loop {
        clear_terminal();
        input.clear();
        println!("What do you want to do?\n's' - List tasks\n'c' - Change task state\n'a' - Add a new task\n'r' - Remove a task\n'q' - Close app");
        let _ = io::stdin().read_line(&mut input);
        match input.trim().parse::<char>() {
            Ok(n) => {
                if n == 's' {
                    clear_terminal();
                    print_tasks(tasks.clone());
                    println!("Press enter to continue");
                    let _ = io::stdin().read_line(&mut input);
                } else if n == 'a' {
                    loop {
                        clear_terminal();
                        input.clear();
                        print_tasks(tasks.clone());
                        println!("Write the new task");
                        let _ = io::stdin().read_line(&mut input);
                        match new_task(tasks.clone(), input.trim().to_string()) {
                            Ok(updated_tasks) => {
                                tasks = updated_tasks;
                                save_tasks(file_path, tasks.clone());
                            }
                            Err(e) => println!("Error: {}", e),
                        }
                        break;
                    }
                    println!("Press enter to continue");
                    let _ = io::stdin().read_line(&mut input);
                } else if n == 'r' {
                    loop {
                        clear_terminal();
                        input.clear();
                        print_tasks(tasks.clone());
                        println!("Write the task to remove");
                        let _ = io::stdin().read_line(&mut input);
                        match remove_task(tasks.clone(), input.trim().to_string()) {
                            Ok(updated_tasks) => {
                                tasks = updated_tasks;
                                save_tasks(file_path, tasks.clone());
                            }
                            Err(e) => println!("Error: {}", e),
                        }
                        break;
                    }
                    println!("Press enter to continue");
                    let _ = io::stdin().read_line(&mut input);
                } else if n == 'c' {
                    loop {
                        clear_terminal();
                        input.clear();
                        print_tasks(tasks.clone());
                        println!("Write the task to change state");
                        let _ = io::stdin().read_line(&mut input);
                        match alternate_state(tasks.clone(), input.trim().to_string()) {
                            Ok(updated_tasks) => {
                                tasks = updated_tasks;
                                save_tasks(file_path, tasks.clone());
                            }
                            Err(e) => println!("Error: {}", e),
                        }
                        break;
                    }
                    println!("Press enter to continue");
                    let _ = io::stdin().read_line(&mut input);
                } else if n == 'q' {
                    break;
                } else {
                    println!("Invalid option");
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
