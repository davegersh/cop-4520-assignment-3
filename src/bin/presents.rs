use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use std::sync::mpsc::{self, Sender};
use std::{sync::{Arc, RwLock}, thread::{self, JoinHandle}};

type ArcLock<T> = Arc<RwLock<T>>;

const NUM_SERVANTS: usize = 4;
const NUM_PRESENTS: u32 = 500_000;

fn generate_random_presents() -> Vec<Present> {
    let mut unordered_bag: Vec<Present> = (1..=NUM_PRESENTS).map(|tag| Present::new(tag)).collect();
    unordered_bag.shuffle(&mut thread_rng());

    unordered_bag
}

// this is what each servant does!
fn do_servant_things(bag: ArcLock<Vec<Present>>, tx: Sender<Present>) -> JoinHandle<()> {
    
    thread::spawn(move || {
        let mut thanked_presents = vec![];

        loop {
            let write_card = thread_rng().gen_bool(0.99);

            if write_card {
                let mut bag = bag.write().unwrap();
                if let Some(mut present) = bag.pop() {
                    present.card_written = true;
                    tx.send(present.clone()).unwrap();
                    thanked_presents.push(present);
                }
                else {
                    break;
                }
            }
            else {
                let search_tag = thread_rng().gen_range(1..=500_000);

                for present in thanked_presents.iter() {
                    if present.tag_number == search_tag {
                        println!("Minotaur found thank you card for tag number {}", search_tag);
                    }
                }
            }
        }
    })
}

#[derive(Clone)]
struct Present {
    tag_number: u32,
    card_written: bool
}

impl Present {
    fn new(tag_number: u32) -> Self {
        Self { 
            tag_number, 
            card_written: false,
        }
    }
}

fn main() {
    // sets up a multi-producers, single-consumer channel for passing data from all threads (sensors)
    let (tx, rx) = mpsc::channel();

    // Initialize unordered bag and present chain as atomically reference-counted Reader-Writer Locks
    let unordered_bag = Arc::new(RwLock::new(generate_random_presents()));

    // Create servant threads
    let mut servant_handles = vec![];

    for _ in 0..NUM_SERVANTS-1 {
        let handle = do_servant_things(unordered_bag.clone(), tx.clone());
        servant_handles.push(handle);
    }

    let handle = do_servant_things(unordered_bag.clone(), tx);
    servant_handles.push(handle);

    for handle in servant_handles {
        handle.join().unwrap();
    }

    let thanked_presents: Vec<Present> = rx.iter().collect();
    println!("Servants Complete!");
    println!("Total Presents / Cards Written: {} / {}", thanked_presents.len(), NUM_PRESENTS);
}

