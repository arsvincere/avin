Например я бы вообще сделал так:

pub enum Asset {
    Share(Share),
    Bond(Bond),
    Future(Future),
}
impl From<Share> for Asset {
    fn from(value: Share) -> Self {
        Self::Share(value)
    }
}

impl From<Bond> for Asset {
    fn from(value: Bond) -> Self {
        Self::Bond(value)
    }
}

impl From<Future> for Asset {
    fn from(value: Future) -> Self {
        Self::Future(value)
    }
}

А AssetList:

pub struct AssetList {
    assets: Vec<Asset>,
}

и API:

impl AssetList {
    pub fn add(&mut self, asset: impl Into<Asset>) {
        self.assets.push(asset.into());
    }
}

Тогда:

let share = Share::new(...);
let future = Future::new(...);

list.add(share);
list.add(future);

И внутри:

Vec<Asset>
