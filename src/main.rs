extern crate rand;

pub mod types;
pub mod kd_tree;
pub mod priority_queue;

use std::time::Instant;
use crate::types::{ Location, Point };
use crate::kd_tree::KdTree;

fn main() {
    let start = Instant::now();
    let mut locactions = Vec::new();
    let address = std::fs::read_to_string("./address.csv").unwrap();
    for (i, line) in address.split("\r\n").enumerate() {
        let v = line.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
        if v.len() > 6 {
            let x = (v[5].parse::<f64>().unwrap() * 1000000.0).floor() as i32;
            let y = (v[6].parse::<f64>().unwrap() * 1000000.0).floor() as i32;
            locactions.push(Location::new(i as u32, v[1].to_string(), x, y, v[5].parse::<f64>().unwrap(), v[6].parse::<f64>().unwrap()));
        }
    }

    let mut tracking = Vec::new();
    let person = std::fs::read_to_string("./1126.csv").unwrap();
    for (_i, line) in person.split("\r\n").enumerate() {
        let v = line.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
        if v.len() > 5 {
            tracking.push(Point::new(v[3].parse::<f64>().unwrap(), v[2].parse::<f64>().unwrap()));
        }
    }

    let mut kd = KdTree::new(&locactions);
    kd.sort(0, locactions.len(), 0);
    for t in tracking.iter() {
        let ans = kd.search_nn(t);
        let loc = kd.get_location(ans.0);
        println!("location:{} {} {} tracking_lng:{} tracking_lat:{}  {:?}", loc.name, loc.lng, loc.lat, t.lng, t.lat, ans);
    }

    let elapsed = start.elapsed();
    println!("Elapsed: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    use std::time::Instant;
    use std::time::Duration;
    use super::types::PrioritySortableItem;
    use std::collections::BinaryHeap;
    use crate::types::{ Location, Point, LngLat };

    #[test]
    fn test1() {
        let mut locactions = Vec::new();
        let address = std::fs::read_to_string("./address.csv").unwrap();
        for (i, line) in address.split("\r\n").enumerate() {
            let v = line.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
            if v.len() > 6 {
                let x = (v[5].parse::<f64>().unwrap() * 1000000.0).floor() as i32;
                let y = (v[6].parse::<f64>().unwrap() * 1000000.0).floor() as i32;
                locactions.push(Location::new(i as u32, v[1].to_string(), x, y, v[5].parse::<f64>().unwrap(), v[6].parse::<f64>().unwrap()));
            }
        }

        let mut kd = KdTree::new(&locactions);
        kd.sort(0, locactions.len(), 0);

        let test = Location::new(1, "a".to_string(), 141021795, 38732815, 141.02179528, 38.7328159);
        locactions.iter().for_each(|p| println!("calc: {:?}  location:{:?}", p.distance_to(&test), p));
    }


    #[test]
    fn test2() {
        let test = Location::new(1, "a".to_string(), 141021795, 38732815, 130.0, 0.0);
        let test2 = Location::new(1, "a".to_string(), 141021795, 38732815, 130.0, 0.1);
        println!("calc: {:?}", test.distance_to(&test2));
    }

    #[test]
    fn test3() {
        let mut locactions = Vec::new();
        let address = std::fs::read_to_string("./address.csv").unwrap();
        for (i, line) in address.split("\r\n").enumerate() {
            let v = line.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
            if v.len() > 6 {
                let x = (v[5].parse::<f64>().unwrap() * 1000000.0).floor() as i32;
                let y = (v[6].parse::<f64>().unwrap() * 1000000.0).floor() as i32;
                locactions.push(Location::new(i as u32, v[1].to_string(), x, y, v[5].parse::<f64>().unwrap(), v[6].parse::<f64>().unwrap()));
            }
        }

        let mut tracking = Vec::new();
        let person = std::fs::read_to_string("./1126.csv").unwrap();
        for (_i, line) in person.split("\r\n").enumerate() {
            let v = line.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
            if v.len() > 5 {
                tracking.push(Point::new(v[3].parse::<f64>().unwrap(), v[2].parse::<f64>().unwrap()));
            }
        }

        let loop_cnt = 60;
        let mut rng = thread_rng();

        let start = Instant::now();
        let mut kd = KdTree::new(&locactions);
        kd.sort(0, locactions.len(), 0);


        let test_data = (0..loop_cnt)
            .map(|_| rng.choose(tracking.as_slice()).unwrap())
            .collect::<Vec<&Point>>();

        for t in test_data.iter() {
            let (index, distance) = kd.search_nn(*t);
            let loc = kd.get_location(index);
            println!("location:{} {} {} tracking_lng:{} tracking_lat:{}  {:?}", loc.name, loc.lng, loc.lat, t.lng, t.lat, distance);
        }

        let elapsed = start.elapsed();
        println!("kd tree {}times Elapsed: {} ms", loop_cnt, (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

        let start2 = Instant::now();

        for t in test_data.iter() {
            let mut min_index = std::usize::MAX;
            let mut min_distance = std::f64::MAX;
            //let t = rng.choose(tracking.as_slice()).unwrap();

            locactions.iter()
                .enumerate()
                .for_each(|(i, p)| {
                    let dist = t.distance_to(p);
                    if dist < min_distance {
                        min_distance = dist;
                        min_index = i;
                    }
                });

            let loc = locactions.get(min_index).unwrap();

            println!("location:{} {} {} tracking_lng:{} tracking_lat:{}  {:?}", loc.name, loc.lng, loc.lat, t.lng, t.lat, min_distance);
        }

        let elapsed2 = start2.elapsed();
        println!("brute force {}times Elapsed: {} ms", loop_cnt, (elapsed2.as_secs() * 1_000) + (elapsed2.subsec_nanos() / 1_000_000) as u64);
    }



    #[test]
    fn test4() {
        let mut heap = BinaryHeap::new();

        for i in 0..20 {
            heap.push(PrioritySortableItem::new(i as usize, 1.0 + (i as f64) / 10.0));
            heap.push(PrioritySortableItem::new(i as usize, 1.0 + (i as f64) / 10.0));
        }
//        heap.push(PriorityItem::new(0, 1.0));
//        heap.push(PriorityItem::new(0, 2.0));
//        heap.push(PriorityItem::new(0, 3.0));
//        heap.push(PriorityItem::new(0, 1.1));
//        heap.push(PriorityItem::new(0, 1.9));
//        heap.push(PriorityItem::new(0, 2.5));
//        heap.push(PriorityItem::new(0, 2.5));

        println!("{:?}", heap);
        println!("{:?}", heap.pop());
        println!("{:?}", heap.pop());
        println!("{:?}", heap.pop());
        println!("{:?}", heap.into_sorted_vec());
    }

    #[test]
    fn test5() {
        let locactions = vec![
            Location::new(0, "a".to_string(), 100, 100, 100.0, 100.0),
            Location::new(1, "b".to_string(), 1, 1, 1.0, 1.0),
            Location::new(2, "c".to_string(), 50, 50, 50.0, 40.0),
            Location::new(3, "c".to_string(), 51, 41, 51.0, 41.0),
            Location::new(4, "c".to_string(), 52, 43, 52.0, 43.0),
            Location::new(5, "d".to_string(), 60, 70,60.0, 70.0),
            Location::new(100, "target".to_string(), 53, 44, 53.0, 44.0)
        ];

        let target = Location::new(4, "c".to_string(), 52, 42, 52.0, 42.5);
        let mut kd = KdTree::new(&locactions);
        kd.sort(0, locactions.len(), 0);
        let (index, dist) = kd.search_nn(&target);
        println!("index:{:?}", index);
        println!("distance:{:?}", dist);
        println!("location:{:?}", kd.get_location(index));

        let v = kd.search_top_nn(&target, 2);
        println!("index:{:?}", v);

        v.iter().for_each(|(i, _d)| {
                println!("location:{:?}", kd.get_location(*i));
            });

    }
}