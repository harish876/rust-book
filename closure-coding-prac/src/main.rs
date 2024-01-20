use std::{time::Duration, thread, collections::HashMap};

struct Cacher<T,E> 
where 
    T: Fn(E) -> E,
    E: Eq + PartialEq + std::hash::Hash + Copy
{
    calculation: T,
    values: HashMap<E,E>
}

impl <T,E> Cacher<T,E>
where
    T: Fn(E) -> E,
    E: Eq + PartialEq + std::hash::Hash + Copy
    {
        fn new(calculation:T) -> Self {
            Self {
                calculation,
                values: HashMap::new()
            }
        }

        fn get(&mut self,val:E) -> E {

            let cached_value = self.values.get(&val);
            
            match cached_value {
                Some(v) => *v,
                None => {
                    let computed_value = (self.calculation)(val);
                    self.values.insert(val,computed_value);
                    computed_value
                }
            }

        }
    }

fn simulate_workout(intensity: u32,random_number: u32){

    let expensive_calculation = |num:u32|{
        println!("Executing expensive calculation");
        thread::sleep(Duration::from_secs(5));
        num
    };

    let mut cached_value = Cacher::new(expensive_calculation);


    if intensity < 25 {
        let expensive_result = cached_value.get(intensity);
        println!("Today is a good day to do {} pushups",expensive_result);

        println!("Also do {} situps",expensive_result);
    }

    else {
        let expensive_result = cached_value.get(intensity);
        if random_number == 3 {
            println!("You deserve a day off , enjoy");
        }
        else {
            println!("You need to run {} km" , expensive_result);
        }
    }
}

fn main() {
    let intensity = 10;
    let random_number = 7;
    simulate_workout(intensity, random_number);
}
