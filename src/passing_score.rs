use std::collections::HashMap;
use crate::models::ApplicationRow;

pub fn calculate_passing_scores(
    mut apps: Vec<ApplicationRow>,
    program_limits: &HashMap<i32, usize>,
) -> HashMap<i32, Option<i32>> {

    apps.retain(|a| a.has_consent);

    apps.sort_by(|a, b| b.total_score.cmp(&a.total_score));

    let mut admitted: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut used_applicants = std::collections::HashSet::new();

    for app in apps {
        if used_applicants.contains(&app.applicant_id) {
            continue;
        }

        let limit = program_limits.get(&app.program_id).unwrap();

        let entry = admitted.entry(app.program_id).or_default();

        if entry.len() < *limit {
            entry.push(app.total_score);
            used_applicants.insert(app.applicant_id);
        }
    }

    let mut result = HashMap::new();

    for (program, limit) in program_limits {
        let scores = admitted.get(program);

        if scores.is_none() || scores.unwrap().len() < *limit {
            result.insert(*program, None); // НЕДОБОР
        } else {
            result.insert(*program, Some(*scores.unwrap().last().unwrap()));
        }
    }

    result
}
