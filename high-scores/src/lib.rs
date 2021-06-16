#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|x| *x)
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max_by(|x,y| x.cmp(y)).map(|x| *x)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.clone().to_vec();
        scores.sort();
        scores.iter().rev().take(3).map(|x| *x).collect()
    }
}
