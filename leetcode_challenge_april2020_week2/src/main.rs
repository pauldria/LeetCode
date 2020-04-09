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