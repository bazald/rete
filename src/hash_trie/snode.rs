use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub(super) struct SNode<T: PartialEq> {
    pub value: T
}

impl<T: PartialEq> SNode<T> {
    fn new(value: T) -> Self {
        Self {
            value
        }
    }
}

fn new<T: PartialEq>(value: T) -> Rc<SNode<T>> {
    Rc::new(SNode::new(value))
}
