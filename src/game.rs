use std::sync::{Arc, Mutex};

use futures_util::future::join_all;
use tokio::spawn;

use crate::{random_generator::RandomGenerator, subject::Subject, observer::Observer};

pub struct Game {
}

impl Game {
    pub async fn start(generators: Vec<RandomGenerator>, scoreboard: &Arc<Mutex<Box<dyn Observer<u64> + Send + 'static>>>) { 
        let mut handles = vec![];
        for mut generator in generators {
            let cloned_scoreboard = scoreboard.clone();
            let handle = spawn(async move {
                generator.add_observer(cloned_scoreboard);
                generator.start().await;
            });

            handles.push(handle);
        }

        join_all(handles).await;
    }
}