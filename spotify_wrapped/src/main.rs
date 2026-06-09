mod current;
mod alltime;

use clap::{Parser};

#[derive(Parser)]
enum WhatStats {
    #[command(name = "current")]
    Current,

    #[command(name = "stats")]
    Stats {
        length: String
    },
}

#[tokio::main]
async fn main() {
    let what_stats = WhatStats::parse();

    match what_stats { 
        WhatStats::Current => {
            println!("Figuring out what you are currently listening to");       
            current::get_current_listening().await;
        },
        WhatStats::Stats { length } => {
            match length.to_lowercase().as_str() {
                "month" => {
                    println!("Last month stats coming up");
                    alltime::get_last_month().await;
                },
                "half_year" => { 
                    println!("Last 6 months of stats coming up");
                    alltime::get_last_six_months().await;
                },
                "year" => { 
                    println!("Last year of stats coming up");
                    alltime::get_last_year().await;
                },
                _ => { 
                    println!("INVALID OPTION... Please input either 'month', 'half_year', or 'year'");
                },
            }
        },
    }   
}