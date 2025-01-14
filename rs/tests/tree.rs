enum BinaryTree {
    Leaf(i32),
    Node { left: Box<BinaryTree>, payload: i32, right: Box<BinaryTree> },
}

fn sample_tree() -> BinaryTree {
    let l1 = Box::new(BinaryTree::Leaf(1));
    let l3 = Box::new(BinaryTree::Leaf(3));
    let n2 = Box::new(BinaryTree::Node { left: l1, payload: 2, right: l3 });
    let l5 = Box::new(BinaryTree::Leaf(5));

    // Returns tree that Looks like:
    //
    //      +----(4)---+
    //      |          |
    //   +-(2)-+      [5]
    //   |     |
    //  [1]   [3]
    //
    BinaryTree::Node { left: n2, payload: 4, right: l5 }
}

#[test]
fn test() {
    fn tree_weight(t: &BinaryTree) -> i32 {
        match *t {
            BinaryTree::Leaf(payload) => payload,
            BinaryTree::Node { ref left, payload, ref right } => {
                tree_weight(left) + payload + tree_weight(&right)
                // or without 'ref' on declaration and no *t dereference and *payload
            }
        }
    }

    let tree = sample_tree();

    assert_eq!(tree_weight(&tree), (1 + 2 + 3) + 4 + 5);
    println!("weight = {}", tree_weight(&tree));
}
