use std::{
    collections::HashMap,
    fs::File,
    io::BufWriter,
    iter::{empty, once},
    path::Path,
    str::FromStr,
    time::SystemTime,
};

use serde::{Deserialize, Serialize};
use serde_json::{from_reader, ser::PrettyFormatter, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::utils::Result;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum TaskState {
    #[default]
    Todo,
    InProgress,
    Done,
}

impl FromStr for TaskState {
    type Err = crate::utils::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = &*s
            .chars()
            .map(|ch| ch.to_ascii_uppercase())
            .collect::<String>();

        match s {
            "TODO" => Ok(TaskState::Todo),
            "IN-PROGRESS" | "IN_PROGRESS" | "INPROGRESS" => Ok(TaskState::InProgress),
            "DONE" => Ok(TaskState::Done),
            _ => Err("".into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Task {
    desc: String,
    state: TaskState,
    created_at: SystemTime,
    last_updated: SystemTime,
}

impl Task {
    #[inline]
    fn new(desc: String) -> Self {
        Self {
            desc,
            state: TaskState::default(),
            created_at: SystemTime::now(),
            last_updated: SystemTime::now(),
        }
    }

    #[inline]
    fn update(&mut self, desc: String) {
        self.last_updated = SystemTime::now();
        self.desc = desc;
    }

    #[inline]
    fn desc(&self) -> &str {
        self.desc.as_str()
    }

    #[inline]
    fn is_state(&self, state: TaskState) -> bool {
        self.state.eq(&state)
    }

    #[inline]
    fn change_state(&mut self, state_code: u8) {
        self.last_updated = SystemTime::now();
        self.state = match state_code {
            0 => TaskState::Todo,
            1 => TaskState::InProgress,
            2 => TaskState::Done,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    tasks: HashMap<String, Task>,
    pub ptr: usize,
}

impl Database {
    #[inline]
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            ptr: 0,
        }
    }

    #[inline]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(from_reader(File::open(path)?)?)
    }

    #[inline]
    pub fn add_task_with_id(&mut self, task_desc: String, id: String) {
        self.tasks.insert(id, Task::new(task_desc));
    }

    #[inline]
    pub fn add_task(&mut self, task_desc: String) {
        self.ptr += 1;
        self.tasks
            .insert(format!("{}", self.ptr), Task::new(task_desc));
    }

    #[inline]
    pub fn delete_task(&mut self, id: String) {
        if self.tasks.remove(&id).is_some() {
            println!("Task removed.")
        } else {
            println!("No task were associated with the provided id: {}", id)
        }
    }

    #[inline]
    pub fn update_task(&mut self, task_desc: String, id: String) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.update(task_desc)
        }
    }

    #[inline]
    pub fn change_task_state(&mut self, task_statecode: u8, id: String) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.change_state(task_statecode)
        }
    }

    #[inline]
    pub fn kv(&self) -> (Vec<(&str, &str)>, usize) {
        self.tasks
            .iter()
            .map(|(s, t)| (s.as_str(), t.desc()))
            .collect()
    }

    #[inline]
    pub fn filt_kv(&self, filt: String) -> (Vec<(&str, &str)>, usize) {
        if let Ok(state) = filt.parse::<TaskState>() {
            self.tasks
                .iter()
                .filter(|&(_, task)| task.is_state(state))
                .map(|(id, task)| (id.as_str(), task.desc()))
                .fold((Vec::new(), 0), |(mut v, maxlen), (id, task)| {
                    v.push((id, task));
                    (v, maxlen.max(id.len()))
                })
        } else if let Some((id, v)) = self.tasks.get_key_value(&filt) {
            (vec![(id.as_str(), v.desc())], id.len())
        } else {
            (Vec::new(), 0)
        }
    }

    #[inline]
    pub fn save_db<P: AsRef<Path>>(self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        let fmt = PrettyFormatter::with_indent(b"    ");
        let mut ser = Serializer::with_formatter(&mut writer, fmt);

        self.serialize(&mut ser)?;

        Ok(())
    }
}
