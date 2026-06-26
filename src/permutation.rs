use crate::group::{Group, ScoredGroup};

pub type Permutation = Vec<Group>;

#[derive(Debug, Clone)]
pub struct ScoredPermutation {
    pub(super) groups: Vec<ScoredGroup>,
    pub(super) score: f32,
}

impl ScoredPermutation {
    pub fn groups(&self) -> &[ScoredGroup] {
        &self.groups
    }

    pub fn score(&self) -> f32 {
        self.score
    }
}

impl From<ScoredPermutation> for Permutation {
    fn from(value: ScoredPermutation) -> Self {
        value.groups.into_iter().map(|g| g.into()).collect()
    }
}
