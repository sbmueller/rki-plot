/// Use this module to expand the program with other
/// data sources.
/// New structs require a date, new_cases and r_value field.
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Nowcasting {
    #[serde(rename = "Datum")]
    pub date: String,
    #[serde(rename = "Schätzer_Neuerkrankungen")]
    pub new_cases: u32,
    #[serde(rename = "Schätzer_Reproduktionszahl_R")]
    pub r_value: f32,
}
