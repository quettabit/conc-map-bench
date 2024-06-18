pub use self::{
    // btreemap::ParkingLotRwLockBTreeMapTable, btreemap::StdRwLockBTreeMapTable,
    // chashmap::CHashMapTable, contrie::ContrieTable, crossbeam_skiplist::CrossbeamSkipMapTable,
    // evmap::EvmapTable,
    // std::ParkingLotRwLockStdHashMapTable,
    // std::StdRwLockStdHashMapTable,
    flurry::FlurryTable,
    dashmap::DashMapTable,
    papaya::PapayaTable,
    scc::SccMapTable,
};

// mod btreemap;
// mod chashmap;
// mod contrie;
// mod crossbeam_skiplist;
// mod evmap;
// mod std;
mod flurry;
mod dashmap;
mod papaya;
mod scc;

type Value = u32;
