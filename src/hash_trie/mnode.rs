#[allow(unused_imports)]
use super::{cnode::*, lnode::LNode, snode::SNode};

#[allow(dead_code)]
enum MNode<T: Clone + Eq> {
    C(LNode<T>),
    L(LNode<T>),
    S(SNode<T>),
}
