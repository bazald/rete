use super::{cnode::*, lnode::LNode, snode::SNode};

enum MNode<T: Clone + Eq> {
  C(LNode<T>),
  L(LNode<T>),
  S(SNode<T>),
}
