use rand::thread_rng;
use rand::seq::SliceRandom;


fn main() {
    println!("Hello, presents!!");

    // Initialize a vector of tag numbers to represent the unordered bag of random presents
    // This can be accessed by multiple threads so we shuld make an Arc!
    let mut unordered_bag: Vec<u32> = (2..500_001).collect();
    unordered_bag.shuffle(&mut thread_rng());

    let mut chain_head: Present = Present::new(1);
    // Each servant:
    // take a value from the unordered bag (shared, arc)
    // insert it into the chain head (shared, arc)
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




struct Present {
    tag_number: u32,
    next: Option<Box<Present>>
}

impl Present {
    fn new(tag_number: u32) -> Self {
        Self { 
            tag_number, 
            next: None 
        }
    }

    fn set_next(&mut self, present: Present) {
        self.next = Some(Box::new(present));
    }
}

