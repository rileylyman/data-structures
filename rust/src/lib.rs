
//Ignore unused code warnings, as this is a library module!
#[allow(dead_code)]
mod min_heap;

#[allow(dead_code)]
mod red_black_tree;

#[cfg(test)]
mod tests {
    use super::min_heap::Heap;
    #[test]
    fn test_min_heap() {
        let mut heap = Heap::<usize>::new();
        for i in (0..100).rev() {
            heap.push(i);
        }
        for i in 0..100 {
            for _ in 0..100 {
               assert_eq!(i, *heap.peek().unwrap_or(&8)); 
            }
            let val = heap.pop().unwrap_or(8);
            println!("{}:{}", i, val);
            assert_eq!(i, val);
        }
    }
}
