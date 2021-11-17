use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ItemStackMeta {
    /// The displayed title (name) of the associated `ItemStack`.
    title: String,

    /// The displayed lore of the associated `ItemStack`.
    lore: String,

    /// The damage taken by the `ItemStack`.
    damage: Option<i32>,

    /// The cost of repairing the `ItemStack`.
    repair_cost: Option<u32>,

    /// The enchantments applied to this `ItemStack`.
    enchantments: Vec<Enchantment>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enchantment {
    #[serde(rename = "id")]
    kind: EnchantmentKind,
    #[serde(rename = "lvl")]
    level: i8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnchantmentKind {
    AquaAffinity,
    BaneOfArthropods,
    BlastProtection,
    Channeling,
    Cleaving,
    CurseOfBinding,
    CurseOfVanishing,
    DepthStrider,
    Efficiency,
    FeatherFalling,
    FireAspect,
    FireProtection,
    Flame,
    Fortune,
    FrostWalker,
    Impaling,
    Infinity,
    Knockback,
    Looting,
    Loyalty,
    LuckOfTheSea,
    Lure,
    Mending,
    Multishot,
    Piercing,
    Power,
    ProjectileProtection,
    Protection,
    Punch,
    QuickCharge,
    Respiration,
    Riptide,
    Sharpness,
    SilkTouch,
    Smite,
    SoulSpeed,
    SweepingEdge,
    Thorns,
    Unbreaking,
}
