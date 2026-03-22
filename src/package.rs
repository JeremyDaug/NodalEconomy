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
    /// 
    /// You may only have 1 good per Package, to simplify logic.
    Good{id: usize, quantity: f64},
    /// The good is of abnormal quality (either higher or lower quality). 
    /// Any satisfaction gotten by the good, as well as ownership, use, and consumption
    /// benefits are multiplied by this factor.
    /// 
    /// This does not alter what the good decays into in any way.
    /// 
    /// Requires a Good at the same level. Good cannot have 'Generic' Tag.
    Quality(f64),
    /// Marks the contained good(s) as being preserved. Preservation is limited to
    /// the number of days given in the value. Each day the value should be reduced by 
    /// 1. Fractional values are used to combine goods of varying levels of preservation.
    /// 
    /// Preserved goods can still fail if the preservation reduction does not reduce
    /// the good's [`decay_chance`] at or below 0.0.
    /// 
    /// Requires a preservable good at the same level.
    Preserved(f64),
    /// A 'packaged' service which can allow a service to be sold multiple timse in a 
    /// day. The service still decays at the end of the day.
    /// 
    /// Requires a Service Good at the same level.
    Voucher,
    /// The good in the package is in a disassembled state, removing it's benefits but
    /// reducing it's bulk by the dictated value in the good and overriding the 
    /// immobile statues of the good.
    /// 
    /// Requires a disassembleable good at the same level.
    Disassembled,
    /// A helper to record storage. Should only ever be on the highest level of a Package
    /// and should be drawn from a good stored in the highest level as well.
    /// 
    /// Requires a storage good at the same level.
    Storage{mass: f64, bulk: f64},

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
    /// 
    /// This safely adds parts to a package, checking that the part is a valid combination
    /// with other parts. 
    /// 
    /// This mostly references goods and the limitations those goods offer.
    pub fn add_part(&mut self, other: Part) -> bool {
        todo!();
        // for idx in 0..self.len() {
        //     let found =
        //     match self.parts[idx].compare(&other) {
        //         // if tag comes before or is the same kind, continue to next step.
        //         std::cmp::Ordering::Less | std::cmp::Ordering::Equal => false,
        //         // 
        //         std::cmp::Ordering::Greater => true,
        //     };
        //     if found {
        //         self.parts.insert(idx, other);
        //         return;
        //     }
        // }
    }

    /// # Is Part Valid
    /// 
    /// Given ourself, check that a part is valid to add to this package.
    /// 
    /// Current Requirements.
    /// - Each Package can only have 1 of each Part, duplicates are likely to be 
    /// combined or averaged as the part requires.
    /// - Only one Good per Package.
    /// - Quality and GoodTag::Commodity are exclusive with each other.
    /// - Preserved requires a preserveable good.
    /// - Voucher requires a service good.
    /// - disassembled requires a disassembleable good.
    /// - storage requires the good be marked as storage.
    pub fn is_part_valid(&self, other: &Part) -> bool {
        false
    }

    /// # Pack
    /// 
    /// Takes two packages and combines them into one, both being sub-packages.
    /// 
    /// This will consume both packages, so be sure to clone before use if you want
    /// to keep both.
    pub fn pack(self, other: Package) -> Package {
        let mut res = Package::new();
        res.add_part(Part::Subpack(Box::new(self)));
        res.add_part(Part::Subpack(Box::new(other)));
        res
    }

    /// # Unpack
    /// 
    /// Unpack takes a package and unpacks any sub-packages.
    /// 
    /// This takes subpacks and unwraps the package out of them.
    /// 
    /// The remaining part of the package are grouped together into a package as well 
    /// and returned at the end of the return vector.
    /// 
    /// This only does the highest level, either repeat this until
    /// the bool returns false, or call 'fully_unpack'.
    pub fn unpack(self) -> (Vec<Package>, bool) {
        let mut result = vec![];
        let mut end = Package::new();
        let mut unpacked = false;
        for part in self.parts {
            if let Part::Subpack(package) = part {
                unpacked = true;
                result.push(*package);
            } else {
                end.add_part(part);
            }
        }
        result.push(end);
        (result, unpacked)
    }

    /// # Apply Down
    /// 
    /// Takes the package, breaks up subpackages, and applies the other parts to
    /// those subpackages as is valid, then returns the resulting subpackages as
    /// packages.
    /// 
    /// ## Notes
    /// 
    /// Some parts have requirements to be passed down. The following Parts in particular.
    pub fn apply_down(self) -> Vec<Package> {
        todo!()
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
            Part::Storage { .. } => 6,
        };
        let o = match other {
            Part::Subpack(_) => 0,
            Part::Good { .. } => 1,
            Part::Quality(_) => 2,
            Part::Preserved(_) => 3,
            Part::Voucher => 4,
            Part::Disassembled => 5,
            Part::Storage { .. } => 6,
        };
        v.cmp(&o)
    }
}