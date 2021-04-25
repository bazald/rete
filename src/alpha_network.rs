use crate::{map_reduce_map::MapReduceMap, map_reduce_set::MapReduceSet};
use crate::{node_id::{NodeId, NodeIdGenerator}, symbol::Symbol, wme::{Wme, WmeIndex}};

#[cfg(feature = "im-rs")]
use im::{HashMap, HashSet};

pub type WorkingMemory = HashMap<Wme, i64>;
#[allow(dead_code)]
pub type WorkingMemoryDelta = HashMap<Wme, i64>;

pub type NodeIdSet = HashSet<NodeId>;
#[allow(dead_code)]
pub type WmeSet = HashSet<Wme>;

#[allow(dead_code)]
pub type AlphaMemories = HashMap<NodeId, WmeSet>;
#[allow(dead_code)]
pub type AlphaMemoriesDelta0 = WmeSet;
#[allow(dead_code)]
pub type AlphaMemoriesDelta12 = HashMap<NodeId, WmeSet>;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct FilterTest {
    wme_index: WmeIndex,
    symbol: Symbol,
}

#[allow(dead_code)]
impl FilterTest {
    fn new(wme_index: WmeIndex, symbol: Symbol) -> FilterTest {
        FilterTest {
            wme_index,
            symbol,
        }
    }
}

pub type FilterTests = HashMap<FilterTest, NodeIdSet>;
pub type AlphaLinks = HashMap<NodeId, FilterTests>;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct AlphaNetwork {
    node_id_generator: NodeIdGenerator,
    working_memory: WorkingMemory,
    filter_tests_0: FilterTests,
    filter_tests_1: AlphaLinks,
    filter_tests_2: AlphaLinks,
}

#[allow(dead_code)]
impl AlphaNetwork {
    fn new() -> Self {
        AlphaNetwork {
            node_id_generator: NodeIdGenerator::default(),
            working_memory: WorkingMemory::default(),
            filter_tests_0: FilterTests::default(),
            filter_tests_1: AlphaLinks::default(),
            filter_tests_2: AlphaLinks::default(),
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
        let reduce = |lamd: AlphaMemoriesDelta12, ramd: AlphaMemoriesDelta12| lamd.xor_subsets(&ramd).0;
        alpha_delta.transform(reduce, |wme| (None, {
            let mut am = AlphaMemories::default();
            let insert = |am: AlphaMemories, dest_node_ids: &NodeIdSet, wme: &Wme| {
                let mut ami = AlphaMemories::default();
                dest_node_ids.iter().for_each(|&dest_node_id| {
                    ami = ami.update_with(dest_node_id, {
                        let mut wmes = WmeSet::default();
                        wmes.insert(wme.clone());
                        wmes
                    }, |l,r| {
                        l.union(r)
                    });
                });
                am.xor(&ami) // as union
            };
            if let Some(dest_node_ids) = first_filter_tests.get(&FilterTest::new(WmeIndex::Identifier, wme.at(WmeIndex::Identifier).clone())) {
                am = insert(am, dest_node_ids, wme).0;
            }
            if let Some(dest_node_ids) = first_filter_tests.get(&FilterTest::new(WmeIndex::Attribute, wme.at(WmeIndex::Attribute).clone())) {
                am = insert(am, dest_node_ids, wme).0;
            }
            if let Some(dest_node_ids) = first_filter_tests.get(&FilterTest::new(WmeIndex::Value, wme.at(WmeIndex::Value).clone())) {
                am = insert(am, dest_node_ids, wme).0;
            }
            am
        })).1
    }

    fn calculate_alpha_link_deltas(alpha_links: &AlphaLinks, alpha_delta: &AlphaMemoriesDelta12) -> AlphaMemoriesDelta12 {
        let reduce = |lamd: Option<AlphaMemoriesDelta12>, ramd: Option<AlphaMemoriesDelta12>| {
            if let Some(lamd) = lamd {
                if let Some(ramd) = ramd {
                    Some(lamd.xor_subsets(&ramd).0)
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
                filter_tests.transform(reduce, |(filter_test, dest_node_ids)| {(None, {
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
                        dest_node_ids.iter().for_each(|&dest_node_id| {
                            amd.insert(dest_node_id, wmes.clone());
                        });
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

    fn next(&self, wm_delta: &WorkingMemoryDelta) -> (Self, AlphaMemoriesDelta12) {
        let (working_memory, delta_0) = Self::update_working_memory(&self.working_memory, &wm_delta);
        let delta_1 = Self::calculate_first_filter_deltas(&self.filter_tests_0, &delta_0);
        let delta_2 = Self::calculate_alpha_link_deltas(&self.filter_tests_1, &delta_1);
        let delta_3 = Self::calculate_alpha_link_deltas(&self.filter_tests_2, &delta_2);

        (Self {
            node_id_generator: self.node_id_generator.clone(),
            working_memory,
            filter_tests_0: self.filter_tests_0.clone(),
            filter_tests_1: self.filter_tests_1.clone(),
            filter_tests_2: self.filter_tests_2.clone(),
        },
        delta_3)
    }
}

impl Default for AlphaNetwork {
    fn default() -> AlphaNetwork {
        AlphaNetwork {
            node_id_generator: NodeIdGenerator::default(),
            working_memory: WorkingMemory::default(),
            filter_tests_0: FilterTests::default(),
            filter_tests_1: AlphaLinks::default(),
            filter_tests_2: AlphaLinks::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::println;
    use super::*;

    #[test]
    fn alpha_network_test() {
        let mut alpha0 = AlphaNetwork::default();
        let node0 = alpha0.node_id_generator.next();
        let node1 = alpha0.node_id_generator.next();
        let node2 = alpha0.node_id_generator.next();
        for i in 3..7 {
            alpha0.filter_tests_0.insert(FilterTest::new(WmeIndex::Value, Symbol::Integer(i)), NodeIdSet::new().update(node0));
        }
        alpha0.filter_tests_1.insert(node0, 
            FilterTests::default().update(FilterTest::new(WmeIndex::Identifier, Symbol::Identifier("A1".into())), NodeIdSet::new().update(node1))
        );
        alpha0.filter_tests_2.insert(node1, {
            let mut links = FilterTests::default();
            for i in 4..6 {
                links.insert(FilterTest::new(WmeIndex::Value, Symbol::Integer(i)), NodeIdSet::new().update(node2));
            }
            links
        });

        let mut wm_delta = WorkingMemoryDelta::default();
        wm_delta.insert(Wme::new(Symbol::Identifier("A1".into()), Symbol::String("value".into()), Symbol::Integer(1)), -1);
        wm_delta.insert(Wme::new(Symbol::Identifier("A1".into()), Symbol::String("value".into()), Symbol::Integer(2)), 0);
        for i in 3..10 {
            wm_delta.insert(Wme::new(Symbol::Identifier("A1".into()), Symbol::String("value".into()), Symbol::Integer(i)), 1);
        }
        for i in 3..10 {
            wm_delta.insert(Wme::new(Symbol::Identifier("A2".into()), Symbol::String("value".into()), Symbol::Integer(i)), 1);
        }

        let (working_memory, delta_0) = AlphaNetwork::update_working_memory(&alpha0.working_memory, &wm_delta);

        println!("AlphaNetwork Delta 0: {:?}\r\nAlphaNetwork WM: {:?}", delta_0, working_memory);

        assert_eq!(working_memory.len(), 15);
        assert_eq!(delta_0.len(), 14);

        let delta_1 = AlphaNetwork::calculate_first_filter_deltas(&alpha0.filter_tests_0, &delta_0);

        println!("AlphaNetwork FFT: {:?}\r\n", alpha0.filter_tests_0);

        println!("AlphaNetwork Delta 1: {:?}", delta_1);

        let delta_2 = AlphaNetwork::calculate_alpha_link_deltas(&alpha0.filter_tests_1, &delta_1);
        
        println!("AlphaNetwork Delta 2: {:?}", delta_2);

        let delta_3 = AlphaNetwork::calculate_alpha_link_deltas(&alpha0.filter_tests_2, &delta_2);
        
        println!("AlphaNetwork Delta 3: {:?}", delta_3);

        let (next_alpha, next_delta) = alpha0.next(&wm_delta);

        assert_eq!(next_alpha.working_memory, working_memory);
        assert_eq!(next_delta, delta_3);
    }
}
