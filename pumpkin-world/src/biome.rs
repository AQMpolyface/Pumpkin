use serde::{Deserialize, Serialize};
// TODO make this work with the protocol
// Send by the registry
#[derive(Serialize, Deserialize, Clone, Copy)]
#[non_exhaustive]
pub enum Biome {
    //offland biomes
    DeepOcean,
    Ocean,
    WarmOcean,
    LukewarmOcean,
    DeepLukewarmOcean,
    ColdOcean,
    DeepColdOcean,
    FrozenOcean,
    DeepFrozenOcean,
    MushroomFields,
    //Mushroom Fields
    JaggedPeaks,
    FrozenPeaks,
    StonyPeaks,
    Meadow,
    CherryGrove,
    Grove,
    SnowySlopes,
    WindsweptHills,
    WindsweptGravellyHills,
    WindsweptForest,
    //Woodland biomes
    Forest,
    FlowerForest,
    Taiga,
    OldGrowthPineTaigaOldGrowthPineTaiga,
    OldGrowthSpruceTaiga,
    SnowyTaiga,
    BirchForest,
    OldGrowthBirchForest,
    DarkForest,
    Jungle,
    SparseJungle,
    BambooJungle,
    // Wetland biomes
    River,
    FrozenRiver,
    Swamp,
    MangroveSwamp,
    Beach,
    SnowyBeach,
    StonyShore,
    //Flatland biomes
    Plains,
    SunflowerPlains,
    SnowyPlains,
    IceSpikes,
    //Arid-land biomes
    Desert,
    Savanna,
    SavannaPlateau,
    WindsweptSavanna,
    Badlands,
    WoodedBadlands,
    ErodedBadlands,
    //Cave biomes
    DeepDark,
    DripstoneCaves,
    LushCaves,
}
#[derive(Clone)]
#[enum_dispatch(BiomeSupplierImpl)]
pub enum BiomeSupplier {
    Debug(DebugBiomeSupplier),
}
#[enum_dispatch]
pub trait BiomeSupplierImpl {
    fn biome(&self, x: i32, y: i32, z: i32, noise: &MultiNoiseSampler) -> Biome;
}
#[derive(Clone)]
pub struct DebugBiomeSupplier {}
impl BiomeSupplierImpl for DebugBiomeSupplier {
    fn biome(&self, *x: i32, *y: i32, *z: i32, *noise: &MultiNoiseSampler) -> Biome {
        Biome::Plains
    }
}
// TODO: Implement
pub struct MultiNoiseSampler {}
