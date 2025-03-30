use crate::core::Asset;
use crate::data::manager::Manager;
use crate::data::market_data::MarketData;
use crate::data::source::Source;

#[derive(Debug)]
pub struct Command {
    pub args: Vec<String>,
}
impl Command {
    pub fn build(args: &Vec<String>) -> Result<Command, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        };

        let mut copy_args: Vec<String> = Vec::new();
        for i in args {
            copy_args.push(i.clone());
        }

        let command = Command { args: copy_args };

        Ok(command)
    }
    pub async fn execute(&self) -> Result<(), &'static str> {
        if self.args[1] == "download" {
            self.try_download().await?;
        }
        if self.args[1] == "convert" {
            self.try_convert()?;
        }

        Ok(())
    }

    async fn try_download(&self) -> Result<(), &'static str> {
        // Command example:
        // avin download -s moex -i moex_share_sber --data_type bar_d
        if self.args.len() < 7 {
            panic!("Маловато аргументов для загрузки")
        };

        assert_eq!(self.args[2], "-s");
        assert_eq!(self.args[3], "moex");
        let source = Source::MOEX;

        assert_eq!(self.args[4], "-i");
        let asset = Asset::from(&self.args[5])?;

        assert_eq!(self.args[6], "--data_type");
        let data_type = MarketData::from(&self.args[7])?;

        if let Some(year_flag) = self.args.get(8) {
            assert_eq!(year_flag, "--year");
            let year: i32 = self.args[9].parse().unwrap();
            Manager::download(&source, &asset, &data_type, Some(year))
                .await?;
        } else {
            Manager::download(&source, &asset, &data_type, None).await?;
        }

        Ok(())
    }
    fn try_convert(&self) -> Result<(), &'static str> {
        // Command example:
        // avin convert -i moex_share_sber bar_1m bar_5m
        if self.args.len() < 6 {
            panic!("Маловато аргументов для конвертации")
        };

        assert_eq!(self.args[2], "-i");
        let asset = Asset::from(&self.args[3])?;

        let in_t = MarketData::from(&self.args[4])?;
        let out_t = MarketData::from(&self.args[5])?;

        Manager::convert(&asset, &in_t, &out_t)?;

        Ok(())
    }
}
