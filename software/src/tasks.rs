use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

#[allow(unused)]
pub fn run() {
    let runtime = Runtime::new().unwrap();

    let /*mut*/ value: u8 = 0;

    let print_task_1 = runtime.spawn(print("print_task_1", value));
    let print_task_2 = runtime.spawn(print("print_task_2", value));
    //let increment_task = runtime.spawn(increment("increment_task", &mut value));

    runtime.block_on(async {
        let _ = tokio::join!(print_task_1, print_task_2, /*increment_task*/);
    });
}

async fn print(name: &str, value: u8) -> ! {
    loop {
        println!("{}: {}", name, value);
        // It is nice to use the async sleep function. But tokio can also handle blocking tasks.
        sleep(Duration::from_secs(1)).await;
    }
}

// async fn increment(name: &str, value: &mut u8) -> ! {
//     loop {
//         *value += 1;
//         println!("{}: {}", name, value);
//         sleep(Duration::from_secs(1)).await;
//     }
// }
