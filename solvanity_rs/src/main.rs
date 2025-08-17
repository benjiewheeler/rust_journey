use clap::{command, value_parser, Arg};
use fancy_regex::Regex;
use num_format::{Locale, ToFormattedString};
use solana_sdk::signer::{keypair::Keypair, Signer};
use std::{
    collections::VecDeque,
    fs,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

enum Message {
    Iterations(usize),
    Key(Keypair),
}

struct SpeedTracker {
    recent_iterations: VecDeque<(Instant, usize)>,
    window_duration: Duration,
}

impl SpeedTracker {
    fn new(window_duration: Duration) -> Self {
        SpeedTracker {
            recent_iterations: VecDeque::new(),
            window_duration,
        }
    }

    fn add_iterations(&mut self, time: Instant, count: usize) {
        self.recent_iterations.push_back((time, count));

        // Remove old entries outside the time window
        let cutoff = time - self.window_duration;
        while let Some((first_time, _)) = self.recent_iterations.front() {
            if first_time >= &cutoff {
                break;
            }

            self.recent_iterations.pop_front();
        }
    }

    fn calculate_speed(&self) -> f64 {
        if self.recent_iterations.is_empty() {
            return 0.0;
        }

        let total_iterations: usize = self.recent_iterations.iter().map(|(_, count)| count).sum();

        if let (Some(first), Some(last)) = (
            self.recent_iterations.front(),
            self.recent_iterations.back(),
        ) {
            let duration = last.0 - first.0;
            if duration.as_secs_f64() > 0.0 {
                return total_iterations as f64 / duration.as_secs_f64();
            }
        }

        return 0.0;
    }
}

fn main() {
    // parse the command line arguments
    let matches = command!()
        .arg(
            Arg::new("pattern")
                .long("pattern")
                .short('p')
                .required(true),
        )
        .arg(
            Arg::new("limit")
                .long("limit")
                .short('l')
                .default_value("1")
                .value_parser(value_parser!(usize)),
        )
        .get_matches();

    // get the pattern and limit arguments
    let pattern = matches.get_one::<String>("pattern").expect("required");
    let limit = matches.get_one::<usize>("limit").clone().unwrap().clone();

    // compile the pattern and validate it
    let pattern = Regex::new(&pattern);
    let pattern = match pattern {
        Ok(pattern) => pattern,
        Err(e) => {
            println!("Invalid pattern: {}", e);
            return;
        }
    };

    let mut speed_tracker = SpeedTracker::new(Duration::from_secs(5));

    // create a channel to communicate with the threads
    let (tx, rx) = mpsc::channel();

    let start_time = Instant::now();
    let num_threads = thread::available_parallelism().unwrap().get();

    // create the threads
    for _ in 0..num_threads {
        // clone the variables so we can move them into the thread
        let tx = tx.clone();
        let pattern = pattern.clone();

        thread::spawn(move || loop {
            let mut iterations: usize = 0;

            loop {
                iterations += 1;

                // store datapoint every N iterations
                if iterations % 1000 == 0 {
                    // send the result to the main thread
                    let _ = tx.send(Message::Iterations(iterations));

                    // reset the counter
                    iterations = 0;
                }

                // generate a new keypair and check if it matches the pattern
                let kp = Keypair::new();
                let pubkey = kp.pubkey().to_string();
                let res = pattern.find(&pubkey);

                if res.is_ok() && res.unwrap().is_some() {
                    // write the base58 private key to a txt file
                    let _ = fs::write(
                        format!("key_{}.txt", kp.pubkey().to_string()),
                        format!("{}", kp.to_base58_string()),
                    );

                    // write the private key to a json file (to match the official solana cli)
                    let _ = fs::write(
                        format!("key_{}.json", kp.pubkey().to_string()),
                        serde_json::to_string(&kp.to_bytes().to_vec()).unwrap(),
                    );

                    // send the result to the main thread
                    let _ = tx.send(Message::Key(kp));
                }
            }
        });
    }

    let mut found_keys: usize = 0;
    let mut total_iterations: usize = 0;
    let mut last_report = Instant::now();

    // loop over the messages in the channel and print the data
    for msg in rx {
        match msg {
            Message::Iterations(num) => {
                total_iterations += num;
                speed_tracker.add_iterations(Instant::now(), num);

                if last_report.elapsed().as_millis() > 1000 {
                    last_report = Instant::now();

                    let elapsed = Duration::from_millis(start_time.elapsed().as_millis() as u64);
                    let speed = speed_tracker.calculate_speed();

                    println!(
                        "Round: {}, Elapsed: {:?}, Speed: {} keys/sec",
                        total_iterations.to_formatted_string(&Locale::en),
                        elapsed,
                        (speed as usize).to_formatted_string(&Locale::en),
                    );
                }
            }
            Message::Key(kp) => {
                let elapsed = Duration::from_millis(start_time.elapsed().as_millis() as u64);

                println!("Found  : {}", kp.pubkey());
                println!("Elapsed: {:?}", elapsed,);
                println!();

                found_keys += 1;
                if found_keys >= limit {
                    break;
                }
            }
        }
    }

    // no need to keep track of the tread handles, and join them
    // the main thread will live as long as the channel is open because of the loop
    // and when the loop breaks, the main thread will exit and so will the other threads
}
