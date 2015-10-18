use std::thread;

pub struct Philosopher {
    name: String,
}

impl Philosopher {
    pub fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    pub fn eat(&self) {
        println!("{} is eating", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating", self.name);
    }
}
