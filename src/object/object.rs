#[derive(Clone)]

pub struct Object {
    name: String,
    description: String,
}

impl Object {
    pub fn init(name: &str, description: &str) -> Object {
        Object {
            name: name.to_string(),
            description: description.to_string(),
        }
    }

    pub fn just_name(name: &str) -> Object {
        Object {
            name: name.to_string(),
            description: "".to_string(),
        }
    }
}

