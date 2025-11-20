#[macro_export]
macro_rules! define_counter_and_increment {
    ( $( $name:ident, $value:expr ),+ ) => {
        ::lazy_static::lazy_static! {
            static ref COUNTERS: ::std::collections::HashMap<&'static str, &'static str> = {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert(stringify!($name), $value);
                )*
                m
            };
        }

        ::paste::paste! {
            $(
                pub async fn [<increase_ $name _counter>]() {
                    let r = COMMAND_TX.get().await.clone().send(yzy_prom::manager::command::PromCommand::IncreaseCounter(stringify!($name))).await.unwrap();
                    println!("{:?}", r);
                    //TODO: make error propagate to the user
                }
            )*
        }
    };
    () => {
        // 인자가 없을 때 실행될 코드
        ::lazy_static::lazy_static! {
            static ref COUNTERS: ::std::collections::HashMap<&'static str, &'static str> = std::collections::HashMap::new();
        }
    };
}

#[macro_export]
macro_rules! define_counter_vec_and_increment {
    ( $( $name:ident, $value:expr, $label:expr ),+ ) => {
        ::lazy_static::lazy_static! {
            static ref COUNTER_VECS: ::std::collections::HashMap<&'static str, (&'static str, Vec<&'static str>)> = {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert(stringify!($name), ($value, $label));
                )*
                m
            };
        }

        ::paste::paste! {
            $(
                pub async fn [<increase_ $name _countervec>](labels: Vec<&'static str>) {
                    let r = COMMAND_TX.get().await.clone().send(yzy_prom::manager::command::PromCommand::IncreaseCounterVec(stringify!($name), labels)).await.unwrap();
                    println!("{:?}", r);
                }
            )*
        }
    };
    () => {
        // 인자가 없을 때 실행될 코드
        ::lazy_static::lazy_static! {
            static ref COUNTER_VECS: ::std::collections::HashMap<&'static str, (&'static str, Vec<&'static str>)> = std::collections::HashMap::new();
        }
    };
}

#[macro_export]
macro_rules! initialize_yzy_prom {
    () => {
        //Define commmand_tx...
        //How do I check if there is static dictionaries or not?
        ::lazy_static::lazy_static! {
            pub static ref COMMAND_TX: ::async_once::AsyncOnce<::tokio::sync::mpsc::Sender<::yzy_prom::manager::command::PromCommand>> = ::async_once::AsyncOnce::new(async {
                println!("called");
                let c = COUNTERS.clone();
                let cv = COUNTER_VECS.clone();
                ::yzy_prom::manager::init_prometheus(c, cv).await.unwrap()
            });
        }

        pub async fn launch() {
            _ = COMMAND_TX.get().await.clone();
        }
    };
}
