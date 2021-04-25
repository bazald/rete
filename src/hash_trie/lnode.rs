use alloc::{borrow::Cow, sync::Arc};

#[derive(Debug)]
pub(super) struct LNode<T: Clone + PartialEq> {
    pub value: T,
    pub next: Option<Arc<LNode<T>>>,
    pub size: u64,
}

#[allow(dead_code)]
impl<T: Clone + PartialEq> LNode<T> {
    fn new_tail(value: T) -> Self {
        Self {
            value,
            next: None,
            size: 1,
        }
    }

    fn new(value: T, next: Arc<Self>) -> Self {
        let size = 1 + next.size;
        Self {
            value,
            next: Some(next),
            size,
        }
    }
}

#[allow(dead_code)]
fn new_tail<T: Clone + PartialEq>(value: T) -> Arc<LNode<T>> {
    Arc::new(LNode::new_tail(value))
}

#[allow(dead_code)]
fn new<T: Clone + PartialEq>(value: T, next: Arc<LNode<T>>) -> Arc<LNode<T>> {
    Arc::new(LNode::new(value, next))
}

#[allow(dead_code)]
fn find<T: Clone + PartialEq>(mut ln: &Arc<LNode<T>>, value: &T) -> Option<Arc<LNode<T>>> {
    loop {
        if ln.value == *value {
            return Some(ln.clone());
        }
        else if let Some(next) = &ln.next {
            ln = next;
        }
        else {
            return None;
        }
    }
}

#[allow(dead_code)]
fn insert<T: Clone + PartialEq>(ln: &Arc<LNode<T>>, value: Cow<T>) -> Arc<LNode<T>> {
    match find(ln, value.as_ref()) {
        Some(_) => ln.clone(),
        None => new(value.into_owned(), ln.clone()),
    }
}

#[allow(dead_code)]
fn remove<T: Clone + PartialEq>(ln: &Arc<LNode<T>>, value: &T) -> Option<Arc<LNode<T>>> {
    if ln.value == *value {
        ln.next.clone()
    }
    else {
        Some(
            if let Some(next) = &ln.next {
                match remove(next, value) {
                    Some(lnode) => if Arc::ptr_eq(next, &lnode) {
                        ln.clone()
                    }
                    else {
                        new(ln.value.clone(), lnode)
                    },
                    None => new_tail(ln.value.clone())
                }
            }
            else {
                ln.clone()
            }
        )
    }
}

#[allow(unused_macros)]
macro_rules! lnode {
    ( $value:expr ) => {
        {
            new_tail($value)
        }
    };
    ( $value:expr, $($rest:expr),+ ) => {
        {
            new($value, lnode!($($rest),*))    
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lnode_insert_3() {
        let ln = lnode!(3, 2, 1);
        assert_eq!(ln.size, 3);
        assert_eq!(find(&ln, &1).unwrap().value, 1);
        assert_eq!(find(&ln, &2).unwrap().value, 2);
        assert_eq!(find(&ln, &3).unwrap().value, 3);
        assert!(find(&ln, &4).is_none());
    }

    #[test]
    fn lnode_insert_3_again() {
        let ln = insert(&lnode!(3, 2, 1), Cow::Owned(3));
        assert_eq!(ln.size, 3);
        assert_eq!(find(&ln, &1).unwrap().value, 1);
        assert_eq!(find(&ln, &2).unwrap().value, 2);
        assert_eq!(find(&ln, &3).unwrap().value, 3);
        assert!(find(&ln, &4).is_none());
    }

    #[test]
    fn lnode_remove_1() {
        let ln = remove(&lnode!(3, 2, 1), &1).unwrap();
        assert_eq!(ln.size, 2);
        assert!(find(&ln, &1).is_none());
        assert_eq!(find(&ln, &2).unwrap().value, 2);
        assert_eq!(find(&ln, &3).unwrap().value, 3);
        assert!(find(&ln, &4).is_none());
    }

    #[test]
    fn lnode_remove_2() {
        let ln = remove(&lnode!(3, 2, 1), &2).unwrap();
        assert_eq!(ln.size, 2);
        assert_eq!(find(&ln, &1).unwrap().value, 1);
        assert!(find(&ln, &2).is_none());
        assert_eq!(find(&ln, &3).unwrap().value, 3);
        assert!(find(&ln, &4).is_none());
    }

    #[test]
    fn lnode_remove_3() {
        let ln = remove(&lnode!(3, 2, 1), &3).unwrap();
        assert_eq!(ln.size, 2);
        assert_eq!(find(&ln, &1).unwrap().value, 1);
        assert_eq!(find(&ln, &2).unwrap().value, 2);
        assert!(find(&ln, &3).is_none());
        assert!(find(&ln, &4).is_none());
    }
}
