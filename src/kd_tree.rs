use super::types::{ LngLat, Point };
use super::priority_queue::MinPriorityQueue;

const K: i32 = 2;
const K_USZ: usize = 2;

pub struct KdTree<'a, T> {
    locations: &'a [T],
    indexes: Vec<usize>,
}

impl<'a, T> KdTree<'a, T> where T: LngLat {
    pub fn new(locations: &'a [T]) -> Self {
        let mut indexes = Vec::new();
        locations.iter()
            .enumerate()
            .for_each(|(i, _p)| indexes.push(i));

        KdTree {
            locations,
            indexes,
        }
    }

    pub fn sort(&mut self, from: usize, count: usize, mut axis: i32) {
        axis %= K;

        if count > 1 {
            let locations = self.locations;
            if axis == 0 {
                self.indexes[from..from + count].sort_by(|a, b|
                    locations.get(*a).unwrap().get_lng().partial_cmp(&locations.get(*b).unwrap().get_lng()).unwrap());
            } else {
                self.indexes[from..from + count].sort_by(|a, b|
                    locations.get(*a).unwrap().get_lat().partial_cmp(&locations.get(*b).unwrap().get_lat()).unwrap());
            }

            let count1 = count / 2;
            let count2 = count - count / 2 - 1;
            self.sort(from, count1, axis + 1);
            self.sort(from + count1 + 1, count2, axis + 1);
        }
    }

    pub fn get_location(&self, index: usize) -> &T {
        &self.locations[index]
    }

    pub fn search_nn(&self, lnglat: &impl LngLat) -> (usize, f64) {
        let v = self.search_top_nn(lnglat, 1);
        v[0]
    }

    pub fn search_top_nn(&self, lnglat: &impl LngLat, top: usize) -> Vec<(usize, f64)> {
        let mut queue = MinPriorityQueue::new(top + 1, std::f64::MAX);
        queue.append(std::usize::MAX, std::f64::MAX);
        self.search(lnglat, 0, self.indexes.len(), 0, &mut queue);

        queue.get_min_value()
            .into_iter()
            .take(top)
            .map(|elm| (self.indexes[elm.element], elm.priority))
            .collect::<Vec<(usize, f64)>>()
    }

    fn search(&self, lnglat: &impl LngLat, range_from: usize, range_count: usize, axis: i32, queue: &mut MinPriorityQueue<f64>) {
        if range_count > 0 {
            let index = range_from + range_count / 2;
            let lnglat2 = &self.locations[self.indexes[index]];
            let distance = lnglat.distance_to(lnglat2);
            queue.append(index, distance);

            let ranges = {
                [(range_from, range_count / 2 ), (range_from + range_count / 2 + 1, range_count - range_count / 2 - 1)]
            };

            let mut search_direction = 1;
            match axis {
                0 => {
                    if lnglat.get_lng() < lnglat2.get_lng() {
                        search_direction = 0;
                    }
                },
                1 => {
                    if lnglat.get_lat() < lnglat2.get_lat() {
                        search_direction = 0;
                    }
                },
                _ => (),
            }

            self.search(lnglat, ranges[search_direction].0, ranges[search_direction].1, (axis + 1) % K, queue);

            //軸までの距離が近傍点より近ければ反対側を見る必要がある。
            let test_lng_lat = if axis == 0 {
                Point::new(lnglat2.get_lng(), lnglat.get_lat())
            } else {
                Point::new(lnglat.get_lng(), lnglat2.get_lat())
            };
            let distance_from_plane =  lnglat.distance_to(&test_lng_lat);


            //反対側を見る。
            if distance_from_plane < *queue.get_max_priority() {
                // search for the opposite plane
                self.search(lnglat, ranges[(search_direction + 1) % K_USZ].0, ranges[(search_direction + 1) % K_USZ].1, (axis + 1) % K, queue);
            }
        }
    }

}