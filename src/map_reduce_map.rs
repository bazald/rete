use std::{collections::HashMap, hash::Hash};

pub trait MapReduceMap<K: Eq + Hash, V> {
    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;

    fn clear(&mut self);

    fn find(&self, key: &K) -> Option<(&'_ K, &'_ V)>;

    fn visit<Op>(&self, op: Op) where Op: Fn((&'_ K, &'_ V));

    fn transform<ReduceT, Op, ReduceOp>
        (&self, reduce_op: ReduceOp, op: Op) -> (Self, ReduceT)
        where
        Self: Sized,
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        Op: Fn((&K, &V)) -> (Option<(K, V)>, ReduceT);

    fn joint_transform<ReduceT, BothOp, LeftOp, RightOp, ReduceOp>
        (&self, right: &Self, reduce_op: ReduceOp, both_op: BothOp, left_op: LeftOp, right_op: RightOp) -> (Self, ReduceT)
        where
        Self: Sized,
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        BothOp: Fn((&K, &V), (&K, &V)) -> (Option<(K, V)>, ReduceT),
        LeftOp: Fn((&K, &V)) -> (Option<(K, V)>, ReduceT),
        RightOp: Fn((&K, &V)) -> (Option<(K, V)>, ReduceT);
}

impl<K: Eq + Hash, V> MapReduceMap<K, V> for HashMap<K, V> {
    fn is_empty(&self) -> bool {
        (self as &HashMap<K, V>).is_empty()
    }

    fn len(&self) -> usize {
        (self as &HashMap<K, V>).len()
    }

    fn clear(&mut self) {
        (self as &mut HashMap<K, V>).clear()
    }

    fn find<'a>(&self, key: &K) -> Option<(&'_ K, &'_ V)> {
        self.iter().find(|k| *(*k).0 == *key)
    }

    fn visit<Op>(&self, op: Op) where Op: Fn((&K, &V)) {
        self.iter().for_each(|kv| op(kv));
    }

    fn transform<ReduceT, Op, ReduceOp>
        (&self, reduce_op: ReduceOp, op: Op) -> (Self, ReduceT)
        where
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        Op: Fn((&K, &V)) -> (Option<(K, V)>, ReduceT)
    {
        let mut transformed = Self::default();
        let mut reduced = Some(ReduceT::default());
        self.iter().for_each(|kv| {
            let op_result = op(kv);
            if let Some(key_value) = op_result.0 {
                transformed.insert(key_value.0, key_value.1);
            }
            reduced = Some(reduce_op(reduced.take().unwrap(), op_result.1));
        });
        (transformed, reduced.unwrap())
    }

    fn joint_transform<ReduceT, BothOp, LeftOp, RightOp, ReduceOp>
        (&self, right: &Self, reduce_op: ReduceOp, both_op: BothOp, left_op: LeftOp, right_op: RightOp) -> (Self, ReduceT)
        where
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        BothOp: Fn((&K, &V), (&K, &V)) -> (Option<(K, V)>, ReduceT),
        LeftOp: Fn((&K, &V)) -> (Option<(K, V)>, ReduceT),
        RightOp: Fn((&K, &V)) -> (Option<(K, V)>, ReduceT)
    {
        let mut transformed = Self::default();
        let mut reduced = Some(ReduceT::default());
        self.iter().for_each(|lkv| {
            let rkv = right.find(lkv.0);
            let op_result = if let Some(rkv) = rkv {
                both_op(lkv, rkv)
            }
            else {
                left_op(lkv)
            };
            if let Some(kv) = op_result.0 {
                transformed.insert(kv.0, kv.1);
            }
            reduced = Some(reduce_op(reduced.take().unwrap(), op_result.1));
        });
        right.iter().for_each(|rkv| {
            if self.find(rkv.0).is_none() {
                let op_result = right_op(rkv);
                if let Some(kv) = op_result.0 {
                    transformed.insert(kv.0, kv.1);
                }
                reduced = Some(reduce_op(reduced.take().unwrap(), op_result.1));
            }
        });
        (transformed, reduced.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use rand::distributions::{Distribution, Uniform};
    use super::*;

    #[test]
    fn map_reduce_map_hashmap() {
        let mut left = HashMap::<i64, ()>::new();
        let mut right = HashMap::<i64, ()>::new();

        let mut rng = rand::thread_rng();
        let range = Uniform::from(1..20);
        for _ in 0..10 {
            left.insert(range.sample(&mut rng), ());
            right.insert(range.sample(&mut rng), ());
        }

        let (intersection, xor): (HashMap::<i64, ()>, HashMap::<i64, ()>) = left.joint_transform(
            &right,
            |mut l: HashMap::<i64, ()>, r: HashMap::<i64, ()>| {r.iter().for_each(|kv| {l.insert(*kv.0, ());}); l},
            |lkv, _| (Some((*lkv.0, *lkv.1)), HashMap::<i64, ()>::default()),
            |lkv| (None, {let mut r = HashMap::<i64, ()>::new(); r.insert(*lkv.0, ()); r}),
            |rkv| (None, {let mut r = HashMap::<i64, ()>::new(); r.insert(*rkv.0, ()); r})
        );

        assert_eq!(left.len() + right.len(), 2 * intersection.len() + xor.len());

        println!("MapReduceMap Left: {:?}\r\nMapReduceMap Right: {:?}\r\nMapReduceMap Intersection: {:?}\r\nMapReduceMap XOR: {:?}",
            left, right, intersection, xor);
    }
}
