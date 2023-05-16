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

pub fn swap_nodes(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
    let (mut temp, mut cnt) = (&head, 0);
    while let Some(n) = temp {
        temp = &n.next;
        cnt += 1;
    }

    let (mut tmp, mut v1, mut v2) = (&head, 0, 0);
    let (mut i, p1, p2) = (0, k - 1, cnt - k);
    if p1 == p2 {  // shortcut: positions are the same (as nothing to change)
        return head;
    }
    while let Some(n) = tmp {
        if i == p1 {
            v1 = n.val;
        } else if i == p2 {
            v2 = n.val;
        }
        tmp = &n.next;
        i += 1;
    }
    if v1 == v2 {
        return head;
    }
    let (mut tmp, mut i) = (&mut head, 0);
    while let Some(n) = tmp {
        if i == p1 {
            n.val = v2;
        } else if i == p2 {
            n.val = v1;
        }
        tmp = &mut n.next;
        i += 1;
    }

    return head;
}