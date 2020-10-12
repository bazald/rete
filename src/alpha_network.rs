use crate::{map_reduce_map::MapReduceMap, symbol::Symbol, wme::WME};
use im::{HashMap, HashSet};

pub type NodeId = u64;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum WorkingMemoryChange {
    Insert,
    Remove,
}

pub type WorkingMemory = HashMap<WME, i64>;
pub type WorkingMemoryDelta = HashMap<WME, i64>;
pub type WorkingMemoryToAlphaDelta = HashMap<WME, WorkingMemoryChange>;

pub type AlphaMemories = HashMap<NodeId, HashSet<WME>>;

pub enum WMEIndex {
    Identifier,
    Attribute,
    Value,
}

pub struct FilterTest {
    wme_index: WMEIndex,
    symbol: Symbol,
}

pub type FilterTests = HashMap<FilterTest, NodeId>;
pub type AlphaLinks = HashMap<NodeId, FilterTests>;

pub struct AlphaNetwork {
    working_memory: WorkingMemory,
    alpha_memories: AlphaMemories,
    first_filter_tests: FilterTests,
    alpha_links_01: AlphaLinks,
    alpha_links_12: AlphaLinks,
}
impl AlphaNetwork {
    fn new() -> Self {
        AlphaNetwork {
            working_memory: WorkingMemory::new(),
            alpha_memories: AlphaMemories::new(),
            first_filter_tests: FilterTests::new(),
            alpha_links_01: AlphaLinks::new(),
            alpha_links_12: AlphaLinks::new(),
      }
    }

    fn update_working_memory(&self, wme_delta: &WorkingMemoryDelta) -> (WorkingMemory, WorkingMemoryToAlphaDelta) {
        self.working_memory.joint_transform(
            wme_delta,
            |lwm, rwm: WorkingMemoryToAlphaDelta| lwm.union(rwm),
            |lwm, rwm| {
                let count = lwm.1 + rwm.1;
                (
                    if count == 0 { Some((lwm.0.clone(), count)) } else { None },
                    if count > 0 && *lwm.1 < 1 { WorkingMemoryToAlphaDelta::new().update(lwm.0.clone(), WorkingMemoryChange::Insert) }
                    else if count < 1 && *lwm.1 > 0 { WorkingMemoryToAlphaDelta::new().update(lwm.0.clone(), WorkingMemoryChange::Remove) }
                    else { WorkingMemoryToAlphaDelta::new() }
                )
            },
            |lwm| (Some((lwm.0.clone(), *lwm.1)), WorkingMemoryToAlphaDelta::new()),
            |rwm| if *rwm.1 != 0 {
                (
                    Some((rwm.0.clone(), *rwm.1)),
                    if *rwm.1 > 0 { WorkingMemoryToAlphaDelta::new().update(rwm.0.clone(), WorkingMemoryChange::Insert) } else { WorkingMemoryToAlphaDelta::new() }
                )
            }
            else {
                (None, WorkingMemoryToAlphaDelta::new())
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_working_memory() {
        let alpha0 = AlphaNetwork::new();
        let mut delta0 = WorkingMemoryDelta::new();

        delta0.insert(WME::new(Symbol::Identifier("A1".into()), Symbol::String("value".into()), Symbol::Integer(1)), -1);
        delta0.insert(WME::new(Symbol::Identifier("A1".into()), Symbol::String("value".into()), Symbol::Integer(2)), 0);
        delta0.insert(WME::new(Symbol::Identifier("A1".into()), Symbol::String("value".into()), Symbol::Integer(3)), 1);

        let (wm1, wm2ad01) = alpha0.update_working_memory(&delta0);

        assert_eq!(wm1.len(), 2);
        assert_eq!(wm2ad01.len(), 1);

        println!("AlphaNetwork WM: {:?}\r\nAlphaNetwork Output: {:?}", wm1, wm2ad01);
    }
}
