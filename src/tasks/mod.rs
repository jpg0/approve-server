use std::vec::Vec;
use serialize::json::{ToJson, Json};

pub struct Task {
	name:(String),
}

impl ToJson for Task {
	fn to_json(& self) -> Json {
			Json::String(format!("Task: {}", self .name))
	}
}

pub fn load_tasks() -> Vec<Task> {
	let mut a = Vec::new();
	a.push(Task{name: "t1".to_string()});
	a.push(Task{name: "t2".to_string()});
	return a;
}