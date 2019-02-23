use super::types::Location;
use super::priority_queue::MinPriorityQueue;

const K: i32 = 2;

pub struct KdTree {
    points: Vec<Location>,
    indexes: Vec<usize>,
}

impl KdTree {
    pub fn new(points: Vec<Location>) -> Self {
        let mut indexes = Vec::new();
        points.iter()
            .enumerate()
            .for_each(|(i, _p)| indexes.push(i));

        KdTree {
            points,
            indexes,
        }
    }

    pub fn sort(&mut self, from: usize, count: usize, mut axis: i32) {
        axis %= K;

        if count > 1 {
            let points = &self.points;
            if axis == 0 {
                self.indexes[from..from + count].sort_by(|a, b|
                    points.get(*a).unwrap().lng.partial_cmp(&points.get(*b).unwrap().lng).unwrap());
            } else {
                self.indexes[from..from + count].sort_by(|a, b|
                    points.get(*a).unwrap().lat.partial_cmp(&points.get(*b).unwrap().lat).unwrap());
            }

            let count1 = count / 2;
            let count2 = count - count / 2 - 1;
            self.sort(from, count1, axis + 1);
            self.sort(from + count1 + 1, count2, axis + 1);
        }
    }

    pub fn get_location(&self, index: usize) -> &Location {
        &self.points[index]
    }

    pub fn search_nn(&self, location: &Location) -> (usize, f64) {
        let v = self.search_nn_range(location, 1);
        v[0]
    }

    pub fn search_nn_range(&self, location: &Location, range: usize) -> Vec<(usize, f64)> {
        let mut queue = MinPriorityQueue::new(range + 1, std::f64::MAX);
        queue.append(std::usize::MAX, std::f64::MAX);
        self.search(location, 0, self.indexes.len(), 0, &mut queue);

        queue.get_min_value()
            .into_iter()
            .take(range)
            .map(|elm| (self.indexes[elm.element], elm.priority))
            .collect::<Vec<(usize, f64)>>()
    }

    fn search(&self, location: &Location, range_from: usize, range_count: usize, axis: i32, queue: &mut MinPriorityQueue<f64>) {
        if range_count > 0 {
            let index = range_from + range_count / 2;
            let location2 = &self.points[self.indexes[index]];
            let distance = location.distance_to(location2) as f64;
            queue.append(index, distance);

            let ranges = {
                [(range_from, range_count / 2 ), (range_from + range_count / 2 + 1, range_count - range_count / 2 - 1)]
            };


            let mut range_index = 1;
            match axis {
                0 => {
                    if location.lng < location2.lng {
                        range_index = 0;
                    }
                },
                1 => {
                    if location.lat < location2.lat {
                        range_index = 0;
                    }
                },
                _ => (),
            }

            self.search(location, ranges[range_index].0, ranges[range_index].1, (axis + 1) % K, queue);

            //軸までの距離が近傍点より近ければ反対側を見る必要がある。
            let test_loc = if axis == 0 {
                Location::new(1, "".to_string(), location2.x, location.y, location2.lng, location.lat)
            } else {
                Location::new(1, "".to_string(), location.x, location2.y, location.lng, location2.lat)
            };
            let distance_from_plane =  location.distance_to(&test_loc) as f64;


            //反対側を見る。
            if distance_from_plane < *queue.get_min_priority() {
                // search for the opposite plane
                self.search(location, ranges[(range_index + 1) % (K as usize)].0, ranges[(range_index + 1) % (K as usize)].1, (axis + 1) % K, queue);
            }
        }
    }

}