use tokio::sync::oneshot;

pub enum PromCommand {
    IncreaseCounter(&'static str),
    IncreaseCounterVec(&'static str, Vec<&'static str>),
    SetGauge(&'static str, f64),
    GetGauge(&'static str, oneshot::Sender<f64>),
    SetGaugeVec(&'static str, Vec<&'static str>, f64),
    GetGaugeVec(&'static str, Vec<&'static str>, oneshot::Sender<f64>),
    ObserveHistogram(&'static str, f64),
    ObserveHistogramVec(&'static str, Vec<&'static str>, f64),
}
