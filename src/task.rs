#[derive(Debug, Deserialize)]

pub struct Task {
    pub title: String,
    pub notes: Option<String>,
}