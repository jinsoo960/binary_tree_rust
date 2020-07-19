/// Generic binary search tree
/// Items are owned by the tree
#[derive(Debug)]
pub struct Tree<T: PartialOrd> {
    root: Option<TreeNode<T>>,
    size: usize
}

impl <T: PartialOrd> Tree<T> {
    /// Create a empty tree.
    pub fn new() -> Tree<T> {
        Tree { root: None, size: 0 }
    }

    /// Move the given `item` to the tree.
    pub fn add(&mut self, item: T) {
        self.size += 1;
        match &mut self.root {
            None => self.root = Some(TreeNode::new(item)),
            Some(t) => t.add(item)
        }
    }

    /// Return `true` if the tree contains `item`.
    pub fn contains(&self, item: &T) -> bool {
        match &self.root {
            None => false,
            Some(t) => t.contains(item)
        }
    }

    /// Return the number of items in the tree.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Return an iterator for traversing the tree in preorder.
    pub fn pre_order_iter(&self) -> PreOrderIterator<T> {
        PreOrderIterator::new(&self)
    } 

    /// Return an iterator for traversing the tree in inorder.
    pub fn in_order_iter(&self) -> InOrderIterator<T> {
        InOrderIterator::new(&self)
    }

}

/// Struct representing each node in the tree
#[derive(Debug)]
struct TreeNode<T: PartialOrd> {
    item: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd> TreeNode<T> {
    /// Constructor
    fn new(item: T) -> TreeNode<T> {
        TreeNode{ item: item, left: None, right: None }
    }

    /// Add `item` with a new node in the subtree with`self`
    /// as the root.
    fn add(&mut self, item: T) {
        if self.item > item {
            match &mut self.left {
                None => self.left = Some(Box::new(TreeNode::new(item))),
                Some(t) => t.add(item)
            }
        } else {
            match &mut self.right {
                None => self.right = Some(Box::new(TreeNode::new(item))),
                Some(t) => t.add(item)
            }
        }
    }

    /// Return `true` if `item` is contained in the subtree with `self`
    /// as the root.
    fn contains(&self, item: &T) -> bool {
        if self.item == *item {
            true
        } else if self.item > *item {
            match &self.left {
                None => false,
                Some(t) => t.contains(item)
            }
        } else {
            match &self.right {
                None => false,
                Some(t) => t.contains(item)
            }
        }
    }
}

/// Iterator for traversing preorder
pub struct PreOrderIterator<'a, T: PartialOrd> {
    /// Stack for nodes to travel next
    stack: Vec<&'a TreeNode<T>>
}

impl<'a, T: PartialOrd> PreOrderIterator<'a, T> {
    fn new(t: &Tree<T>) -> PreOrderIterator<T> {
        match &t.root {
            None => PreOrderIterator { stack: vec![] },
            Some(r) => PreOrderIterator { stack: vec![r] }
        }
    }
}

impl<'a, T: PartialOrd + Copy> Iterator for PreOrderIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.stack.len() {
            0 => None,
            n => {
                let current = self.stack[n - 1];
                self.stack.pop();
                match &current.right {
                    Some(t) => self.stack.push(&t),
                    None => {}
                }
                match &current.left {
                    Some(t) => self.stack.push(&t),
                    None => {}
                }
                Some(current.item)
            }
        }
    }
}

/// Iterator for traversing inorder
pub struct InOrderIterator<'a, T: PartialOrd> {
    /// Stack to contain tree nodes to travel next.
    /// The left children of the nodes in the stack
    /// are already traveled.
    stack: Vec<&'a TreeNode<T>>
}

impl<'a, T: PartialOrd> InOrderIterator<'a, T> {
    fn new(t: &Tree<T>) -> InOrderIterator<T> {
        let mut it = InOrderIterator { stack: Vec::new() };
        match &t.root {
            None => {},
            Some(r) => it.add_all_left(r)
        }
        it
    }

    /// Repeatedly add left decendants of `t`.
    fn add_all_left(&mut self, t: &'a TreeNode<T>) {
        self.stack.push(t);
        match &t.left {
            None => {},
            Some(left) => self.add_all_left(&left)
        }
    }

}

impl<'a, T: PartialOrd + Copy + std::fmt::Debug> Iterator for InOrderIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.stack.len() {
            0 => None,
            n => {
                let current = self.stack[n - 1];
                self.stack.pop();
                // No need to check for left children since
                // they are already added to the stack.
                match &current.right {
                    None => {},
                    // The left decendatns of `current.right`
                    // get precedant over `current.right`
                    // since they are more to the left.
                    Some(right) => self.add_all_left(&right)
                }
                Some(current.item)
            }
        }
    }
}
