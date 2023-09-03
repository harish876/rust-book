use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;

pub fn binary_search(
    mut start: usize,
    mut end: usize,
    target: i32,
    list: Vec<i32>,
) -> Option<usize> {
    while start <= end {
        let mid = start + (end - start) / 2;
        let value = list[mid];
        match target.cmp(&value) {
            Ordering::Equal => {
                return Some(mid);
            }
            Ordering::Greater => {
                end = mid;
            }
            Ordering::Less => {
                start = mid + 1;
            }
        }
    }
    None
}

struct Heap<T: Ord> {
    container: Vec<T>,
}

impl<T: Ord> Heap<T> {
    fn new() -> Self {
        Self {
            container: Vec::new(),
        }
    }
    fn push(&mut self, val: T) {
        self.container.push(val);
        self.bubble_up();
    }
    fn pop(&mut self) -> Option<T> {
        let len = self.container.len();
        if len == 1 {
            self.container.pop()
        } else if len > 1 {
            self.container.swap(0, len - 1);
            let val: Option<T> = self.container.pop();
            self.bubble_down(0);
            val
        } else {
            None
        }
    }
    fn bubble_up(&mut self) {
        let mut idx: usize = self.container.len() - 1;
        while idx > 0 {
            let parent_idx = (idx - 1) / 2;
            if let Some(parent_value) = self.container.get(parent_idx) {
                if parent_value > &self.container[idx] {
                    self.container.swap(idx, parent_idx);
                }
            }
            idx = parent_idx;
        }
        return;
    }
    fn bubble_down(&mut self, idx: usize) {
        let l_child = 2 * idx + 1;
        let r_child = 2 * idx + 2;
        let len = self.container.len();

        let mut largest = idx;

        if l_child < len && self.container[largest] < self.container[l_child] {
            largest = l_child;
        } else if r_child < len && self.container[largest] < self.container[r_child] {
            largest = r_child;
        }

        if largest != idx {
            self.container.swap(largest, idx);
            self.bubble_down(largest);
        }
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    distance: f32,
    point: Vec<i32>,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}

impl Eq for Point {}

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut pq: Heap<Reverse<Point>> = Heap::new();
    for point in points {
        let dist: i32 = point.iter().map(|x| x.pow(2)).sum();
        let dist: f32 = (dist as f32).sqrt();
        pq.push(Reverse(Point {
            distance: dist,
            point,
        }));
    }

    let mut result = Vec::new();
    for _ in 0..k {
        if let Some(Reverse(v)) = pq.pop() {
            result.push(v.point);
        } else {
            break;
        }
    }
    result
}

fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in nums.iter() {
        map.entry(*i).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut result: Vec<i32> = map
        .iter()
        .filter(|(_, val)| **val == 2)
        .map(|(key, _)| *key)
        .collect();

    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2,3]);
    }

    #[test]
    fn test1() {
        assert_eq!(find_duplicates(vec![]), vec![]);
    }
}
