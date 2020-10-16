use crate::{map_reduce_map::MapReduceMap, map_reduce_set::MapReduceSet, symbol::Symbol, wme::{WME, WMEIndex}};
use im::{HashMap, HashSet};

pub type NodeId = u64;

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

impl FilterTest {
    fn new(wme_index: WMEIndex, symbol: Symbol) -> FilterTest {
        FilterTest {
            wme_index,
            symbol,
        }
    }
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

    fn update_working_memory(working_memory: &WorkingMemory, wme_delta: &WorkingMemoryDelta) -> (WorkingMemory, AlphaMemoriesDelta0) {
        working_memory.joint_transform(
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

    fn calculate_first_filter_deltas(first_filter_tests: &FilterTests, alpha_delta: &AlphaMemoriesDelta0) -> AlphaMemoriesDelta12 {
        let reduce = |lamd: AlphaMemoriesDelta12, ramd: AlphaMemoriesDelta12| lamd.union(ramd);
        alpha_delta.transform(reduce, |wme| (None, {
            let mut am = AlphaMemories::default();
            let insert = |am: AlphaMemories, dest_node_id, wme: WME| {
                let mut ami = AlphaMemories::default();
                ami.insert(dest_node_id, {
                    let mut wmes = WMESet::default();
                    wmes.insert(wme);
                    wmes
                });
                am.xor(&ami) // as union
            };
            if let Some(&dest_node_id) = first_filter_tests.get(&FilterTest::new(WMEIndex::Identifier, wme.at(WMEIndex::Identifier).clone())) {
                am = insert(am, dest_node_id, wme.clone()).0;
            }
            if let Some(&dest_node_id) = first_filter_tests.get(&FilterTest::new(WMEIndex::Attribute, wme.at(WMEIndex::Attribute).clone())) {
                am = insert(am, dest_node_id, wme.clone()).0;
            }
            if let Some(&dest_node_id) = first_filter_tests.get(&FilterTest::new(WMEIndex::Value, wme.at(WMEIndex::Value).clone())) {
                am = insert(am, dest_node_id, wme.clone()).0;
            }
            am
        })).1
    }

    fn calculate_alpha_link_deltas(alpha_links: &AlphaLinks, alpha_delta: &AlphaMemoriesDelta12) -> AlphaMemoriesDelta12 {
        let reduce = |lamd: Option<AlphaMemoriesDelta12>, ramd: Option<AlphaMemoriesDelta12>| {
            if let Some(lamd) = lamd {
                if let Some(ramd) = ramd {
                    Some(lamd.union(ramd))
                }
                else {
                    Some(lamd)
                }
            }
            else {
                ramd
            }
        };
        let amd = alpha_delta.transform(reduce, |(src_node_id, wmes)| (None, {
            if let Some(filter_tests) = alpha_links.get(src_node_id) {
                filter_tests.transform(reduce, |(filter_test, &dest_node_id)| {(None, {
                    let (wmes, non_empty) = wmes.transform(
                        |lb,rb| lb || rb,
                        |wme| {
                            if *wme.at(filter_test.wme_index) == filter_test.symbol {
                                (Some(wme.clone()), true)
                            }
                            else {
                                (None, false)
                            }
                        }
                    );
                    if non_empty {
                        let mut amd = AlphaMemoriesDelta12::default();
                        amd.insert(dest_node_id, wmes);
                        Some(amd)
                    }
                    else {
                        None
                    }
                })}).1
            }
            else {
                None
            }
        })).1;
        amd.unwrap_or_default()
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
        let mut alpha0 = AlphaNetwork::default();
        alpha0.first_filter_tests.insert(FilterTest::new(WMEIndex::Value, Symbol::Integer(3)), 1);
        alpha0.alpha_links_01.insert(1, {
            let mut links = FilterTests::default();
            links.insert(FilterTest::new(WMEIndex::Value, Symbol::Integer(1)), 2);
            links.insert(FilterTest::new(WMEIndex::Value, Symbol::Integer(3)), 3);
            links
        });

        let mut delta0 = WorkingMemoryDelta::default();
        delta0.insert(WME::new(Symbol::Identifier("A1".into()), Symbol::String("value".into()), Symbol::Integer(1)), -1);
        delta0.insert(WME::new(Symbol::Identifier("A1".into()), Symbol::String("value".into()), Symbol::Integer(2)), 0);
        delta0.insert(WME::new(Symbol::Identifier("A1".into()), Symbol::String("value".into()), Symbol::Integer(3)), 1);

        let (working_memory, delta1) = AlphaNetwork::update_working_memory(&alpha0.working_memory, &delta0);

        assert_eq!(working_memory.len(), 2);
        assert_eq!(delta1.len(), 1);

        let delta1 = AlphaNetwork::calculate_first_filter_deltas(&alpha0.first_filter_tests, &delta1);
        let alpha_memories1 = alpha0.alpha_memories.xor_subsets(&delta1).0; // insert or remove

        let delta2 = AlphaNetwork::calculate_alpha_link_deltas(&alpha0.alpha_links_01, &delta1);
        let alpha_memories2 = alpha0.alpha_memories.xor_subsets(&delta2).0; // insert or remove
        
        println!("AlphaNetwork WM: {:?}
AlphaNetwork Updated WM: {:?}
AlphaNetwork Delta1: {:?}
AlphaNetwork AM1: {:?}
AlphaNetwork Delta2: {:?}
AlphaNetwork AM2: {:?}", working_memory, delta1, delta1, alpha_memories1, delta2, alpha_memories2);
    }
}
