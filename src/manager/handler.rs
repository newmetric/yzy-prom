use crate::manager::{PromManager, command::PromCommand};

pub(super) fn handle_command(m: &mut PromManager, c: PromCommand) {
    match c {
        PromCommand::IncreaseCounter(name) => {
            let counter = m.counters.get(name);
            match counter {
                Some(counter) => {
                    counter.inc();
                }
                None => {
                    //TODO: add warn log
                }
            }
        }
        PromCommand::IncreaseCounterVec(name, items) => {
            let counter_vec = m.counter_vecs.get(name);
            match counter_vec {
                Some(counter_vec) => {
                    counter_vec.with_label_values(&items).inc();
                }
                None => {
                    //TODO: add warn log
                }
            }
        }
    }
}
