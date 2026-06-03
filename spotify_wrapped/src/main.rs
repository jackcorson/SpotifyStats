mod current;

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
        WhatStats::Current => {current::get_current_listening().await;},
        WhatStats::Stats {length} => {
            match length.to_lowercase().as_str() {
                "monthly" => println!("Monthly stats coming up"),
                "yearly" => println!("Yearly stats coming up"),
                "alltime" => println!("Alltime stats coming up"),
                _ => println!("INVALID OPTION... Please input either 'monthly', 'yearly', or 'alltime'"),
            }
        },
    }   
}