use std::cell::RefCell;
use std::rc::{Rc, Weak};

// 双方向リストの要素の1つ。
pub struct DNode {
    data: i32,
    next: Option<Rc<RefCell<DNode>>>,
    prev: Option<Weak<RefCell<DNode>>>,
}

// 双方向リストをまとめる構造体。
pub struct DList {
    head: Option<Rc<RefCell<DNode>>>,
    // ここは Weak ではなく Option<Weak> になっている。
    tail: Option<Rc<RefCell<DNode>>>,
}

impl DList {
    pub fn new() -> Self {
        DList {
            head: None,
            tail: None,
        }
    }

    fn new_node(v: i32) -> Rc<RefCell<DNode>> {
        Rc::new(RefCell::new(DNode {
            data: v,
            next: None,
            prev: None,
        }))
    }

    // 末尾にデータを追加する。
    pub fn push(&mut self, data: i32) {
        let n = DList::new_node(data);
        match self.tail.take() {
            None => {
                self.tail = Some(Rc::clone(&n));
                self.head = Some(n);
            }
            Some(old_tail) => {
                self.tail = Some(Rc::clone(&n));
                n.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(n);
            }
        }
    }

    // 先頭にデータを追加する。
    pub fn unshift(&mut self, data: i32) {
        let n = DList::new_node(data);
        match self.head.take() {
            None => {
                self.tail = Some(Rc::clone(&n));
                self.head = Some(n);
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&old_head));
                n.borrow_mut().next = Some(old_head);
                self.head = Some(n);
            }
        }
    }

    pub fn iter(&mut self) -> DListIter {
        match &self.head {
            None => DListIter { cur: None },
            Some(head) => DListIter {
                cur: Some(Rc::clone(head)),
            },
        }
    }
}

pub struct DListIter {
    pub cur: Option<Rc<RefCell<DNode>>>,
}

impl Iterator for DListIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur.take() {
            None => None,
            Some(cur) => {
                let cb = cur.borrow();
                let data = cb.data;
                match &cb.next {
                    None => self.cur = None,
                    Some(next) => self.cur = Some(Rc::clone(next)),
                }
                Some(data)
            }
        }
    }
}
