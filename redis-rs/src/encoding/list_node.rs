use std::{cell::RefCell, rc::Rc};
/*
 * doubly linked list
 */
pub struct ListNode<T> {
    val: T,
    pre: Option<Rc<RefCell<ListNode<T>>>>,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    pub fn new(
        val: T,
        pre: Option<Rc<RefCell<ListNode<T>>>>,
        next: Option<Rc<RefCell<ListNode<T>>>>,
    ) -> Self {
        Self { val, pre, next }
    }
}
