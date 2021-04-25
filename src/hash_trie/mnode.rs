#[allow(unused_imports)]
use super::{cnode::*, lnode::LNode, snode::SNode};
use alloc::sync::Arc;

#[allow(dead_code)]
pub(super) enum MNode<T: Clone + PartialEq> {
    C(Arc<LNode<T>>),
    L(Arc<LNode<T>>),
    S(Arc<SNode<T>>),
}
