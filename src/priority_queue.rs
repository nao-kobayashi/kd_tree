use crate::types::PrioritySortableItem;
use std::collections::BinaryHeap;

pub struct MinPriorityQueue<T> {
    size: usize,
    min: T,
    elements: BinaryHeap<PrioritySortableItem<T>>,
}

impl<T> MinPriorityQueue<T> where T: Clone + PartialOrd {
    pub fn new(size: usize, init: T) -> Self {
        MinPriorityQueue {
            size,
            min: init,
            elements: BinaryHeap::new(),
        }
    }

    pub fn append(&mut self, element: usize, priority: T) {
        if self.min > priority {
            self.min = priority.clone();
        }

        if self.elements.len() < self.size {
            self.elements.push(PrioritySortableItem::new(element, priority));
        } else {
            if self.elements.peek().unwrap().priority > priority {
                let _ = self.elements.pop();
                self.elements.push(PrioritySortableItem::new(element, priority));
            }
        }
    }

    pub fn get_min_priority(&self) -> &T {
        &self.min
    }

    pub fn get_min_value(self) -> Vec<PrioritySortableItem<T>> {
        println!("{:?}", self.elements.len());
        self.elements.into_sorted_vec()
    }
}
