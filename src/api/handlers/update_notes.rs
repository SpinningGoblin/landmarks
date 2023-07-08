use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateNotes {
    pub notes: String,
}
