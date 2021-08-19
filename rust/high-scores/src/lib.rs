#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores().last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores().iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three: Vec<u32> = Vec::new();
        let scores_len = self.scores.len();

        if scores_len == 0 {
            return top_three;
        }

        let mut top_x_count = 3;
        if scores_len < 3 {
            top_x_count = scores_len;
        }

        let mut ref_scores = self.scores.to_vec();
        while top_three.len() != top_x_count {
            if let Some(max) = ref_scores.iter().max() {
                if let Some(index) = ref_scores.iter().position(|x| x == max) {
                    top_three.push(*max);
                    ref_scores.remove(index);
                }
            }
        }
        top_three
    }
}
