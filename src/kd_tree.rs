use super::types::Location;

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

//        println!("from {} count {} axis {}", from , count, axis);
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
        &self.points[self.indexes[index]]
    }

    pub fn search_nn(&self, location: &Location) -> (usize, f64) {
        self.search(location, 0, self.indexes.len(), std::usize::MAX, std::f64::MAX, 0)
    }

    fn search(&self, location: &Location, range_from: usize, range_count: usize, mut min_index: usize, mut min_distance: f64, axis: i32) -> (usize, f64) {
        if range_count > 0 {
            let index = range_from + range_count / 2;
            let location2 = &self.points[self.indexes[index]];
            let distance = location.distance_to(location2) as f64;
            if distance < min_distance {
                min_index = index;
                min_distance = distance;
            }

            let ranges = {
                [(range_from, range_count / 2 ),
                    (range_from + range_count / 2 + 1, range_count - range_count / 2 - 1)]
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

            let ret = self.search(location, ranges[range_index].0, ranges[range_index].1, min_index, min_distance, (axis + 1) % K);
            min_index = ret.0;
            min_distance = ret.1;

            //軸までの距離が近傍点より近ければ反対側を見る必要がある。
            let test_loc = if axis == 0 {
                Location::new(1, "".to_string(), location2.x, location.y, location2.lng, location.lat)
            } else {
                Location::new(1, "".to_string(), location.x, location2.y, location.lng, location2.lat)
            };
            let distance_from_plane =  location.distance_to(&test_loc) as f64;

            //反対側を見る。
            if distance_from_plane < min_distance {
                // search for the opposite plane
                let ret = self.search(location, ranges[(range_index + 1) % (K as usize)].0, ranges[(range_index + 1) % (K as usize)].1, min_index, min_distance, (axis + 1) % K);
                min_index = ret.0;
                min_distance = ret.1;
            }
        }

        (min_index, min_distance)
    }

}