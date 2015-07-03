#[macro_use]
extern crate rustless;

extern crate iron;
extern crate url;
extern crate rustc_serialize as serialize;
extern crate valico;
extern crate cookie;

mod tasks;

use std::fmt;
use std::error;
use std::error::Error as StdError;
use valico::json_dsl;

use rustless::server::status;
use rustless::errors::{Error};
use rustless::{Nesting};

use serialize::json::ToJson;

#[derive(Debug)]
pub struct UnauthorizedError;

impl error::Error for UnauthorizedError {
	fn description(& self) -> &str {
		return "UnauthorizedError";
	}
}

impl fmt::Display for UnauthorizedError {
	fn fmt(& self, formatter: &mut fmt::Formatter) -> fmt::Result {
				self.description().fmt(formatter)
	}
}



fn main() {

	let app = rustless::Application::new(rustless::Api::build(|api| {

		api.get("tasks", |endpoint| {
			endpoint.handle(|client, params| {
				let tasks = &tasks::load_tasks();
				client.json(&tasks.to_json())
			})
		});


		api.post("greet/:name", |endpoint| {
			endpoint.summary("Sends greeting");
			endpoint.desc("Use this to talk to yourself");
			endpoint.params(|params| {
				params.req_typed("name", json_dsl::string());
				params.req_typed("greeting", json_dsl::string());
			});

			endpoint.handle(|client, params| {
				client.text(
					format!("{}, {}",
							params.find("greeting").unwrap().to_string(),
							params.find("name").unwrap().to_string())
				)
			})
		});


		api.get("echo", |endpoint| {
			endpoint.summary("Sends back what it gets");
			endpoint.desc("Use this to talk to yourself");
			endpoint.handle(|client, params| {
					client.json(params)
			})
		});


	}));

	let chain = iron::Chain::new(app);
	let iron = iron::Iron::new(chain);
	let handler = iron.http("localhost:4000");
	let result = handler.unwrap();

	println!("On 4000");

}