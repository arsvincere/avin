/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::PathBuf;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use super::cmd::Cmd;

pub static CFG: LazyLock<Configuration> = LazyLock::new(Configuration::read);

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub dir: DirSettings,
    pub connect: ConnectSettings,
    pub usr: UsrSettings,
    pub log: LogSettings,
    pub data: DataSettings,
    pub core: CoreSettings,
    pub tester: TesterSettings,
    pub trader: TraderSettings,
    pub gui: GuiSettings,
}
impl Configuration {
    fn read() -> Configuration {
        let path = Configuration::find();
        let text = Cmd::read(&path).unwrap();
        let cfg: Configuration = toml::from_str(&text).unwrap();

        cfg
    }
    fn find() -> PathBuf {
        let file_name = "config.toml";

        // try find user config in current dir
        let mut path = std::env::current_dir().unwrap();
        path.push(file_name);
        if Cmd::is_exist(&path) {
            return path;
        };

        // try find in user home ~/.config/avin/
        let mut path = std::env::home_dir().unwrap();
        path.push(".config");
        path.push("avin");
        path.push(file_name);
        if Cmd::is_exist(&path) {
            return path;
        };

        // try use default config in ~/avin/res/config.toml
        let mut path = std::env::home_dir().unwrap();
        path.push("avin");
        path.push("res");
        path.push("config.toml");
        if Cmd::is_exist(&path) {
            return path;
        };

        // panic
        log::error!("Config file not found: {path:?}");
        panic!()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DirSettings {
    root: String,
    data: String,
}
impl DirSettings {
    pub fn root(&self) -> PathBuf {
        let mut path = std::env::home_dir().unwrap();
        path.push(&self.root);

        path
    }
    pub fn data(&self) -> PathBuf {
        let mut path = std::env::home_dir().unwrap();
        path.push(&self.data);

        path
    }
    pub fn cache(&self) -> PathBuf {
        let mut path = self.data();
        path.push("cache");

        path
    }
    pub fn asset(&self) -> PathBuf {
        let mut path = self.root();
        path.push("asset");

        path
    }
    pub fn scan(&self) -> PathBuf {
        let mut path = self.root();
        path.push("scan");

        path
    }
    pub fn test(&self) -> PathBuf {
        let mut path = self.root();
        path.push("test");

        path
    }
    pub fn tmp(&self) -> PathBuf {
        let mut path = self.root();
        path.push("tmp");

        path
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectSettings {
    moex_account: Option<String>,
    moex_token: Option<String>,
    tinkoff_token: Option<String>,
}
impl ConnectSettings {
    pub fn moex_account(&self) -> PathBuf {
        let mut path = std::env::home_dir().unwrap();
        path.push(self.moex_account.as_ref().unwrap());

        path
    }
    pub fn moex_token(&self) -> PathBuf {
        let mut path = std::env::home_dir().unwrap();
        path.push(self.moex_token.as_ref().unwrap());

        path
    }
    pub fn tinkoff(&self) -> PathBuf {
        let mut path = std::env::home_dir().unwrap();
        path.push(self.tinkoff_token.as_ref().unwrap());

        path
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

pub struct DataSettings {
    pub format: String,
    pub convert: Vec<ConvertRule>,
    pub record_tics: Vec<RecordTic>,
    pub record_ob: Vec<RecordOB>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ConvertRule {
    pub iid: String,
    pub source: String,
    pub input: String,
    pub output: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordTic {
    pub iid: String,
    pub source: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordOB {
    pub iid: String,
    pub source: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoreSettings {
    pub default_asset_list: String,
    pub default_bars_count: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TesterSettings {
    pub default_commission: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TraderSettings {
    pub work_list: Vec<WorkCfg>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkCfg {
    pub iid: String,
    pub strategy: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GuiSettings {
    pub color: GuiColorSettings,
    pub chart: GuiChartSettings,
    pub test: GuiTestSettings,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GuiColorSettings {
    pub red: String,
    pub orange: String,
    pub yellow: String,
    pub green: String,
    pub cyan: String,
    pub blue: String,
    pub violet: String,
    pub white: String,
    pub grey: String,
    pub black: String,

    pub cross: String,
    pub cross_opacity: f32,

    pub bear: String,
    pub bear_opacity: f32,

    pub bull: String,
    pub bull_opacity: f32,

    pub dodji: String,
    pub dodji_opacity: f32,

    pub trend_t1: String,
    pub trend_t2: String,
    pub trend_t3: String,
    pub trend_t4: String,
    pub trend_t5: String,
    pub trend_t1_opacity: f32,
    pub trend_t2_opacity: f32,
    pub trend_t3_opacity: f32,
    pub trend_t4_opacity: f32,
    pub trend_t5_opacity: f32,
    pub auto_bar_opacity: bool,

    pub trade_open: String,
    pub trade_stop: String,
    pub trade_take: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GuiChartSettings {
    pub bottom_pane_height: f32,
    pub left_pane_width: f32,
    pub right_pane_width: f32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GuiTestSettings {
    pub trade_shift: f64,
    pub trade_size: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration() {
        let p = CFG.dir.root();
        assert_eq!(p.display().to_string(), "/home/alex/trading/usr")
    }
}
