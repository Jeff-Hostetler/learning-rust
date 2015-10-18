use std::sync::Mutex;

pub struct Table {
    forks: Vec<Mutex<()>>,
}
