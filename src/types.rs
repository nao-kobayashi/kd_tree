use std::cmp::{ Ordering, PartialEq };

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub lng: f64,
    pub lat: f64,
    e2: f64,
}

const A: f64 = 6378.137;          // 長半径
const F: f64 = 1.0 / 298.257222101; // 扁平率

impl Location {
    pub fn new(id: u32, name: String, x: i32, y: i32, lng: f64, lat: f64) -> Self {
        Location { id, name, x, y, lng, lat, e2: (2.0 - F) * F, }
    }

    pub fn distance_to(&self, location: &Location) -> f64 {
        let x1 = self.lng * std::f64::consts::PI / 180.0;
        let y1 = self.lat * std::f64::consts::PI / 180.0;
        let x2 = location.lng * std::f64::consts::PI / 180.0;
        let y2 = location.lat * std::f64::consts::PI / 180.0;

        let dx = x1 - x2;
        let dy = y1 - y2;
        let y0 = (y1 + y2) / 2.0;

        let w2 = 1.0 - self.e2 * y0.sin().powi(2);
        let w = w2.sqrt();
        let w3 = w * w2;

        let n = A / w;             // 卯酉線曲率半径
        let m = A * (1.0 - self.e2) / w3; // 子午線曲率半径
        let dist2 = (dy * m).powi(2) + (dx * n * y0.cos()).powi(2);

        dist2.sqrt()
    }

}

#[derive(Debug, Clone)]
pub struct PrioritySortableItem<T> {
    pub element: usize,
    pub priority: T
}

impl<T> PrioritySortableItem<T> {
    pub fn new(element: usize, priority: T) -> Self {
        PrioritySortableItem { element, priority }
    }
}

impl<T> PartialOrd<PrioritySortableItem<T>> for PrioritySortableItem<T> where T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq<PrioritySortableItem<T>> for PrioritySortableItem<T> where T: PartialOrd {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T> Ord for PrioritySortableItem<T> where T: PartialOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.partial_cmp(&self.priority).unwrap()
    }
}

impl<T> Eq for PrioritySortableItem<T> where T: PartialOrd {}
