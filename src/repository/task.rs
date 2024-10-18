use std::{fs::{self, File}, io::{BufRead, BufReader, ErrorKind, Write}};

use crate::domain::task::Task;

const DATA_FILE: &'static str = "stored";

pub struct TaskRepository {
	pub list: Vec<Task>
}

impl TaskRepository {
	pub fn new() -> Self {
		let mut list: Vec<Task> = vec![];

		let file = File::options().read(true).write(true).create(true).open(DATA_FILE).unwrap();
		let reader = BufReader::new(file);
		
		for line in reader.lines() {
			let line = line.unwrap();
			let mut tmp_task = Task{
				id: 0,
				title: String::new(),
				description: String::new()
			};
			let data: Vec<&str> = line.split("$$").collect();
			for (i, section) in data.iter().enumerate() {
				if i == 0 {
					tmp_task.id = section.parse::<u16>().unwrap();
				} else if i == 1 {
					tmp_task.title = section.to_string();
				} else if i == 2 {
					tmp_task.description = section.to_string();
				}
			}
			list.push(tmp_task);
		}

		TaskRepository{list}
	}

	pub fn list(&self) -> &Vec<Task> {
		&self.list
	}

	pub fn get(&self, id: &u16) -> Result<&Task, ErrorKind> {
		for task in self.list.iter() {
			if task.id == *id {
				return Ok(&task);
			}
		}
		Err(ErrorKind::NotFound)
	}

	pub fn del(&self, id: &u16) -> Option<ErrorKind> {
		let file = File::open(DATA_FILE).unwrap();
		let reader = BufReader::new(file);
		let mut file_str = String::new();
		
		for (i, line) in reader.lines().enumerate() {
			let line = line.unwrap();
			let data: Vec<&str> = line.split("$$").collect();
			if data[0] != id.to_string() {
				if i != 1 {
					println!("{}", i);
					file_str += "\n";
				}
				file_str += &line;
			}
		}

		let mut file = File::create(DATA_FILE).unwrap();
		file.write_all(file_str.as_bytes()).unwrap();

		None
	}

	pub fn add(&self, title: &String, description: &String) -> Option<ErrorKind> {
		let mut max_id = 1;

		for task in self.list.iter() {
			println!("{}", task.id);
			if task.id >= max_id {
				max_id = task.id + 1;
			}
		}

		let file_str = fs::read_to_string(DATA_FILE).unwrap();
		let mut file = File::create(DATA_FILE).unwrap();
		file.write_all(file_str.as_bytes()).unwrap();
		if file_str != "" {
			file.write_all(b"\n").unwrap();
		}

		let data = String::from(max_id.to_string() + "$$" + title + "$$" + description);

		file.write_all(data.as_bytes()).unwrap();

		None
	}
}