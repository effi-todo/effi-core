use rand::{self, distributions::Alphanumeric, Rng};

pub type Id = [char; 4];

pub fn generate_id() -> Id {
    let mut id: Id = [' ', ' ', ' ', ' '];

    for i in 0..4 {
        id[i] = rand::thread_rng().sample(&Alphanumeric).into();
    }

    return id;
}
