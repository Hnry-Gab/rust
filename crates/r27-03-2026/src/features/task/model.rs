use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskState {
    ToDo,
    Doing,
    Done
}

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let texto = match self {
            TaskState::ToDo => "A Fazer",
            TaskState::Doing => "Em Andamento",
            TaskState::Done => "Concluído",
        };
        write!(f, "{}", texto)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub state: TaskState,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tarefa: {}\n\tDescrição: {}\n\tStatus: {}\n\tCriada em: {} | Atualizada: {}",
            self.id,
            self.description.trim(),
            self.state,
            self.created_at.format("%d/%m/%Y %H:%M"),
            self.updated_at.format("%d/%m/%Y %H:%M")
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub username: String,
    pub data: Vec<Task>
}