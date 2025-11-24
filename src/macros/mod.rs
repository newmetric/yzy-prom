//COUNTER, COUNTERVECS
#[macro_export]
macro_rules! define_counter_functions {
    ( $name:ident, $value:expr ) => {
        ::paste::paste! {
            pub async fn [<increment_ $name _counter>]() {
                use ::yzy_prom::manager::command::PromCommand;

                let _ = COMMAND_TX.get().await.clone().send(PromCommand::IncreaseCounter(stringify!($name))).await.unwrap();
                //TODO: make error propagate to the user
            }
        }
    };
}

#[macro_export]
macro_rules! define_counter_vec_functions {
    ( $name:ident, $value:expr, $label:expr ) => {
        ::paste::paste! {
            pub async fn [<increment_ $name _countervec>](labels: Vec<&'static str>) {
                use ::yzy_prom::manager::command::PromCommand;
                let _ = COMMAND_TX.get().await.clone().send(PromCommand::IncreaseCounterVec(stringify!($name), labels)).await.unwrap();
                //TODO: make error propagate to the user
            }
        }
    };
}

//GAUGEs, GAUGE_VECS
#[macro_export]
macro_rules! define_gauge_functions {
    ( $name:ident, $value:expr ) => {
        ::paste::paste! {
            pub async fn [<set $name _gauge>]() {
                use ::yzy_prom::manager::command::PromCommand;
                let _ = COMMAND_TX.get().await.clone().send(PromCommand::SetGauge(stringify!($name))).await.unwrap();
                //TODO: make error propagate to the user
            }

            pub async fn [<get $name _gauge>]() -> f64 {
                use ::yzy_prom::manager::command::PromCommand;
                use tokio::sync::oneshot::channe;
                let (tx, rx) = channel();
                let _ = COMMAND_TX.get().await.clone().send(PromCommand::GetGauge(stringify!($name), tx)).await.unwrap();

                rx.await.unwrap()
                //TODO: make error propagate to the user
            }
        }
    };
}

#[macro_export]
macro_rules! define_gauge_vec_functions {
    ( $name:ident, $value:expr, $label:expr ) => {
        ::paste::paste! {
            pub async fn [<set $name _gaugevec>](labels: Vec<&'static str>) {
                use ::yzy_prom::manager::command::PromCommand;
                let _ = COMMAND_TX.get().await.clone().send(PromCommand::SetGaugeVec(stringify!($name), labels)).await.unwrap();
                //TODO: make error propagate to the user
            }

            pub async fn [<get $name _gaugevec>](labels: Vec<&'static str>) -> f64 {
                use ::yzy_prom::manager::command::PromCommand;
                use tokio::sync::oneshot::channe;
                let (tx, rx) = channel();
                let _ = COMMAND_TX.get().await.clone().send(PromCommand::GetGaugeVec(stringify!($name), labels, tx)).await.unwrap();

                rx.await.unwrap()
                //TODO: make error propagate to the user
            }
        }
    };
}

//HISTOGRAMS HISTOGRAM_VECS
#[macro_export]
macro_rules! define_histogram_functions {
    ( $name:ident, $value:expr ) => {
        ::paste::paste! {
            pub async fn [<observe $name _histogram>](value: f64) {
                use ::yzy_prom::manager::command::PromCommand;
                let _ = COMMAND_TX.get().await.clone().send(PromCommand::ObserveHistogram(stringify!($name), value)).await.unwrap();
                //TODO: make error propagate to the user
            }
        }
    };
}

#[macro_export]
macro_rules! define_histogram_vec_functions {
    ( $name:ident, $value:expr, $label:expr ) => {
        ::paste::paste! {
            pub async fn [<observe $name _histogramvec>](labels: Vec<&'static str>, value: f64) {
                use ::yzy_prom::manager::command::PromCommand;
                let _ = COMMAND_TX.get().await.clone().send(PromCommand::ObserveHistogramVec(stringify!($name), labels, value)).await.unwrap();
                //TODO: make error propagate to the user
            }

            pub async fn [<get $name _histogramvec>](labels: Vec<&'static str>) -> f64 {
                use ::yzy_prom::manager::command::PromCommand;
                use tokio::sync::oneshot::channe;
                let (tx, rx) = channel();
                let _ = COMMAND_TX.get().await.clone().send(PromCommand::GetGaugeVec(stringify!($name), labels, tx)).await.unwrap();

                rx.await.unwrap()
                //TODO: make error propagate to the user
            }
        }
    };
}

#[macro_export]
macro_rules! initialize_yzy_prom {
    (
        $( counter = [ $( $counter_name:ident, $counter_detail:expr, )* ], )?
        $( counter_vec = [ $( $counter_vec_name:ident, $counter_vec_detail:expr, $counter_vec_labels:expr, )* ], )?
        $( gauge = [ $( $gauge_name:ident, $gauge_detail:expr, )* ], )?
        $( gauge_vec = [ $( $gauge_vec_name:ident, $gauge_vec_detail:expr, $gauge_vec_labels:expr, )* ], )?
        $( histogram = [ $( $histogram_name:ident, $histogram_detail:expr, )* ], )?
        $( histogram_vec = [ $( $histogram_vec_name:ident, $histogram_vec_detail:expr, $histogram_vec_labels:expr, )* ], )?
    ) => {
        const COUNTER_KEY: &str = "counter";
        const COUNTER_VEC_KEY: &str = "counter_vec";
        const GAUGE_KEY: &str = "gauge";
        const GAUGE_VEC_KEY: &str = "gauge_vec";
        const HISTOGRAM_KEY: &str = "histogram";
        const HISTOGRAM_VEC_KEY: &str = "histogram_vec";

        ::lazy_static::lazy_static! {
            static ref MAPS: ::tokio::sync::Mutex<::std::collections::HashMap<&'static str, ::std::collections::HashMap<&'static str, &'static str>>> = {
                ::tokio::sync::Mutex::new(::std::collections::HashMap::new())
            };

            static ref VEC_MAPS: ::tokio::sync::Mutex<::std::collections::HashMap<&'static str, ::std::collections::HashMap<&'static str, (&'static str, Vec<&'static str>)>>> = {
                ::tokio::sync::Mutex::new(::std::collections::HashMap::new())
            };
        }

        ::lazy_static::lazy_static! {
            static ref COMMAND_TX: ::async_once::AsyncOnce<::tokio::sync::mpsc::Sender<::yzy_prom::manager::command::PromCommand>> = ::async_once::AsyncOnce::new(async {
                use ::std::collections::HashMap;
                use ::std::option::Option;

                let mut c = Option::None;
                let mut cv = Option::None;
                let mut g = Option::None;
                let mut gv = Option::None;
                let mut h = Option::None;
                let mut hv = Option::None;

                let maps = MAPS.lock().await;
                let vec_maps = VEC_MAPS.lock().await;

                c = maps.get(COUNTER_KEY);
                cv = vec_maps.get(COUNTER_VEC_KEY);
                g = maps.get(GAUGE_KEY);
                gv = vec_maps.get(GAUGE_VEC_KEY);
                h = maps.get(HISTOGRAM_KEY);
                hv = vec_maps.get(HISTOGRAM_VEC_KEY);

                ::yzy_prom::manager::init_prometheus(
                    c.clone(),
                    cv.clone(),
                    g.clone(),
                    gv.clone(),
                    h.clone(),
                    hv.clone(),
                ).await.unwrap()
            });
        }

        pub async fn launch() {
            use ::std::collections::HashMap;

            {
                let mut maps = MAPS.lock().await;
                let mut vec_maps = VEC_MAPS.lock().await;

                let mut counter = HashMap::new();
                let mut counter_vec = HashMap::new();
                let mut gauge = HashMap::new();
                let mut gauge_vec = HashMap::new();
                let mut histogram = HashMap::new();
                let mut histogram_vec = HashMap::new();
                //COUNTER, COUNTER_VEC
                $(
                    $(
                        counter.insert(stringify!($counter_name), $counter_detail);
                    )*
                )?
                maps.insert(COUNTER_KEY, counter);

                $(
                    $(
                        counter_vec.insert(stringify!($counter_vec_name), ($counter_vec_detail, $counter_vec_labels));
                    )*
                )?
                vec_maps.insert(COUNTER_VEC_KEY, counter_vec);

                //GAUGE, GAUGE_VEC
                $(
                    $(
                        gauge.insert(stringify!($gauge_name), $gauge_detail);
                    )*
                )?
                maps.insert(GAUGE_KEY, gauge);

                $(
                    $(
                        gauge_vec.insert(stringify!($gauge_vec_name), ($gauge_vec_detail, $gauge_vec_labels));
                    )*
                )?
                vec_maps.insert(GAUGE_VEC_KEY, gauge_vec);

                //HISTOGRAM, HISTGRAM_VEC
                $(
                    $(
                        histogram.insert(stringify!($histogram_name), $histogram_detail);
                    )*
                )?
                maps.insert(HISTOGRAM_KEY, histogram);

                $(
                    $(
                        histogram_vec.insert(stringify!($histogram_vec_name), ($histogram_vec_detail, $histogram_vec_labels));
                    )*
                )?
                vec_maps.insert(HISTOGRAM_VEC_KEY, histogram_vec);
            }

            _ = COMMAND_TX.get().await.clone();
        }

        $(
            $(
                ::yzy_prom::define_counter_functions!($counter_name, $counter_name);
            )*
        )?

        $(
            $(
                ::yzy_prom::define_counter_vec_functions!($counter_vec_name, $counter_vec_detail, $counter_vec_labels);
            )*
        )?

        $(
            $(
                ::yzy_prom::define_gauge_functions!($gauge_name, $gauge_name);
            )*
        )?

        $(
            $(
                ::yzy_prom::define_gauge_vec_functions!($gauge_vec_name, $gauge_vec_detail, $gauge_vec_labels);
            )*
        )?

        $(
            $(
                ::yzy_prom::define_histogram_functions!($histogram_name, $histogram_name);
            )*
        )?

        $(
            $(
                ::yzy_prom::define_histogram_vec_functions!($histogram_vec_name, $histogram_vec_detail, $histogram_vec_labels);
            )*
        )?
    };
}
