use crate::{map_reduce_map::MapReduceMap, map_reduce_set::MapReduceSet, symbol::Symbol, wme::{WME, WMEIndex}};
use im::{HashMap, HashSet};

pub type NodeId = u64;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum MemoryChange {
    Insert,
    Remove,
}

pub type WorkingMemory = HashMap<WME, i64>;
pub type WorkingMemoryDelta = HashMap<WME, i64>;

pub type WMESet = HashSet<WME>;

pub type AlphaMemories = HashMap<NodeId, WMESet>;
pub type AlphaMemoriesDelta0 = WMESet;
pub type AlphaMemoriesDelta12 = HashMap<NodeId, WMESet>;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
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
            working_memory: WorkingMemory::default(),
            alpha_memories: AlphaMemories::default(),
            first_filter_tests: FilterTests::default(),
            alpha_links_01: AlphaLinks::default(),
            alpha_links_12: AlphaLinks::default(),
      }
    }

    fn update_working_memory(&self, wme_delta: &WorkingMemoryDelta) -> (WorkingMemory, AlphaMemoriesDelta0) {
        self.working_memory.joint_transform(
            wme_delta,
            |lamd, ramd: AlphaMemoriesDelta0| lamd.union(ramd),
            |lwm, rwm| {
                let count = lwm.1 + rwm.1;
                (
                    if count == 0 { Some((lwm.0.clone(), count)) } else { None },
                    if (count > 0 && *lwm.1 < 1) || (count < 1 && *lwm.1 > 0) { AlphaMemoriesDelta0::default().update(lwm.0.clone()) }
                    else { AlphaMemoriesDelta0::default() }
                )
            },
            |lwm| (Some((lwm.0.clone(), *lwm.1)), AlphaMemoriesDelta0::default()),
            |rwm| if *rwm.1 != 0 {
                (
                    Some((rwm.0.clone(), *rwm.1)),
                    if *rwm.1 > 0 { AlphaMemoriesDelta0::default().update(rwm.0.clone()) } else { AlphaMemoriesDelta0::default() }
                )
            }
            else {
                (None, AlphaMemoriesDelta0::default())
            }
        )
    }

    fn calculate_first_filter_deltas(&self, alpha_delta: &AlphaMemoriesDelta0) -> AlphaMemoriesDelta12 {
        self.first_filter_tests.transform(
            |lamd, ramd: AlphaMemoriesDelta12| lamd.union(ramd),
            |ft| (None,
            {
                alpha_delta.transform(
                    |lamd, ramd: AlphaMemoriesDelta12| lamd.union(ramd),
                    |ad| (None, if *ad.at(ft.0.wme_index) == ft.0.symbol {
                        AlphaMemories::default().update(*ft.1, WMESet::default().update(ad.clone()))
                    }
                    else {
                        AlphaMemories::default()
                    })
                ).1
            })
        ).1
    }
}

impl Default for AlphaNetwork {
    fn default() -> AlphaNetwork {
        AlphaNetwork {
            working_memory: WorkingMemory::default(),
            alpha_memories: AlphaMemories::default(),
            first_filter_tests: FilterTests::default(),
            alpha_links_01: AlphaLinks::default(),
            alpha_links_12: AlphaLinks::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_network_test() {
        let alpha0 = AlphaNetwork::default();
        let mut delta0 = WorkingMemoryDelta::default();

        delta0.insert(WME::new(Symbol::Identifier("A1".into()), Symbol::String("value".into()), Symbol::Integer(1)), -1);
        delta0.insert(WME::new(Symbol::Identifier("A1".into()), Symbol::String("value".into()), Symbol::Integer(2)), 0);
        delta0.insert(WME::new(Symbol::Identifier("A1".into()), Symbol::String("value".into()), Symbol::Integer(3)), 1);

        let (wm1, wm2ad0) = alpha0.update_working_memory(&delta0);

        assert_eq!(wm1.len(), 2);
        assert_eq!(wm2ad0.len(), 1);

        alpha0.alpha_links_01

        let delta1 = alpha0.calculate_first_filter_deltas(&wm2ad0);

        println!("AlphaNetwork WM: {:?}\r\nAlphaNetwork WM Output: {:?}\r\nAlphaNetwork Filter0 Delta: {:?}", wm1, wm2ad0, delta1);
    }
}
