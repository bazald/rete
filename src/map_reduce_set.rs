use std::{collections::HashSet, hash::Hash};

pub trait MapReduceSet<K: Eq + Hash> {
    fn find(&self, key: &K) -> Option<&'_ K>;

    fn visit<Op>(&self, op: Op) where Op: Fn(&'_ K);

    fn transform<ReduceT, ReduceOp, Op>
        (&self, reduce_op: ReduceOp, op: Op) -> (Self, ReduceT)
        where
        Self: Sized,
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        Op: Fn(&K) -> (Option<K>, ReduceT);

    fn joint_transform<ReduceT, ReduceOp, BothOp, LeftOp, RightOp>
        (&self, right: &Self, reduce_op: ReduceOp, both_op: BothOp, left_op: LeftOp, right_op: RightOp) -> (Self, ReduceT)
        where
        Self: Sized,
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        BothOp: Fn(&K, &K) -> (Option<K>, ReduceT),
        LeftOp: Fn(&K) -> (Option<K>, ReduceT),
        RightOp: Fn(&K) -> (Option<K>, ReduceT);
}

impl<K: Eq + Hash> MapReduceSet<K> for HashSet<K> {
    fn find<'a>(&self, key: &K) -> Option<&'_ K> {
        self.iter().find(|k| **k == *key)
    }

    fn visit<Op>(&self, op: Op) where Op: Fn(&K) {
        self.iter().for_each(|k| op(k));
    }

    fn transform<ReduceT, ReduceOp, Op>
        (&self, reduce_op: ReduceOp, op: Op) -> (Self, ReduceT)
        where
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        Op: Fn(&K) -> (Option<K>, ReduceT)
    {
        let mut transformed = Self::default();
        let mut reduced = Some(ReduceT::default());
        self.iter().for_each(|key| {
            let op_result = op(key);
            if let Some(key) = op_result.0 {
                transformed.insert(key);
            }
            reduced = Some(reduce_op(reduced.take().unwrap(), op_result.1));
        });
        (transformed, reduced.unwrap())
    }

    fn joint_transform<ReduceT, ReduceOp, BothOp, LeftOp, RightOp>
        (&self, right: &Self, reduce_op: ReduceOp, both_op: BothOp, left_op: LeftOp, right_op: RightOp) -> (Self, ReduceT)
        where
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        BothOp: Fn(&K, &K) -> (Option<K>, ReduceT),
        LeftOp: Fn(&K) -> (Option<K>, ReduceT),
        RightOp: Fn(&K) -> (Option<K>, ReduceT)
    {
        let mut transformed = Self::default();
        let mut reduced = Some(ReduceT::default());
        self.iter().for_each(|lkey| {
            let rkey = right.find(lkey);
            let op_result = if let Some(rkey) = rkey {
                both_op(lkey, rkey)
            }
            else {
                left_op(lkey)
            };
            if let Some(key) = op_result.0 {
                transformed.insert(key);
            }
            reduced = Some(reduce_op(reduced.take().unwrap(), op_result.1));
        });
        right.iter().for_each(|rkey| {
            if self.find(rkey).is_none() {
                let op_result = right_op(rkey);
                if let Some(key) = op_result.0 {
                    transformed.insert(key);
                }
                reduced = Some(reduce_op(reduced.take().unwrap(), op_result.1));
            }
        });
        (transformed, reduced.unwrap())
    }
}

impl<K: Clone + Eq + Hash> MapReduceSet<K> for im::HashSet<K> {
    fn find<'a>(&self, key: &K) -> Option<&'_ K> {
        self.iter().find(|k| **k == *key)
    }

    fn visit<Op>(&self, op: Op) where Op: Fn(&K) {
        self.iter().for_each(|k| op(k));
    }

    fn transform<ReduceT, ReduceOp, Op>
        (&self, reduce_op: ReduceOp, op: Op) -> (Self, ReduceT)
        where
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        Op: Fn(&K) -> (Option<K>, ReduceT)
    {
        let mut transformed = Self::default();
        let mut reduced = Some(ReduceT::default());
        self.iter().for_each(|key| {
            let op_result = op(key);
            if let Some(key) = op_result.0 {
                transformed.insert(key);
            }
            reduced = Some(reduce_op(reduced.take().unwrap(), op_result.1));
        });
        (transformed, reduced.unwrap())
    }

    fn joint_transform<ReduceT, ReduceOp, BothOp, LeftOp, RightOp>
        (&self, right: &Self, reduce_op: ReduceOp, both_op: BothOp, left_op: LeftOp, right_op: RightOp) -> (Self, ReduceT)
        where
        ReduceT: Default,
        ReduceOp: Fn(ReduceT, ReduceT) -> ReduceT,
        BothOp: Fn(&K, &K) -> (Option<K>, ReduceT),
        LeftOp: Fn(&K) -> (Option<K>, ReduceT),
        RightOp: Fn(&K) -> (Option<K>, ReduceT)
    {
        let mut transformed = Self::default();
        let mut reduced = Some(ReduceT::default());
        self.iter().for_each(|lkey| {
            let rkey = right.find(lkey);
            let op_result = if let Some(rkey) = rkey {
                both_op(lkey, rkey)
            }
            else {
                left_op(lkey)
            };
            if let Some(key) = op_result.0 {
                transformed.insert(key);
            }
            reduced = Some(reduce_op(reduced.take().unwrap(), op_result.1));
        });
        right.iter().for_each(|rkey| {
            if self.find(rkey).is_none() {
                let op_result = right_op(rkey);
                if let Some(key) = op_result.0 {
                    transformed.insert(key);
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
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn map_reduce_map_hashset() {
        let mut left = HashSet::<i64>::new();
        let mut right = HashSet::<i64>::new();

        let mut rng = rand::thread_rng();
        let range = Uniform::from(1..20);
        for _ in 0..10 {
            left.insert(range.sample(&mut rng));
            right.insert(range.sample(&mut rng));
        }

        let (intersection, xor): (HashSet::<i64>, HashSet::<i64>) = left.joint_transform(
            &right,
            |mut l: HashSet::<i64>, r: HashSet::<i64>| {r.iter().for_each(|key| {l.insert(*key);}); l},
            |lkey, _| (Some(*lkey), HashSet::<i64>::default()),
            |lkey| (None, {let mut r = HashSet::<i64>::new(); r.insert(*lkey); r}),
            |rkey| (None, {let mut r = HashSet::<i64>::new(); r.insert(*rkey); r})
        );

        assert_eq!(left.len() + right.len(), 2 * intersection.len() + xor.len());

        println!("MapReduceSet Left: {:?}\r\nMapReduceSet Right: {:?}\r\nMapReduceSet Intersection: {:?}\r\nMapReduceSet XOR: {:?}",
            left, right, intersection, xor);
    }
}
