use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct QueueItem<T> {
    pub cell: T,
    pub cost: usize,
}

pub struct Dijkstra<'d, T> {
    neighbors: &'d dyn Fn(T) -> Vec<T>,
    cost: &'d dyn Fn(&'_ T) -> Option<usize>,
    end: &'d dyn Fn(&'_ T) -> bool,
}

impl<T> QueueItem<T> {
    pub fn new(cell: T, cost: usize) -> Self {
        Self { cell, cost }
    }
}

impl<T> PartialOrd for QueueItem<T>
where
    T: PartialEq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for QueueItem<T>
where
    T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.cell.cmp(&other.cell))
    }
}

impl<'d, T> Dijkstra<'d, T> {
    pub fn new(
        neighbors: &'d dyn Fn(T) -> Vec<T>,
        cost: &'d dyn Fn(&'_ T) -> Option<usize>,
        end: &'d dyn Fn(&'_ T) -> bool,
    ) -> Self {
        Self {
            neighbors,
            cost,
            end,
        }
    }

    pub fn cost(&self, start: Vec<T>) -> Option<usize>
    where
        T: Hash + Eq + PartialEq + Ord + Clone,
    {
        let mut heat_map: HashMap<T, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();

        for s in start {
            heat_map.insert(s.clone(), 0);
            heap.push(QueueItem::new(s, 0));
        }

        while let Some(QueueItem { cost, cell }) = heap.pop() {
            if (self.end)(&cell) {
                return Some(cost);
            }

            let dist = *heat_map.get(&cell).unwrap_or(&usize::MAX);

            if cost > dist {
                continue;
            }

            for neighbor in (self.neighbors)(cell.clone()) {
                // let n_cost = (self.cost)(&cell);
                let n_cost = match (self.cost)(&cell) {
                    Some(c) => cost + c,
                    _ => continue,
                };

                let dist_to_next = heat_map.get(&neighbor).unwrap_or(&usize::MAX);

                if n_cost < *dist_to_next {
                    heat_map
                        .entry(neighbor.clone())
                        .and_modify(|c| *c = n_cost)
                        .or_insert(n_cost);
                    let next = QueueItem::new(neighbor, n_cost);

                    heap.push(next);
                }
            }
        }

        None
    }
}
