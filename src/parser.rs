use itertools::Itertools;
use std::{env, collections::HashMap};

pub fn parse_args() -> (String, HashMap<String, String>) {
    let args : Vec<String> = env::args().collect();
    let mut args_iter = args.into_iter();
    let command_name = args_iter.next().unwrap();
    let final_args : Vec<String> = args_iter.collect();
    (command_name, final_args.into_iter().tuples().collect())
}
