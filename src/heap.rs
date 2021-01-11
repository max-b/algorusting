#[derive(Debug, Clone)]
struct Heap<T: PartialOrd> {
    queue: Vec<T>,
}

impl<T: PartialOrd + Clone> Heap<T> {
    fn new(items: &[T]) -> Self {
        let mut new_heap = Heap { queue: vec![] };
        for i in items {
            new_heap.add(i.clone());
        }
        new_heap
    }

    fn add(&mut self, item: T) {
        self.queue.push(item);
        self.bubble_up(self.queue.len() - 1);
    }

    fn bubble_up(&mut self, index: usize) {
        if let Some(parent_index) = get_parent_index(index) {
            if self.queue[parent_index] > self.queue[index] {
                self.queue.swap(index, parent_index);
                self.bubble_up(parent_index);
            }
        }
    }

    fn bubble_down(&mut self, index: usize) {
        let first_child_index = get_first_child_index(index);
        if let Some(first_child) = self.queue.get(first_child_index) {
            if *first_child < self.queue[index] {
                self.queue.swap(index, first_child_index);
                self.bubble_down(first_child_index);
            }
        }
        let second_child_index = first_child_index + 1;
        if let Some(second_child) = self.queue.get(second_child_index) {
            if *second_child < self.queue[index] {
                self.queue.swap(index, second_child_index);
                self.bubble_down(second_child_index);
            }
        }
    }

    pub fn extract_min(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            return None;
        }

        // pop from front
        let last_index = self.queue.len() - 1;
        self.queue.swap(0, last_index);
        let min = self.queue.pop();

        self.bubble_down(0);

        min
    }
}

impl<T: PartialOrd + Clone> Iterator for Heap<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.extract_min()
    }
}

fn get_parent_index(n: usize) -> Option<usize> {
    if n == 0 {
        return None;
    }
    Some((n - 1) / 2)
}

fn get_first_child_index(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    2 * n + 1
}

pub fn heapsort<T: PartialOrd + Clone>(items: &[T]) -> Vec<T> {
    let new_heap = Heap::new(items);
    let mut new_vec = Vec::new();

    for i in new_heap {
        new_vec.push(i);
    }

    new_vec
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creating() {
        let new_heap = Heap::new(&[9]);
        assert_eq!(new_heap.queue.len(), 1);
        assert_eq!(new_heap.queue[0], 9);
        check_correctness(&new_heap);

        let new_heap = Heap::new(&[1, 9, 2, 8, 3, 7, 4, 6, 5, 0]);
        assert_eq!(new_heap.queue.len(), 10);
        assert_eq!(new_heap.queue[0], 0);
        check_correctness(&new_heap);
    }

    #[test]
    fn getting_parent() {
        assert_eq!(get_parent_index(0), None);
        assert_eq!(get_parent_index(1), Some(0));
        assert_eq!(get_parent_index(2), Some(0));
        assert_eq!(get_parent_index(3), Some(1));
        assert_eq!(get_parent_index(4), Some(1));
        assert_eq!(get_parent_index(5), Some(2));
    }

    #[test]
    fn getting_first_child() {
        assert_eq!(get_first_child_index(0), 1);
        assert_eq!(get_first_child_index(1), 3);
        assert_eq!(get_first_child_index(2), 5);
        assert_eq!(get_first_child_index(3), 7);
    }

    fn check_correctness(heap: &Heap<usize>) {
        for i in 0..heap.queue.len() {
            if let Some(parent_index) = get_parent_index(i) {
                assert!(heap.queue[i] > heap.queue[parent_index]);
            }
        }
    }

    #[test]
    fn adding() {
        let mut new_heap = Heap::new(&[9]);
        new_heap.add(1);
        assert_eq!(new_heap.queue.len(), 2);
        assert_eq!(new_heap.queue[0], 1);
        assert_eq!(new_heap.queue[1], 9);

        for i in 2..8 {
            new_heap.add(i);
        }

        check_correctness(&new_heap);
    }

    #[test]
    fn extracting_min() {
        let mut new_heap = Heap::new(&[1, 9, 2, 8, 3, 7, 4, 6, 5, 0]);
        println!("new heap: {:?}", &new_heap);
        for i in 0..10 {
            let min = new_heap.extract_min();
            assert_eq!(min, Some(i));
            check_correctness(&new_heap);
        }
    }

    #[test]
    fn iterating() {
        let new_heap = Heap::new(&[1, 9, 2, 8, 3, 7, 4, 6, 5, 0]);
        for (i, j) in new_heap.enumerate() {
            assert_eq!(i, j);
        }
    }

    #[test]
    fn sorting() {
        let test1 = &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let sorted = heapsort(test1);
        assert_eq!(sorted, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
