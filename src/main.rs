/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::future::Future;

#[tokio::main]
async fn main() {
    let e = BarEvent {
        asset: "SBER".to_string(),
        timeframe: "5M".to_string(),
        bar: "bar_1".to_string(),
    };
    let event = Event::Bar(e);

    let mut sig_newbar = Signal::new();

    // let x = slot_1;
    // x = 5;

    sig_newbar.connect(slot_1);
    sig_newbar.connect(slot_2);

    sig_newbar.emit(&event);
}

#[derive(Debug)]
enum Event {
    Bar(BarEvent),
}

#[derive(Debug)]
struct Signal {
    receivers: Vec<fn(&Event)>,
}
impl Signal {
    pub fn new() -> Signal {
        Signal {
            receivers: Vec::new(),
        }
    }
    pub fn connect(
        &mut self,
        call_back: for<'a> fn(&'a Event) -> Future<Output = ()>,
    ) {
        self.receivers.push(call_back);
    }
    pub fn emit(&self, e: &Event) {
        for receiver in self.receivers.iter() {
            receiver(e);
        }
    }
}

#[derive(Debug)]
struct BarEvent {
    pub asset: String,
    pub timeframe: String,
    pub bar: String,
}

async fn slot_1(e: &Event) {
    let x = match e {
        Event::Bar(x) => x,
    };
    println!("slot_1 {}", x.bar);
}
async fn slot_2(e: &Event) {
    let x = match e {
        Event::Bar(x) => x,
    };
    println!("slot_2 {}", x.bar);
}
