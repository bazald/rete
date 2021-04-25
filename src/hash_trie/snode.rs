use alloc::sync::Arc;

#[derive(Debug, PartialEq)]
pub(super) struct SNode<T: Clone + PartialEq> {
    pub value: T
}

#[allow(dead_code)]
impl<T: Clone + PartialEq> SNode<T> {
    fn new(value: T) -> Self {
        Self {
            value
        }
    }
}

#[allow(dead_code)]
fn new<T: Clone + PartialEq>(value: T) -> Arc<SNode<T>> {
    Arc::new(SNode::new(value))
}
