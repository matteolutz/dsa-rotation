use std::sync::Arc;

use itertools::Itertools;
use rand::RngExt;

use crate::{
    person::{CourseId, Person},
    weights::{PTCWeights, PTPWeights},
};

#[derive(Debug, Clone)]
pub struct Group {
    course_id: CourseId,
    person_indices: Vec<usize>,
}

impl Group {
    pub fn new(course_id: CourseId, persons: impl IntoIterator<Item = usize>) -> Self {
        Self {
            course_id,
            person_indices: persons.into_iter().collect(),
        }
    }

    pub fn course_id(&self) -> CourseId {
        self.course_id
    }

    pub fn person_indices(&self) -> &[usize] {
        &self.person_indices
    }

    pub fn persons<'a>(
        &'a self,
        all_persons: &'a [Arc<Person>],
    ) -> impl Iterator<Item = &'a Arc<Person>> + 'a {
        self.person_indices.iter().map(|i| &all_persons[*i])
    }

    pub fn random_person<'a>(&'a self, all_persons: &'a [Arc<Person>]) -> &'a Arc<Person> {
        let index = self.person_indices[rand::rng().random_range(0..self.person_indices.len())];
        &all_persons[index]
    }

    pub fn swap_random_person_with(&mut self, other: &mut Group) {
        let own_index = rand::rng().random_range(0..self.person_indices.len());
        let other_index = rand::rng().random_range(0..other.person_indices.len());

        let tmp = self.person_indices[own_index];
        self.person_indices[own_index] = other.person_indices[other_index];
        other.person_indices[other_index] = tmp;
    }

    pub fn score(self, ptp_weights: &PTPWeights, ptc_weights: &PTCWeights) -> ScoredGroup {
        let mut score = 0.0;

        // PTP weights
        for comb in self.person_indices.iter().combinations(2).into_iter() {
            let (a, b) = (comb[0], comb[1]);
            score += ptp_weights.get(*a, *b).unwrap();
        }

        // PTC weights
        for person in self.person_indices.iter() {
            score += ptc_weights.get(self.course_id, *person).unwrap();
        }

        ScoredGroup { group: self, score }
    }
}

#[derive(Debug, Clone)]
pub struct ScoredGroup {
    group: Group,
    score: f32,
}

impl ScoredGroup {
    pub fn group(&self) -> &Group {
        &self.group
    }

    pub fn score(&self) -> f32 {
        self.score
    }
}

impl From<ScoredGroup> for Group {
    fn from(value: ScoredGroup) -> Self {
        value.group
    }
}
