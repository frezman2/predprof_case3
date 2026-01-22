mod db;
mod models;
mod update;
mod passing_score;
mod generator;

use std::collections::HashMap;
use db::create_pool;
use generator::generate_applications;
use update::update_admission_day;
use passing_score::calculate_passing_scores;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Admission analysis system ===");

    // 1️⃣ Подключение к БД
    let pool = create_pool().await;
    println!("DB connected");

    // 2️⃣ Генерация данных (эмуляция Excel)
    let day_id = 1;
    let program_id = 101;

    let applications = generate_applications(
        program_id,
        1_000_000,
        120,
    );

    println!("Generated {} applications", applications.len());

    // 3️⃣ Обновление БД
    update_admission_day(
        &pool,
        day_id,
        program_id,
        applications.clone(),
    )
    .await?;

    println!("Database updated");

    // 4️⃣ Лимиты мест
    let mut limits = HashMap::new();
    limits.insert(program_id, 80);

    // 5️⃣ Расчёт проходного балла
    let passing_scores =
        calculate_passing_scores(applications, &limits);

    // 6️⃣ Вывод результата
    for (program, score) in passing_scores {
        match score {
            Some(s) => println!(
                "Program {} passing score = {}",
                program, s
            ),
            None => println!(
                "Program {} has under-enrollment",
                program
            ),
        }
    }

    println!("=== Finished ===");
    Ok(())
}
