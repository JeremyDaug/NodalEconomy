use std::collections::HashMap;

/// # Good
/// 
/// Goods are things that people desire, either phsyical or otherwise.
/// 
/// For the author's note. Goods include Wants
/// 
/// Goods may have a class, which includes an ID for the good they are based off of.
/// If a good's class is it's own Id, then it is the original of the class.
/// 
/// Goods also have a Mass and Bulk, representing the weight and size of the good. This 
/// can be negative, but shouldn't be in most cases. Prefer using storage instead.
/// 
/// Goods have Ownership, Use, and Consmuption benefits, producing other goods in those cases.
/// Most goods produce 'want' type goods with this.
/// 
/// Goods may decay into other goods. Decay occurs at [`decay_chance`] chance, and when 
/// it does they decay into [`decay_result`], which is another good, at the defined by 
/// [`decay_rate`].
#[derive(Debug, Clone)]
pub struct Good {
    /// The unique ID of the good.
    pub id: usize,
    /// The unique name of the good.
    pub name: String,

    /// The class is the good belongs to. If it points to itself, then the good
    /// is the class example.
    pub class: Option<usize>,

    /// The mass (in kg) of the good per unit.
    pub mass: f64,
    /// The space (in m^3) that the good takes up.
    pub bulk: f64,

    /// What goods this good produces for owning it.
    /// 
    /// It is advised that this should only produce goods which decay rapidly or that
    /// cannot be traded.
    pub ownership_benefits: HashMap<usize, f64>,

    /// What goods this produces when used alone. Does not destroy the good in use.
    /// 
    /// It is advised that this should only produce goods which decay rapidly or that
    /// cannot be traded.
    pub use_benefits: HashMap<usize, f64>,
    /// How long it takes to use the good.
    pub use_time: f64,

    /// What goods this produces when consumed in use. Along with these outputs, 
    /// conusmed goods also decay.
    /// 
    /// It is advised that this should only produce goods which decay rapidly or that
    /// cannot be traded.
    pub consume_benefits: HashMap<usize, f64>,
    /// How long it takes to consume the good.
    pub consume_time: f64,

    /// The chance that a unit of the good decays.
    /// 
    /// Roll under this value and the good decays.
    /// 
    /// 0.0 means it doesn't decay. 1.0 means it always decays.
    pub decay_chance: f64,
    /// What the good decays into and in what ratios.
    /// 
    /// We assume that the good decays into each evenly.
    /// 
    /// TODO: Consider adding a factor to let this be semi-random.
    pub decay_result: HashMap<usize, f64>,

    /// Additional tags the good has. Modifies default functionality of a good.
    /// 
    /// NOTE: Duplicate tags are allowed to exist, but only the first will be used. Don't add duplicates please and thanks.
    /// TODO: Make it so when we add tags to a good, we chech for duplicates.
    pub good_tags: Vec<GoodTag>,
}

// utilty functions, split for ease of viewing.
impl Good {
    pub fn is_immobile(&self) -> bool {
        self.good_tags.iter().any(|x| {
            x.is_immobile()
        })
    }

    pub fn is_service(&self) -> bool {
        self.good_tags.iter().any(|x| {
            x.is_service()
        })
    }

    pub fn is_storage(&self) -> bool {
        self.good_tags.iter().any(|x| {
            x.is_storage()
        })
    }

    pub fn is_record(&self) -> bool {
        self.good_tags.iter().any(|x| {
            x.is_record()
        })
    }

    pub fn is_deconstructablee(&self) -> bool {
        self.good_tags.iter().any(|x| {
            x.is_deconstructablee()
        })
    }

    pub fn is_preserveable(&self) -> bool {
        self.good_tags.iter().any(|x| {
            x.is_preserveable()
        })
    }

    /// # Has Tag
    /// 
    /// Checks for a tag in the good based on the tag given.
    /// 
    /// Does not care about data matching, only enum variant matching.
    pub fn has_tag(&self, tag: &GoodTag) -> bool {
        self.good_tags.iter()
            .any(|x| std::mem::discriminant(x) == std::mem::discriminant(tag))
    }

    /// # Get Tag
    /// 
    /// A general tag getter. Gets the first tag that matches the input type.
    /// 
    /// If that tag is not found, it returns None.
    pub fn get_tag(&self, tag: &GoodTag) -> Option<GoodTag> {
        if let Some(res) = self.good_tags
        .iter().find(|&x| std::mem::discriminant(x) == std::mem::discriminant(tag)) {
            Some(res.clone())
        } else {
            None
        }
    }
    /// # Decay Good
    /// 
    /// Decays the given quanity of goods into it's resulting outputs.
    /// 
    /// This does not do any rolling for decay chance, just decays all of the goods.
    /// 
    /// Returns a hashmap for future proofing.
    /// 
    /// # Arguments
    /// 
    /// * `quantity` - hov many of the good we are decaying.
    pub fn decay_good(&self, quantity: f64) -> HashMap<usize, f64> {
        let mut result = HashMap::new();

        for (good, ratio) in self.decay_result.iter() {
            result.insert(*good, *ratio * quantity);
        }

        result
    }
}

// New functions and fluent editors.
impl Good {
    /// # New
    /// 
    /// Makes a new default good with the id and name given, which should be unique.
    /// 
    /// Other values set to:
    /// mass: 0.0,
    /// bulk: 0.0,
    /// ownership_benefits: HashMap::new(),
    /// use_benefits: HashMap::new(),
    /// use_time: 0.0,
    /// consume_benefits: HashMap::new(),
    /// consume_time: 0.0,
    /// decay_chance: 1.0,
    /// decay_rate: 0.0,
    /// decay_result: 0,
    /// good_tags: vec![],
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            class: None,
            mass: 0.0,
            bulk: 0.0,
            ownership_benefits: HashMap::new(),
            use_benefits: HashMap::new(),
            use_time: 0.0,
            consume_benefits: HashMap::new(),
            consume_time: 0.0,
            decay_chance: 1.0,
            decay_result: HashMap::new(),
            good_tags: vec![],
        }
    }

    /// # With Tags
    /// 
    /// A fluent inserter of good tags.
    /// 
    /// Note, while duplicate tags are allowed, only the first in the list will be used.
    pub fn with_tags(mut self, good_tags: Vec<GoodTag>) -> Self {
        self.good_tags = good_tags;
        self
    }

    /// # With Decay
    /// 
    /// Fluent Setter
    /// 
    /// # Arguments
    /// 
    /// * `chance` - must be between 0.0 and 1.0 inclusive.
    /// * `result` - What it decays into. Goods (keys) must exist (not checkable here) 
    /// and values should be positive (but may be negative).
    pub fn with_decay(mut self, chance: f64, result: HashMap<usize, f64>) -> Self {
        assert!(0.0 <= chance && chance <= 1.0,  "Chance must be between 0.0 and 1.0 inclusive.");
        self.decay_chance = chance;
        self.decay_result = result;
        self
    }

    pub fn with_consumption(mut self, benefits: HashMap<usize, f64>, time: f64) -> Self {
        self.consume_benefits = benefits;
        self.consume_time = time;
        self
    }

    pub fn with_use(mut self, benefits: HashMap<usize, f64>, time: f64) -> Self {
        self.use_benefits = benefits;
        self.use_time = time;
        self
    }

    pub fn with_ownership_benefits(mut self, benefits: HashMap<usize, f64>) -> Self {
        self.ownership_benefits = benefits;
        self
    }

    pub fn with_bulk(mut self, bulk: f64) -> Self {
        self.bulk = bulk;
        self
    }

    pub fn with_mass(mut self, mass: f64) -> Self {
        self.mass = mass;
        self
    }

    pub fn with_class(mut self, class: usize) -> Self {
        self.class = Some(class);
        self
    }
}

/// # Good Tags
/// 
/// Good Tags redefine the default logic of a good
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum GoodTag {
    /// Good cannot be moved without first being packed up or disassembled.
    Immobile,
    /// The good can only be produced, then sold once. Once transferred, it becomes
    /// impossible to tranfer it again. This may be overridden with a 'voucher' tag on 
    /// the wrapper.
    /// 
    /// Services, even with the voucher, cannot be moved out of the current region the good is in.
    Service,
    /// The good includes some amount of storage, letting it hold some ammount of other 
    /// things.
    Storage{mass: f64, bulk: f64},
    /// The good is a record which can store 'lines' worth of data.
    /// NOTE: Lines, in not a defined unit, but should be equal to one storage slot of an actor's memory.
    Record{lines: usize},
    /// The good is able to be deconstructed, allowing it's size to be reduced at the 
    /// cost of making it no longer usebale for anything, and the time to deconstruct
    /// and reconstruct it.
    /// 
    /// When deconstructed, it overrides Immobile.
    Deconstructable { reduction: f64, deconstruction_time: f64, reconstruction_time: f64 },
    /// The good is able to be safely stored, removing an amount of [`decay_chance`] 
    /// from the good. While it should not reduce [`decay_chance`] below 0.0, it
    /// is allowed to. Reducing below 0.0 does nothing more.
    Preserveable{reduction: f64},
}

impl GoodTag {
    pub fn storage() -> Self {
        GoodTag::Storage { mass: 0.0, bulk: 0.0 }
    }

    pub fn record() -> Self {
        GoodTag::Record { lines: 0 }
    }

    pub fn deconstructable() -> Self {
        GoodTag::Deconstructable { reduction: 0.0, deconstruction_time: 0.0, reconstruction_time: 0.0 }
    }

    pub fn preserveable() -> Self {
        GoodTag::Preserveable { reduction: 0.0 }
    }

    pub fn is_immobile(&self) -> bool {
        match self {
            GoodTag::Immobile => true,
            _ => false
        }
    }

    pub fn is_service(&self) -> bool {
        match self {
            GoodTag::Service => true,
            _ => false
        }
    }

    pub fn is_storage(&self) -> bool {
        match self {
            GoodTag::Storage { .. } => true,
            _ => false
        }
    }

    pub fn is_record(&self) -> bool {
        match self {
            GoodTag::Record { .. } => true,
            _ => false
        }
    }

    pub fn is_deconstructablee(&self) -> bool {
        match self {
            GoodTag::Deconstructable { .. } => true,
            _ => false
        }
    }

    pub fn is_preserveable(&self) -> bool {
        match self {
            GoodTag::Preserveable { .. } => true,
            _ => false
        }
    }
}