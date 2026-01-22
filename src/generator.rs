use rand::Rng;
use crate::models::ApplicationRow;

pub fn generate_applications(
    program_id: i32,
    start_id: i32,
    count: usize,
) -> Vec<ApplicationRow> {

    let mut rng = rand::thread_rng();
    let mut result = Vec::new();

    for i in 0..count {
        let physics = rng.gen_range(60..100);
        let russian = rng.gen_range(60..100);
        let math = rng.gen_range(60..100);
        let indiv = rng.gen_range(0..10);

        result.push(ApplicationRow {
            applicant_id: start_id + i as i32,
            program_id,
            priority: rng.gen_range(1..=4),
            has_consent: rng.gen_bool(0.6),
            physics_score: physics,
            russian_score: russian,
            math_score: math,
            individual_score: indiv,
            total_score: physics + russian + math + indiv,
        });
    }

    result
}
