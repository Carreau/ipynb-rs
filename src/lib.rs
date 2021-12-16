use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Notebook {
    pub cells: Vec<Cell>,
    pub metadata: HashMap<String, Value>,
    pub nbformat: usize,
    pub nbformat_minor: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "cell_type")]
pub enum Cell {
    #[serde(rename = "markdown")]
    Markdown(MarkdownCell),
    #[serde(rename = "code")]
    Code(CodeCell),
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "output_type")]
#[serde(rename_all = "snake_case")]
pub enum Output {
    ExecuteResult(ExecuteResultOutput),
    Stream(StreamOutput),
    DisplayData(DisplayDataOutput),
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodeCell {
    pub metadata: HashMap<String, Value>,
    pub source: Vec<String>,
    pub execution_count: usize,
    pub outputs: Vec<Output>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreamOutput {
    pub data: Option<HashMap<String, Value>>,
    pub metadata: Option<HashMap<String, Value>>,
    pub name: Option<String>,
    pub text: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecuteResultOutput {
    pub data: Option<HashMap<String, Value>>,
    pub metadata: Option<HashMap<String, Value>>,
    pub name: Option<String>,
    pub execution_count: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisplayDataOutput {
    pub data: Option<HashMap<String, Value>>,
    pub metadata: Option<HashMap<String, Value>>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarkdownCell {
    pub metadata: HashMap<String, Value>,
    pub source: Vec<String>,
}

#[cfg(test)]
mod tests {

    use super::Notebook;

    #[test]
    fn it_works() {
        let data = include_str!("test4.ipynb");
        let notebook: Notebook = serde_json::from_str(data).unwrap();
        assert_eq!(notebook.nbformat, 4);
    }
}
