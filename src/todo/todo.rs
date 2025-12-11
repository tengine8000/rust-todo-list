use std::collections::HashMap;
use std::io::Read;

pub struct Todo {
    map: HashMap<String, String>,
}

impl Todo {
    pub fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("todo.db")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String, String> = content
            .lines()
            .map(|line| line.split(" : ").collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), String::from(v)))
            .collect();
        Ok(Todo { map })
    }

    pub fn insert(&mut self, key: String) {
        self.map.insert(key, String::from("To Do"));
    }
    pub fn start(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = String::from("In Progress")),
            None => None,
        }
    }
    pub fn done(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = String::from("Done")),
            None => {
                println!("'{}' not present in ToDo list", key);
                None
            }
        }
    }
    pub fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{} : {}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("todo.db", content)
    }
}
