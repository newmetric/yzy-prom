pub mod command;
mod handler;

use anyhow::Ok;
use std::collections::HashMap;
use tokio::sync::mpsc::{Sender, channel};

use prometheus_exporter::{
    self,
    prometheus::{self, register_counter, register_counter_vec},
};

use crate::manager::{command::PromCommand, handler::handle_command};

pub struct PromManager {
    counters: HashMap<&'static str, prometheus::Counter>,
    counter_vecs: HashMap<&'static str, prometheus::CounterVec>,
}

fn get_manager() -> PromManager {
    PromManager {
        counters: HashMap::new(),
        counter_vecs: HashMap::new(),
    }
}

//TODO: explicit하게 모든타입에대해서 이걸 받게끔 바꿔야함
pub async fn init_prometheus(
    counters: HashMap<&'static str, &'static str>,
    counter_vecs: HashMap<&'static str, (&'static str, Vec<&'static str>)>,
) -> anyhow::Result<Sender<PromCommand>> {
    let addr = "127.0.0.1:9090";
    let mut manager = get_manager();

    for (name, detail) in counters.iter() {
        let r = register_counter!(*name, *detail).unwrap();
        manager.counters.insert(&name, r);
    }

    for (name, detail) in counter_vecs.iter() {
        let r = register_counter_vec!(*name, detail.0, &detail.1).unwrap();
        manager.counter_vecs.insert(&name, r);
    }

    let (command_tx, mut command_rx) = channel(256);

    tokio::spawn(async move {
        loop {
            match command_rx.recv().await {
                Some(command) => {
                    handle_command(&mut manager, command);
                }
                None => break,
            }
        }
    });
    println!("handler launched");

    let _exporter = prometheus_exporter::start(addr.parse()?).unwrap();
    println!("Prometheus exporter running on {}", addr);

    Ok(command_tx)
}
