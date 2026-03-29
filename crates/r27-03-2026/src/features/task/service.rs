use chrono::Utc;

use crate::features::task::model::{Data, Task, TaskState};
use super::repository::{load, save};

const PATH: &str = "src/db/data.json";

fn load_data() -> Result<Data, String> {
    load(PATH).map_err(|e| format!("Failed to load data: {e}"))
}

fn save_data(data: &Data) -> Result<(), String> {
    save(PATH, data).map_err(|e| format!("Failed to save data: {e}"))
}

pub fn create_task(description: String) -> Result<Task, String> {
    let mut data = load_data()?;

    let next_id = data.data.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let now = Utc::now();

    let task = Task {
        id: next_id,
        description: description.trim().to_string(),
        created_at: now,
        updated_at: now,
        state: TaskState::ToDo,
    };

    data.data.push(task);
    save_data(&data)?;

    data.data.pop().ok_or_else(|| "Unexpected error".to_string())
}

pub fn delete_task(id: u32) -> Result<bool, String> {
    let mut data = load_data()?;

    let original_len = data.data.len();
    data.data.retain(|t| t.id != id);

    if data.data.len() == original_len {
        return Ok(false);
    }

    save_data(&data)?;
    Ok(true)
}

pub fn edit_task(id: u32, description: String) -> Result<bool, String> {
    let mut data = load_data()?;

    let Some(task) = data.data.iter_mut().find(|t| t.id == id) else {
        return Ok(false);
    };

    task.description = description.trim().to_string();
    task.updated_at = Utc::now();

    save_data(&data)?;
    Ok(true)
}

pub fn change_state(id: u32, state: TaskState) -> Result<bool, String> {
    let mut data = load_data()?;

    let Some(task) = data.data.iter_mut().find(|t| t.id == id) else {
        return Ok(false);
    };

    task.state = state;
    task.updated_at = Utc::now();

    save_data(&data)?;
    Ok(true)
}

pub fn get_tasks() -> Result<Vec<Task>, String> {
    Ok(load_data()?.data)
}

pub fn get_task_by_id(id: u32) -> Result<Option<Task>, String> {
    let data = load_data()?;
    Ok(data.data.into_iter().find(|t| t.id == id))
}

pub fn change_username(username: String) -> Result<(), String> {
    let mut data = load_data()?;
    data.username = username.trim().to_string();
    save_data(&data)
}

pub fn reset() -> Result<(), String> {
    let data = Data {
        username: String::from("User"),
        data: Vec::new(),
    };
    save_data(&data)
}

pub fn get_username() -> Result<String, String> {
    Ok(load_data()?.username)
}