use std::sync::{Arc, Mutex};
use crate::observer::Observer;

pub trait Subject<T> {
    fn add_observer(&mut self, observer: Arc<Mutex<Box<dyn Observer<T> + Send>>>);
    fn notify_observers(&mut self, value: T);
}