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

    println!("{:?}", v);
    let result = SolutionMiddleLinkedList::middle_node(v);
    println!("  --> {:?}", result);
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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut count = 0;                          // Already count ptr
        let mut ptr: &Option<Box<ListNode>> = &head;
        loop {
            count += 1;
            match ptr {
                None                  => break,
                Some(v) => ptr = &v.next
            }
        }

        let val : i32 = if count % 2 == 0 { count/2 } else { count/2 + 1 };
        let mut ptr: &Option<Box<ListNode>> = &head;
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