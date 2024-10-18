use std::io::ErrorKind;

use crate::{domain::task::Task, repository::task};

pub struct TaskService {
	repository: task::TaskRepository
}

impl TaskService {
	pub fn new(repository: task::TaskRepository) -> TaskService {
		TaskService{repository}
	}

	pub fn list(&self) -> &Vec<Task> {
		self.repository.list()
	}

	pub fn get(&self, id: &u16) -> Result<&Task, ErrorKind> {
		self.repository.get(id)
	}

	pub fn del(&self, id: &u16) -> Option<ErrorKind> {
		self.repository.del(id)
	}

	pub fn add(&self, title: &String, description: &String) -> Option<ErrorKind> {
		self.repository.add(title, description)
	}
}