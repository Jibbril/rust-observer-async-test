mod game;
mod observer;
mod subject;
mod participant;
mod scoreboard;
mod random_generator;


use std::sync::{Arc, Mutex};

use anyhow::Result;
use game::Game;
use observer::Observer;
use random_generator::RandomGenerator;
use scoreboard::ScoreBoard;

use crate::participant::Participant;

#[tokio::main]
async fn main() -> Result<()> {
    // Init game components
    let generators = vec![
        RandomGenerator::new(1),
        RandomGenerator::new(2),
        RandomGenerator::new(3),
        RandomGenerator::new(4),
        RandomGenerator::new(5),
        RandomGenerator::new(6),
        RandomGenerator::new(7),
        RandomGenerator::new(8),
        RandomGenerator::new(9),
        RandomGenerator::new(10),
        RandomGenerator::new(11),
        RandomGenerator::new(12),
    ];
    let participants = vec![
        Participant::new("A"),
        Participant::new("B"),
        Participant::new("C"),
        Participant::new("D"),
        Participant::new("E"),
        Participant::new("F"),
        Participant::new("G"),
        Participant::new("H"),
        Participant::new("I"),
        Participant::new("J"),
    ];
    let mut board = ScoreBoard::new(1);

    // Add participants to board 
    for participant in participants {
        board.add_participant(participant);
    }

    // Create a line of participant names
    let mut names_line = String::new();
    for participant in board.participants.iter() {
        names_line.push_str(&format!("{: <3} ", participant.name));
    }
    println!("{}", names_line);

    // Put board onto the heap 
    let board: Box<dyn Observer<u64> + Send> = Box::new(board);
    let board = Arc::new(Mutex::new(board));

    // Start the game
    Game::start(generators, &board).await;

    Ok(())
}