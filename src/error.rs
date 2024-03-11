use crate::parser::{Parser,Position};
use colored::Colorize;
pub struct Error(Vec<Box<dyn Component>>);

pub struct Info {
    pub name: String,
    pub code: u64,
    pub level: Level,
}

pub enum Level {
    Error,
    Warning,
    Info,
}

impl Error {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push<T: Component + 'static>(&mut self, component: T) {
        self.0.push(Box::new(component));
    }

    pub fn render(self) -> Vec<String> {
        self.0.into_iter().flat_map(|component| component.render()).collect()
    }
}

pub trait Component {
    fn render(&self) -> Vec<String>;
}

pub struct Basic {
    pub position: Position,
    pub info: Info,
    pub data: String,
    pub message: String,
    pub messages: Vec<(String, Position)>,
}

impl Basic {
    pub fn new(position: Position, info:Info, parser: &mut Parser,mut messages: Vec<(String, Position)>, message:String) -> Option<Self> {
        messages.sort();
        if position.is_multi_line() { return None }
        for i in &messages {
            if !position.contains(&i.1) { return None }
        }
        let data = parser.aquire(&position.expand_to_line());
        Some(Self { position, data, info, messages, message })
    }
}

impl Component for Basic {
    fn render(&self) -> Vec<String> {
        ErrorBuilder::new()
            .init(&self.message, &self.info, &self.position)
            .push(&self.data, &self.position, &self.messages)
            .build()
    }
}

pub struct ErrorBuilder {
    pub result: Vec<String>,
}

impl ErrorBuilder {
    pub fn new() -> Self {
        Self { result: Vec::new() }
    }

    pub fn init(mut self, message: &str, info: &Info, position: &Position) -> Self {
        let mut first = String::new();
        match info.level {
            Level::Error => first.push_str(&format!("error[{}]: ", info.code).red().bold().to_string()),
            Level::Warning => first.push_str(&format!("warning[{}]: ", info.code).yellow().bold().to_string()),
            Level::Info => first.push_str(&format!("info[{}]: ", info.code).green().bold().to_string()),
        }
        first.push_str(&message);
        self.result.push(first);
        self.result.push(format!("  {} {}","-->".blue().to_string(),String::from(position.clone())));
        self
    }

    pub fn push(
        mut self,
        line: &String,
        position: &Position,
        messages: &[(String, Position)],
    ) -> Self {
        if position.is_multi_line() { todo!() }
        self.result.push(format!("{}{}", " ".repeat(4), "|".blue().to_string()));
        self.result.push(format!("{:4}{}{}{}",position.line.1,"|".blue().to_string(), " ".repeat(4), line));
        let mut str = format!("{}{}{}", " ".repeat(4), "|".blue().to_string(), " ".repeat(4));
        let mut index = 0;
        for (message, position) in messages {
            let mut offset = position.column.0 - index;
            if offset < 0 {
                self.result.push(str);
                str = format!("{}{}{}", " ".repeat(4), "|".blue().to_string(), " ".repeat(4));
                offset = position.column.0;
            }
            let len = str.len();
            str.push_str(&" ".repeat(offset));
            str.push_str(&"^".red().to_string().repeat(position.column.1 - position.column.0));
            str.push_str(&format!(" {}", message.red().bold().to_string()));
            index += str.len() - len;
        }
        if str != format!("{}{}{}", " ".repeat(4), "|".blue().to_string(), " ".repeat(4)) {
            self.result.push(str);
        }
        self
    }

    pub fn build(self) -> Vec<String> {
        self.result
    }
} 
