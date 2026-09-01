а если сделать trait AddUserFriendlyCreateAsset {
   fn new(...)
}
и его уже имплементить где то выше... не в домене... и там уже дергать и сервисы и все что надо... то есть мы просто впихиваем в неймспейс Asset потом функцию... а текущий конструктор из InstrumentInfo переделать в TryFrom или в from_info(..)?

Да. Это реально рабочий и довольно красивый хак, и он как раз позволяет получить желаемый UX, не загрязняя avin_domain зависимостями на service/storage.

Схема такая.

В avin_domain:

pub enum Asset {
    Share(Share),
    Future(Future),
}

И текущий конструктор из InstrumentInfo я бы убрал из имени new.

Лучше всего:

impl TryFrom<InstrumentInfo> for Asset {
    type Error = AvinError;

    fn try_from(info: InstrumentInfo) -> Result<Self, Self::Error> {
        match info.category() {
            Category::Share => Ok(Self::Share(Share::new(info)?)),
            Category::Future => Ok(Self::Future(Future::new(info)?)),
            category => Err(...),
        }
    }
}

Потому что это буквально conversion:

InstrumentInfo -> Asset

и может завершиться ошибкой.

А уже в верхнем avin crate:

pub trait AssetCreate {
    fn new(iid: &str) -> Result<Self, AvinError>
    where
        Self: Sized;
}
impl AssetCreate for Asset {
    fn new(iid: &str) -> Result<Self, AvinError> {
        let info = /* service/storage lookup */;
        let mut asset = Asset::try_from(info)?;

        // load market data etc.

        Ok(asset)
    }
}

И если trait в scope:

use avin::{Asset, AssetCreate};

let asset = Asset::new("moex.share.sber")?;

Да, именно Asset::new(...).

Rust умеет находить associated function trait'а через тип.

А с твоим будущим:

use avin::prelude::*;

пользователь вообще не думает об этом:

let asset = Asset::new("moex.share.sber")?;

LSP знает, откуда new, docs.rs показывает trait implementation — все нормально.
