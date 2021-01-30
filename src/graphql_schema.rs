// graphql_schema.rs
use juniper::{EmptyMutation, RootNode};

struct Jurisdiction {
	id: i32,
	// parent: i32,
	name: String,
	source: String,
	public_link: String,
}

#[juniper::object(description = "A member of a team")]
impl Jurisdiction {
	pub fn id(&self) -> i32 {
	self.id  
	}

	pub fn name(&self) -> &str {
	self.name.as_str()
	}

	pub fn source(&self) -> &str {
	self.source.as_str()
	}

	pub fn public_link(&self) -> &str {
	self.public_link.as_str()
	}
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
	fn jurisdictions() -> Vec<Jurisdiction> {
	vec![
		Jurisdiction {
		id: 1,
		name: "Test1".to_owned(),
		source: "https://127.0.0.1/source".to_owned(),
		public_link: "https://127.0.0.1/link".to_owned(),
		},
		Jurisdiction {
		id: 2,
		name: "Test2".to_owned(),
		source: "https://127.0.0.1/source".to_owned(),
		public_link: "https://127.0.0.1/link".to_owned(),
		}
	]
	}
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
	Schema::new(QueryRoot {}, EmptyMutation::new())
}