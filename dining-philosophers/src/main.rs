mod philosopher;
mod table;

use std::thread;

fn main() {
    let p1 = philosopher::Philosopher::new("Judith Butler");
    let p2 = philosopher::Philosopher::new("Gilles Deleuze");
    let p3 = philosopher::Philosopher::new("Karl Marx");
    let p4 = philosopher::Philosopher::new("Emma Goldman");
    let p5 = philosopher::Philosopher::new("Michel Foucault");

    let philosophers = vec![p1,p2,p3,p4,p5];

    let handles: Vec<_> = philosophers.into_iter().map(|philosopher| {
        thread::spawn(move || {
            philosopher.eat();
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
