use std::io::ErrorKind;

use crate::service::task::TaskService;

pub struct TaskController {
	service: TaskService
}

impl TaskController {
	pub fn new(service: TaskService) -> TaskController {
		TaskController{service}
	}

	pub fn list(&self) {
		let task_list = self.service.list();
		if task_list.len() > 0 {
			for task in task_list {
				println!("{}: {}", task.id, task.title);
			}
		} else {
			println!("There are no tasks")
		}
	}

	pub fn get(&self, id: u16) {
		let task = self.service.get(&id);

		if let Ok(task) = task {
			println!("ID: {}", task.id);
			println!("Title: {}", task.title);
			if task.description.len() > 0 {
				println!("Description: {}", task.description);
			}
		} else if let Err(e) = task {
			if e == ErrorKind::NotFound {
				println!("Task not found");
			} else {
				println!("Unexpected error");
			}
		}
	}

	pub fn del(&self, id: u16) {
		let err = self.service.del(&id);
		if err == None {
			println!("Deleted task with ID: {}", id)
		} else if let Some(error) = err {
			if error == ErrorKind::NotFound {
				println!("Task not found")
			} else {
				println!("Unexpected error")
			}
		}
	}

	pub fn add(&self, title: &String, description: &String) {
		let err = self.service.add(title, description);
		if err == None {
			println!("Added task!")
		} else {
			println!("Unexpected error")
		}
	}
}