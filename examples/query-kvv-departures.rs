use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Wake},
    thread::{self, Thread},
    time::{Duration, Instant}
};

use kvv_efa_api::{
    request::*,
    response::DepartureMonitorResponseData
};

enum LiveStatus<T> {
    InitiateUpdate,
    UpdateInProgress(Pin<Box<T>>, Instant),
    Idle(Instant)
}

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut args = std::env::args();
    let exec_name = args.next().unwrap();

    let mut n = 2;
    let mut station_id = None;
    let mut live = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("Usage: {exec_name} [OPTIONS...]

Options:
    -station <station id>   Set the station id (REQUIRED)

    -n <number>             Set the number of departures printed [default: {n}]
    -l, --live              Keep the process active and update periodically
    -h, --help              Print this help text and exit."
                );

                return Ok(())
            }
            "-n" => {
                n = args.next()
                    .and_then(|s| s.parse().ok())
                    .expect("-i expects an integer argument");
            }
            "-station" => {
                station_id = Some(
                    args.next()
                    .and_then(|s| s.parse::<i32>().ok())
                    .expect("-i expects an integer argument")
                );
            }
            "--live" | "-l" => {
                live = true;
            }
            _ => {
                panic!("{exec_name}: unknown argument \"{arg}\"");
            } 
        }
    }

    if station_id.is_none() {
        panic!("{exec_name}: no `-station` argument provided");
    }

    let request = DepartureMonitorRequest::builder()
        .name(station_id.unwrap())
        .limit(n)
        .build();

    if !live {
        println!("{}", parse_response(&request.get().await?));
        return Ok(())
    }

    let mut str = String::new();
    let mut scroll_offset = 0;

    let mut status = LiveStatus::InitiateUpdate;

    let waker = Arc::new(ThreadWaker(thread::current())).into();
    let mut cx = Context::from_waker(&waker);

    let update_interval = Duration::from_nanos(60_000_000_000);
    let scroll_width = 40;
    
    loop {
        match &mut status {
            LiveStatus::InitiateUpdate => {
                status = LiveStatus::UpdateInProgress(Box::pin(request.clone().get()), Instant::now());
            }
            LiveStatus::UpdateInProgress(response_pin, update_time) => {
                match response_pin.as_mut().poll(&mut cx) {
                    Poll::Ready(res) => {
                        str = parse_response(&res?);
                        status = LiveStatus::Idle(update_time.to_owned());
                    }
                    Poll::Pending => ()
                }
            }
            LiveStatus::Idle(instant) if Instant::now() - *instant > update_interval => {
                status = LiveStatus::InitiateUpdate; 
            }
            _ => ()
        }

        let n_chars = str.chars().count();
        if n_chars > 0 {
            for i in scroll_offset.. scroll_offset + scroll_width {
                print!("{}", str.chars().nth(i % n_chars).unwrap());
            }
            println!();

            scroll_offset += 1;
            if scroll_offset >= str.len() {
                scroll_offset = 1;
            }
        }
        std::thread::sleep(Duration::from_nanos(75_000_000)); 
    }
}

fn parse_response(data: &DepartureMonitorResponseData) -> String {
    fn countdown(countdown: &str) -> String {
        match countdown {
            "-9999" => "cancelled".into(),
            "" => "unknown".into(),
            c => {
                if c.parse::<i32>().unwrap_or(0) <= 0 {
                    format!("now")
                }
                else {
                    format!("{c} min")
                }
            }
        }
    }

    return data.departure_list.iter()
        .map(|dep| format!(" :: ({}) [{}] {}", countdown(&dep.countdown), dep.serving_line.symbol, dep.serving_line.direction))
        .fold(String::new(), |a, b| a + b.as_str())
}
