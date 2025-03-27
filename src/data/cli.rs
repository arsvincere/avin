use crate::data::instrument::Instrument;
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

        Ok(())
    }
    async fn try_download(&self) -> Result<(), &'static str> {
        // avin-data download -s moex -i moex_share_sber --data_type bar_d
        if self.args.len() < 7 {
            panic!("Маловато аргументов для загрузки")
        };

        assert_eq!(self.args[2], "-s");
        assert_eq!(self.args[3], "moex");
        let source = Source::MOEX;

        assert_eq!(self.args[4], "-i");
        let instrument = Instrument::from(&self.args[5])?;

        assert_eq!(self.args[6], "--data_type");
        let data_type = MarketData::from(&self.args[7])?;

        Manager::download(&source, &instrument, &data_type).await?;

        Ok(())
    }
}
