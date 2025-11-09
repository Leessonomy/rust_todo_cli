use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

mod id_generation {
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(1);

    pub fn next() -> u32 {
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}

struct Task {
    id: u32,
    title: String,
    description: String,
    date: String,
    done: bool,
}
impl Task {
    fn new(title: String, description: String, date: String, done: bool) -> Self {
        Self {
            id: id_generation::next(),
            title,
            description,
            date,
            done,
        }
    }
}


struct TasksModel {
    tasks: Vec<Task>,
}
impl TasksModel {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    
    pub fn add(&mut self, item: Task) {
        self.tasks.push(item);
    }

    pub fn get_all(&self) -> &[Task] {
        &self.tasks
    }

    pub fn delete_all(&mut self) {
        self.tasks.clear();
    }

    pub fn delete(&mut self, id: u32) -> Result<(), String> {
        if let Some(i) = self.tasks.iter().position(|item| item.id == id) {
            self.tasks.remove(i);
            Ok(())
        } else {
            Err(format!("Task with id {} not found.", id))
        }
    }

    pub fn toggle(&mut self, id: u32) -> Result<(), String> {
        if let Some(item) = self.tasks.iter_mut().find(|item| item.id == id) {
            item.done = !item.done;
            Ok(())
        } else {
            Err(format!("Task with id {} not found.", id))
        }
    }
}


struct CliView;
impl CliView {
    fn new() -> Self {
        Self {}
    }

    pub fn show_menu(&self) {
        println!(
            r#"
******************************************
*              TODO LIST                 *
******************************************
1. Show all tasks
2. Add a task
3. Delete a task
4. Toggle task status
5. Clear all
0. Exit
******************************************
"#
        );
    }

    pub fn display_tasks(&self, tasks: &[Task]) {
        if tasks.is_empty() {
            println!("Todo list is empty.");
            return;
        }
        println!("Your tasks");
        println!("******************************************");
        for task in tasks {
            let status = if task.done { "✓ Done" } else { "✗ Not done" };

            let description = if !task.description.trim().is_empty() {
                format!(" 📝 {:<40}\n", task.description.trim())
            } else {
                String::new()
            };
            println!(
                "id: {} | status: {} | title: {}\n{} 📅 {}\n",
                task.id,
                status,
                task.title.trim(),
                description,
                task.date
            );
            println!("******************************************");
        }
    }

    pub fn get_user_input(&self, prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input).expect("Failed to read.");
        input.trim().to_string()
    }
}


struct Presenter {
    model: TasksModel,
    view: CliView,
}
impl Presenter {
    pub fn new(model: TasksModel, view: CliView) -> Self {
        Self { model, view }
    }

    pub fn interaction_loop(&mut self) {
        loop {
            self.view.show_menu();

            let option: u32 = self
                .view
                .get_user_input("Select an option:")
                .parse::<u32>()
                .unwrap_or(0);

            match option {
                1 => self.show_tasks(),
                2 => self.add_task(),
                3 => self.delete_task(),
                4 => self.toggle_status(),
                5 => self.delete_tasks(),
                0 => break,
                _ => println!("Invalid option"),
            }
        }
    }

    pub fn show_tasks(&mut self) {
        self.view.display_tasks(&self.model.get_all());
    }

    pub fn add_task(&mut self) {
        let title = self.view.get_user_input("Enter task title:");
        let description = self.view.get_user_input("Enter task description:");

        if title.is_empty() || description.is_empty() {
            return;
        }

        let now = SystemTime::now();
        let since_epoch = now
            .duration_since(UNIX_EPOCH)
            .expect("System time");

        let secs_since = since_epoch.as_secs().to_string();
        let task = Task::new(title, description, secs_since, false);
        self.model.add(task);
    }

    pub fn delete_task(&mut self) {
        let input: String = self.view.get_user_input("Enter task id to delete:");
        input
            .parse::<u32>()
            .ok()
            .and_then(|id| self.model.delete(id).err().map(|e| println!("{}", e)));
    }

    pub fn toggle_status(&mut self) {
        self.view.display_tasks(&self.model.get_all());
        let input = self.view.get_user_input("Enter task id to toggle:");
        input
            .parse::<u32>()
            .ok()
            .and_then(|id| self.model.toggle(id).err().map(|e| println!("{}", e)));
    }

    pub fn delete_tasks(&mut self) {
        self.model.delete_all();
    }
}



fn main() {
    let view = CliView::new();
    let model = TasksModel::new();
    let mut presenter = Presenter::new(model, view);

    presenter.interaction_loop();
}
