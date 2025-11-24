pub mod command;
mod handler;

use anyhow::Ok;
use std::collections::HashMap;
use tokio::sync::mpsc::{Sender, channel};

use prometheus_exporter::{
    self,
    prometheus::{
        self, register_counter, register_counter_vec, register_gauge, register_gauge_vec,
        register_histogram, register_histogram_vec,
    },
};

use crate::manager::{command::PromCommand, handler::handle_command};

pub struct PromManager {
    counters: HashMap<&'static str, prometheus::Counter>,
    counter_vecs: HashMap<&'static str, prometheus::CounterVec>,
    gauges: HashMap<&'static str, prometheus::Gauge>,
    gauge_vecs: HashMap<&'static str, prometheus::GaugeVec>,
    histograms: HashMap<&'static str, prometheus::Histogram>,
    histogram_vecs: HashMap<&'static str, prometheus::HistogramVec>,
}
::lazy_static::lazy_static! {
    pub static ref COUNTERS: tokio::sync::Mutex<HashMap<&'static str, &'static str>> = {
        tokio::sync::Mutex::new(HashMap::new())
    };
}

fn get_manager() -> PromManager {
    PromManager {
        counters: HashMap::new(),
        counter_vecs: HashMap::new(),
        gauges: HashMap::new(),
        gauge_vecs: HashMap::new(),
        histograms: HashMap::new(),
        histogram_vecs: HashMap::new(),
    }
}

//TODO: explicit하게 모든타입에대해서 이걸 받게끔 바꿔야함
pub async fn init_prometheus(
    counters: Option<&HashMap<&'static str, &'static str>>,
    counter_vecs: Option<&HashMap<&'static str, (&'static str, Vec<&'static str>)>>,
    gauges: Option<&HashMap<&'static str, &'static str>>,
    gauge_vecs: Option<&HashMap<&'static str, (&'static str, Vec<&'static str>)>>,
    histograms: Option<&HashMap<&'static str, &'static str>>,
    histogram_vecs: Option<&HashMap<&'static str, (&'static str, Vec<&'static str>)>>,
) -> anyhow::Result<Sender<PromCommand>> {
    let addr = "127.0.0.1:9090";
    let mut manager = get_manager();

    match counters {
        Some(counters) => {
            for (name, detail) in counters.iter() {
                let r = register_counter!(*name, *detail).unwrap();
                manager.counters.insert(&name, r);
            }
        }
        _ => {}
    }

    match counter_vecs {
        Some(counter_vecs) => {
            for (name, detail) in counter_vecs.iter() {
                let r = register_counter_vec!(*name, detail.0, &detail.1).unwrap();
                manager.counter_vecs.insert(&name, r);
            }
        }
        _ => {}
    }

    match gauges {
        Some(gauges) => {
            for (name, detail) in gauges.iter() {
                let r = register_gauge!(*name, *detail).unwrap();
                manager.gauges.insert(&name, r);
            }
        }
        _ => {}
    }

    match gauge_vecs {
        Some(gauge_vecs) => {
            for (name, detail) in gauge_vecs.iter() {
                let r = register_gauge_vec!(*name, detail.0, &detail.1).unwrap();
                manager.gauge_vecs.insert(&name, r);
            }
        }
        _ => {}
    }

    match histograms {
        Some(histograms) => {
            for (name, detail) in histograms.iter() {
                let r = register_histogram!(*name, *detail).unwrap();
                manager.histograms.insert(&name, r);
            }
        }
        _ => {}
    }

    match histogram_vecs {
        Some(histogram_vecs) => {
            for (name, detail) in histogram_vecs.iter() {
                let r = register_histogram_vec!(*name, detail.0, &detail.1).unwrap();
                manager.histogram_vecs.insert(&name, r);
            }
        }
        _ => {}
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
