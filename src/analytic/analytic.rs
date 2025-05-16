/*****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::PathBuf;

use polars::frame::DataFrame;

use crate::{Asset, Cmd, Share, TimeFrame};

pub trait Analytic {
    fn name() -> &'static str;
    fn analyse(share: &Share, tf: &TimeFrame) -> Result<(), String>;
    fn analyse_all() -> Result<(), String>;
    fn save(share: &Share, name: &str, df: &mut DataFrame) {
        let path = create_path(share, name);
        Cmd::write_pqt(df, &path).unwrap();

        log::info!("   Analytic save {}", path.display());
    }
    fn load(share: &Share, name: &str) -> Result<DataFrame, String> {
        let path = create_path(share, name);

        if !Cmd::is_exist(&path) {
            let msg = format!("analyse not found: {}", path.display());
            return Err(msg);
        }

        let result = Cmd::read_pqt(&path);
        match result {
            Ok(df) => {
                log::info!("   Analytic load {}", path.display());
                Ok(df)
            }
            Err(why) => Err(format!("{}", why)),
        }
    }
}

// private
fn create_path(share: &Share, analyse_name: &str) -> PathBuf {
    let mut path = share.path();

    path.push("analyse");

    let analyse_name = format!("{}.pqt", analyse_name);
    for part in analyse_name.split(' ') {
        path.push(part);
    }

    path
}
