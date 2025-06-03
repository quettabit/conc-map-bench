use std::{fmt::Debug, str::FromStr};

use bustle::*;

use super::bench::Options;

#[derive(Debug)]
pub enum WorkloadKind {
    ReadHeavy,
    UpdateHeavy1,
    UpdateHeavy2,
    Exchange,
    RapidGrow,
}

impl FromStr for WorkloadKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ReadHeavy" => Ok(Self::ReadHeavy),
            "UpdateHeavy1" => Ok(Self::UpdateHeavy1),
            "UpdateHeavy2" => Ok(Self::UpdateHeavy2),
            "Exchange" => Ok(Self::Exchange),
            "RapidGrow" => Ok(Self::RapidGrow),
            _ => Err("unknown workload"),
        }
    }
}

fn read_heavy(threads: u32) -> Workload {
    let mix = Mix {
        read: 98,
        insert: 1,
        remove: 1,
        update: 0,
        upsert: 0,
    };

    *Workload::new(threads as usize, mix)
        .initial_capacity_log2(25)
        .prefill_fraction(0.75)
}

fn update_heavy_1(threads: u32) -> Workload {
    let mix = Mix {
        read: 10,
        insert: 5,
        remove: 5,
        update: 80,
        upsert: 0,
    };

    *Workload::new(threads as usize, mix)
        .initial_capacity_log2(25)
        .prefill_fraction(0.75)
}

fn update_heavy_2(threads: u32) -> Workload {
    let mix = Mix {
        read: 45,
        insert: 5,
        remove: 5,
        update: 45,
        upsert: 0,
    };

    *Workload::new(threads as usize, mix)
        .initial_capacity_log2(25)
        .prefill_fraction(0.75)
}

fn rapid_grow(threads: u32) -> Workload {
    let mix = Mix {
        read: 5,
        insert: 80,
        remove: 5,
        update: 10,
        upsert: 0,
    };

    *Workload::new(threads as usize, mix)
        .initial_capacity_log2(25)
        .prefill_fraction(0.0)
}

fn exchange(threads: u32) -> Workload {
    let mix = Mix {
        read: 10,
        insert: 40,
        remove: 40,
        update: 10,
        upsert: 0,
    };

    *Workload::new(threads as usize, mix)
        .initial_capacity_log2(25)
        .prefill_fraction(0.75)
}

pub(crate) fn create(options: &Options, threads: u32) -> Workload {
    let mut workload = match options.workload {
        WorkloadKind::ReadHeavy => read_heavy(threads),
        WorkloadKind::UpdateHeavy1 => update_heavy_1(threads),
        WorkloadKind::UpdateHeavy2 => update_heavy_2(threads),
        WorkloadKind::Exchange => exchange(threads),
        WorkloadKind::RapidGrow => rapid_grow(threads),
    };

    workload.operations(options.operations);
    workload
}
