/// copy


mod config;

use config::{Task, Config, RequestTask};

#[tokio::main]
async fn main() {

    let c = Config::new("config.json");

    for t in c.tasks {
        start_task(&t);
    }

    // println!("{:?}", c);

    let handle = tokio::spawn(async {
        println!("doing some work, asynchronously");

        // Return a value for the example
        "result of the computation"
    });

    // Wait for the spawned task to finish
    let res = handle.await;

    println!("got {:?}", res);
}

fn start_task(t: &Task){
    thread::spawn(move || {
        let mut c = 1u64;
        loop {
            if times >0 && c > times {
                break;
            }
            func(c);
            println!("当前任务执行完成,下次执行时间为: {:?}", Local::now().naive_local() + Duration::seconds(delay as i64));
            thread::sleep(StdDuration::new(delay, 0));
            c += 1;
        }
    });
}

/// 执行请求
/// param: 请求任务
fn do_request(req_task: &RequestTask) {
    let client = reqwest::blocking::Client::new();
    let res = client.post(req_task.url)
        .json(req_task.data)
        .send();

    println!("{:?}", res);
}