use alloc::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SNode<T: Clone> {
    pub value: T,
}

#[allow(dead_code)]
impl<T: Clone> SNode<T> {
    fn new(value: T) -> Self {
        Self {
            value
        }
    }
}

#[allow(dead_code)]
fn new<T: Clone>(value: T) -> Arc<SNode<T>> {
    Arc::new(SNode::new(value))
}
