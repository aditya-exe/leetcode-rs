#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let mut res = 0;
    let mut a = vec![];
    let mut cur = head;

    while let Some(node) = cur {
        a.push(node.val);
        cur = node.next;
    }

    let n = a.len();
    let (mut l, mut r) = (0, n - 1);

    while l<r{
        res = std::cmp::max(res, a[l]+a[r]);
        l+=1;
        r-=1;
    }

    return res;
}