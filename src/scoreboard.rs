use crate::participant::Participant;

pub struct ScoreBoard {
    participants: Vec<Participant>,
}

impl ScoreBoard {
    pub fn start() {
        println!("ScoreBoard started");
    }
}
