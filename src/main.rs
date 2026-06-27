use std::io::Write;

use clap::Parser;
use itertools::Itertools;

use crate::{
    person::{CourseId, Person},
    solver::Solver,
};

mod constraint;
mod group;
mod permutation;
mod person;
mod solver;
mod weights;

const N_COURSES: usize = 6;
const N_TIME_SLOTS: usize = 6;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {}

fn main() {
    let args = Args::parse();

    let persons = (0..N_COURSES as CourseId).flat_map(|course_id| {
        let kl_a = Person::kl(course_id, 0, None);
        let kl_b = Person::kl(course_id, 1, None);
        [kl_a, kl_b]
    });

    let solver = Solver::new(persons, N_COURSES, N_TIME_SLOTS);
    let result = solver.solve();

    // check for more than one course visit
    for person_idx in 0..result.num_persons() {
        for course_id in 0..N_COURSES as CourseId {
            let visits = result.course_visits_for(course_id, person_idx);
            if visits > 1 {
                println!(
                    "{} had to visit course {} {} times",
                    result.get_person(person_idx),
                    course_id + 1,
                    visits
                );
            }
        }

        if person_idx < result.num_persons() - 1 {
            for other_person_idx in person_idx + 1..result.num_persons() {
                let pairings = result.pairings_for(person_idx, other_person_idx);
                if pairings > 1 {
                    println!(
                        "{} and {} had {} pairings",
                        result.get_person(person_idx),
                        result.get_person(other_person_idx),
                        pairings
                    );
                }
            }
        }
    }

    // create output csv file with output-{random}.csv
    let current_local = chrono::Local::now();
    let output_path = format!("output_{}.csv", current_local.format("%H-%M-%S"));
    let mut output_file = std::fs::File::create(&output_path).unwrap();

    let _ = writeln!(
        output_file,
        "Kurs 1;Kurs 2;Kurs 3;Kurs 4;Kurs 5;Kurs 6;Score"
    );
    for round in result.rounds() {
        for g in round
            .groups()
            .iter()
            .sorted_by_key(|g| g.group().course_id())
        {
            for person_idx in g.group().person_indices() {
                let person = result.get_person(*person_idx);
                let _ = write!(output_file, "{}, ", person);
            }

            let _ = write!(output_file, ";");
        }

        let _ = writeln!(output_file, "{}", round.score());
    }

    let _ = output_file.flush();
}
