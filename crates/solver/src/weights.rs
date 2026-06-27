use std::sync::Arc;

use crate::person::{CourseId, Person};

#[derive(Debug, Clone)]
pub struct PTPWeights(Vec<Vec<f32>>);

impl PTPWeights {
    pub fn get(&self, person_a_idx: usize, person_b_idx: usize) -> Option<f32> {
        self.0
            .get(person_a_idx)
            .and_then(|map| map.get(person_b_idx))
            .copied()
    }

    pub fn add(&mut self, person_a_idx: usize, person_b_idx: usize, amount: f32) {
        // A -> B
        *self
            .0
            .get_mut(person_a_idx)
            .unwrap()
            .get_mut(person_b_idx)
            .unwrap() += amount;

        // B -> A
        *self
            .0
            .get_mut(person_b_idx)
            .unwrap()
            .get_mut(person_a_idx)
            .unwrap() += amount;
    }
}

impl<P> From<P> for PTPWeights
where
    P: AsRef<[Arc<Person>]>,
{
    fn from(persons: P) -> Self {
        let persons = persons.as_ref();
        let weights = persons
            .iter()
            .map(|person_a| {
                persons
                    .iter()
                    .map(|person_b| {
                        if person_a == person_b || person_a.is_pairing_forbidden(person_b) {
                            f32::INFINITY
                        } else {
                            0.0
                        }
                    })
                    .collect()
            })
            .collect();

        Self(weights)
    }
}

#[derive(Debug, Clone)]
pub struct PTCWeights(Vec<Vec<f32>>);

impl PTCWeights {
    pub fn new(persons: impl AsRef<[Arc<Person>]>, n_courses: usize) -> Self {
        let persons = persons.as_ref();

        let weights = (0..n_courses)
            .map(|course_id| {
                persons
                    .iter()
                    .map(|person| {
                        if person.is_course_forbidden(course_id as CourseId) {
                            f32::INFINITY
                        } else {
                            0.0
                        }
                    })
                    .collect()
            })
            .collect();

        Self(weights)
    }

    pub fn get(&self, course_id: CourseId, person_idx: usize) -> Option<f32> {
        self.0
            .get(course_id as usize)
            .and_then(|map| map.get(person_idx))
            .copied()
    }

    pub fn add(&mut self, course_id: CourseId, person_idx: usize, amount: f32) {
        *self
            .0
            .get_mut(course_id as usize)
            .unwrap()
            .get_mut(person_idx)
            .unwrap() += amount;
    }
}
