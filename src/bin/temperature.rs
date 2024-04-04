use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use rand::{thread_rng, Rng};

const NUM_SENSORS: usize = 8;

/// Runs a sensor (spawns a thread) for an hour (60 ms), enough to generate a report
fn run_sensor(tx: Sender<f32>) {
    thread::spawn(move || {
        for _ in 0..60 {
            let temperature = thread_rng().gen_range(-100.0..70.0); // generates a random temperature from -100 to 70 F
            tx.send(temperature).unwrap(); // sends the temperature to the transmitter
            thread::sleep(Duration::from_millis(1)); // sleeps as it waits to read another temperature
        }
    });
}

// Returns a tuple representing the largest 10-min interval temperature difference
// Tuple structured as (minute, difference), if it was at 1-10 mins with an 80 degree diff it would be (1, 80)
fn get_largest_10_min_diff(readings: &Vec<f32>) -> (usize, f32) {
    let mut minute_ranges = vec![];

    for minute in 0..60 {
        let start = minute * NUM_SENSORS;
        let end = start + NUM_SENSORS;
        let minute_slice = &readings[start..end];

        let minute_min = *minute_slice.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let minute_max = *minute_slice.into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        minute_ranges.push(minute_min..minute_max);
    }

    let mut largest_diff = (0, 0.0);
    for minute in 0..50 {
        let one_range = &minute_ranges[minute];
        let ten_range = &minute_ranges[minute + 10];

        let diff = f32::max((one_range.start - ten_range.end).abs(), (one_range.end - ten_range.start).abs());
        if diff > largest_diff.1 {
            largest_diff.0 = minute;
            largest_diff.1 = diff;
        }
    }

    largest_diff
} 

fn main() {
    // sets up a multi-producers, single-consumer channel for passing data from all threads (sensors)
    let (tx, rx) = mpsc::channel();



    // runs sensors - 1 specifically for cloning the transmitter
    for _ in 0..(NUM_SENSORS - 1) {
        run_sensor(tx.clone());
    }

    // runs a sensor without cloning the transmitter (so that the lifetime is on this sensor)
    run_sensor(tx);

    let mut readings: Vec<f32> = rx.iter().collect();

    // sorts all readings for the report
    readings.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // get the largest 10 minute interval difference
    let largest_10_min_diff = get_largest_10_min_diff(&readings);
    
    println!(" -- Hourly Report --");
    println!("Note: Temperature rankings are from lowest to highest temperature in degrees F.");
    println!();
    println!("Lowest 5 Temperatures: {:?}", &readings[..5]);
    println!("Highest 5 Temperatures: {:?}", &readings[readings.len()-5..readings.len()]);
    println!();
    println!("10-Minute interval from {}-{} had the largest temperature difference of {} degrees F.", largest_10_min_diff.0, largest_10_min_diff.0 + 10, largest_10_min_diff.1);
}