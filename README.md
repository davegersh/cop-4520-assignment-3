## COP 4520 - Assignment #3
A minotaur has half a million birthday presents and a mars rover has a few thermometers.

Note that this repo is based on the assignment outlined in the `assignment.pdf` file.
All below explanations require understanding of the assignment first.

To run any of the two problems, clone this repo to a desired location and make sure to have rust and cargo installed, if not, install them using [rustup](https://rustup.rs).

## Problem 1: The Birthday Presents Party
This is referring to code in `src/bin/presents.rs`.

### Design Breakdown
The servants, in there best efforts to solve this problem decided to talk to the Minotaur about problems with their implementation, as a result, the servants the minotaur came to a compromise: don't use a linked list! As a result, the presents where simply placed on the ground instead of linked in a chain. This way, the minotaur could also save money by not needing to purchase several hundreds of thousands of chains. After deciding not to use a linked list, the minotaur realized how archaic and problematic the usage of chains might be (especially for a memory safe language like Rust!).

So following that change, the servants had a much easier time working with the presents and they did so in a simple and clean way. Each servant was assigned some subset of the tag numbers that they would work with. When they grabbed a present from the bag, they check to see if it is assigned to them, if so, they make a "thank you" card and send it. The present is then added to a list containing all presents that have been thanked. Notice how the extra-step of a chain is completely removed and makes things significantly more efficient! The minotaur is happy!

This is all implemented by having a lock on the unordered bag and have each servant manage their list of thanked presents. Whenever a present is removed from the bag, it is locked beforehand, then unlocked once a present is removed.This ensures that any contension is handled approprately with the unordered bag. Also with the ability for each servant to handle their set of thanked presents, we can completely avoid contention there. For searching, the servants are occassional interrupted (about 1% of the time) with a request to search for a compelted present within their list. After all presents are thanked, each servant, sends the completed list to a Multi-Producer Single-Consumer transmitter. This is explained better in problem 2 of the assignment, where this is heavily used.

### Evaluation
In this case it doesn't make much sense to do a proper evaluation as it can be a bit random with the occassional search request from the minotaur. The entire algorithm though runs in a $O(n/s)$ time where $n$ represents the total number of presents and $s$ represnts the number of servants. The number of servants massively improves performance as each servant handles their own set of presents.

Below is an example of the output (last 5 lines shown for brevity):
```
...
Minotaur found thank you card for tag number 480405
Minotaur found thank you card for tag number 283370
Minotaur found thank you card for tag number 162739
Servants Complete!
Total Presents / Cards Written: 500000 / 500000
```

### Running
Type in the following command to run the code for problem 1:
```
cargo run --bin presents
```

## Problem 2: Atmospheric Temperature Reading Module
This is referring to code in `src/bin/temperature.rs`.

### Design Breakdown
Solving this problem is quite simple if we consider it as a Multi-Producer, Single-Consumer (MPSC) problem.

Where we have multiple producers in the form of the temperature sensors on the rover and a single-consumer represented by the shared-resource that we can consider the CPU on the rover itself.

Rust makes it extremely easy to create these kinds of objects since it has a type called `mpsc` which is used to create a "channel" at the start of the main function. 
This "channel" comes in the form of two main parts: a transmitter and a receiver. 

The idea is that we can represent each sensor as a thread and each thread is given a copy of the transmitter before being spawned in.
Each transmitter copy has a `send` function to send in the temperature readings to the queue that formed by the Rust MPSC type. 

Each reading is sent in at a certain interval about 60 times to represent the 60 minutes in an hour before a report is complete.

Note that the receiver part of the channel is being used only on the main thread and it represents the single-consumer.
As the data is sent with the transmitter, the receiver takes in the sent temperature readings and collect them all into an array.
The array is then sorted from smallest to largest and the top 5 highest and lowest temperatures are printed to the console.

Later on, a function is used to iterate through all 10-minute intervals of the collected readings to calculate the largest temperature difference across those intervals.
These are also printed to the console.

Below is an example output:
```
-- Hourly Report --
Note: Temperature rankings are from lowest to highest temperature in degrees F.

Lowest 5 Temperatures: [-99.97339, -99.12756, -99.0766, -99.07054, -99.02156]
Highest 5 Temperatures: [68.74765, 69.111435, 69.27263, 69.80205, 69.950516]

10-Minute interval from 25-35 had the largest temperature difference of 40.20719 degrees F.
```
### Evaluation
In terms of evaluation of this code, since the concurrency primtiive is a simply queue in the form of a MPSC problem, the solution is lock-free and effectively wait-free. As the progress is made on each thread, the results are simply sent to a receiver and processed after all threads complete the "hour" of gathering temperature data. There is no way for any deadlocks or data races to occur, all threads are guaranteed to progress as no threads wait for any other thread. 

For runtime, this is not very relevant to the problem as it entirely depends on how long the threads are slept for between each reading. 
For my code I've set it to wait 1ms between readings to represent one minute.


### Running
Type in the following command to run the code for problem 2:
```
cargo run --bin temperature
```