use crate::{symbol::Symbol, wme::WME};
use im::{HashMap, HashSet};

pub type NodeId = u64;
pub type WorkingMemory = HashMap<WME, i64>;
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

  // fn update_working_memory(&self, wme_deltas: &WorkingMemory) -> AlphaNetwork {
  //   let working_memory = self.working_memory.union_with(wme_deltas)
  // }
}
