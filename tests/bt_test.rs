use binary_tree::Tree;

#[test]
fn test_new() {
    let t: Tree<i32> = Tree::new();
    assert_eq!(t.size(), 0);
    assert!(t.root().is_none());
}

#[test]
fn test_add_once() {
    let mut t: Tree<i32> = Tree::new();
    let item = 0;
    t.add(item);
    assert_eq!(t.size(), 1);
    assert_eq!(*t.root().unwrap().item(), item);
}

#[test]
fn test_add() {
    let mut t: Tree<i32> = Tree::new();
    let items = [1, 2, 5, 3, 6, 4, 9];
    for i in items.iter() {
        t.add(*i);
    }
    for i in items.iter() {
        println!("{:?}", i);
        assert!(t.contains(i));
    }
    assert_eq!(t.size(), items.len());
}


#[test]
fn test_in_order_iter() {
    let mut t: Tree<i32> = Tree::new();
    let items = vec![5, 1, 3, 4, 6, 2, 9];
    for i in items.iter() {
        t.add(*i);
    }

    let iter_result: Vec<i32> = t.in_order_iter().collect();

    assert_eq!(iter_result, vec![5, 1, 3, 2, 4, 6, 9]);
}

#[test]
fn test_left_iter() {
    let mut t: Tree<i32> = Tree::new();
    let items = vec![5, 1, 3, 4, 6, 2, 9];
    for i in items.iter() {
        t.add(*i);
    }

    let iter_result: Vec<i32> = t.left_iter().collect();

    assert_eq!(iter_result, vec![1, 2, 3, 4, 5, 6, 9]);
}