use crate::conf::DATA_DIR;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Instrument {
    pub exchange: String,
    pub itype: String,
    pub ticker: String,
}
impl Instrument {
    pub fn path(&self) -> PathBuf {
        let mut p = std::path::PathBuf::new();
        p.push(&DATA_DIR);
        p.push(&self.exchange);
        p.push(&self.itype);
        p.push(&self.ticker);

        return p;

        // format!(
        //     "{DATA_DIR}/{}/{}/{}",
        //     self.exchange, self.itype, self.ticker
        // )
    }
    pub fn clone(&self) -> Self {
        let instrument = Instrument {
            exchange: self.exchange.clone(),
            itype: self.itype.clone(),
            ticker: self.ticker.clone(),
        };
        return instrument;
    }
    pub fn from(s: &str) -> Result<Instrument, &'static str> {
        let parts: Vec<&str> = s.split("_").collect();
        if parts.len() != 3 {
            eprintln!("Fail to create instrument from str: {s}");
            return Err("Invalid instrument");
        };

        // TODO: пока работает только биржа MOEX
        let exchange = parts[0].to_uppercase();
        assert_eq!(exchange, "MOEX");

        // TODO: пока работает только тип инструмента SHARE
        let itype = parts[1].to_uppercase();
        assert_eq!(itype, "SHARE");

        // TODO: пока не сделал кэширование информации о доступных
        // инструментах работает только ниже указанные тикеры
        let ticker = parts[2].to_uppercase();
        assert!("GAZP LKOH MOEX ROSN SBER VTBR YDEX".contains(&ticker));

        let instrument = Instrument {
            exchange,
            itype,
            ticker,
        };
        Ok(instrument)
    }
}
