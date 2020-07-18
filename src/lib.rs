#[derive(Debug)]
pub struct Tree<T: PartialOrd> {
    root: Option<TreeNode<T>>,
    size: usize
}

impl <T: PartialOrd> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { root: None, size: 0 }
    }

    pub fn add(&mut self, item: T) {
        self.size += 1;
        match &mut self.root {
            None => self.root = Some(TreeNode::new(item)),
            Some(t) => t.add(item)
        }
    }

    pub fn contains(&self, item: &T) -> bool {
        match &self.root {
            None => false,
            Some(t) => t.contains(item)
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn root(&self) -> Option<&TreeNode<T>> {
        self.root.as_ref()
    }

    pub fn in_order_iter(&self) -> InOrderIterator<T> {
        InOrderIterator::new(&self)
    } 

    pub fn left_iter(&self) -> LeftIterator<T> {
        LeftIterator::new(&self)
    }

}

#[derive(Debug)]
pub struct TreeNode<T: PartialOrd> {
    item: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd> TreeNode<T> {
    pub fn new(item: T) -> TreeNode<T> {
        TreeNode{ item: item, left: None, right: None }
    }

    pub fn add(&mut self, item: T) {
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

    pub fn item(&self) -> &T {
        &self.item
    } 

    pub fn contains(&self, item: &T) -> bool {
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

pub struct InOrderIterator<'a, T: PartialOrd> {
    stack: Vec<&'a TreeNode<T>>
}

impl<'a, T: PartialOrd> InOrderIterator<'a, T> {
    fn new(t: &Tree<T>) -> InOrderIterator<T> {
        match &t.root {
            None => InOrderIterator { stack: vec![] },
            Some(r) => InOrderIterator { stack: vec![r] }
        }
    }
}

impl<'a, T: PartialOrd + Copy> Iterator for InOrderIterator<'a, T> {
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

pub struct LeftIterator<'a, T: PartialOrd> {
    stack: Vec<&'a TreeNode<T>>
}

impl<'a, T: PartialOrd> LeftIterator<'a, T> {
    fn new(t: &Tree<T>) -> LeftIterator<T> {
        let mut it = LeftIterator { stack: Vec::new() };
        match &t.root {
            None => {},
            Some(r) => it.add_all_left(r)
        }
        it
    }

    fn add_all_left(&mut self, t: &'a TreeNode<T>) {
        self.stack.push(t);
        match &t.left {
            None => {},
            Some(left) => self.add_all_left(&left)
        }
    }

}

impl<'a, T: PartialOrd + Copy + std::fmt::Debug> Iterator for LeftIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.stack.len() {
            0 => None,
            n => {
                let current = self.stack[n - 1];
                self.stack.pop();
                match &current.right {
                    None => {},
                    Some(right) => self.add_all_left(&right)
                }
                Some(current.item)
            }
        }
    }
}
