use std::sync::Arc;

use itertools::Itertools;
use rand::seq::SliceRandom;
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::{
    group::Group,
    permutation::{Permutation, ScoredPermutation},
    person::{CourseId, Person},
    weights::{PTCWeights, PTPWeights},
};

const N_PAIR_SWAP_MUTATIONS: usize = 100;
const N_RANDOM_MUTATIONS: usize = 100;

const N_GENERATIONS: usize = 100;
const N_MAX_DESCENDANTS: usize = 5;

pub struct Solver<const NCOURSES: usize> {
    ptp_weights: PTPWeights,
    ptc_weights: PTCWeights<NCOURSES>,

    n_time_slots: usize,
    group_size: usize,

    persons: Vec<Arc<Person>>,
}

impl<const NCOURSES: usize> Solver<NCOURSES> {
    pub fn new<I, P>(persons: I, n_time_slots: usize) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<Arc<Person>>,
    {
        let persons: Vec<Arc<Person>> = persons.into_iter().map_into().collect();

        if persons.len() % NCOURSES != 0 {
            panic!("Number of persons must be divisible by the number of courses");
        }

        Self {
            ptp_weights: PTPWeights::from(&persons),
            ptc_weights: PTCWeights::from(&persons),

            group_size: persons.len() / NCOURSES,

            persons,
            n_time_slots,
        }
    }

    pub fn get_person(&self, index: usize) -> Arc<Person> {
        self.persons[index].clone()
    }

    fn generate_permutation(&self) -> Permutation {
        // Shuffle the list of persons
        let mut shuffled_persons = (0..self.persons.len()).collect::<Vec<_>>();
        shuffled_persons.shuffle(&mut rand::rng());

        // slice them up into groups of size `group_size`
        shuffled_persons
            .into_iter()
            .chunks(self.group_size)
            .into_iter()
            .enumerate()
            .map(|(coures_id, chunk)| Group::new(coures_id as CourseId, chunk))
            .collect()
    }

    fn score_permutation(&self, permutation: Permutation) -> ScoredPermutation {
        let mut scored_groups = Vec::with_capacity(permutation.len());
        let mut total_score = 0.0;

        for group in permutation {
            let scored_group = group.score(&self.ptp_weights, &self.ptc_weights);
            total_score += scored_group.score();

            scored_groups.push(scored_group);
        }

        ScoredPermutation {
            groups: scored_groups,
            score: total_score,
        }
    }

    fn score_permutation_filtered(&self, permutation: Permutation) -> Option<ScoredPermutation> {
        let scored = self.score_permutation(permutation);
        if scored.score.is_infinite() {
            None
        } else {
            Some(scored)
        }
    }

    fn update_weights<'a>(&mut self, groups: impl IntoIterator<Item = &'a Group>) {
        for group in groups {
            // PTP weights
            for comb in group.person_indices().iter().combinations(2) {
                let (a, b) = (comb[0], comb[1]);
                self.ptp_weights.add(*a, *b, 1.0);
            }

            // PTC weights
            for person in group.person_indices() {
                self.ptc_weights.add(group.course_id(), *person, 5.0);
            }
        }
    }

    fn make_mutations(&self, best_options: Vec<ScoredPermutation>) -> Vec<ScoredPermutation> {
        best_options
            .into_par_iter()
            .flat_map_iter(|parent| {
                std::iter::once(parent.clone())
                    .chain((0..N_PAIR_SWAP_MUTATIONS).filter_map(move |_| {
                        let mut groups: Permutation = parent.clone().into();
                        groups.shuffle(&mut rand::rng());

                        let (front, back) = groups.split_at_mut(1);
                        let (a, b) = (&mut front[0], &mut back[0]);

                        a.swap_random_person_with(b);

                        self.score_permutation_filtered(groups)
                    }))
                    .chain((0..N_RANDOM_MUTATIONS).filter_map(|_| {
                        self.score_permutation_filtered(self.generate_permutation())
                    }))
            })
            .collect()
    }

    fn solve_slot(&mut self) -> ScoredPermutation {
        let mut best_options = (0..5)
            .map(|_| self.score_permutation(self.generate_permutation()))
            .collect::<Vec<_>>();

        for _ in 0..N_GENERATIONS {
            if best_options.first().is_some_and(|best| best.score <= 0.0) {
                // we have found a perfect solution
                break;
            }

            let mut mutations = self.make_mutations(best_options);
            mutations.par_sort_unstable_by(|a, b| a.score.total_cmp(&b.score));

            let best_total_score = mutations.first().unwrap().score;

            best_options = mutations
                .into_iter()
                .take_while(|m| m.score <= best_total_score)
                .collect::<Vec<_>>();
            best_options.shuffle(&mut rand::rng());

            best_options.truncate(N_MAX_DESCENDANTS);
        }

        let the_best = best_options.into_iter().nth(0).unwrap();
        self.update_weights(the_best.groups.iter().map(|g| g.group()));

        the_best
    }

    pub fn solve(&mut self) -> Vec<ScoredPermutation> {
        let mut rounds = Vec::with_capacity(self.n_time_slots);

        for i in 0..self.n_time_slots {
            let round_result = self.solve_slot();

            println!("solved round {}: {}", i, round_result.score());
            // println!("{:#?}", round_result);

            rounds.push(round_result);
        }

        rounds
    }
}
