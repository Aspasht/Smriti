#[derive(Debug)]
pub struct Command {
    pub id: i64,
    pub command: String,
    pub alias: String,
    pub info: String,
    pub service: String,
}
