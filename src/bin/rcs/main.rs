use rtools::parser::parse_args;
mod config;

fn main() {
    let rcs_config : config::RCSConfig =  parse_args();
}