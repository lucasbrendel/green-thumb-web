use chrono::{DateTime, Local};
use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};

/// Standard task for things that need to be completed
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    /// Database unique id to identify instance
    id: i64,
    /// Task description of what needs to be done
    text: String,
    /// Denote that task has been completed
    is_completed: bool,
    /// Date and Time that task is completed
    completed_date: DateTime<Local>,
}

impl Task {
    /// Create a new instance of a Task
    ///
    /// # Example
    ///
    /// ```
    /// let mgr = datamgr::DataMgr::new(String::from("./data/green-thumb-test-new_task.db"));
    /// let task = Task::new(&mgr.conn, String::from("Water garden"));
    /// assert_eq!(String::from("Water garden"), task.text);
    /// ```
    pub fn new(conn: &Connection, text: String) -> Self {
        conn.execute(
            "INSERT INTO tasks (text, is_completed, completed_date) VALUES (?1, ?2, ?3)",
            &[&text, &false as &ToSql, &Local::now()],
        )
        .unwrap();

        Task {
            id: conn.last_insert_rowid(),
            text,
            is_completed: false,
            // TODO: This is terrible and needs to be changed
            completed_date: Local::now(),
        }
    }

    /// Get the completed state
    pub fn get_completed(&self) -> bool {
        self.is_completed
    }

    /// Mark a task as completed
    pub fn set_completed(&mut self, conn: &Connection) {
        self.is_completed = true;
        self.completed_date = Local::now();

        conn.execute_named("UPDATE tasks SET is_completed = :iscompleted, completed_date = :completeddate WHERE id = :id", 
            &[(":iscompleted", &true as &ToSql), 
              (":completeddate", &Local::now()), 
              (":id", &self.id)]).unwrap();
    }

    /// Get the date that the task was completed.
    /// This cannot be trusted if set_completed returns false.
    pub fn get_completed_date(&self) -> DateTime<Local> {
        self.completed_date
    }

    /// Get Task text field
    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    /// Update Task text field
    pub fn set_text(&mut self, conn: &Connection, text: String) {
        self.text = text;
        conn.execute_named(
            "UPDATE tasks SET text = :text WHERE id = :id",
            &[(":text", &self.text), (":id", &self.id)],
        )
        .unwrap();
    }

    /// Access all tasks
    pub fn get_tasks(conn: &Connection) -> Result<Vec<Task>> {
        let mut tasks: Vec<Task> = Vec::new();
        let mut stmt = conn.prepare("SELECT id, text, is_completed, completed_date FROM tasks")?;
        let map_tasks = stmt.query_map(NO_PARAMS, |row| Task {
            id: row.get(0),
            text: row.get(1),
            is_completed: row.get(2),
            completed_date: row.get(3),
        })?;
        for task in map_tasks {
            info!("Accessing {:?}", task);
            tasks.push(task.unwrap());
        }
        Ok(tasks)
    }

    pub fn get_task_by_id(conn: &Connection, id: i64) -> Result<Task> {
        let mut stmt =
            conn.prepare("SELECT text, is_completed, completed_date FROM tasks WHERE id = :id")?;
        let task = stmt.query_map(&[&id], |row| Task {
            id,
            text: row.get(0),
            is_completed: row.get(1),
            completed_date: row.get(2),
        })?;
        Ok(task.last().unwrap().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::DataMgr;

    #[test]
    fn new_task() {
        let mgr = DataMgr::new(String::from("./data/green-thumb-test-new_task.db"));
        let task = Task::new(&mgr.conn, String::from("Water garden"));
        assert_eq!(String::from("Water garden"), task.text);
    }

    #[test]
    fn get_tasks() {
        let mgr = DataMgr::new(String::from("./data/green-thumb-test-get_task.db"));
        let task1 = Task::new(&mgr.conn, String::from("Water garden"));
        let task2 = Task::new(&mgr.conn, String::from("Weed garden"));
        let tasks = Task::get_tasks(&mgr.conn).unwrap();
        assert_eq!(task1.text, tasks[0].text);
        assert_eq!(task2.text, tasks[1].text);
    }

    #[test]
    fn set_completed() {
        let mgr = DataMgr::new(String::from("./data/green-thumb-test-set_completed.db"));
        let mut task1 = Task::new(&mgr.conn, String::from("Test completion"));
        assert_eq!(false, task1.get_completed());
        task1.set_completed(&mgr.conn);
        assert_eq!(true, task1.get_completed());
    }

    #[test]
    fn completed_date() {
        let mgr = DataMgr::new(String::from("./data/green-thumb-test-completed_date.db"));
        let mut task = Task::new(&mgr.conn, String::from("Test completion"));
        // Accessing the value assigned at creation to ensure that it changes when the task is marked completed
        let date = task.get_completed_date();
        task.set_completed(&mgr.conn);
        assert_ne!(date, task.get_completed_date());
    }

    #[test]
    fn get_text() {
        let mgr = DataMgr::new(String::from("./data/green-thumb-test-completed_date.db"));
        let mut task = Task::new(&mgr.conn, String::from("Test completion"));
        assert_eq!(String::from("Test completion"), task.get_text());
        task.set_text(&mgr.conn, String::from("Updated Text."));
        assert_eq!(String::from("Updated Text."), task.get_text());
    }

    #[test]
    fn get_task_by_id() {
        let mgr = DataMgr::new(String::from("./data/green-thumb-test-get_task_by_id.db"));
        let task = Task::new(&mgr.conn, String::from("Get by Id"));
        let task2 = Task::get_task_by_id(&mgr.conn, task.id);
        assert_eq!(task.get_text(), task2.unwrap().get_text());
    }
}
