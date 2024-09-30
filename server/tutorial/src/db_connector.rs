use super::models::*;
use super::schema::*;
use diesel::dsl::date;
use diesel::dsl::now;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

pub fn create_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_project(conn: &PgConnection, name: &str, url_name: &str) -> Result<Project, Error> {
    let new_project = NewProject { name, url_name };

    // https://docs.rs/diesel/latest/diesel/fn.insert_into.html
    diesel::insert_into(projects::table)
        .values(&new_project)
        .get_result(conn)
}

pub fn insert_technology(
    conn: &PgConnection,
    name: &str,
    url_name: &str,
    image_url: &str,
) -> Result<Technology, Error> {
    let new_technology = NewTechnology {
        name,
        url_name,
        image_url,
    };
    diesel::insert_into(technologies::table)
        .values(&new_technology)
        .get_result(conn)
}

pub fn insert_adopt(
    conn: &PgConnection,
    projects_id: i64,
    technologies_id: i64,
    created_at: chrono::NaiveDateTime,
) -> Result<Adopt, Error> {
    let new_adopt = NewAdopt {
        projects_id,
        technologies_id,
        created_at,
    };
    diesel::insert_into(adopts::table)
        .values(&new_adopt)
        .get_result(conn)
}

fn get_technology_by_url_name(
    conn: &PgConnection,
    tech_url_name: &str,
) -> Result<Technology, Error> {
    technologies::table
        .filter(technologies::url_name.eq(tech_url_name))
        .first(conn)
}

fn get_projects_by_technologies_id(
    conn: &PgConnection,
    tech_id: i64,
) -> Result<Vec<Project>, Error> {
    projects::table
        .inner_join(adopts::table.on(projects::id.eq(adopts::projects_id)))
        .filter(adopts::technologies_id.eq(tech_id))
        .select(projects::all_columns)
        .load(conn)
}

// tuple: https://doc.rust-jp.rs/rust-by-example-ja/primitives/tuples.html
pub fn get_technology_page_by_url_name(
    conn: &PgConnection,
    tech_url_name: &str,
) -> Result<(Technology, Vec<Project>), Error> {
    // ref: https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html
    let tech = get_technology_by_url_name(conn, tech_url_name)?;
    let projects = get_projects_by_technologies_id(conn, tech.id)?;

    Ok((tech, projects))
}

// pub fn test_transaction<F, R>(test_fn: F) -> Result<R, Error>
// where
//     F: FnOnce(&PgConnection) -> Result<R, Error>,
//
// ↑ は以下のように書き換えられる
pub fn test_transaction<F: FnOnce(&PgConnection) -> Result<R, Error>, R>(
    test_fn: F,
) -> Result<R, Error> {
    let conn = create_connection();
    conn.transaction(|| test_fn(&conn))
}

#[cfg(test)]
#[allow(non_snake_case)]
mod unit_DB_test {
    use super::*;

    use chrono::NaiveDate as NativeDate;

    #[test]
    fn get_technology_page_by_url_name_関数はTechnologyページの情報をDBから一括で取得する() {
        test_transaction(|conn| {
            // Arrange
            let project_name = "TestProject";
            let project_url_name = "test_project";
            let tech_name = "TestTechnology";
            let tech_url_name = "test_technology";
            let tech_image_url = "https://example.com/image.jpg";
            let dt = NativeDate::from_ymd_opt(2020, 1, 1)
                .unwrap()
                .and_hms_opt(8, 19, 34)
                .unwrap();

            let new_project = insert_project(&conn, project_name, project_url_name)?;
            let new_technology =
                insert_technology(&conn, tech_name, tech_url_name, tech_image_url)?;
            let _ = insert_adopt(&conn, new_project.id, new_technology.id, dt)?;

            // Act
            let (tech, projects) = get_technology_page_by_url_name(&conn, tech_url_name)?;

            // Assert
            let project = &projects[0];
            assert_eq!(tech.name, tech_name);
            assert_eq!(tech.url_name, tech_url_name);
            assert_eq!(tech.image_url, tech_image_url);
            assert_eq!(project.name, project_name);
            assert_eq!(project.url_name, project_url_name);

            Ok(())
        })
        .unwrap();
    }
}
