#[derive(Serialize, Deserialize, Default)]
pub struct QHyperCubeDef {
    #[serde(rename = "qStateName")] q_state_name: Option<String>,

    #[serde(rename = "qDimensions")] q_dimensions: Vec<QDimension>,

    #[serde(rename = "qMeasures")] q_measures: Vec<QMeasure>,

    #[serde(rename = "qInitialDataFetch")] q_initial_data_fetch: Vec<QInitialDataFetch>,
}

#[derive(Serialize, Deserialize)]
pub struct QDimension {
    #[serde(rename = "qLibraryId")] q_library_id: Option<String>,

    #[serde(rename = "qNullSuppression")] q_null_suppression: bool,

    #[serde(rename = "qDef")] q_def: QDimensionQDef,
}

#[derive(Serialize, Deserialize)]
pub struct QDimensionQDef {
    #[serde(rename = "qGrouping")] q_grouping: String,

    #[serde(rename = "qFieldDefs")] q_field_defs: Vec<String>,

    #[serde(rename = "qFieldLabels")] q_field_labels: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct QInitialDataFetch {
    #[serde(rename = "qTop")] q_top: i64,

    #[serde(rename = "qLeft")] q_left: i64,

    #[serde(rename = "qHeight")] q_height: i64,

    #[serde(rename = "qWidth")] q_width: i64,
}

#[derive(Serialize, Deserialize)]
pub struct QMeasure {
    #[serde(rename = "qLibraryId")] q_library_id: String,

    #[serde(rename = "qSortBy")] q_sort_by: QSortBy,

    #[serde(rename = "qDef")] q_def: QMeasureQDef,
}

#[derive(Serialize, Deserialize)]
pub struct QMeasureQDef {
    #[serde(rename = "qLabel")] q_label: String,

    #[serde(rename = "qDescription")] q_description: String,

    #[serde(rename = "qTags")] q_tags: Vec<String>,

    #[serde(rename = "qGrouping")] q_grouping: String,

    #[serde(rename = "qDef")] q_def: String,
}

#[derive(Serialize, Deserialize)]
pub struct QSortBy {
    #[serde(rename = "qSortByState")] q_sort_by_state: i64,

    #[serde(rename = "qSortByFrequency")] q_sort_by_frequency: i64,

    #[serde(rename = "qSortByNumeric")] q_sort_by_numeric: i64,

    #[serde(rename = "qSortByAscii")] q_sort_by_ascii: i64,

    #[serde(rename = "qSortByLoadOrder")] q_sort_by_load_order: i64,

    #[serde(rename = "qSortByExpression")] q_sort_by_expression: i64,

    #[serde(rename = "qExpression")] q_expression: QExpression,
}

#[derive(Serialize, Deserialize)]
pub struct QExpression {
    #[serde(rename = "qv")] qv: String,
}

impl QHyperCubeDef {
    pub fn new() -> QHyperCubeDef {
        QHyperCubeDef {
            q_state_name: Some(String::from("$")),
            ..Default::default()
        }
    }
}
