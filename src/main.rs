use std::{env, process::exit};

pub mod repository {
    pub mod task;
}

pub mod service {
    pub mod task;
}

pub mod controller {
    pub mod task;
}

pub mod domain {
    pub mod task;
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        println!("Try \"help\" command");
        exit(0)
    } else if arguments[1] == "version" {
        println!("TODO - 0.0.1");
        exit(0)
    } else if arguments[1] == "help" {
        println!("list: display the task list in the format \"<id>: <title>\"");
        println!("get <id>: display a single task with full details");
        println!("del <id>: delete a single task");
        println!("add <title> <description>: add a new task with the specified title and description");
    }

    let task_repository = repository::task::TaskRepository::new();
    let task_service = service::task::TaskService::new(task_repository);
    let task_controller = controller::task::TaskController::new(task_service);

    if arguments.len() == 2 && arguments[1] == "list" {
        task_controller.list();
        exit(0);
    } else if arguments.len() == 3 && arguments[1] == "get" {
        let parsed_id = arguments[2].parse::<u16>();
        if let Ok(id) = parsed_id {
            task_controller.get(id);
        } else {
            println!("Wrong task ID")
        }
    } else if arguments.len() == 3 && arguments[1] == "del" {
        let parsed_id = arguments[2].parse::<u16>();
        if let Ok(id) = parsed_id {
            task_controller.del(id);
        } else {
            println!("Wrong task ID")
        }
    } else if arguments.len() == 4 && arguments[1] == "add" {
        task_controller.add(&arguments[2], &arguments[3])
    } else {
        println!("Command \"{}\" not found", arguments[1]);
    }
}