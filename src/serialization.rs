pub mod serialization{
    use std::{fmt, error::Error};
use error_stack::{IntoReport, ResultExt};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ParseConfigError;

impl fmt::Display for ParseConfigError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Could not parse configuration file")
    }
}

//impl Context for ParseConfigError {}
impl Error for ParseConfigError {}

#[derive(Serialize, Deserialize)]
pub struct TimedCoordinates {
    name: String,
    timestamp: u8,
    vector: Vec<u32>,
    rotation: Vec<u32>,

}

pub fn serialize_from_struct()  -> error_stack::Result<String,ParseConfigError>{

    let address = TimedCoordinates {
        name: "todo".to_string(),
        timestamp: 10,
        vector: [1,2,3].to_vec(),
        rotation: [10,20,30].to_vec(),
    };

        // Serialize it to a JSON string.
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&address).into_report().change_context(ParseConfigError)?;

        // Print, write to a file, or send to an HTTP server.
        println!("Serialized into string: {}", j);
        Ok((j))

}

pub fn deserialize_into_struct()  -> error_stack::Result<TimedCoordinates,ParseConfigError>{
        // Some JSON input data as a &str. Maybe this comes from the user.
        let data = r#"
        {
            "name": "John Doe",
            "timestamp": 43,
            "vector": [
                441234567,
                442345678
            ],
            "rotation": [
                551234567,
                552345678
            ]
        }"#;
    let p: TimedCoordinates = serde_json::from_str(data).into_report().change_context(ParseConfigError)?;
    Ok(p)

}
}