use rand::Rng;
use std::{thread, time::Duration, collections::HashMap};
use core::hash::Hash;

#[derive(Debug)]
struct Cacher<T,U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    lookup: HashMap<U,U>,
}

impl<T,U> Cacher<T,U>
where
    T: Fn(U) -> U,
    U: Eq + PartialEq + Hash + Clone + Copy
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            lookup: HashMap::new(),
        }
    }

    fn value(&mut self, input: U) -> U {

        match self.lookup.get(&input) {
            Some(res) => {
                res.clone()
            },
            None => {
                let computed_result = (self.calculation)(input);
                self.lookup.insert(input, computed_result);
                computed_result
            }
        }

    }
}

fn generate_workout_plan(intensity: i32, random_number: i32) {
    let mut cached_result = Cacher::new(|num: i32| -> i32 {
        println!("Printing slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "You need to do {} pushups a day",
            cached_result.value(intensity)
        );
        println!(
            "You need to do {} situps a day",
            cached_result.value(intensity)
        )
    } else {
        if random_number == 3 {
            println!("Today is your day off... Enjoy your day");
        } else {
            println!(
                "Today Jog for {} minutes",
                cached_result.value(random_number)
            );
            println!(
                "Today run for {} minutes",
                cached_result.value(random_number)
            )
        }
    }
}
fn main() {
    let intensity = 30;
    let random_number = rand::thread_rng().gen_range(1..61);

    generate_workout_plan(intensity, random_number);
}
