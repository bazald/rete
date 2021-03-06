use super::map_reduce_set::MapReduceSet;
use core::hash::Hash;
use hashbrown::HashMap;

pub trait MapReduceMap<K: Eq + Hash, V> {
    fn find(&self, key: &K) -> Option<(&'_ K, &'_ V)>;

    fn visit<Op>(&self, op: Op) where Op: Fn((&'_ K, &'_ V));

    fn transform<ReduceT, ReduceOp, Op>
        (&self, reduce_op: ReduceOp, op: Op) -> (Self, ReduceT)
        where
        Self: Sized,
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        Op: Fn((&K, &V)) -> (Option<(K, V)>, ReduceT);

    fn joint_transform<ReduceT, ReduceOp, RightV, RightT, BothOp, LeftOp, RightOp>
        (&self, right: &RightT, reduce_op: ReduceOp, both_op: BothOp, left_op: LeftOp, right_op: RightOp) -> (Self, ReduceT)
        where
        Self: Sized,
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        RightT: MapReduceMap<K, RightV>,
        for<'i> &'i RightT: IntoIterator<Item = (&'i K, &'i RightV)>,
        BothOp: Fn((&K, &V), (&K, &RightV)) -> (Option<(K, V)>, ReduceT),
        LeftOp: Fn((&K, &V)) -> (Option<(K, V)>, ReduceT),
        RightOp: Fn((&K, &RightV)) -> (Option<(K, V)>, ReduceT);

    fn xor<RightT>
        (&self, right: &RightT) -> (Self, usize)
        where
        Self: Sized,
        K: Clone,
        V: Clone,
        RightT: MapReduceMap<K, V>,
        for<'i> &'i RightT: IntoIterator<Item = (&'i K, &'i V)>,
    {
        let op = |(k, v) : (&K, &V)| (Some((k.clone(), v.clone())), 1);
        self.joint_transform(right, |ls,rs| ls + rs, |_,_| (None, 0), op, op)
    }

    fn xor_subsets<SubK, RightT>
        (&self, right: &RightT) -> (Self, usize)
        where
        Self: Sized,
        K: Clone,
        SubK: Clone + Eq + Hash,
        V: MapReduceSet<SubK> + Clone,
        for<'i> &'i V: IntoIterator<Item = &'i SubK>,
        RightT: MapReduceMap<K, V>,
        for<'i> &'i RightT: IntoIterator<Item = (&'i K, &'i V)>,
    {
        let op = |(k, v) : (&K, &V)| (Some((k.clone(), v.clone())), 1);
        self.joint_transform(right, |ls,rs| ls + rs, |ls,rs| {
            let xored = ls.1.xor(rs.1);
            if xored.1 == 0 { (None, 0) } else { (Some((ls.0.clone(), xored.0)), 1) }
        }, op, op)
    }
}

impl<K: Eq + Hash, V> MapReduceMap<K, V> for HashMap<K, V> {
    fn find<'a>(&self, key: &K) -> Option<(&'_ K, &'_ V)> {
        self.iter().find(|k| *(*k).0 == *key)
    }

    fn visit<Op>(&self, op: Op) where Op: Fn((&K, &V)) {
        self.iter().for_each(|kv| op(kv));
    }

    fn transform<ReduceT, ReduceOp, Op>
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

    fn joint_transform<ReduceT, ReduceOp, RightV, RightT, BothOp, LeftOp, RightOp>
        (&self, right: &RightT, reduce_op: ReduceOp, both_op: BothOp, left_op: LeftOp, right_op: RightOp) -> (Self, ReduceT)
        where
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        RightT: MapReduceMap<K, RightV>,
        for<'i> &'i RightT: IntoIterator<Item = (&'i K, &'i RightV)>,
        BothOp: Fn((&K, &V), (&K, &RightV)) -> (Option<(K, V)>, ReduceT),
        LeftOp: Fn((&K, &V)) -> (Option<(K, V)>, ReduceT),
        RightOp: Fn((&K, &RightV)) -> (Option<(K, V)>, ReduceT)
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
        right.into_iter().for_each(|rkv| {
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

#[cfg(feature = "im-rs")]
impl<K: Clone + Eq + Hash, V: Clone> MapReduceMap<K, V> for im::HashMap<K, V> {
    fn find<'a>(&self, key: &K) -> Option<(&'_ K, &'_ V)> {
        self.iter().find(|k| *(*k).0 == *key)
    }

    fn visit<Op>(&self, op: Op) where Op: Fn((&K, &V)) {
        self.iter().for_each(|kv| op(kv));
    }

    fn transform<ReduceT, ReduceOp, Op>
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

    fn joint_transform<ReduceT, ReduceOp, RightV, RightT, BothOp, LeftOp, RightOp>
        (&self, right: &RightT, reduce_op: ReduceOp, both_op: BothOp, left_op: LeftOp, right_op: RightOp) -> (Self, ReduceT)
        where
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        RightT: MapReduceMap<K, RightV>,
        for<'i> &'i RightT: IntoIterator<Item = (&'i K, &'i RightV)>,
        BothOp: Fn((&K, &V), (&K, &RightV)) -> (Option<(K, V)>, ReduceT),
        LeftOp: Fn((&K, &V)) -> (Option<(K, V)>, ReduceT),
        RightOp: Fn((&K, &RightV)) -> (Option<(K, V)>, ReduceT)
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
        right.into_iter().for_each(|rkv| {
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
    use std::println;
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
