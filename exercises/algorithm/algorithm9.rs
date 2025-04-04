/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,       // 元素个数
    items: Vec<T>,      // 储存元素
    comparator: fn(&T, &T) -> bool, // 比较函数
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;
        self.items.push(value);
        self.heapify_up(self.count);
        
    }

    fn  heapify_up(&mut self, idx: usize) {
        let mut cur_idx = idx;
        while cur_idx > 1 {
            let parent_idx = self.parent_idx(cur_idx);
            if ((self.comparator)(&self.items[cur_idx], &self.items[parent_idx])) {
                self.items.swap(cur_idx, parent_idx);
                cur_idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right <= self.count
            && (self.comparator)(&self.items[right], &self.items[left])
        {
            right // 如果右子节点比左子节点小，返回右子节点的索引
        } else {
            left // 否则返回左子节点的索引
        }
    }

    fn heapify_down(&mut self, idx: usize) {
        let mut current_idx = idx;
        loop {
            // 如果当前节点没有子节点，则结束调整
            if !self.children_present(current_idx) {
                break;
            }

            // 获取最小的子节点
            let smallest_child = self.smallest_child_idx(current_idx);

            // 如果当前节点大于最小子节点，则交换它们的位置
            if (self.comparator)(&self.items[smallest_child], &self.items[current_idx]) {
                self.items.swap(current_idx, smallest_child);
                current_idx = smallest_child; // 更新当前元素的索引，继续向下调整
            } else {
                break; // 如果符合堆的性质，就结束调整
            }
        }
    }

    // 删除堆顶元素并返回，删除操作会破坏堆的结构，需要重新调整
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None; // 如果堆为空，返回 None
        }

        // 将堆顶元素与最后一个元素交换位置
        self.items.swap(1, self.count);
        let popped_value = self.items.pop(); // 弹出最后一个元素（即堆顶元素）
        self.count -= 1; // 更新元素个数

        if !self.is_empty() {
            self.heapify_down(1); // 调整堆结构，使其重新满足堆的性质
        }

        popped_value // 返回删除的堆顶元素
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}