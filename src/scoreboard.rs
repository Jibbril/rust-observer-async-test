use rand::random;
use crate::{participant::Participant, observer::Observer};

#[derive(Debug,Clone)]
pub struct ScoreBoard {
    pub id: u64,
    pub participants: Vec<Participant>,
}

impl ScoreBoard {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            participants: vec![]
        }
    }

    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
    }
}

impl Observer<u64> for ScoreBoard {
    fn update(&mut self, value: u64) {
        let index = random::<usize>() % self.participants.len();

        self.participants[index].score += value;

        // Print scores
        let mut scores_line = String::new();
        for participant in self.participants.iter() {
            scores_line.push_str(&format!("{: <3} ", participant.score));
        }
        println!("{}", scores_line);
         
    }
}