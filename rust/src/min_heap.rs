
pub struct Heap<T> where T: Ord {
    tree_elements: Vec<Option<T>>,
    next_to_fill: usize,
}

impl<T> Heap<T> where T: Ord + Clone {
   pub fn new() -> Self {
       let mut vector = Vec::<Option<T>>::new();
       vector.resize_with(2, || None);
       Heap { 
           tree_elements: vector,
           next_to_fill: 1
       }
   }

   //TODO: Think about cloning versus moving, maybe borrowing?
   pub fn push(&mut self, new_element: T) -> () {
       if self.next_to_fill >= self.tree_elements.len() {
           self.tree_elements
                .resize_with(self.tree_elements.len() * 2, || None); 
       }
       self.tree_elements[self.next_to_fill] = Some(new_element); 
       self.bubble_up(self.next_to_fill);
       self.next_to_fill = self.get_next_to_fill();
   }

   pub fn pop(&mut self) -> Option<T> {
       if self.tree_elements.len() <= 1 { return None; }
       let return_value = self.tree_elements[1].clone();
       let last_filled_index = self.get_last_filled();
       self.tree_elements[1] = self.tree_elements[last_filled_index].clone();
       self.tree_elements[last_filled_index] = None;
       self.bubble_down(1);
       return_value
   }

   pub fn peek(&self) -> Option<&T> {
      if self.tree_elements.len() <= 1 { return None; }
      
      match &self.tree_elements[1] {
          Some(ref element) => Some(element),
          None => None
      }
   }

   fn get_next_to_fill(&self) -> usize  {
       for next in self.next_to_fill..self.tree_elements.len() {
           if let None = self.tree_elements[next] {
                return next; 
           }
       }
       self.tree_elements.len()
   }

   fn get_last_filled(&self) -> usize {
       for index in (1..self.tree_elements.len()).rev() {
           if let Some(_) = self.tree_elements[index] { return index; }
       }
       return 1;
   }

   fn bubble_up(&mut self, index: usize) -> () {
       let mut index = index;
       let current = if let Some(element) = self.tree_elements[index].clone() {
           element 
       } else { return; };

       loop {
           match &self.tree_elements[self.parent(index)] {
                Some(p) if current < *p =>  {
                   self.tree_elements[index] = Some(p.clone()); 
                   let parent_index = self.parent(index);
                   self.tree_elements[parent_index] = Some(current.clone());
                   index = self.parent(index);
                }
                _ => { break; }
           }
       }
   }

   fn bubble_down(&mut self, index: usize) -> () {
       let mut index = index;
       let current = if let Some(element) = self.tree_elements[index].clone() {
           element 
       } else { return; };

       loop {
           //TODO: check for None, Some, since this would be a bug
           match (&self.tree_elements[self.left_child(index)], 
                  &self.tree_elements[self.right_child(index)]) {
                (Some(left), Some(right)) if left < &current || right < &current => {
                    let (min_child, min_child_index) = if left < right {
                        (left, self.left_child(index)) 
                    } else { (right, self.right_child(index)) };

                    self.tree_elements[index] = Some(min_child.clone());
                    self.tree_elements[min_child_index] = Some(current.clone());
                    index = min_child_index;
                },
                (Some(left), _) if left < &current => {
                    self.tree_elements[index] = Some(left.clone());
                    let left_child_index = self.left_child(index);
                    self.tree_elements[left_child_index] = Some(current.clone());
                    index = self.left_child(index);
                }
                (_, _) => { break; } 
           }
       }
   }

   //These functions return 0 upon out of bounds accesses, since 0
   //will always contain None and so will signal to the calling code
   //that nothing is there.
   fn parent(&self, index: usize) -> usize {
       if self.in_bounds(index / 2) {
           index / 2 
       } else { 0 }
   }

   fn left_child(&self, index: usize) -> usize {
       if self.in_bounds(index * 2) {
           index * 2 
       } else { 0 }
   }

   fn right_child(&self, index: usize) -> usize {
       if self.in_bounds(index * 2 + 1) {
           index * 2 + 1 
       } else { 0 }
   }

   fn in_bounds(&self, index: usize) -> bool {
       index >= 1 && index < self.tree_elements.len()
   }
}

