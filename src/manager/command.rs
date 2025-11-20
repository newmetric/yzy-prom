pub enum PromCommand {
    IncreaseCounter(&'static str),
    IncreaseCounterVec(&'static str, Vec<&'static str>),
    //Gauge
    //histogram
}
