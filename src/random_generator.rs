use std::sync::{Arc, Mutex};

use rand::random;
use tokio::time::{sleep,Duration};

use crate::{subject::Subject, observer::Observer};

pub struct RandomGenerator {
    pub id: u64,
    pub observers: Arc<Mutex<Vec<Box<dyn Observer<u64> + Send>>>>,
}

impl RandomGenerator {
    pub async fn start(&self) {
        loop {
            let sleep_time = random::<u64>() % 10;
            println!("Generator {} sleept for {:#?} seconds", self.id, sleep_time);
            sleep(Duration::from_secs(sleep_time)).await;
            self.notify_observers(sleep_time);
        }
    }

    pub fn new(id: u64) -> Self {
        Self {
            id,
            observers: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl Subject<u64> for RandomGenerator {
    fn add_observer(&mut self, observer: Box<dyn Observer<u64> + Send>) {
        self.observers.lock().unwrap().push(observer);
    }

    fn notify_observers(&self, value: u64) {
        for observer in self.observers.lock().unwrap().iter_mut() {
            observer.update(value);
        }
    }
}

