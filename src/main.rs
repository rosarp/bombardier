mod parser;
mod file;
mod cmd;
mod executor;
mod http;
mod report;
mod influxdb;

use log::{info, error};

fn main() {
    pretty_env_logger::init_timed();

    let args = cmd::get_args()
        .expect("Args validation failed");

    match args.command.as_str() {
        "bombard" => {
            // Get scenarios
            info!("Reading collections file");
            let contents = file::get_content(&args.collection_file);
            
            //Get config
            info!("Reading environments file");
            let env_map = parser::get_env(&args.environment_file);

            info!("Generating bombardier requests");
            let requests = parser::parse_requests(contents, &env_map);
           
            info!("Bombarding !!!");
            executor::execute(args, env_map, requests);

            info!("Execution Complete. Run report command to get details");
        },
        "report" => {
            info!("Generating report");
            report::display(args.report_file); 
        },
        _ => {
            error!("Invalid command");
        }
    }
}