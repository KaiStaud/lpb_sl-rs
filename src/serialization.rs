pub mod serde_helpers {
    use error_stack::{IntoReport, ResultExt};
    use serde::{Deserialize, Serialize};
    use std::{error::Error, fmt};

    #[derive(Debug)]
    pub struct ParseConfigError;

    impl fmt::Display for ParseConfigError {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.write_str("Could not parse configuration file")
        }
    }

    //impl Context for ParseConfigError {}
    impl Error for ParseConfigError {}

    #[derive(Debug)]
    pub struct TimedCoordinates {
        pub name: String,
        pub timestamp: u8,
        pub vector: Vec<i32>,
        pub rotation: Vec<i32>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SerdeVector {
        pub vectors: Vec<i32>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SerdeRotations {
        rotations: Vec<i32>,
    }

    pub fn serialize_vector(v: SerdeVector) -> error_stack::Result<String, ParseConfigError> {
        let j = serde_json::to_string(&v)
            .into_report()
            .change_context(ParseConfigError)?;
        Ok(j)
    }

    pub fn serialize_rotations(r: SerdeRotations) -> error_stack::Result<String, ParseConfigError> {
        let j = serde_json::to_string(&r)
            .into_report()
            .change_context(ParseConfigError)?;
        Ok(j)
    }

    pub fn deserialize_vector(data: &str) -> error_stack::Result<SerdeVector, ParseConfigError> {
        let p = serde_json::from_str(data)
            .into_report()
            .change_context(ParseConfigError)?;
        Ok(p)
    }

    pub fn deserialize_rotations(
        data: &str,
    ) -> error_stack::Result<SerdeRotations, ParseConfigError> {
        let p = serde_json::from_str(data)
            .into_report()
            .change_context(ParseConfigError)?;
        Ok(p)
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CheckboxConfig {
        pub checkbox_enabled: Vec<bool>,
        pub checkbox_types: Vec<String>,
        pub checkbox_vector: Vec<u32>,
        pub checkbox_rotation: Vec<u32>,
    }

    pub fn deserialize_into_config(
        path: &str,
    ) -> error_stack::Result<CheckboxConfig, ParseConfigError> {
        use std::fs;
        let contents = fs::read_to_string(path)
            .into_report()
            .change_context(ParseConfigError)?;
        println!("Read from file:{contents}");
        let p: CheckboxConfig = serde_json::from_str(&contents)
            .into_report()
            .change_context(ParseConfigError)?;
        Ok(p)
    }
}

pub mod db_abstraction {

    use error_stack::{IntoReport, ResultExt};
    use sqlx::sqlite::SqlitePool;
    use std::{error::Error, fmt};
    #[derive(Debug)]
    pub struct DBTransactionError;

    impl fmt::Display for DBTransactionError {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.write_str("Could not parse configuration file")
        }
    }

    //impl Context for ParseConfigError {}
    impl Error for DBTransactionError {}

    use super::serde_helpers::*;

    pub fn connect_db() -> () {
        todo!("Replace with something like :let pool = SqlitePool::connect().await;")
    }

    pub async fn add_constellation(
        pool: &SqlitePool,
        v: SerdeVector,
        r: SerdeRotations,
        next_cst: i32,
        alias: String,
    ) -> error_stack::Result<String, DBTransactionError> {
        let mut conn = pool
            .acquire()
            .await
            .into_report()
            .change_context(DBTransactionError)?;

        let vector = serialize_vector(v)
            .change_context(DBTransactionError)
            .attach_printable(format!("vector could not be parsed."))?;

        let rotation = serialize_rotations(r)
            .change_context(DBTransactionError)
            .attach_printable(format!("roation could not be parsed."))?;

        // Insert the task, then obtain the ID of this row
        let id = sqlx::query!(
            r#"
    INSERT INTO nodes ( alias,vectors,rotations,following_node)
    VALUES ( ?1,?2,?3,?4 )
            "#,
            "name",
            vector,
            rotation,
            1,
        )
        .execute(&mut conn)
        .await
        .into_report()
        .change_context(DBTransactionError)?;
        Ok(alias)
    }
    //.last_insert_rowid();

    pub fn get_constellation(id: i32) -> error_stack::Result<TimedCoordinates, DBTransactionError> {
        todo!()
    }
}
