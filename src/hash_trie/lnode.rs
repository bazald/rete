use std::{borrow::Cow, sync::Arc};

#[derive(Debug)]
pub(super) struct LNode<T: Clone + PartialEq> {
  pub value: T,
  pub next: Option<Arc<LNode<T>>>,
  pub size: u64,
}

impl<T: Clone + PartialEq> LNode<T> {
  fn has_next(&self, next: &Option<Arc<LNode<T>>>) -> bool {
    match &self.next {
      Some(sn) => match next {
        Some(n) => Arc::ptr_eq(sn, n),
        None => false,
      },
      None => next.is_none(),
    }
  }
}

impl<T: Clone + PartialEq> LNode<T> {
  fn new(value: T, next: Option<Arc<Self>>) -> Self {
    let size = match &next {
      Some(ln) => 1 + ln.size,
      None => 1,
    };
    Self {
      value,
      next,
      size,
    }
  }
}

fn new<T: Clone + PartialEq>(value: T, next: Option<Arc<LNode<T>>>) -> Option<Arc<LNode<T>>> {
  Some(Arc::new(LNode::new(value, next)))
}

fn find_in_lnode<T: Clone + PartialEq>(ln: &Option<Arc<LNode<T>>>, value: &T) -> Option<Arc<LNode<T>>> {
  if ln.is_none() {
    return None;
  }

  let ln = ln.as_ref().unwrap();
  if ln.value == *value {
    Some(ln.clone())
  }
  else {
    find_in_lnode(&ln.next, value)
  }
}

fn insert_in_lnode<T: Clone + PartialEq>(ln: &Option<Arc<LNode<T>>>, value: Cow<T>) -> Arc<LNode<T>> {
  match find_in_lnode(ln, value.as_ref()) {
    Some(_) => ln.as_ref().unwrap().clone(),
    None => Arc::new(LNode::new(value.into_owned(), ln.clone())),
  }
}

fn remove_from_lnode<T: Clone + PartialEq>(ln: &Option<Arc<LNode<T>>>, value: &T) -> Option<Arc<LNode<T>>> {
  if ln.is_none() {
    return None;
  }

  let ln = ln.as_ref().unwrap();
  if ln.value == *value {
    ln.next.clone()
  }
  else {
    let found = remove_from_lnode(&ln.next, value);
    if ln.has_next(&found) {
      Some(ln.clone())
    }
    else {
      Some(Arc::new(LNode::new(ln.value.clone(), found)))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lnode_insert_3() {
    let mut ln = new(1, None);
    ln = new(2, ln);
    ln = new(3, ln);
    assert_eq!(ln.as_ref().unwrap().size, 3);
    assert_eq!(find_in_lnode(&ln, &1).unwrap().value, 1);
    assert_eq!(find_in_lnode(&ln, &2).unwrap().value, 2);
    assert_eq!(find_in_lnode(&ln, &3).unwrap().value, 3);
    assert_eq!(find_in_lnode(&ln, &4).is_none(), true);
  }

  #[test]
  fn lnode_insert_3_again() {
    let mut ln = Some(insert_in_lnode(&None, Cow::Owned(1)));
    ln = insert_in_lnode(&ln, Cow::Owned(2)).into();
    ln = insert_in_lnode(&ln, Cow::Owned(3)).into();
    ln = insert_in_lnode(&ln, Cow::Owned(1)).into();
    assert_eq!(ln.as_ref().unwrap().size, 3);
    assert_eq!(find_in_lnode(&ln, &1).unwrap().value, 1);
    assert_eq!(find_in_lnode(&ln, &2).unwrap().value, 2);
    assert_eq!(find_in_lnode(&ln, &3).unwrap().value, 3);
    assert_eq!(find_in_lnode(&ln, &4).is_none(), true);
  }

  #[test]
  fn lnode_remove_1() {
    let mut ln = new(1, None);
    ln = new(2, ln);
    ln = new(3, ln);
    ln = remove_from_lnode(&ln, &1);
    assert_eq!(ln.as_ref().unwrap().size, 2);
    assert_eq!(find_in_lnode(&ln, &1).is_none(), true);
    assert_eq!(find_in_lnode(&ln, &2).unwrap().value, 2);
    assert_eq!(find_in_lnode(&ln, &3).unwrap().value, 3);
    assert_eq!(find_in_lnode(&ln, &4).is_none(), true);
  }

  #[test]
  fn lnode_remove_2() {
    let mut ln = new(1, None);
    ln = new(2, ln);
    ln = new(3, ln);
    ln = remove_from_lnode(&ln, &2);
    assert_eq!(ln.as_ref().unwrap().size, 2);
    assert_eq!(find_in_lnode(&ln, &1).unwrap().value, 1);
    assert_eq!(find_in_lnode(&ln, &2).is_none(), true);
    assert_eq!(find_in_lnode(&ln, &3).unwrap().value, 3);
    assert_eq!(find_in_lnode(&ln, &4).is_none(), true);
  }

  #[test]
  fn lnode_remove_3() {
    let mut ln = new(1, None);
    ln = new(2, ln);
    ln = new(3, ln);
    ln = remove_from_lnode(&ln, &3);
    assert_eq!(ln.as_ref().unwrap().size, 2);
    assert_eq!(find_in_lnode(&ln, &1).unwrap().value, 1);
    assert_eq!(find_in_lnode(&ln, &2).unwrap().value, 2);
    assert_eq!(find_in_lnode(&ln, &3).is_none(), true);
    assert_eq!(find_in_lnode(&ln, &4).is_none(), true);
  }
}
