mod arg_parser;

use arg_parser::Args;
use clap::Parser;
use reqwest::Response;

fn main() {
    let args_parsed = Args::parse();
    send_requests(args_parsed);
}

#[tokio::main]
async fn send_requests(args: Args) {
    let pool_blocks = calc_req_blocks(args.request_number, args.pool_size);

    println!(
        "Starting {} requests to {}...",
        args.request_number, &args.url
    );
    println!(
        "{} requests will be made in {:?} pool blocks, starting now...\n",
        args.request_number, &pool_blocks
    );

    for index in 0..pool_blocks.len() {
        let mut tasks = Vec::new();
        for req in 0..pool_blocks[index] {
            let url = args.url.clone();
            let task = tokio::spawn(async move {
                let res = make_request(&url).await;
                match res {
                    Ok(_) => (),
                    Err(err) => println!("Request n.: {} - {}.", req, err),
                }
            });
            tasks.push(task);
        }
        for task in tasks {
            task.await.unwrap();
        }
        println!(
            "Pool n.: {} of {} - Done | {} requests were sent.",
            index + 1,
            pool_blocks.len(),
            pool_blocks[index]
        );
    }
    println!("\nSent {} requests.", args.request_number);
}

fn calc_req_blocks(number: u32, pool_size: u32) -> Vec<u32> {
    let mut pool_blocks = Vec::new();
    let division = number / pool_size;
    let remainder = number % pool_size;

    for _ in 0..division {
        pool_blocks.push(pool_size);
    }
    if remainder > 0 {
        pool_blocks.push(remainder);
    }
    pool_blocks
}

async fn make_request(url: &str) -> Result<Response, reqwest::Error> {
    reqwest::get(url).await
}
