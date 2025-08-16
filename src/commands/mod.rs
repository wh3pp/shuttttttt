use crate::{error::Error, Data};

mod search;

#[allow(dead_code)]
pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![search::search()]
}
