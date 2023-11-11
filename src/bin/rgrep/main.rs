use std::path::Path;

use rtools::parser::parse_args;

mod search;
fn main() {
    let (command_name, args) = parse_args();

    let mut new_search = search::Search::new();
    new_search.recurse_and_return(
        args.get("--term").unwrap(),
        Path::new(args.get("--path").unwrap()),
    );
    new_search.print_results();
}
