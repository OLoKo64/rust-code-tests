/// Example of a Promise.all implementation in Rust.
pub async fn call_all_futures() {
    let futures = vec![
        async_function(1),
        async_function(2),
        async_function(3),
        async_function(4),
        async_function(5),
    ];
    // This will execute all futures concurrently and return a vector of the results.
    // The results will be in the same order as the futures.
    // flatten() is used to flatten the Option<i32> into i32, getting rid of the None values.
    let results = futures::future::join_all(futures)
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    println!("Results: {:?}", results);
}

async fn async_function(num: i32) -> Option<i32> {
    if num % 2 == 0 {
        Some(num)
    } else {
        None
    }
}
