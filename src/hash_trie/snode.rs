use alloc::{borrow::Cow, fmt::{self, Debug, Formatter}, sync::Arc};
use core::hash::Hash;
use super::mnode::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SNode<T: Clone + 'static> {
    pub value: T,
}

#[allow(dead_code)]
impl <T: Clone + Debug + Eq + PartialEq + Hash + 'static> SNode<T> {
    pub(super) fn new(value: T) -> Arc<SNode<T>> {
        Arc::new(Self {value})
    }

    pub(super) fn get(&self) -> &T {
        &self.value
    }
}

impl <T: Clone + Debug + Eq + PartialEq + Hash + 'static> MNode<T> for SNode<T> {
    fn find<'a>(&'a self, value: &T) -> FindResult<'a, T> {
        if self.value == *value {
            FindResult::Found(&self.value)
        }
        else {
            FindResult::NotFound
        }
    }

    fn insert<'a>(&'a self, value: Cow<T>) -> InsertResult<'a, T> {
        if self.value == *value {
            InsertResult::Found(&self.value)
        }
        else {
            InsertResult::Inserted(SNode::<T>::new(value.into_owned()), &self.value) // TODO: Actually do insertion
        }
    }

    fn remove<'a>(&'a self, value: &T) -> RemoveResult<'a, T> {
        if self.value == *value {
            RemoveResult::FoundInSNode(&self.value)
        }
        else {
            RemoveResult::NotFound
        }
    }

    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        (&self as &dyn Debug).fmt(formatter)
    }
}
