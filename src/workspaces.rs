use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    Enabled {
        dimensions: (u32, u32),
        position: (u32, u32),
        rerfresh_rate: u32,
        scaling: f32,
    },
    Disabled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub name: String,
    state: State,
}

impl Monitor {
    #[allow(unused)]
    pub fn new(name: &str, state: State) -> Self {
        Self {
            name: name.to_owned(),
            state,
        }
    }

    pub fn get_info(&self) -> Option<((u32, u32), (u32, u32), u32, f32)> {
        match self.state {
            State::Enabled {
                dimensions,
                position,
                rerfresh_rate,
                scaling,
            } => Some((dimensions, position, rerfresh_rate, scaling)),
            State::Disabled => None,
        }
    }

    pub fn get_position(&self) -> Option<(u32, u32)> {
        match self.state {
            State::Enabled {
                dimensions: _,
                position,
                rerfresh_rate: _,
                scaling: _,
            } => Some(position),
            State::Disabled => None,
        }
    }

    pub fn get_size(&self) -> Option<(u32, u32)> {
        match self.state {
            State::Enabled {
                dimensions,
                position: _,
                rerfresh_rate: _,
                scaling: _,
            } => Some(dimensions),
            State::Disabled => None,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WorkSpace {
    #[serde(skip)]
    pub workspace_name: String,
    monitors: Vec<Monitor>,
}

impl WorkSpace {
    #[allow(unused)]
    pub fn new(workspace_name: &str, monitors: Vec<Monitor>) -> Self {
        Self {
            workspace_name: workspace_name.to_owned(),
            monitors,
        }
    }

    pub fn command(&self) -> String {
        let mut output = String::new();

        for monitor in &self.monitors {
            let cmd: String = match monitor.state {
                State::Enabled {
                    dimensions,
                    position,
                    rerfresh_rate,
                    scaling,
                } => {
                    let (x, y) = dimensions;
                    let (x_pos, y_pos) = position;
                    format!(
                        "{}x{}@{},{}x{},{}",
                        x, y, rerfresh_rate, x_pos, y_pos, scaling
                    )
                }
                State::Disabled => "disable".to_owned(),
            };

            let full = format!("hyprctl keyword monitor {},{};", monitor.name, cmd);
            output.push_str(&full);
        }

        // dbg!("{}", &output);

        output
    }
}
