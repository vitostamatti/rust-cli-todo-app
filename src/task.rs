use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub text: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Status {
    Pending,
    Doing,
    Done,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Doing => "Doing".to_string(),
            Status::Pending => "Pending".to_string(),
            Status::Done => "Done".to_string(),
        }
    }
}

impl Task {
    pub fn new(title: &str) -> TaskBuilder {
        TaskBuilder {
            name: title.to_string(),
            text: None,
            status: Status::Pending,
        }
    }
}

pub struct TaskBuilder {
    name: String,
    text: Option<String>,
    status: Status,
}

impl TaskBuilder {
    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = status;
        self
    }

    pub fn build(&mut self) -> Task {
        Task {
            name: self.name.clone(),
            text: self.text.clone(),
            status: self.status.clone(),
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::task::Status;

    use super::Task;

    #[test]
    fn todo_new() {
        let task = Task::new("Task title").build();
        assert_eq!(task.status, Status::Pending);
        assert_eq!(task.text, None);
        assert_eq!(task.name, "Task title".to_string());
    }

    #[test]
    fn todo_new_with_text() {
        let task = Task::new("Task title").text("adding text").build();
        assert_eq!(task.status, Status::Pending);
        assert_eq!(task.text, Some("adding text".to_string()));
        assert_eq!(task.name, "Task title".to_string());
    }
    #[test]
    fn todo_new_with_status() {
        let task = Task::new("Task title").status(Status::Doing).build();
        assert_eq!(task.status, Status::Doing);
        assert_eq!(task.name, "Task title".to_string());
    }
}
