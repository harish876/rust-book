use std::collections::HashMap;
fn main() {
    let sentence = String::from("Hello World This is my World");
    let mut hmap:HashMap<String,i32> = HashMap::new();

    for word in sentence.split_whitespace() {
        hmap
        .entry(String::from(word))
        .and_modify(|counter| *counter+=1)
        .or_insert(1);
    }

    println!("{:#?}",hmap)

}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hmap: HashMap<i32,i32> = HashMap::new();
    for (idx,num) in nums.into_iter().enumerate(){
        match hmap.contains_key(&(target-num)){
            true => {
                return vec![idx as i32, hmap.get(&(target-num)).unwrap().clone() as i32]
            },
            false => {
                hmap.insert(num,idx as i32)
            },
        };
    }
    return vec![];
}