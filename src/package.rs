use std::collections::HashMap;

/// # Package
/// 
/// The collection of Packs or data which can be bought, sold, and traded between the 
/// various actors.
/// 
/// They typically include a good and some data or modifiers about that good.
/// 
/// Packages are loosely sorted in enum order to expedite searching and management.
#[derive(Debug, Clone)]
pub struct Package {
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone)]
pub enum Part {
    /// A sub-pack is for a pack made of packs, allowing for packages to have packages
    /// within them. For example, a pair of goods in different preserved states, or a 
    /// service voucher and another non-vouchered service.
    Subpack(Box<Package>),
    /// A good (id), and the amount of the good owned.
    Good{id: usize, quantity: f64},
    /// The good is of abnormal quality (either higher or lower quality). 
    /// Any satisfaction gotten by the good, as well as ownership, use, and consumption
    /// benefits are multiplied by this factor.
    /// 
    /// This does not alter what the good decays into in any way.
    Quality(f64),
    /// Marks the contained good(s) as being preserved. Preservation is limited to
    /// the number of days given in the value. Each day the value should be reduced by 
    /// 1. Fractional values are used to combine goods of varying levels of preservation.
    /// 
    /// Preserved goods can still fail if the preservation reduction does not reduce
    /// the good's [`decay_chance`] at or below 0.0.
    Preserved(f64),
    /// A 'packaged' service which can allow a service to be sold multiple timse in a 
    /// day. The service still decays at the end of the day.
    Voucher,
    /// The good in the package is in a disassembled state, removing it's benefits but
    /// reducing it's bulk by the dictated value in the good and overriding the 
    /// immobile statues of the good.
    Disassembled,
}

impl Package {
    /// # New
    /// 
    /// Quick new up function.
    pub fn new() -> Self {
        Self {
            parts: vec![]
        }
    }

    /// # Length
    /// 
    /// How many parts are in the package.
    pub fn len(&self) -> usize {
        self.parts.len()
    }

    /// # Add Part
    /// 
    /// Adds a part to a Package. They are inserted in loose sort order, as dictated by
    /// the order of Parts.
    /// 
    /// Note, subpackages should only be self-mutable.
    pub fn add_part(&mut self, other: Part) {
        for idx in 0..self.len() {
            let found =
            match self.parts[idx].compare(&other) {
                // if tag comes before or is the same kind, continue to next step.
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => false,
                // 
                std::cmp::Ordering::Greater => true,
            };
            if found {
                self.parts.insert(idx, other);
                return;
            }
        }
    }

    /// # All Goods
    /// 
    /// Counts up all goods in the Package, includes sub-packages.
    /// 
    /// Returns a count of all the goods included.
    pub fn all_goods(&self) -> HashMap<usize, f64> {
        let mut res = HashMap::new();
        for part in self.parts.iter() {
            match part {
                Part::Good{id, quantity} => {
                    res.entry(*id)
                        .and_modify(|x| *x += *quantity)
                        .or_insert(*quantity);
                },
                Part::Subpack(b) => {
                    let sub = b.all_goods();
                    for item in sub {
                        res.entry(item.0)
                            .and_modify(|x| *x += item.1)
                            .or_insert(item.1);
                    }
                },
                _ => {}
            }
        }

        res
    }
}

impl Part {
    pub fn compare(&self, other: &Part) -> std::cmp::Ordering {
        let v = match self {
            Part::Subpack(_) => 0,
            Part::Good { .. } => 1,
            Part::Quality(_) => 2,
            Part::Preserved(_) => 3,
            Part::Voucher => 4,
            Part::Disassembled => 5,
        };
        let o = match other {
            Part::Subpack(_) => 0,
            Part::Good { .. } => 1,
            Part::Quality(_) => 2,
            Part::Preserved(_) => 3,
            Part::Voucher => 4,
            Part::Disassembled => 5,
        };
        v.cmp(&o)
    }
}