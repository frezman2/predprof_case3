use sqlx::PgPool;
use crate::models::ApplicationRow;

pub async fn update_admission_day(
    pool: &PgPool,
    day_id: i32,
    program_id: i32,
    rows: Vec<ApplicationRow>,
) -> Result<(), sqlx::Error> {

    let new_ids: Vec<i32> = rows.iter().map(|r| r.applicant_id).collect();

    // УДАЛЕНИЕ
    sqlx::query!(
        r#"
        DELETE FROM applications
        WHERE admission_day_id = $1
          AND program_id = $2
          AND applicant_id NOT IN (SELECT UNNEST($3::int[]))
        "#,
        day_id,
        program_id,
        &new_ids
    )
    .execute(pool)
    .await?;

    // ДОБАВЛЕНИЕ / ОБНОВЛЕНИЕ
    for row in rows {
        sqlx::query!(
            r#"
            INSERT INTO applicants (id)
            VALUES ($1)
            ON CONFLICT (id) DO NOTHING
            "#,
            row.applicant_id
        )
        .execute(pool)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO applications (
                applicant_id, program_id, admission_day_id,
                priority, has_consent,
                physics_score, russian_score, math_score,
                individual_score, total_score
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
            ON CONFLICT (applicant_id, program_id, admission_day_id)
            DO UPDATE SET
                priority = EXCLUDED.priority,
                has_consent = EXCLUDED.has_consent,
                physics_score = EXCLUDED.physics_score,
                russian_score = EXCLUDED.russian_score,
                math_score = EXCLUDED.math_score,
                individual_score = EXCLUDED.individual_score,
                total_score = EXCLUDED.total_score
            "#,
            row.applicant_id,
            program_id,
            day_id,
            row.priority,
            row.has_consent,
            row.physics_score,
            row.russian_score,
            row.math_score,
            row.individual_score,
            row.total_score
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
