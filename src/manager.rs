use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{ErrorKind, Seek, SeekFrom},
    path::PathBuf,
};

use chrono::Local;

use crate::{
    cli::{CreateTaskArgs, StatusArg, UpdateTaskArgs},
    task::{Status, Task},
};

pub fn find_default_journal_file() -> Option<PathBuf> {
    // home::home_dir().map(|mut path| {
    //     path.push("todo-tasks.json");
    //     path
    // })
    Some(PathBuf::from("./todo-tasks.json"))
}

pub struct TasksManager {
    file: PathBuf,
}

impl TasksManager {
    pub fn new(file: PathBuf) -> TasksManager {
        TasksManager { file }
    }

    pub fn list_tasks(&self) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file)?;
        let tasks = self.load(&file)?;
        println!(
            "{0: <5} | {1: <30} | {2: <80} | {3: <10} | {4: <20}",
            "ID", "NAME", "TEXT", "STATUS", "CREATED_AT"
        );
        for (idx, task) in tasks.into_iter().enumerate() {
            let created_at = task
                .created_at
                .with_timezone(&Local)
                .format("%F %H:%M")
                .to_string();
            let text = match task.text {
                Some(text) => text,
                None => "".to_string(),
            };
            println!(
                "{0: <5} | {1: <30} | {2: <80} | {3: <10} | {4: <20}",
                idx,
                task.name,
                text,
                task.status.to_string(),
                created_at
            )
        }
        Ok(())
    }

    fn add_task(&mut self, task: Task) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file)?;
        let mut tasks = self.load(&file)?;
        tasks.push(task);
        self.save(&file, tasks)?;
        Ok(())
    }

    pub fn create_task(&mut self, create_args: CreateTaskArgs) -> Result<(), Box<dyn Error>> {
        if let Some(text) = create_args.text {
            if let Some(status) = create_args.status {
                let status = match status {
                    StatusArg::Doing => Status::Doing,
                    StatusArg::Pending => Status::Pending,
                    StatusArg::Done => Status::Done,
                };
                let task = Task::new(&create_args.name)
                    .text(&text)
                    .status(status)
                    .build();
                self.add_task(task)?;
            } else {
                let task = Task::new(&create_args.name).text(&text).build();
                self.add_task(task)?;
            }
        } else {
            let task = Task::new(&create_args.name).build();
            self.add_task(task)?;
        }
        Ok(())
    }

    pub fn update_task(&mut self, update_args: UpdateTaskArgs) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file)?;

        let mut tasks = self.load(&file)?;

        if let Some(task) = tasks.get_mut(update_args.id) {
            if let Some(name) = update_args.name {
                task.name = name;
            }
            if let Some(text) = update_args.text {
                task.text = Some(text);
            }
            if let Some(status) = update_args.status {
                let status = match status {
                    StatusArg::Doing => Status::Doing,
                    StatusArg::Pending => Status::Pending,
                    StatusArg::Done => Status::Done,
                };
                task.status = status;
            }
        } else {
            return Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid Task ID",
            ))?;
        }
        println!("{:?}", &tasks);
        // Rewind and truncate the file.
        file.seek(SeekFrom::Start(0))?;
        file.set_len(0)?;
        self.save(&file, tasks)?;
        Ok(())
    }

    pub fn delete_task(&mut self, index: usize) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file)?;

        let mut tasks = self.load(&file)?;

        if tasks.get(index).is_some() {
            tasks.remove(index);
        } else {
            return Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid Task ID",
            ))?;
        }

        // Rewind and truncate the file.
        file.seek(SeekFrom::Start(0))?;
        file.set_len(0)?;

        self.save(&file, tasks)?;
        Ok(())
    }

    pub fn save(&self, file: &File, tasks: Vec<Task>) -> Result<(), Box<dyn Error>> {
        serde_json::to_writer(file, &tasks)?;
        Ok(())
    }

    pub fn load(&self, mut file: &File) -> Result<Vec<Task>, Box<dyn Error>> {
        file.seek(SeekFrom::Start(0))?; // Rewind the file before.
        let tasks = match serde_json::from_reader(file) {
            Ok(tasks) => tasks,
            Err(e) if e.is_eof() => Vec::new(),
            Err(e) => Err(e)?,
        };
        file.seek(SeekFrom::Start(0))?; // Rewind the file after.
        Ok(tasks)
    }
}

#[cfg(test)]
mod tests {
    // use std::path::PathBuf;

    // use super::TasksManager;
    // use crate::task::Task;

    #[test]
    fn test_new() {
        // let path = "./filename.json";
        // let manager = TasksManager::new(PathBuf::from(path));
        // assert!(true);
    }

    #[test]
    fn test_add_task() {
        // let path = "./filename.json";
        // let mut manager = TasksManager::new(PathBuf::from(path));
        // let task = Task::new("task title").build();
        // manager.add_task(task);
        // assert!(true);
    }

    #[test]
    fn test_list_tasks() {
        // let path = "./filename.json";
        // let mut manager = TasksManager::new(PathBuf::from(path));

        // let task = Task::new("task title").build();
        // manager.add_task(task);
        // manager.list_tasks();
        // assert!(true);
    }

    #[test]
    fn test_load_save() {
        // let path = "./filename.json";
        // let mut manager = TasksManager::new(PathBuf::from(path));

        // let task = Task::new("task title").build();

        // manager.add_task(task);
        // manager.save(path).unwrap();

        // let mut manager2 = TasksManager::new();
        // manager2.load(path).unwrap();

        // assert_eq!(manager.tasks.len(), manager2.tasks.len());
    }
}
