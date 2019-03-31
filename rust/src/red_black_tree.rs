
struct RedBlackNode<T> {
    val  :  T,
    red  :  bool,
}

struct RedBlackTree<T> where T: Ord + Clone {
    node  :  RedBlackNode<T>,
    left  :  Option<Box<RedBlackTree<T>>>,
    right :  Option<Box<RedBlackTree<T>>>,
}

impl<T> RedBlackTree<T> where T: Ord + Clone {
    pub fn new(initial_val: T) -> Self {
        RedBlackTree {
            node: RedBlackNode {
                val : initial_val,
                red : false,
            },
            left  : None,
            right : None,
        }
    }

    fn right_rotate(&mut self) -> Self {
        if let Some(left_child) = self.left {
            let left_right_child = self.left.right;
            self.left = left_right_child;
            left_child.right = Box::new(self);
            left_child
        } else { self }
    }

    fn left_rotate(&mut self) -> Self {
        if let Some(right_child) = self.right {
            let right_left_child = right_child.left;
            self.right = right_left_child;
            right_child.left = Box::new(self);
            right_child
        } else { self }
    }
}
