/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use super::cmd::Cmd;

pub static CFG: LazyLock<Configuration> = LazyLock::new(find_config);

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub dir: DirSettings,
    pub usr: UsrSettings,
    pub log: LogSettings,
    pub connect: ConnectSettings,
    pub data: DataSettings,
    pub core: CoreSettings,
    pub tester: TesterSettings,
    pub gui: GuiSettings,
}
impl Configuration {
    pub fn read_config(path: &Path) -> Configuration {
        let s = Cmd::read(path).unwrap();
        let cfg: Configuration = toml::from_str(&s).unwrap();

        cfg
    }
}

fn find_config() -> Configuration {
    let file_name = "config.toml";

    // try find user config in current dir
    let mut path = std::env::current_dir().unwrap();
    path.push(file_name);
    if Cmd::is_exist(&path) {
        return Configuration::read_config(&path);
    };

    // try find in user home ~/.config/avin/
    let mut path = std::env::home_dir().unwrap();
    path.push(".config");
    path.push("avin");
    path.push(file_name);
    if Cmd::is_exist(&path) {
        return Configuration::read_config(&path);
    };

    // try use default config in ~/avin/res/default_config.toml
    let mut path = std::env::home_dir().unwrap();
    path.push("avin");
    path.push("res");
    path.push("default_config.toml");
    if Cmd::is_exist(&path) {
        return Configuration::read_config(&path);
    };

    // panic
    log::error!("Config file not found: {:?}", path);
    panic!()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DirSettings {
    root: String,
    data: String,
}
impl DirSettings {
    pub fn root(&self) -> PathBuf {
        PathBuf::from(&self.root)
    }
    pub fn data(&self) -> PathBuf {
        PathBuf::from(&self.data)
    }
    pub fn cache(&self) -> PathBuf {
        PathBuf::from(&self.data).join("cache")
    }
    pub fn test(&self) -> PathBuf {
        PathBuf::from(&self.root).join("test")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UsrSettings {
    pub offset: i32,
    pub dt_fmt: String,
    pub editor: String,
    pub python: String,
    pub terminal: String,
    pub exec: String,
    pub opt: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LogSettings {
    pub history: u32,
    pub debug: bool,
    pub info: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectSettings {
    pub moexalgo: Option<String>,
    pub tinkoff: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DataSettings {
    pub format: String,
    pub converter: Vec<ConvertRule>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ConvertRule {
    pub iid: String,
    pub input: String,
    pub output: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CoreSettings {
    pub default_bars_count: usize,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TesterSettings {
    pub default_commission: f64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GuiSettings {
    pub color: ColorSettings,
    pub chart: ChartSettings,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ColorSettings {
    pub cross: String,
    pub cross_opacity: f32,
    pub bear: String,
    pub bear_opacity: f32,
    pub bull: String,
    pub bull_opacity: f32,
    pub nobody: String,
    pub nobody_opacity: f32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ChartSettings {
    pub bottom_pane_height: f32,
    pub left_pane_width: f32,
    pub right_pane_width: f32,
}
