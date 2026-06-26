use std::{io::Write, sync::Arc};

use itertools::Itertools;

use crate::{
    person::{CourseId, Person},
    solver::Solver,
};

mod group;
mod person;
mod solver;
mod weights;

const N_COURSES: usize = 6;
const N_TIME_SLOTS: usize = 6;

fn main() {
    let persons: Vec<Arc<Person>> = (0..N_COURSES as CourseId)
        .flat_map(|course_id| {
            let kl_a = Arc::new(Person::kl(course_id, 0, "KL A"));
            let kl_b = Arc::new(Person::kl(course_id, 1, "KL B"));
            [kl_a, kl_b]
        })
        .collect();

    let mut solver = Solver::<N_COURSES>::new(persons, N_TIME_SLOTS);
    let rounds = solver.solve();

    // create output csv file with output-{random}.csv
    let output_path = format!("output-{}.csv", rand::random_range(1000000..9999999));
    let mut output_file = std::fs::File::create(&output_path).unwrap();

    writeln!(output_file, "Kurs 1;Kurs 2;Kurs 3;Kurs 4;Kurs 5;Kurs 6;");
    for round in rounds {
        for g in round
            .groups()
            .iter()
            .sorted_by_key(|g| g.group().course_id())
        {
            for person_idx in g.group().person_indices() {
                let person = solver.get_person(*person_idx);
                write!(output_file, "{}, ", person);
            }

            write!(output_file, ";");
        }

        writeln!(output_file, "");
    }

    output_file.flush();
}
