struct Solution {

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

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            //match all branches
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                // check if sum is greater so it can be carried
                if sum < 10 {
                    // return Option<Box<ListNote>>, next values generated recursively
                    Some(Box::new(ListNode {
                        next: Solution::add_two_numbers(n1.next, n2.next),
                        val: sum
                    }))
                } else {
                    // sum % 10 always = 1 (carry), add all three with 2 calls
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        next: Solution::add_two_numbers(Solution::add_two_numbers(carry, n1.next), n2.next),
                        val: sum - 10
                    }))
                }
            } 
        }
    }
}

fn main() {
    Solution {

    };
}