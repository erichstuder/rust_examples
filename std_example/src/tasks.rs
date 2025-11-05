use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

static VALUE: u8 = 0;

#[allow(unused)]
pub fn run() {
    let runtime = Runtime::new().unwrap();

    let print_task_1 = runtime.spawn(print("print_task_1", &VALUE));
    let print_task_2 = runtime.spawn(print("print_task_2", &VALUE));
    // let increment_task = runtime.spawn(increment("increment_task", &mut VALUE));

    runtime.block_on(async {
        let _ = tokio::join!(print_task_1, print_task_2, /*increment_task*/);
    });
}

async fn print(name: &str, value: &u8) -> ! {
    loop {
        println!("{}: {}", name, value);
        sleep(Duration::from_secs(1)).await;
    }
}

#[allow(unused)]
async fn increment(name: &str, value: &mut u8) -> ! {
    loop {
        *value += 1;
        println!("{}: {}", name, value);
        sleep(Duration::from_secs(1)).await;
    }
}
