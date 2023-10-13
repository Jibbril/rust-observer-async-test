use std::sync::{Arc, Mutex};

use rand::random;
use tokio::time::{sleep,Duration};

use crate::{subject::Subject, observer::Observer};

#[derive(Clone)]
pub struct RandomGenerator {
    pub id: u64,
    pub observers: Vec<Arc<Mutex<Box<dyn Observer<u64> + Send>>>>,
}

impl RandomGenerator {
    pub async fn start(&mut self) {
        loop {
            let sleep_time = random::<u64>() % 10 + 1;
            sleep(Duration::from_secs(sleep_time)).await;
            self.notify_observers(sleep_time);
        }
    }

    pub fn new(id: u64) -> Self {
        Self {
            id,
            observers: vec![]
        }
    }
}

impl Subject<u64> for RandomGenerator {
    fn add_observer(&mut self, observer: Arc<Mutex<Box<dyn Observer<u64> + Send>>>) {
        self.observers.push(observer);
    }

    fn notify_observers(&mut self, value: u64) {
        for observer in self.observers.iter_mut() {
            observer.lock().unwrap().update(value);
        }
    }
}

