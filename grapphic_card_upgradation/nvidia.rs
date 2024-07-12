use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Data {
    value: i32,
}

impl Data {
    fn new(value: i32) -> Self {
        Data { value }
    }

    fn update(&mut self, delta: i32) {
        self.value += delta;
    }
}

trait Processor {
    fn process(&self, data: &mut Data);
}

struct Adder {
    amount: i32,
}

impl Processor for Adder {
    fn process(&self, data: &mut Data) {
        data.update(self.amount);
    }
}

struct Multiplier {
    factor: i32,
}

impl Processor for Multiplier {
    fn process(&self, data: &mut Data) {
        data.update(data.value * (self.factor - 1));
    }
}

fn run_processors<'a, T: Processor + Send + 'a>(processors: Vec<&'a T>, data: Arc<Mutex<Data>>) {
    let mut handles = vec![];

    for processor in processors {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data.lock().unwrap();
            processor.process(&mut data);
            println!("Processed data: {:?}", *data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    let data = Arc::new(Mutex::new(Data::new(1)));

    let adder = Adder { amount: 5 };
    let multiplier = Multiplier { factor: 3 };

    let processors: Vec<&dyn Processor> = vec![&adder, &multiplier];

    println!("Original data: {:?}", data.lock().unwrap());

    run_processors(processors, data.clone());

    thread::sleep(Duration::from_secs(1)); // Giving some time for threads to finish

    println!("Final data: {:?}", data.lock().unwrap());
}
