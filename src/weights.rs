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
                        if person_a == person_b || person_a.is_forbidden_with(person_b) {
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
pub struct PTCWeights<const N: usize>([Vec<f32>; N]);

impl<const N: usize> PTCWeights<N> {
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

impl<P, const N: usize> From<P> for PTCWeights<N>
where
    P: AsRef<[Arc<Person>]>,
{
    fn from(persons: P) -> Self {
        let persons = persons.as_ref();

        let weights: [u8; N] = std::array::from_fn(|i| i as u8);
        let weights = weights.map(|course_id| {
            persons
                .iter()
                .map(|person| {
                    if person.is_in_course(course_id) {
                        f32::INFINITY
                    } else {
                        0.0
                    }
                })
                .collect()
        });

        Self(weights)
    }
}
