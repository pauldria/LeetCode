use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
use std::collections::BinaryHeap;

fn main() {
    let v: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None
            }))
        }))
    }));

    let v: Option<Box<ListNode>> = None;

    println!("{:?}", v);
    let result2 = SolutionMiddleLinkedList::get_nth_node(v, 3);
    println!("  --> {:?}", result2);

    let a: String = String::from("foo#bar");
    let b: String = String::from("foob##bar");
    println!("{} {}", a, b);
    let m = SolutionBackspace::backspace_compare(a, b);
    println!("  --> {}", m);

    let mut min_stack = MinStack::new();
    println!("{:?}", min_stack);
    min_stack.push(2147483646);
    min_stack.push(2147483646);
    min_stack.push(2147483647);
    let top = min_stack.top();
    println!("Just got top = {}, stack is {:?}", top, min_stack);
    min_stack.pop();
    println!("Just popped, stack is {:?}", min_stack);
    let cur_min = min_stack.get_min();
    println!("Just got min = {}, stack is {:?}", cur_min, min_stack);
    min_stack.pop();
    println!("Just popped, stack is {:?}", min_stack);
    let cur_min = min_stack.get_min();
    println!("Just got min = {}, stack is {:?}", cur_min, min_stack);
    min_stack.pop();
    println!("Just popped, stack is {:?}", min_stack);
    min_stack.push(2147483647);
    let top = min_stack.top();
    println!("Just got top = {}, stack is {:?}", top, min_stack);
    let cur_min = min_stack.get_min();
    println!("Just got min = {}, stack is {:?}", cur_min, min_stack);
    min_stack.push(-2147483648);
    println!("Just pushed -2147483648, stack is {:?}", min_stack);
    let top = min_stack.top();
    println!("Just got top = {}, stack is {:?}", top, min_stack);
    let cur_min = min_stack.get_min();
    println!("Just got min = {}, stack is {:?}", cur_min, min_stack);
    min_stack.pop();
    println!("Just popped, stack is {:?}", min_stack);
    let cur_min = min_stack.get_min();
    println!("Just got min = {}, stack is {:?}", cur_min, min_stack);

    let v: i64 = 2147483647;
    println!("v:i64 = {}", v);
    println!("(v + v)/2 = {}", (v + v)/2);
    println!("(v + v)/2 as i32 = {}", ((v + v)/2) as i32);
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct SolutionMiddleLinkedList {}

impl SolutionMiddleLinkedList {
    pub fn get_nth_node(head: Option<Box<ListNode>>, n:usize) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut count = 1;
        let mut ptr: &Option<Box<ListNode>> = &head;
        while count < n {
            count += 1;
            match ptr {
                None                    => return None,
                Some(v) => ptr = &v.next
            }
        }
        return (*ptr).clone();
    }
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut count = 0;
        let mut ptr: &Option<Box<ListNode>> = &head;
        loop {
            count += 1;
            match ptr {
                None                    => break,
                Some(v) => ptr = &v.next
            }
        }

        // Reset
        let val : i32 = if count % 2 == 0 { count/2 } else { count/2 + 1 };
        ptr = &head;
        let mut i = 1;

        while i < val {
            if let Some(v) = &ptr {
                ptr = &v.next;
            }
            i += 1;
        }
        return (*ptr).clone();
    }
}

struct SolutionBackspace {}

impl SolutionBackspace {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_vec: Vec<char> = Vec::new();
        let mut t_vec: Vec<char> = Vec::new();

        for c in s.chars() {
            if c == '#' {
                s_vec.pop();
            }
            else {
                s_vec.push(c);
            }
        }

        for c in t.chars() {
            if c == '#' {
                t_vec.pop();
            }
            else {
                t_vec.push(c);
            }
        }

        let s_final: String = s_vec.into_iter().collect();
        let t_final: String = t_vec.into_iter().collect();

        return s_final == t_final;
    }
}

#[derive(Debug)]
struct MinStack {
    v: Vec<(i32, i32)>,
    m: Option<i32>
}

impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        return
            MinStack {
                v: Vec::new(),
                m: None
            }
    }

    fn push(&mut self, x: i32) {
        match self.m {
            None            => {
                self.m = Some(x);
                self.v.push((x, x));
            }
            Some(val) => {
                // If this is the new min, then we make it so.
                if x < val {
                    self.v.push((x, val));
                    self.m = Some(x);
                }
                // If this is not a new min, then we push 2*x - val, which
                // will always be bigger than val.
                else {
                    self.v.push((x, val));
                }
            }
        }
    }

    fn pop(&mut self) {
        match self.v.pop() {
            None            => (),
            Some((val, m)) => {
                match self.m {
                    None                => panic!("[pop] I'm popping a nonempty stack with no min element!"),
                    Some(min_val) => {
                        // In this case, we hit the min val and need to update
                        if m != min_val {
                            self.m = Some(m)
                        }
                        else { }
                    }
                }
            }
        }
        // Edge case - if v is now empty . . .
        if self.v.len() == 0 {
            self.m = None;
        }
    }

    fn top(&self) -> i32 {
        match self.v.last() {
            None => panic!("[top] There's nothing left in this stack!"),
            Some((val, m)) => {
                return *val
            }
        }
    }

    fn get_min(&self) -> i32 {
        match self.m {
            None            => panic!("[get_min] There's no minimum value yet!"),
            Some(val) => return val
        }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct SolutionLargestDiameter { }

impl SolutionLargestDiameter {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return SolutionLargestDiameter::diameter_of_binary_tree_ref(&root);
    }
    fn diameter_of_binary_tree_ref(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(val) => {
                let val_ptr = val.borrow();
                let max_diam_l = SolutionLargestDiameter::diameter_of_binary_tree_ref(&val_ptr.left);
                let max_diam_r = SolutionLargestDiameter::diameter_of_binary_tree_ref(&val_ptr.right);
                let max_diam_sub = max(max_diam_l, max_diam_r);
                let mut max_l_path_l = 0;
                let mut max_l_path_r = 0;
                let mut max_r_path_l = 0;
                let mut max_r_path_r = 0;
                match &val_ptr.left {
                    None => {
                        match &val_ptr.right {
                            None => 0,
                            Some(val_right) => {
                                max_l_path_r = SolutionLargestDiameter::longest_left_branch(Some(val_right));
                                max_r_path_r = SolutionLargestDiameter::longest_right_branch(Some(val_right));
                                return max(max_diam_sub, 1 + max(max_l_path_r, max_r_path_r));
                            }
                        }
                    },
                    Some(val_left) => {
                        match &val_ptr.right {
                            None => {
                                max_l_path_l = SolutionLargestDiameter::longest_left_branch(Some(val_left));
                                max_r_path_l = SolutionLargestDiameter::longest_right_branch(Some(val_left));
                                return max(max_diam_sub, 1 + max(max_l_path_l, max_r_path_l));
                            },
                            Some(val_right) => {
                                max_l_path_r = SolutionLargestDiameter::longest_left_branch(Some(val_right));
                                max_r_path_r = SolutionLargestDiameter::longest_right_branch(Some(val_right));
                                max_l_path_l = SolutionLargestDiameter::longest_left_branch(Some(val_left));
                                max_r_path_l = SolutionLargestDiameter::longest_right_branch(Some(val_left));
                                return max(max_diam_sub, 2 + max(max_l_path_l, max_r_path_l)
                                                                   + max(max_l_path_r, max_r_path_r));
                            }
                        }
                    }
                }
            }
        }
    }

    fn longest_left_branch(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(val) => {
                let val_ptr = val.borrow();
                match &val_ptr.left {
                    None => 0,
                    Some(val_left) => {
                        let max_l = SolutionLargestDiameter::longest_left_branch(Some(val_left));
                        let max_r = SolutionLargestDiameter::longest_right_branch(Some(val_left));
                        return 1 + max(max_l, max_r);
                    }
                }
            }
        }
    }

    fn longest_right_branch(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(val) => {
                let val_ptr = val.borrow();
                match &val_ptr.right {
                    None => 0,
                    Some(val_right) => {
                        let max_l = SolutionLargestDiameter::longest_left_branch(Some(val_right));
                        let max_r = SolutionLargestDiameter::longest_right_branch(Some(val_right));
                        return 1 + max(max_l, max_r);
                    }
                }
            }
        }
    }
}

struct SolutionLastStone { }

impl SolutionLastStone {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        for val in stones.into_iter() {
            heap.push(val);
        }
        while heap.len() > 1 {
            let first  = heap.pop().unwrap();
            let second = heap.pop().unwrap();
            if first != second {
                heap.push(first - second);
            }
        }
        if heap.len() == 0 {
            return 0
        }
        return heap.pop().unwrap();
    }
}