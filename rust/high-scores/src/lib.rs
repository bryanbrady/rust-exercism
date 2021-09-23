use std::cmp;

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    last: Option<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: Vec::from(scores),
            last: match scores.len() {
                0 => None,
                _ => Some(scores[scores.len()-1])
            }
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores[..]
    }

    pub fn latest(&self) -> Option<u32> {
        self.last
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.iter().max() {
            Some(x) => Some(*x),
            None => None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scores.to_vec();
        sorted.sort();
        sorted.reverse();
        sorted[0..cmp::min(3, self.scores.len())].to_vec()
    }
}
