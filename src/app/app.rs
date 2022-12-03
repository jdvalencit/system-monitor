use crate::process::ProcessData;
use std::collections::HashMap;

pub struct App {
    data: HashMap<i32, ProcessData>,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new(data: HashMap<i32, ProcessData>) -> Self {
        let data = data;
        Self { data }
    }

    pub fn data(&mut self) -> &HashMap<i32, ProcessData> {
        &self.data
    }

    pub fn update_data(&mut self, data: &HashMap<i32, ProcessData>) -> () {
        self.data = data.clone();
    }
}

pub enum InputEvent<I> {
    Input(I),
    Tick,
}
