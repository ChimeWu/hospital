use bevy::prelude::*;

#[derive(Resource)]
pub struct TerminalIOBuffer {
    pub index_now: usize,
    pub capacity: usize,
    pub buf_lock: MyLock,
    pub pipes: Vec<BufText>,
    pub head: PipeHead,
    pub tail: PipeTail,
    pub timer: Timer,
    pub flag: bool,
}

impl Default for TerminalIOBuffer {
    fn default() -> Self {
        TerminalIOBuffer {
            index_now: 0,
            capacity: 8,
            buf_lock: MyLock::Locked,
            pipes: vec![BufText::new(); 8],
            head: PipeHead::default(),
            tail: PipeTail::default(),
            timer: Timer::from_seconds(0.8, TimerMode::Repeating),
            flag: true,
        }
    }
}

impl TerminalIOBuffer {
    pub fn clean(&mut self) {
        for pipe in self.pipes.iter_mut() {
            pipe.clean();
        }
        self.index_now = 0;
    }
}

#[derive(PartialEq, Debug)]
pub enum MyLock {
    Locked,
    Unlocked,
}

pub struct PipeHead {
    pub sleep: String,
    pub wake: String,
}

impl Default for PipeHead {
    fn default() -> Self {
        PipeHead {
            sleep: String::from("_>>"),
            wake: String::from("$>>"),
        }
    }
}

pub struct PipeTail {
    pub sleep: String,
    pub wake: String,
}

impl Default for PipeTail {
    fn default() -> Self {
        PipeTail {
            sleep: String::from(" "),
            wake: String::from("|"),
        }
    }
}

impl PipeTail {
    pub fn now(&self, flag: bool) -> &str {
        if flag {
            &self.wake
        } else {
            &self.sleep
        }
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct BufText {
    text: String,
}

#[derive(Component)]
pub struct TextPipeId(pub usize);

impl BufText {
    pub fn new() -> Self {
        BufText {
            text: String::new(),
        }
    }

    pub fn get_str(&mut self) -> &str {
        self.remove_whitespace_except_space();
        &self.text
    }

    pub fn get_string(&mut self) -> String {
        self.remove_whitespace_except_space();
        self.text.clone()
    }

    pub fn push_str(&mut self, text: &str) {
        self.text.push_str(text);
    }

    pub fn clean(&mut self) {
        self.text.clear();
    }

    pub fn pop(&mut self) -> Option<char> {
        self.remove_whitespace_except_space();
        self.text.pop()
    }

    fn remove_whitespace_except_space(&mut self) {
        self.text
            .retain(|c| !c.is_whitespace() && c != '\x08' || c == ' ');
    }
}
