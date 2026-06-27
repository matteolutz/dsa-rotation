use itertools::Itertools;
use solver::{CourseId, Person, Solver};
use tauri::async_runtime::spawn_blocking;

#[derive(serde::Serialize)]
struct SolveResponse {
    result: Vec<Vec<Vec<String>>>,
    total_score: usize,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn solve(n_time_slots: usize, persons: Vec<Vec<String>>) -> Result<SolveResponse, String> {
    let n_courses = persons.len();
    let persons = persons
        .into_iter()
        .enumerate()
        .flat_map(|(course_idx, course)| {
            course
                .into_iter()
                .enumerate()
                .map(move |(person_course_idx, person_name)| {
                    Person::kl(
                        course_idx as CourseId,
                        person_course_idx as u8,
                        (!person_name.is_empty()).then_some(person_name.as_str()),
                    )
                })
        });

    let solver = Solver::new(persons, n_courses, n_time_slots);
    let result = spawn_blocking(move || solver.solve())
        .await
        .map_err(|e| e.to_string())?;

    let total_score = result.total_score();

    let result = result
        .rounds()
        .iter()
        .map(|round| {
            round
                .groups()
                .iter()
                .sorted_by_key(|g| g.group().course_id())
                .map(|group| {
                    group
                        .group()
                        .persons(result.persons())
                        .map(|p| p.to_short_string())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(SolveResponse {
        result,
        total_score,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![solve])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
