/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord + Copy,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord + Copy,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord + Copy,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Copy,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if self.search(value) {
            return;
        }
        if let Some(b_node) = &mut self.root {
            b_node.insert(value)
        } else {
            self.root = Some(Box::new(TreeNode::<T>::new(value)));
        }
    }
    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        if let Some(node) = &self.root {
            if node.value == value {
                return true;
            } else if node.value > value {
                // 左子树找
                return node.search(value);
            } else {
                // 右子树找
                return node.search(value);
            }
        }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord + Copy,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if self.value > value {
            // 往左插
            if let Some(a) = &mut self.left {
                a.insert(value)
            } else {
                self.left = Some(Box::new(TreeNode::<T>::new(value)));
            }
        } else {
            // 往右插
            if let Some(a) = &mut self.right {
                a.insert(value)
            } else {
                self.right = Some(Box::new(TreeNode::<T>::new(value)));
            }
        }
    }
    fn search(&self, value: T) -> bool {
        if self.value == value {
            return true;
        } else if self.value > value {
            // 左子树找
            if let Some(node) = &self.left {
                return node.search(value);
            }
            false
        } else {
            // 右子树找
            if let Some(node) = &self.right {
                return node.search(value);
            }
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


