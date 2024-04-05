use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use std::borrow::BorrowMut;
use std::sync::atomic::{AtomicU32, Ordering};
use std::{sync::{Arc, RwLock}, thread::{self, JoinHandle}, time::Duration};

type ArcLock<T> = Arc<RwLock<T>>;

const NUM_SERVANTS: usize = 4;
const NUM_PRESENTS: u32 = 500_000;

enum Action {
    ChainAdd, // adding a present to the chain
    ChainSearch(u32), // search for a present with the given tag number
    WriteCard,
}

fn get_random_action() -> Action {
    match thread_rng().gen_range(0..=2) {
        0 => Action::ChainAdd,
        1 => Action::ChainSearch(thread_rng().gen_range(0..=NUM_PRESENTS)),
        _ => Action::WriteCard
    }
}

fn main() {
    println!("Hello, presents!!");

    // Initialize a vector of tag numbers to represent the unordered bag of random presents
    let mut unordered_bag: Vec<u32> = (1..=NUM_PRESENTS).collect();
    unordered_bag.shuffle(&mut thread_rng());

    // Initialize unordered bag and present chain as atomically reference-counted Reader-Writer Locks
    let unordered_bag = Arc::new(RwLock::new(unordered_bag));
    let chain = Arc::new(RwLock::new(PresentChain::default()));

    let num_thanked_presents = Arc::new(RwLock::new(0));

    // Create servant threads
    let mut servant_handles = vec![];

    for _ in 0..NUM_SERVANTS {
        let handle = do_servant_things(unordered_bag.clone(), chain.clone(), num_thanked_presents.clone());
        servant_handles.push(handle);
    }

    for handle in servant_handles {
        handle.join().unwrap();
    }

    println!("{:?}", unordered_bag.read().unwrap());
}

// this is what each thread will do!
fn do_servant_things(bag: ArcLock<Vec<u32>>, chain: ArcLock<PresentChain>, num_thanked_presents: ArcLock<u32>) -> JoinHandle<()> {
    
    thread::spawn(move || {
        loop {
            let action = get_random_action();
            
            match get_random_action() {
                Action::ChainAdd => {
                    let mut bag = bag.write().unwrap();

                    if let Some(tag_num) = bag.pop() {
                        drop(bag); // release the bag lock

                        let mut new_present = Present::new(tag_num);

                    }
                    else {
                        break; // exit the loop if the bag is empty
                    }
                },
                Action::ChainSearch(search_tag) => {
                    let chain = chain.read().unwrap();
                    chain.search(search_tag);
                },
                Action::WriteCard => {
                    let mut chain = chain.write().unwrap();
                    chain.remove_random();
                    let mut num_thanked = num_thanked_presents.write().unwrap();
                    *num_thanked += 1;
                },
            }
            

        }

        println!("Servant done!");
    })
    
    // randomely do the following:
    /*
    1. Take present from bag
    
     */


}

/*
Things the servants do:
1. Take a present from the bag and add it to the chain in 
the correct location by hooking it to the predecessors link (based on tag number)
- insertion (in the correct order based on tag number)!

2. Write a "thank you" card to the guest and remove the present from the chain
Doing this involves the servant unlinking the presnt from the chain and linking it to then next one
- deletion!

3. Minotaur can randomely ask this servant to check whether a gift is present in the chain or not
- search!

*/

#[derive(Default)]
struct PresentChain {
    size: u32,
    head: Option<Present>
}

impl PresentChain {
    fn insert(&mut self, mut present: Present) {
        if let Some(head_present) = &self.head {
            if head_present.tag_number > present.tag_number {
                
                self.head = Some(present);
            }
        }
        else {
            self.head = Some(present);
        }
        self.size += 1;
    }

    // removes a random present from the chain
    fn remove_random(&mut self) {
        self.size -= 1;
    }

    fn search(&self, tag_number: u32) -> bool {
        let mut cur_present = &self.head;
        while let Some(present) = cur_present {
            if present.tag_number == tag_number {
                return true;
            }
            else {
                //cur_present = &present.next;
            }
        }
        false
    }
}

struct Present {
    tag_number: u32,
    next: Option<Box<Present>>
}

impl Present {
    fn new(tag_number: u32) -> Self {
        Self { 
            tag_number, 
            next: None,
        }
    }

    fn set_next(&mut self, present: Present) {
        self.next = Some(Box::new(present));
    }

    fn insert() {

    }
}

