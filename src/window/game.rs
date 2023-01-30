use std::path::PathBuf;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use bevy::winit::WinitSettings;

const CONFIG_DIR: &str = "Solid";
const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize)]
struct WindowSettings {
    title: String,
    width: f32,
    height: f32,
    resizable: bool,
    monitor: i64,
    mode: i64,
}

#[derive(Serialize, Deserialize)]
struct Config {
    window: WindowSettings,
}

#[derive(Clone)]
pub struct GamePlugin {
    pub window: WindowDescriptor,
}

impl Config {
    fn new(wd: WindowDescriptor) -> Self {
        let monitor_index = match wd.monitor {
            bevy::window::MonitorSelection::Index(monitor) => monitor as i64,
            bevy::window::MonitorSelection::Primary => 0,
            bevy::window::MonitorSelection::Current => 0
        };

        let mode = match wd.mode {
            bevy::window::WindowMode::Windowed => 0 as i64,
            bevy::window::WindowMode::BorderlessFullscreen => 1 as i64,
            bevy::window::WindowMode::SizedFullscreen { .. } => 2 as i64,
            bevy::window::WindowMode::Fullscreen { .. } => 3 as i64,
        };

        Self {
            window: WindowSettings {
                title: wd.title,
                width: wd.width,
                height: wd.height,
                resizable: wd.resizable,
                monitor: monitor_index,
                mode: mode,
            }
        }
    }
    fn to_descriptor(self) -> WindowDescriptor {
        let monitor = match self.window.monitor {
            0 => bevy::window::MonitorSelection::Primary,
            _ => bevy::window::MonitorSelection::Index(self.window.monitor as usize),
        };
        let mode = match self.window.mode {
            0 => bevy::window::WindowMode::Windowed,
            1 => bevy::window::WindowMode::BorderlessFullscreen,
            2 => bevy::window::WindowMode::SizedFullscreen,
            3 => bevy::window::WindowMode::Fullscreen,
            _ => bevy::window::WindowMode::Windowed,
        };
        WindowDescriptor {
            title: self.window.title,
            width: self.window.width,
            height: self.window.height,
            resizable: self.window.resizable,
            monitor: monitor,
            mode: mode,
            ..Default::default()
        }
    }
}

impl GamePlugin {
    fn check_config(&self) -> Self {
        let mut config_dir = dirs::config_dir().unwrap();
        config_dir.push(CONFIG_DIR);
        let config_file = config_dir.join(CONFIG_FILE);
        info!("Config file: {:?}", config_file);
        if config_file.exists() {
            self.load(config_file)
        } else {
            self.create(config_dir, config_file)
        }
    }

    fn load(&self, path: PathBuf) -> Self {
        let config = std::fs::read_to_string(path).unwrap();
        let settings: Config = toml::from_str(&config).unwrap();
        let window = settings.to_descriptor();
        Self { window }
    }

    fn new() -> Self {
        let window = WindowDescriptor {
            title: "Solid".to_string(),
            width: 1280.0,
            height: 720.0,
            resizable: false,
            ..Default::default()
        };
        Self { window }.check_config()
    }

    fn create(&self, dir: PathBuf, path: PathBuf) -> Self {
        let settings = Config::new(self.window.clone());
        let config = toml::to_string(&settings).unwrap();
        if !dir.exists() {
            std::fs::create_dir_all(dir).unwrap();
        }
        std::fs::write(path, config).unwrap();
        self.clone()
    }
}

impl Default for GamePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let mut window = WindowPlugin::default();
        window.window = self.window.clone();
        app.add_plugins(DefaultPlugins.set(window))
            .insert_resource(WinitSettings::desktop_app());
        }
}