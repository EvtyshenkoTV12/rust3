use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write, Read};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let id = self.tasks.len() as u32 + 1;
        let task = Task { id, description, completed: false };
        self.tasks.push(task);
    }

    fn remove_task(&mut self, id: u32) {
        self.tasks.retain(|task| task.id != id);
    }

    fn edit_task(&mut self, id: u32, description: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.description = description;
        }
    }

    fn mark_completed(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
        }
    }

    fn clear(&mut self) {
        self.tasks.clear();
    }

    fn save(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string(&self.tasks)?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn load(filename: &str) -> io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let tasks: Vec<Task> = serde_json::from_str(&contents)?;
        Ok(TodoList { tasks })
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    // Загрузити завдання з файлу, якщо він існує
    match TodoList::load("tasks.json") {
        Ok(loaded_list) => todo_list = loaded_list,
        Err(_) => println!("Не вдалося завантажити завдання, створюється новий список."),
    }

    loop {
        println!("\nОберіть дію:");
        println!("1. Додати завдання");
        println!("2. Видалити завдання");
        println!("3. Відзначити завдання як виконане");
        println!("4. Очищення списку завдань");
        println!("5. Вивести список завдань");
        println!("6. Вийти");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Не вдалося прочитати рядок");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Будь ласка, введіть номер дії.");
                continue;
            }
        };

        match choice {
            1 => {
                let mut task_description = String::new();
                println!("Введіть опис завдання:");
                io::stdin().read_line(&mut task_description).expect("Не вдалося прочитати рядок");
                let task_description = task_description.trim().to_string();
                todo_list.add_task(task_description);
            }
            2 => {
                if todo_list.tasks.is_empty() {
                    println!("Список завдань порожній. Немає завдань для видалення.");
                } else {
                    println!("Список завдань:");
                    for task in &todo_list.tasks {
                        let status = if task.completed { "✓" } else { "✗" };
                        println!("{}: {} [{}]", task.id, task.description, status);
                    }

                    let mut task_id = String::new();
                    println!("Введіть номер завдання для видалення:");
                    io::stdin().read_line(&mut task_id).expect("Не вдалося прочитати рядок");

                    match task_id.trim().parse::<u32>() {
                        Ok(id) => {
                            todo_list.remove_task(id);
                            println!("Завдання {} видалено.", id);
                        }
                        Err(_) => {
                            println!("Будь ласка, введіть коректний номер завдання.");
                        }
                    }
                }
            }
            3 => {
                let mut task_id = String::new();
                println!("Введіть номер завдання, яке потрібно відзначити як виконане:");
                io::stdin().read_line(&mut task_id).expect("Не вдалося прочитати рядок");
                let task_id: u32 = task_id.trim().parse().expect("Будь ласка, введіть коректний номер завдання");
                todo_list.mark_completed(task_id);
            }
            4 => {
                todo_list.clear();
                println!("Список завдань очищено.");
            }
            5 => {
                if todo_list.tasks.is_empty() {
                    println!("Список завдань порожній.");
                } else {
                    for task in &todo_list.tasks {
                        let status = if task.completed { "✓" } else { "✗" };
                        println!("{}: {} [{}]", task.id, task.description, status);
                    }
                }
            }
            6 => break,
            _ => println!("Невірний вибір. Спробуйте ще раз."),
        }

        // Зберегти завдання у файл
        if let Err(e) = todo_list.save("tasks.json") {
            eprintln!("Помилка при збереженні завдань: {}", e);
        }
    }
}
