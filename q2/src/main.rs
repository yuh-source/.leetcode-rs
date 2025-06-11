// 2. Add Two Numbers

// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, 
// and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.

// Example 2:

// Input: l1 = [0], l2 = [0]
// Output: [0]

// Example 3:

// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// Output: [8,9,9,9,0,0,0,1]

// Constraints:

//     The number of nodes in each linked list is in the range [1, 100].
//     0 <= Node.val <= 9
//     It is guaranteed that the list represents a number that does not have leading zeros.


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

fn gen_ll(nodes: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut current = &mut dummy_head;

    for val in nodes {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }

    dummy_head.next
}

fn print_ll(head: Option<Box<ListNode>>) {
    let mut vals = vec![];
    let mut current = head.as_ref();

    while let Some(node) = current {
        vals.push(node.val);
        current = node.next.as_ref();
    }

    println!("{:?}", vals);
}

fn main() {
    let l1 = gen_ll(vec![9,9,9,9,9,9,9]);
    let l2 = gen_ll(vec![9,9,9,9]);
    let answer = Solution::add_two_numbers(l1, l2);
    print_ll(answer)
}