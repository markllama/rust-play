// Load and merge two JSON files describing network hosts
// The two files have different schema but both have a MAC address field that can be used to
// pair records for merge

// * Get two file names from the CLI
// * Open both files and load them into memory
// * compare and detect matching records in both lists
// * generate a new record for every unique record in both lists
// * generate a single new record for each matching record in both lists

fn main() {

    if let Err(e) = merge_hosts::get_args().and_then(merge_hosts::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
   
}
