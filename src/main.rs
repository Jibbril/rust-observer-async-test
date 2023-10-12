
mod observer;
mod subject;
mod participant;
mod scoreboard;
mod random_generator;

use std::vec;

use anyhow::Result;
use random_generator::RandomGenerator;
use tokio::try_join;

#[tokio::main]
async fn main() -> Result<()> {
    let generator_1 = RandomGenerator::new(1); 
    let generator_2 = RandomGenerator::new(2); 

    let handle_1 = tokio::spawn(async move {
        generator_1.start().await;
    });

    let handle_2 = tokio::spawn(async move {
        generator_2.start().await;
    });

     try_join!(handle_1, handle_2)?;

    Ok(())
}