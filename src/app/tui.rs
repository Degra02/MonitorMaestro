use std::{collections::HashMap, fs::File, io::Write, process::Command, string::FromUtf8Error};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{block::Title, Block, BorderType, Borders, List, ListItem},
    Frame,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::workspaces::{Monitor, State, WorkSpace};

use super::Tui;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct App {
    #[serde(flatten)]
    pub workspaces: HashMap<String, WorkSpace>,

    #[serde(skip)]
    pub ws_names: Vec<String>,

    #[serde(skip)]
    index: usize,
    #[serde(skip)]
    exit: bool,
}

impl App {
    #[allow(unused)]
    pub fn new(workspaces: HashMap<String, WorkSpace>) -> Self {
        let mut ws_names = Vec::<String>::new();
        for (name, ws) in workspaces.iter() {
            ws_names.push(name.clone());
        }

        Self {
            workspaces,
            ws_names,
            index: 0,
            exit: false,
        }
    }

    pub fn from_config(path: &str) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let mut app: App = serde_json::from_str(&data)?;

        let mut ws_names = Vec::<String>::new();
        for name in app.workspaces.keys() {
            ws_names.push(name.clone());
        }
        // Sorting alfabetically
        ws_names.sort();
        app.ws_names = ws_names;

        Ok(app)
    }

    pub fn run_tui(&mut self, terminal: &mut Tui) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    pub fn get_state(&mut self) -> std::io::Result<()> {
        let state = std::fs::read_to_string("/tmp/monitor_maestro_state.txt")?;
        println!("{}", state);

        Ok(())
    }

    pub fn connected_monitors() -> std::io::Result<Vec<Monitor>> {
        let output = Command::new("sh")
            .arg("-c")
            .arg("hyprctl -j monitors")
            .output()?;

        let mut monitors = Vec::<Monitor>::new();

        let json: Vec<Value> =
            serde_json::from_str(&String::from_utf8(output.stdout).unwrap()).unwrap();
        for mon_str in json.iter() {
            let name = mon_str["name"].as_str().unwrap();
            let (width, height) = (
                mon_str["width"].as_u64().unwrap(),
                mon_str["height"].as_u64().unwrap(),
            );
            let rr = mon_str["refreshRate"].as_f64().unwrap();
            let (x, y) = (
                mon_str["x"].as_u64().unwrap(),
                mon_str["y"].as_u64().unwrap(),
            );
            let scale = mon_str["scale"].as_f64().unwrap();

            let state = State::Enabled {
                dimensions: (width as u32, height as u32),
                position: (x as u32, y as u32),
                rerfresh_rate: rr as u32,
                scaling: scale as f32,
            };
            let monitor = Monitor::new(name, state);
            println!("{:?}", monitor);
            println!("{},{}x{}@{},{}x{},{}", name, width, height, rr, x, y, scale);
        }

        Ok(monitors)
    }

    pub fn start_workspace(&mut self, workspace: &str) -> std::io::Result<()> {
        let ws = &self.workspaces[workspace];
        let mut file = File::create("/tmp/monitor_maestro_state.txt")?;
        file.write_all(workspace.as_bytes())?;
        let _ = Command::new("sh").arg("-c").arg(ws.command()).output();

        Ok(())
    }

    fn render_frame(&mut self, f: &mut Frame) {
        let title = Title::from("WorkSpaces");
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::LightCyan))
            .title(title)
            .title_style(Style::default().fg(Color::White))
            .title_alignment(Alignment::Center);

        let mut list = Vec::<ListItem>::new();

        for (i, ws_name) in self.ws_names.iter().enumerate() {
            let style = if i == self.index {
                Style::default().fg(Color::LightYellow)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            list.push(ListItem::new(
                Line::from(Span::from(ws_name.to_string()))
                    .alignment(Alignment::Center)
                    .style(style),
            ));
        }

        let list = List::new(list).block(block).highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC),
        );
        f.render_widget(list, f.size());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event)
            }
            _ => Ok(()),
        }
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> std::io::Result<()> {
        match key_event.code {
            KeyCode::Char('j') => {
                if self.index < self.workspaces.len() - 1 {
                    self.index += 1;
                }
            }
            KeyCode::Char('k') => {
                if self.index > 0 {
                    self.index -= 1;
                }
            }
            KeyCode::Enter => self.execute_selected()?,
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            _ => {}
        }

        Ok(())
    }

    fn execute_selected(&mut self) -> std::io::Result<()> {
        let ws_name = &self.ws_names[self.index];
        let _ = Command::new("sh")
            .arg("-c")
            .arg(self.workspaces[ws_name].command())
            .output();

        let mut file = File::create("/tmp/monitor_maestro_state.txt")?;
        file.write_all(ws_name.as_bytes())?;

        Ok(())
    }
}
