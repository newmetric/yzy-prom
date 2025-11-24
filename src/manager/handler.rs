use crate::manager::{PromManager, command::PromCommand};

pub(super) fn handle_command(m: &mut PromManager, c: PromCommand) {
    match c {
        PromCommand::IncreaseCounter(name) => {
            let counter = m.counters.get(name);
            match counter {
                Some(counter) => {
                    counter.inc();
                }
                None => {}
            }
        }
        PromCommand::IncreaseCounterVec(name, items) => {
            let counter_vec = m.counter_vecs.get(name);
            match counter_vec {
                Some(counter_vec) => {
                    counter_vec.with_label_values(&items).inc();
                }
                None => {}
            }
        }
        PromCommand::SetGauge(name, value) => {
            let gauge = m.gauges.get(name);
            match gauge {
                Some(gauge) => {
                    gauge.set(value);
                }
                None => {}
            }
        }
        PromCommand::GetGauge(name, sender) => {
            let gauge = m.gauges.get(name);
            match gauge {
                Some(gauge) => {
                    let _ = sender.send(gauge.get());
                }
                None => {
                    //TODO: send error instead of -1
                    let _ = sender.send(-1f64);
                }
            }
        }
        PromCommand::SetGaugeVec(name, items, value) => {
            let gauge_vec = m.gauge_vecs.get(name);
            match gauge_vec {
                Some(gauge_vec) => {
                    gauge_vec.with_label_values(&items).set(value);
                }
                None => {}
            }
        }
        PromCommand::GetGaugeVec(name, items, sender) => {
            let gauge_vec = m.gauge_vecs.get(name);
            match gauge_vec {
                Some(gauge_vec) => {
                    let v = gauge_vec.with_label_values(&items).get();
                    let _ = sender.send(v);
                }
                None => {
                    //TODO: send error instead of -1
                    let _ = sender.send(-1f64);
                }
            }
        }
        PromCommand::ObserveHistogram(name, value) => {
            let histogram = m.histograms.get(name);
            match histogram {
                Some(histogram) => {
                    histogram.observe(value);
                }
                None => {}
            }
        }
        PromCommand::ObserveHistogramVec(name, items, value) => {
            let histogram_vec = m.histogram_vecs.get(name);
            match histogram_vec {
                Some(histogram_vec) => {
                    histogram_vec.with_label_values(&items).observe(value);
                }
                None => {}
            }
        }
    }
}
