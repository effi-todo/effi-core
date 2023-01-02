use std::error;

use rand::{self, distributions::Alphanumeric, Rng};

use crate::EffiCoreError;

pub type Id = [char; 4];

pub fn generate_id() -> Id {
    let mut id: Id = [' ', ' ', ' ', ' '];

    for i in 0..4 {
        id[i] = rand::thread_rng().sample(&Alphanumeric).into();
    }

    return id;
}

pub fn id_to_string(id: &Id) -> String {
    return format!("{}{}{}{}", id[0], id[1], id[2], id[3]);
}

pub fn string_to_id(id: &str) -> Result<Id, Box<dyn error::Error + '_>> {
    if id.len() != 4 {
        return Err(Box::new(EffiCoreError::InvalidStringForId(
            id.to_string(),
            "Invalid length".to_string(),
        )));
    };
    let id = id.as_bytes();
    return Ok([id[0] as char, id[1] as char, id[2] as char, id[3] as char]);
}
