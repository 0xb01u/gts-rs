/*
 * GTS-RS - Rust tool for downloading/uploading Pokémon to Gen IV/V games via the in-game GTS.
 * (Rust re-implementation of IR-GTS-MG: https://github.com/ScottehMax/IR-GTS-MG/tree/gen-5)
 * Copyright (C) 2025  Bolu <bolu@tuta.io>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{data_maps::*, should_be_ok, should_not_happen};
use getset::{CopyGetters, Getters};
use num_enum::TryFromPrimitive;
use std::io::{Error, ErrorKind, Result};
use strum::Display;

/// Structure that represent a Pokémon feature that can be identified by a name and an ID.
///
/// This includes species, abilities, natures, moves and items.
#[derive(Clone, Debug, Default, Getters, CopyGetters)]
pub struct IdFeature {
    /// The ID of the feature.
    #[get_copy = "pub"]
    id: u16,
    /// The name of the feature.
    #[get = "pub"]
    name: String,
}

impl IdFeature {
    /// Creates a new species `IdFeature` from the species name.
    ///
    /// # Arguments
    /// * `name` - The name of the species to identify.
    ///
    /// Returns `None` if the name is not a valid species name.
    pub fn from_species_name(name: &str) -> Option<Self> {
        let &id = SPECIES.get_by_right(name)?;
        Some(Self {
            id,
            name: name.to_string(),
        })
    }

    /// Creates a new species `IdFeature` from the species ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the species to identify.
    ///
    /// Returns `None` if the ID is not a valid species ID.
    pub fn from_species_id(id: u16) -> Option<Self> {
        let name = SPECIES.get_by_left(&id)?;
        Some(Self {
            id,
            name: name.clone(),
        })
    }

    /// Creates a new ability `IdFeature` from the ability name.
    ///
    /// # Arguments
    /// * `name` - The name of the ability to identify.
    ///
    /// Returns `None` if the name is not a valid ability name.
    pub fn from_nature_name(name: &str) -> Option<Self> {
        let &id = NATURES.get_by_right(name)?;
        Some(Self {
            id,
            name: name.to_string(),
        })
    }

    /// Creates a new ability `IdFeature` from the ability ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the ability to identify.
    ///
    /// Returns `None` if the ID is not a valid ability ID.
    pub fn from_nature_id(id: u16) -> Option<Self> {
        let name = NATURES.get_by_left(&id)?;
        Some(Self {
            id,
            name: name.clone(),
        })
    }

    /// Creates a new ability `IdFeature` from the ability name.
    ///
    /// # Arguments
    /// * `name` - The name of the ability to identify.
    ///
    /// Returns `None` if the name is not a valid ability name.
    pub fn from_ability_name(name: &str) -> Option<Self> {
        let &id = ABILITIES.get_by_right(name)?;
        Some(Self {
            id,
            name: name.to_string(),
        })
    }

    /// Creates a new ability `IdFeature` from the ability ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the ability to identify.
    ///
    /// Returns `None` if the ID is not a valid ability ID.
    pub fn from_ability_id(id: u16) -> Option<Self> {
        let name = ABILITIES.get_by_left(&id)?;
        Some(Self {
            id,
            name: name.clone(),
        })
    }

    /// Creates a new move `IdFeature` from the move name.
    ///
    /// # Arguments
    /// * `name` - The name of the move to identify.
    ///
    /// Returns `None` if the name is not a valid move name.
    pub fn from_move_name(name: &str) -> Option<Self> {
        let id = MOVES.iter().position(|m| m == name)?;
        Some(Self {
            id: id as u16,
            name: name.to_string(),
        })
    }

    /// Creates a new move `IdFeature` from the move ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the move to identify.
    ///
    /// Returns `None` if the ID is not a valid move ID.
    pub fn from_move_id(id: u16) -> Option<Self> {
        let name = MOVES.get(id as usize)?;
        Some(Self {
            id,
            name: name.clone(),
        })
    }

    /// Creates a new Gen 4 item `IdFeature` from the item name.
    ///
    /// # Arguments
    /// * `name` - The name of the item to identify.
    ///
    /// Returns `None` if the name is not a valid item name.
    pub fn from_gen4_item_name(name: &str) -> Option<Self> {
        let &id = ITEMS_GEN4.get_by_right(name)?;
        Some(Self {
            id,
            name: name.to_string(),
        })
    }

    /// Creates a new Gen 5 item `IdFeature` from the item ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the item to identify.
    ///
    /// Returns `None` if the ID is not a valid item ID.
    pub fn from_gen4_item_id(id: u16) -> Option<Self> {
        let name = ITEMS_GEN4.get_by_left(&id)?;
        Some(Self {
            id,
            name: name.clone(),
        })
    }

    /// Creates a new Gen 5 item `IdFeature` from the item name.
    ///
    /// # Arguments
    /// * `name` - The name of the item to identify.
    ///
    /// Returns `None` if the name is not a valid item name.
    pub fn from_gen5_item_name(name: &str) -> Option<Self> {
        let &id = ITEMS_GEN5.get_by_right(name)?;
        Some(Self {
            id,
            name: name.to_string(),
        })
    }

    /// Creates a new Gen 5 item `IdFeature` from the item ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the item to identify.
    ///
    /// Returns `None` if the ID is not a valid item ID.
    pub fn from_gen5_item_id(id: u16) -> Option<Self> {
        let name = ITEMS_GEN5.get_by_left(&id)?;
        Some(Self {
            id,
            name: name.clone(),
        })
    }
}

impl std::fmt::Display for IdFeature {
    /// Formats the `IdFeature` as a printed string.
    ///
    /// The printing format is: `<self.name> (id: <self.id>)`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (id: {})", self.name, self.id)
    }
}

/// Structure that reprensents a Pokémon feature that affects all its stats.
///
/// This includes EVs, IVs, and base stats.
#[derive(Clone, Copy, Debug, Default)]
pub struct StatsFeature {
    /// Value for the HP stat.
    pub hp: u16,
    /// Value for the Attack stat.
    pub atk: u16,
    /// Value for the Defense stat.
    pub def: u16,
    /// Value for the Special Attack stat.
    pub spa: u16,
    /// Value for the Special Defense stat.
    pub spd: u16,
    /// Value for the Speed stat.
    pub spe: u16,
}

impl StatsFeature {
    pub fn get(&self, stat: &Stat) -> u16 {
        match stat {
            Stat::Hp => self.hp,
            Stat::Atk => self.atk,
            Stat::Def => self.def,
            Stat::SpA => self.spa,
            Stat::SpD => self.spd,
            Stat::Spe => self.spe,
        }
    }
}

/// Enum that identifies the different Pokémon stats.
#[derive(Clone, Copy, Debug, Default, Display)]
pub enum Stat {
    /// Health stat.
    Hp,
    /// Attack stat.
    #[default]
    Atk,
    /// Defense stat.
    Def,
    /// Special Attack stat.
    SpA,
    /// Special Defense stat.
    SpD,
    /// Speed stat.
    Spe,
}

impl Stat {
    /// Converts a `usize` value to a `Stat`, following the special IV ordering.
    ///
    /// The conversion is the following:
    /// * `0` - `Stat::Atk`
    /// * `1` - `Stat::Def`
    /// * `2` - `Stat::Spe`
    /// * `3` - `Stat::SpA`
    /// * `4` - `Stat::SpD`
    /// * Any other value - `Err(Error::InvalidData)`
    pub fn try_from_iv_index(index: usize) -> Result<Self> {
        match index {
            0 => Ok(Stat::Atk),
            1 => Ok(Stat::Def),
            2 => Ok(Stat::Spe),
            3 => Ok(Stat::SpA),
            4 => Ok(Stat::SpD),

            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid IV index")),
        }
    }
}

/// Structure that represents a Pokémon nature.
///
/// Natures have name and ID. Each nature increases one stat by 10% and decreases another by 10%.
/// The increased and decreased stat can be the same, resulting in a neutral nature (i.e., no
/// actual stat changes from base stats). There are 6 neutral natures.
#[derive(Clone, Debug, Default)]
pub struct Nature {
    /// The ID and name of the nature.
    pub id_and_name: IdFeature,
    /// The stat that is increased.
    pub increased_stat: Stat,
    /// The stat that is decreased.
    pub decreased_stat: Stat,
}

impl Nature {
    /// Creates a new nature from its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the nature to create.
    ///
    /// Returns `None` if the ID is not a valid nature ID.
    pub fn from_id(id: u16) -> Option<Self> {
        // Construct the `IdFeature` for the nature:
        let id_and_name = IdFeature::from_nature_id(id)?;

        // Create the nature:
        Some(Self::new(id_and_name))
    }

    /// Creates a new nature from its name.
    ///
    /// # Arguments
    /// * `name` - The name of the nature to create.
    ///
    /// Returns `None` if the name is not a valid nature name.
    pub fn from_name(name: &str) -> Option<Self> {
        // Construct the `IdFeature` for the nature:
        let id_and_name = IdFeature::from_nature_name(name)?;

        // Create the nature:
        Some(Self::new(id_and_name))
    }

    /// Creates a new nature from its ID and name as an `IdFeature`.
    ///
    /// # Arguments
    /// * `id_and_name` - The `IdFeature` representing the nature.
    ///
    /// **Note:** This function is intended for internal use only.
    fn new(id_and_name: IdFeature) -> Self {
        // Get the stat changes for the nature:
        let stat_changes = NATURE_MODIFIERS[id_and_name.id as usize];

        // Find the increased and decreased stats:
        let mut increased_stat = None;
        let mut decreased_stat = None;
        for (i, &multiplier) in stat_changes.iter().enumerate() {
            // Nature stat changes are indexed 0..4 from `atk` to `spe`.
            let idx = i;

            if multiplier == 1.1 {
                increased_stat = match Stat::try_from_iv_index(idx) {
                    Ok(stat) => Some(stat),
                    Err(_) => should_not_happen!("Invalid stat index for increased stat"),
                };
            } else if multiplier == 0.9 {
                decreased_stat = match Stat::try_from_iv_index(idx) {
                    Ok(stat) => Some(stat),
                    Err(_) => should_not_happen!("Invalid stat index for decreased stat"),
                };
            }
        }

        // Handle neutral natures:
        if increased_stat.is_none() && decreased_stat.is_none() {
            increased_stat = Some(should_be_ok!(
                Stat::try_from_iv_index((id_and_name.id / 6) as usize),
                "Invalid stat index for neutral nature"
            ));
            decreased_stat = Some(should_be_ok!(
                Stat::try_from_iv_index((id_and_name.id / 6) as usize),
                "Invalid stat index for neutral nature"
            ));
        }

        // Unpack the stat changes:
        let Some(increased_stat) = increased_stat else {
            should_not_happen!("Increased stat has to have been determined");
        };
        let Some(decreased_stat) = decreased_stat else {
            should_not_happen!("Decreased stat has to have been determined");
        };

        Self {
            id_and_name,
            increased_stat,
            decreased_stat,
        }
    }
}

/// Structure that represents a Pokémon's feature that affects is contest stats.
///
/// This is used exclusively to represent the Pokémon's contest stats.
///
/// This structure is analogous to `StatsFeature`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ContestStatsFeature {
    /// Value for the Cool contest stat.
    pub cool: u8,
    /// Value for the Beauty contest stat.
    pub beauty: u8,
    /// Value for the Cute contest stat.
    pub cute: u8,
    /// Value for the Smart contest stat.
    pub smart: u8,
    /// Value for the Tough contest stat.
    pub tough: u8,
    /// Value for the Sheen contest stat.
    pub sheen: u8,
}

/// Enum that identifies the different contest stats.
#[derive(Clone, Copy, Debug, Default, Display)]
pub enum ContestStat {
    /// Cool contest stat.
    #[default]
    Cool,
    /// Beauty contest stat.
    Beauty,
    /// Cute contest stat.
    Cute,
    /// Smart contest stat.
    Smart,
    /// Tough contest stat.
    Tough,
    /// Sheen contest stat.
    Sheen,
}

/// Enum the represent the different Pokémon and trainer genders.
///
/// A trainer cannot be genderless.
#[derive(Clone, Copy, Debug, Default, Display, TryFromPrimitive)]
#[repr(u8)]
pub enum Gender {
    /// Male gender.
    Male, // 0x0
    /// Female gender.
    Female, // 0x1
    /// Genderless, only for certain special Pokémon.
    #[default]
    Genderless, // 0x2
}

/// Enum that identifies the different shiny leaves a Pokémon can have in HeartGold and SoulSilver.
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash)]
pub enum ShinyLeaf {
    /// The first shiny leaf from the left.
    A,
    /// The second shiny leaf from the left.
    B,
    /// The third shiny leaf from the left.
    C,
    /// The fourth shiny leaf from the left.
    D,
    /// The fifth shiny leaf from the left.
    E,
    /// The shiny crown awarded after collecting all five shiny leaves.
    Crown,
}

/// Enum that identifies the different games a Pokémon can originate from.
#[derive(Clone, Copy, Debug, Default, Display, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Game {
    /// Pokémon Sapphire.
    Sapphire = 1,
    /// Pokémon Ruby.
    Ruby = 2,
    /// Pokémon Emerald.
    Emerald = 3,
    /// Pokémon FireRed.
    FireRed = 4,
    /// Pokémon LeafGreen.
    LeafGreen = 5,
    /// Pokémon HeartGold.
    HeartGold = 7,
    /// Pokémon SoulSilver.
    SoulSilver = 8,
    /// Pokémon Diamond.
    #[default]
    Diamond = 10,
    /// Pokémon Pearl.
    Pearl = 11,
    /// Pokémon Platinum.
    Platinum = 12,
    /// Pokémon Colosseum or Pokémon XD.
    ColosseumXD = 15,
    /// Pokémon White.
    White = 20,
    /// Pokémon Black.
    Black = 21,
    /// Pokémon White 2.
    White2 = 22,
    /// Pokémon Black 2.
    Black2 = 23,
}

/// Enum that identifies the different Gen 4 and Gen 5 Poké Balls.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Display, TryFromPrimitive,
)]
#[repr(u8)]
pub enum Pokeball {
    /// Master Ball.
    MasterBall = 0x01,
    /// Ultra Ball.
    UltraBall,
    /// Great Ball.
    GreatBall,
    /// Poké Ball.
    #[default]
    PokeBall,
    /// Safari Ball.
    SafariBall,
    /// Net Ball.
    NetBall,
    /// Dive Ball.
    DiveBall,
    /// Nest Ball.
    NestBall,
    /// Repeat Ball.
    RepeatBall,
    /// Timer Ball.
    TimerBall,
    /// Luxury Ball.
    LuxuryBall,
    /// Premier Ball.
    PremierBall,
    /// Dusk Ball.
    DuskBall,
    /// Heal Ball.
    HealBall,
    /// Quick Ball.
    QuickBall,
    /// Cherish Ball.
    CherishBall,
    /// Fast Ball.
    FastBall,
    /// Level Ball.
    LevelBall,
    /// Lure Ball.
    LureBall,
    /// Heavy Ball.
    HeavyBall,
    /// Love Ball.
    LoveBall,
    /// Friend Ball.
    FriendBall,
    /// Moon Ball.
    MoonBall,
    /// Sport Ball.
    SportBall,
    /// Dream Ball.
    DreamBall,
}

impl Pokeball {
    pub const FIRST_HGSS_BALL: Self = Pokeball::FastBall;
}

// List of Gen 4 locations:
// https://bulbapedia.bulbagarden.net/wiki/List_of_locations_by_index_number_in_Generation_IV
/// Enum that identifies the different Gen 4 locations.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Display, TryFromPrimitive,
)]
#[repr(u16)]
pub enum Gen4Location {
    /// Mystery Zone.
    MysteryZone,
    /// Twinleaf Town.
    TwinleafTown,
    /// Sandgem Town.
    SandgemTown,
    /// Floaroma Town.
    FloaromaTown,
    /// Solaceon Town.
    SolaceonTown,
    /// Celestic Town.
    CelesticTown,
    /// Jubilife City.
    JubilifeCity,
    /// Canalave City.
    CanalaveCity,
    /// Oreburgh City.
    OreburghCity,
    /// Eterna City.
    EternaCity,
    /// Hearthome City.
    HearthomeCity,
    /// Pastoria City.
    PastoriaCity,
    /// Veilstone City.
    VeilstoneCity,
    /// Sunyshore City.
    SunyshoreCity,
    /// Snowpoint City.
    SnowpointCity,
    /// Pokémon League.
    PokémonLeague,
    /// Route 201.
    Route201,
    /// Route 202.
    Route202,
    /// Route 203.
    Route203,
    /// Route 204.
    Route204,
    /// Route 205.
    Route205,
    /// Route 206.
    Route206,
    /// Route 207.
    Route207,
    /// Route 208.
    Route208,
    /// Route 209.
    Route209,
    /// Route 210.
    Route210,
    /// Route 211.
    Route211,
    /// Route 212.
    Route212,
    /// Route 213.
    Route213,
    /// Route 214.
    Route214,
    /// Route 215.
    Route215,
    /// Route 216.
    Route216,
    /// Route 217.
    Route217,
    /// Route 218.
    Route218,
    /// Route 219.
    Route219,
    /// Route 220.
    Route220,
    /// Route 221.
    Route221,
    /// Route 222.
    Route222,
    /// Route 223.
    Route223,
    /// Route 224.
    Route224,
    /// Route 225.
    Route225,
    /// Route 226.
    Route226,
    /// Route 227.
    Route227,
    /// Route 228.
    Route228,
    /// Route 229.
    Route229,
    /// Route 230.
    Route230,
    /// Oreburgh Mine.
    OreburghMine,
    /// Valley Windworks.
    ValleyWindworks,
    /// Eterna Forest.
    EternaForest,
    /// Fuego Ironworks.
    FuegoIronworks,
    /// Mt. Coronet.
    MtCoronet,
    /// Spear Pillar.
    SpearPillar,
    /// Great Marsh.
    GreatMarsh,
    /// Solaceon Ruins.
    SolaceonRuins,
    /// Victory Road (Sinnoh).
    VictoryRoadSinnoh,
    /// Pal Park.
    PalPark,
    /// Amity Square.
    AmitySquare,
    /// Ravaged Path.
    RavagedPath,
    /// Floaroma Meadow.
    FloaromaMeadow,
    /// Oreburgh Gate.
    OreburghGate,
    /// Fullmoon Island.
    FullmoonIsland,
    /// Sendoff Spring.
    SendoffSpring,
    /// Turnback Cave.
    TurnbackCave,
    /// Flower Paradise.
    FlowerParadise,
    /// Snowpoint Temple.
    SnowpointTemple,
    /// Wayward Cave.
    WaywardCave,
    /// Ruin Maniac Cave.
    RuinManiacCave,
    /// Maniac Tunnel.
    ManiacTunnel,
    /// Trophy Garden.
    TrophyGarden,
    /// Iron Island.
    IronIsland,
    /// Old Chateau.
    OldChateau,
    /// Galactic HQ.
    GalacticHQ,
    /// Verity Lakefront.
    VerityLakefront,
    /// Valor Lakefront.
    ValorLakefront,
    /// Acuity Lakefront.
    AcuityLakefront,
    /// Spring Path.
    SpringPath,
    /// Lake Verity.
    LakeVerity,
    /// Lake Valor.
    LakeValor,
    /// Lake Acuity.
    LakeAcuity,
    /// Newmoon Island.
    NewmoonIsland,
    /// Battle Tower.
    BattleTower,
    /// Fight Area.
    FightArea,
    /// Survival Area.
    SurvivalArea,
    /// Resort Area.
    ResortArea,
    /// Stark Mountain.
    StarkMountain,
    /// Seabreak Path.
    SeabreakPath,
    /// Hall of Origin.
    HallOfOrigin,
    /// Verity Cavern.
    VerityCavern,
    /// Valor Cavern.
    ValorCavern,
    /// Acuity Cavern.
    AcuityCavern,
    /// Jubilife TV.
    JubilifeTV,
    /// Pokétch Co.
    PoketchCo,
    /// GTS.
    GTS,
    /// Trainers' School.
    TrainersSchool,
    /// Mining Museum.
    MiningMuseum,
    /// Flower Shop.
    FlowerShop,
    /// Cycle Shop.
    CycleShop,
    /// Contest Hall.
    ContestHall,
    /// Poffin House.
    PoffinHouse,
    /// Foreign Building.
    ForeignBuilding,
    /// Pokémon Day Care.
    PokemonDayCare,
    /// Veilstone Store.
    VeilstoneStore,
    /// Game Corner.
    GameCorner,
    /// Canalave Library.
    CanalaveLibrary,
    /// Vista Lighthouse.
    VistaLighthouse,
    /// Sunyshore Market.
    SunyshoreMarket,
    /// Pokémon Mansion.
    PokemonMansion,
    /// Footstep House.
    FootstepHouse,
    /// Cafe (D/P) / Café (Pt/HG/SS).
    Cafe,
    /// Grand Lake.
    GrandLake,
    /// Restaurant.
    Restaurant,
    /// Battle Park.
    BattlePark,
    /// Battle Frontier.
    // Platinum-introduced locations:
    BattleFrontier,
    /// Battle Factory.
    BattleFactory,
    /// Battle Castle.
    BattleCastle,
    /// Battle Arcade.
    BattleArcade,
    /// Battle Hall.
    BattleHall,
    /// Distortion World.
    DistortionWorld,
    /// Global Terminal.
    GlobalTerminal,
    /// Villa.
    Villa,
    /// Battleground.
    Battleground,
    /// ROTOM's Room.
    RotomsRoom,
    /// Team Galactic Eterna Building.
    TGEternaBuilding,
    /// Iron Ruins.
    IronRuins,
    /// Iceberg Ruins.
    IcebergRuins,
    /// Rock Peak Ruins.
    RockPeakRuins,
    // HG/SS-introduced locations:
    /// New Bark Town.
    NewBarkTown,
    /// Cherrygrove City.
    CherrygroveCity,
    /// Violet City.
    VioletCity,
    /// Azalea Town.
    AzaleaTown,
    /// Cianwood City.
    CianwoodCity,
    /// Goldenrod City.
    GoldenrodCity,
    /// Olivine City.
    OlivineCity,
    /// Ecruteak City.
    EcruteakCity,
    /// Mahogany Town.
    MahoganyTown,
    /// Lake of Rage.
    LakeOfRage,
    /// Blackthorn City.
    BlackthornCity,
    /// Mt. Silver.
    MtSilver,
    /// Pallet Town.
    PalletTown,
    /// Viridian City.
    ViridianCity,
    /// Pewter City.
    PewterCity,
    /// Cerulean City.
    CeruleanCity,
    /// Lavender Town.
    LavenderTown,
    /// Vermilion City.
    VermilionCity,
    /// Celadon City.
    CeladonCity,
    /// Fuchsia City.
    FuchsiaCity,
    /// Cinnabar Island.
    CinnabarIsland,
    /// Indigo Plateau.
    IndigoPlateau,
    /// Saffron City.
    SaffronCity,
    /// Route 1.
    Route1,
    /// Route 2.
    Route2,
    /// Route 3.
    Route3,
    /// Route 4.
    Route4,
    /// Route 5.
    Route5,
    /// Route 6.
    Route6,
    /// Route 7.
    Route7,
    /// Route 8.
    Route8,
    /// Route 9.
    Route9,
    /// Route 10.
    Route10,
    /// Route 11.
    Route11,
    /// Route 12.
    Route12,
    /// Route 13.
    Route13,
    /// Route 14.
    Route14,
    /// Route 15.
    Route15,
    /// Route 16.
    Route16,
    /// Route 17.
    Route17,
    /// Route 18.
    Route18,
    /// Route 19.
    Route19,
    /// Route 20.
    Route20,
    /// Route 21.
    Route21,
    /// Route 22.
    Route22,
    /// Route 23.
    Route23,
    /// Route 24.
    Route24,
    /// Route 25.
    Route25,
    /// Route 26.
    Route26,
    /// Route 27.
    Route27,
    /// Route 28.
    Route28,
    /// Route 29.
    Route29,
    /// Route 30.
    Route30,
    /// Route 31.
    Route31,
    /// Route 32.
    Route32,
    /// Route 33.
    Route33,
    /// Route 34.
    Route34,
    /// Route 35.
    Route35,
    /// Route 36.
    Route36,
    /// Route 37.
    Route37,
    /// Route 38.
    Route38,
    /// Route 39.
    Route39,
    /// Route 40.
    Route40,
    /// Route 41.
    Route41,
    /// Route 42.
    Route42,
    /// Route 43.
    Route43,
    /// Route 44.
    Route44,
    /// Route 45.
    Route45,
    /// Route 46.
    Route46,
    /// Route 47.
    Route47,
    /// Route 48.
    Route48,
    /// DIGLETT's Cave.
    DiglettsCave,
    /// Mt. Moon.
    MtMoon,
    /// Cerulean Cave.
    CeruleanCave,
    /// Rock Tunnel.
    RockTunnel,
    /// Power Plant.
    PowerPlant,
    /// Safari Zone.
    SafariZone,
    /// Seafoam Islands.
    SeafoamIslands,
    /// Sprout Tower.
    SproutTower,
    /// Bell Tower.
    BellTower,
    /// Burned Tower.
    BurnedTower,
    /// National Park.
    NationalPark,
    /// Radio Tower.
    RadioTower,
    /// Ruins of Alph.
    RuinsOfAlph,
    /// Union Cave.
    UnionCave,
    /// SLOWPOKE Well.
    SlowpokeWell,
    /// Lighthouse.
    Lighthouse,
    /// Team Rocket HQ.
    TeamRocketHQ,
    /// Ilex Forest.
    IlexForest,
    /// Goldenrod Tunnel.
    GoldenrodTunnel,
    /// Mt. Mortar.
    MtMortar,
    /// Ice Path.
    IcePath,
    /// Whirl Islands.
    WhirlIslands,
    /// Mt. Silver Cave.
    MtSilverCave,
    /// Dark Cave.
    DarkCave,
    /// Victory Road (Kanto).
    VictoryRoadKanto,
    /// Dragon's Den.
    DragonDen,
    /// Tohjo Falls.
    TohjoFalls,
    /// Viridian Forest.
    ViridianForest,
    /// Pokéathlon Dome.
    PokeathlonDome,
    /// S.S. Aqua.
    SSAqua,
    /// Safari Zone Gate.
    SafariZoneGate,
    /// Cliff Cave.
    CliffCave,
    /// Frontier Access.
    FrontierAccess,
    /// Bellchime Trail.
    BellchimeTrail,
    /// Sinjoh Ruins.
    SinjohRuins,
    /// Embedded Tower.
    EmbeddedTower,
    /// Pokéwalker.
    Pokewalker,
    /// Cliff Edge Gate.
    CliffEdgeGate,
    // Misc. in-game locations:
    /// Day-Care Couple.
    DayCareCouple = 0x07D0,
    /// Link trade (arrive).
    LinkTradeArrive,
    /// Link trade (met).
    LinkTradeMet,
    /// Kanto.
    Kanto,
    /// Johto.
    Johto,
    /// Hoenn.
    Hoenn,
    /// Sinnoh.
    Sinnoh,
    /// ----------- (D/P/HG/SS) / ---- (Pt).
    Dashes,
    /// Distant land.
    DistantLand,
    /// Traveling Man
    TravelingMan,
    /// Riley.
    Riley,
    // Platinum-introduced misc. in-game locations:
    /// Cynthia.
    Cynthia,
    /// Mystery Zone.
    MysteryZoneMisc,
    // HG/SS-introduced misc. in-game locations:
    /// Mr. Pokémon.
    MrPokemon,
    /// Primo.
    Primo,
    // Misc event locations:
    /// Lovely place.
    LovelyPlace = 0x0BB8,
    /// Pokémon Ranger.
    PokémonRanger,
    /// Faraway place.
    #[default]
    FarawayPlace,
    /// Pokémon Movie.
    PokemonMovie,
    /// Pokémon Movie 06.
    PokemonMovie06,
    /// Pokémon Movie 07.
    PokemonMovie07,
    /// Pokémon Movie 08.
    PokemonMovie08,
    /// Pokémon Movie 09.
    PokemonMovie09,
    /// Pokémon Movie 10.
    PokemonMovie10,
    /// Pokémon Movie 11.
    PokemonMovie11,
    /// Pokémon Movie 12.
    PokemonMovie12,
    /// Pokémon Movie 13.
    PokemonMovie13,
    /// Pokémon Movie 14.
    PokemonMovie14,
    /// Pokémon Movie 15.
    PokemonMovie15,
    /// Pokémon Movie 16.
    PokemonMovie16,
    /// Pokémon Cartoon.
    PokemonCartoon,
    /// Space World.
    SpaceWorld,
    /// Space World 06.
    SpaceWorld06,
    /// Space World 07.
    SpaceWorld07,
    /// Space World 08.
    SpaceWorld08,
    /// Space World 09.
    SpaceWorld09,
    /// Space World 10.
    SpaceWorld10,
    /// Space World 11.
    SpaceWorld11,
    /// Space World 12.
    SpaceWorld12,
    /// Space World 13.
    SpaceWorld13,
    /// Space World 14.
    SpaceWorld14,
    /// Space World 15.
    SpaceWorld15,
    /// Space World 16.
    SpaceWorld16,
    /// Pokémon Festa.
    PokemonFesta,
    /// POKéPARK.
    PokePark,
    /// POKéPARK 06.
    PokePark06,
    /// POKéPARK 07.
    PokePark07,
    /// POKéPARK 08.
    PokePark08,
    /// POKéPARK 09.
    PokePark09,
    /// POKéPARK 10.
    PokePark10,
    /// POKéPARK 11.
    PokePark11,
    /// POKéPARK 12.
    PokePark12,
    /// POKéPARK 13.
    PokePark13,
    /// POKéPARK 14.
    PokePark14,
    /// POKéPARK 15.
    PokePark15,
    /// POKéPARK 16.
    PokePark16,
    /// Pokémon Center.
    PokemonCenter,
    /// Pokémon Center Tokyo.
    PCTokyo,
    /// Pokémon Center Osaka.
    PCOsaka,
    /// Pokémon Center Fukuoka.
    PCFukuoka,
    /// Pokémon Center Nagoya.
    PCNagoya,
    /// Pokémon Center Sapporo.
    PCSapporo,
    /// Pokémon Center Yokohama.
    PCYokohama,
    /// Nintendo World.
    NintendoWorld,
    /// Pokémon Event.
    PokemonEvent,
    /// Pokémon Event 06.
    PokemonEvent06,
    /// Pokémon Event 07.
    PokemonEvent07,
    /// Pokémon Event 08.
    PokemonEvent08,
    /// Pokémon Event 09.
    PokemonEvent09,
    /// Pokémon Event 10.
    PokemonEvent10,
    /// Pokémon Event 11.
    PokemonEvent11,
    /// Pokémon Event 12.
    PokemonEvent12,
    /// Pokémon Event 13.
    PokemonEvent13,
    /// Pokémon Event 14.
    PokemonEvent14,
    /// Pokémon Event 15.
    PokemonEvent15,
    /// Pokémon Event 16.
    PokemonEvent16,
    /// Wi-Fi Event.
    WiFiEvent,
    /// Wi-Fi Gift.
    WiFiGift,
    /// Pokémon Fan Club.
    PokemonFanClub,
    /// Event Site.
    EventSite,
    /// Concert Event.
    ConcertEvent,
}

impl Gen4Location {
    pub const DP_LAST_LOCATION: Self = Gen4Location::BattlePark;
}

// List of Gen 5 locations:
// https://bulbapedia.bulbagarden.net/wiki/List_of_locations_by_index_number_in_Generation_V
/// Enum that identifies the different Gen 5 locations.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Display, TryFromPrimitive,
)]
#[repr(u16)]
pub enum Gen5Location {
    /// ----------
    Dashes,
    /// Mystery Zone.
    MysteryZone,
    /// Faraway place.
    #[default]
    FarawayPlace,
    /// <name>'s <location> (Entralink).
    PlayerLocationEntralink,
    /// Nuvema Town.
    NuvemaTown,
    /// Accumula Town.
    AccumulaTown,
    /// Striaton City.
    StriatonCity,
    /// Nacrene City.
    NacreneCity,
    /// Castelia City.
    CasteliaCity,
    /// Nimbasa City.
    NimbasaCity,
    /// Driftveil City.
    DriftveilCity,
    /// Mistralton City.
    MistraltonCity,
    /// Icirrus City.
    IcirrusCity,
    /// Opelucid City.
    OpelucidCity,
    /// Route 1.
    Route1,
    /// Route 2.
    Route2,
    /// Route 3.
    Route3,
    /// Route 4.
    Route4,
    /// Route 5.
    Route5,
    /// Route 6.
    Route6,
    /// Route 7.
    Route7,
    /// Route 8.
    Route8,
    /// Route 9.
    Route9,
    /// Route 10.
    Route10,
    /// Route 11.
    Route11,
    /// Route 12.
    Route12,
    /// Route 13.
    Route13,
    /// Route 14.
    Route14,
    /// Route 15.
    Route15,
    /// Route 16.
    Route16,
    /// Route 17.
    Route17,
    /// Route 18.
    Route18,
    /// Dreamyard.
    Dreamyard,
    /// Pinwheel Forest.
    PinwheelForest,
    /// Desert Resort.
    DesertResort,
    /// Relic Castle.
    RelicCastle,
    /// Cold Storage (B/W) / PWT (B2/W2).
    ColdStorageOrPWT,
    /// Chargestone Cave.
    ChargestoneCave,
    /// Twist Mountain.
    TwistMountain,
    /// Dragonspiral Tower.
    DragonspiralTower,
    /// Victory Road (Black and White).
    VictoryRoadBW,
    /// Lacunosa Town.
    LacunosaTown,
    /// Undella Town.
    UndellaTown,
    /// Anville Town.
    AnvilleTown,
    /// Pokémon League.
    PokémonLeague,
    /// N's Castle.
    NsCastle,
    /// Royal Unova.
    RoyalUnova,
    /// Gear Station.
    GearStation,
    /// Battle Subway.
    BattleSubway,
    /// Musical Theater.
    MusicalTheater,
    /// Black City.
    BlackCity,
    /// White Forest.
    WhiteForest,
    /// Unity Tower.
    UnityTower,
    /// Wellspring Cave.
    WellspringCave,
    /// Mistralton Cave.
    MistraltonCave,
    /// Rumination Field.
    RuminationField,
    /// Celestial Tower.
    CelestialTower,
    /// Moor of Icirrus.
    MoorOfIcirrus,
    /// Shopping Mall.
    ShoppingMall,
    /// Challenger's Cave.
    ChallengersCave,
    /// Poké Transfer Lab.
    PokéTransferLab,
    /// Giant Chasm.
    GiantChasm,
    /// Liberty Garden.
    LibertyGarden,
    /// P2 Laboratory.
    P2Laboratory,
    /// Skyarrow Bridge.
    SkyarrowBridge,
    /// Driftveil Drawbridge.
    DriftveilDrawbridge,
    /// Tubeline Bridge.
    TubelineBridge,
    /// Village Bridge.
    VillageBridge,
    /// Marvelous Bridge.
    MarvelousBridge,
    /// Entralink.
    Entralink,
    /// Abundant Shrine.
    AbundantShrine,
    /// Undella Bay.
    UndellaBay,
    /// Lostlorn Forest.
    LostlornForest,
    /// Trial Chamber.
    TrialChamber,
    /// Guidance Chamber.
    GuidanceChamber,
    /// Entree Forest.
    EntreeForest,
    /// Nimbasa City (Entralink).
    NimbasaCityEntralink,
    /// Driftveil City (Entralink).
    DriftveilCityEntralink,
    /// Mistralton City (Entralink).
    MistraltonCityEntralink,
    /// Icirrus City (Entralink).
    IcirrusCityEntralink,
    /// Opelucid City (Entralink).
    OpelucidCityEntralink,
    /// Nuvema Town (Entralink).
    NuvemaTownEntralink,
    /// White Forest (Entralink).
    WhiteForestEntralink,
    /// Black City (Entralink).
    BlackCityEntralink,
    /// Cold Storage (Entralink).
    ColdStorageEntralink,
    /// Chargestone Cave (Entralink).
    ChargestoneCaveEntralink,
    /// Twist Mountain (Entralink).
    TwistMountainEntralink,
    /// Dragonspiral Tower (Entralink).
    DragonspiralTowerEntralink,
    /// Giant Chasm (Entralink).
    GiantChasmEntralink,
    /// Driftveil Drawbridge (Entralink).
    DriftveilDrawbridgeEntralink,
    /// Tubeline Bridge (Entralink).
    TubelineBridgeEntralink,
    /// Village Bridge (Entralink).
    VillageBridgeEntralink,
    /// Marvelous Bridge (Entralink).
    MarvelousBridgeEntralink,
    /// Route 5 (Entralink).
    Route5Entralink,
    /// Route 6 (Entralink).
    Route6Entralink,
    /// Route 7 (Entralink).
    Route7Entralink,
    /// Route 8 (Entralink).
    Route8Entralink,
    /// Route 9 (Entralink).
    Route9Entralink,
    /// Route 10 (Entralink).
    Route10Entralink,
    /// Route 11 (Entralink).
    Route11Entralink,
    /// Route 12 (Entralink).
    Route12Entralink,
    /// Route 13 (Entralink).
    Route13Entralink,
    /// Route 14 (Entralink).
    Route14Entralink,
    /// Route 15 (Entralink).
    Route15Entralink,
    /// Route 16 (Entralink).
    Route16Entralink,
    /// Lacunosa Town (Entralink).
    LacunosaTownEntralink,
    /// Undella Town (Entralink).
    UndellaTownEntralink,
    /// Accumula Gate.
    AccumulaGate,
    /// Undella Gate.
    UndellaGate,
    /// Nacrene Gate.
    NacreneGate,
    /// Castelia Gate.
    CasteliaGate,
    /// Nimbasa Gate.
    NimbasaGate,
    /// Opelucid Gate.
    OpelucidGate,
    /// Black Gate.
    BlackGate,
    /// White Gate.
    WhiteGate,
    /// Bridge Gate.
    BridgeGate,
    /// Route Gate.
    RouteGate,
    /// Abyssal Ruins.
    AbyssalRuins,
    // Black 2/White 2-introduced locations:
    /// Aspertia City.
    AspertiaCity,
    /// Virbank City.
    VirbankCity,
    /// Humilau City.
    HumilauCity,
    /// Pokéstar Studios.
    PokestarStudios,
    /// Join Avenue.
    JoinAvenue,
    /// Floccesy Town.
    FloccesyTown,
    /// Lentimas Town.
    LentimasTown,
    /// Route 19.
    Route19,
    /// Route 20.
    Route20,
    /// Route 21.
    Route21,
    /// Route 22.
    Route22,
    /// Route 23.
    Route23,
    /// Castelia Sewers.
    CasteliaSewers,
    /// Floccesy Ranch.
    FloccesyRanch,
    /// Virbank Complex.
    VirbankComplex,
    /// Reversal Mountain.
    ReversalMountain,
    /// Strange House.
    StrangeHouse,
    /// Victory Road (Black 2 and White 2).
    VictoryRoadB2W2,
    /// Plasma Frigate.
    PlasmaFrigate,
    /// Relic Passage.
    RelicPassage,
    /// Clay Tunnel.
    ClayTunnel,
    /// ----------
    DashesB2W2,
    /// White Treehollow.
    WhiteTreehollow,
    /// Black Tower.
    BlackTower,
    /// Seaside Cave.
    SeasideCave,
    /// Cave of Being.
    CaveOfBeing,
    /// Hidden Grotto.
    HiddenGrotto,
    /// Marine Tube.
    MarineTube,
    /// Virbank Gate.
    VirbankGate,
    /// Aspertia Gate.
    AspertiaGate,
    /// Nature Preserve.
    NaturePreserve,
    /// Medal Office.
    MedalOffice,
    /// Underground Ruins.
    UndergroundRuins,
    /// Rock Peak Chamber.
    RockPeakChamber,
    /// Iceberg Chamber.
    IcebergChamber,
    /// Iron Chamber.
    IronChamber,
    /// Pledge Grove.
    PledgeGrove,
    // Misc. in-game locations:
    /// Hoenn/Kanto/Sinnoh/Johto/Distant Land
    /// (the region displayed is dependent on the game of origin).
    OtherRegionDistantLand = 0x7531,
    /// In-game link trade.
    LinkTradeInGame,
    /// Egg link trade.
    LinkTradeEgg,
    /// Kanto.
    Kanto, // Unused.
    /// Johto.
    Johto, // Unused.
    /// Hoenn.
    Hoenn, // Only used for some event Groudon and Kyogre.
    /// Sinnoh.
    Sinnoh, // Unused.
    /// Distant Land.
    DistantLand, // Unused.
    /// A special place (event Celebi, before triggering the Zorua event).
    CelebiSpecialPlace,
    /// N/A (event Celebi, after triggering the Zorua event).
    CelebiFatefulEncounter,
    /// A special place (event Legendary Beasts, before triggering the Zororark event).
    BeastsSpecialPlace,
    /// N/A (event Legendary Beasts, after triggering the Zororark event).
    BeastsFatefulEncounter,
    /// Entralink.
    Entralink2,
    /// Pokémon Dream Radar.
    PokemonDreamRadar, // B2/W2 exclusive.
    // Misc. event locations:
    /// Lovely place.
    LovelyPlace,
    /// Faraway place.
    FarawayPlaceEvent,
    /// Pokémon Movie.
    PokemonMovie,
    /// Pokémon Movie 10.
    PokemonMovie10,
    /// Pokémon Movie 11.
    PokemonMovie11,
    /// Pokémon Movie 12.
    PokemonMovie12,
    /// Pokémon Movie 13.
    PokemonMovie13,
    /// Pokémon Movie 14.
    PokemonMovie14,
    /// Pokémon Movie 15.
    PokemonMovie15,
    /// Pokémon Movie 16.
    PokemonMovie16,
    /// Pokémon Movie 17.
    PokemonMovie17,
    /// Pokémon Movie 18.
    PokemonMovie18,
    /// Pokémon Movie 19.
    PokemonMovie19,
    /// Pokémon Movie 20.
    PokemonMovie21,
    /// Pokémon Center.
    PokemonCenter,
    /// Pokémon Cartoon.
    PokemonCartoon,
    /// Pokémon Center Tokyo.
    PCTokyo,
    /// Pokémon Center Osaka.
    PCOsaka,
    /// Pokémon Center Fukuoka.
    PCFukuoka,
    /// Pokémon Center Nagoya.
    PCNagoya,
    /// Pokémon Center Sapporo.
    PCSapporo,
    /// Pokémon Center Yokohama.
    PCYokohama,
    /// World Championships.
    WCS,
    /// World Championships 2010.
    WCS2010,
    /// World Championships 2011.
    WCS2011,
    /// World Championships 2012.
    WCS2012,
    /// World Championships 2013.
    WCS2013,
    /// World Championships 2014.
    WCS2014,
    /// World Championships 2015.
    WCS2015,
    /// World Championships 2016.
    WCS2016,
    /// World Championships 2017.
    WCS2017,
    /// World Championships 2018.
    WCS2018,
    /// World Championships 2019.
    WCS2019,
    /// World Championships 2020.
    WCS2020,
    /// Worlds.
    Worlds,
    /// Worlds 2010.
    Worlds2010,
    /// Worlds 2011.
    Worlds2011,
    /// Worlds 2012.
    Worlds2012,
    /// Worlds 2013.
    Worlds2013,
    /// Worlds 2014.
    Worlds2014,
    /// Worlds 2015.
    Worlds2015,
    /// Worlds 2016.
    Worlds2016,
    /// Worlds 2017.
    Worlds2017,
    /// Worlds 2018.
    Worlds2018,
    /// Worlds 2019.
    Worlds2019,
    /// Worlds 2020.
    Worlds2020,
    /// VGE.
    VGE,
    /// VGE 2010.
    VGE2010,
    /// VGE 2011.
    VGE2011,
    /// VGE 2012.
    VGE2012,
    /// VGE 2013.
    VGE2013,
    /// VGE 2014.
    VGE2014,
    /// VGE 2015.
    VGE2015,
    /// VGE 2016.
    VGE2016,
    /// VGE 2017.
    VGE2017,
    /// VGE 2018.
    VGE2018,
    /// VGE 2019.
    VGE2019,
    /// VGE 2020.
    VGE2020,
    /// Pokémon Event.
    PokemonEvent,
    /// Battle Competition.
    BattleCompetition,
    /// Game Event.
    GameEvent,
    /// Pokémon Fan Club.
    PokemonFanClub,
    /// Pokémon Sunday.
    PokemonSunday,
    /// Challenge Plaza.
    ChallengePlaza,
    /// Battle Challenge.
    BattleChallenge,
    /// Trainers' Court.
    TrainersCourt,
    /// Battle Tour.
    BattleTour,
    /// Concert.
    Concert,
    /// Wi-Fi Gift.
    WiFiGift,
    /// Pokémon Ranger.
    PokemonRanger,
    /// Ranger Union.
    RangerUnion,
    /// PBR.
    PBR,
    /// Pokémon Dream World.
    PokemonDW,
    /// Pokémon Event 10.
    PokemonEvent10,
    /// Pokémon Event 11.
    PokemonEvent11,
    /// Pokémon Event 12.
    PokemonEvent12,
    /// Pokémon Event 13.
    PokemonEvent13,
    /// Pokémon Event 14.
    PokemonEvent14,
    /// Pokémon Event 15.
    PokemonEvent15,
    /// Pokémon Event 16.
    PokemonEvent16,
    /// Pokémon Event 17.
    PokemonEvent17,
    /// Pokémon Event 18.
    PokemonEvent18,
    /// Pokémon Event 19.
    PokemonEvent19,
    /// Pokémon Event 20.
    PokemonEvent20,
    /// Pokémon Festa.
    PokemonFesta,
    /// Pokémon Festa 10.
    PokemonFesta10,
    /// Pokémon Festa 11.
    PokemonFesta11,
    /// Pokémon Festa 12.
    PokemonFesta12,
    /// Pokémon Festa 13.
    PokemonFesta13,
    /// Pokémon Festa 14.
    PokemonFesta14,
    /// Pokémon Festa 15.
    PokemonFesta15,
    /// Pokémon Festa 16.
    PokemonFesta16,
    /// Pokémon Festa 17.
    PokemonFesta17,
    /// Pokémon Festa 18.
    PokemonFesta18,
    /// Pokémon Festa 19.
    PokemonFesta19,
    /// Pokémon Festa 20.
    PokemonFesta20,
    /// POKÉPARK.
    PokePark,
    /// POKÉPARK 10.
    PokePark10,
    /// POKÉPARK 11.
    PokePark11,
    /// POKÉPARK 12.
    PokePark12,
    /// POKÉPARK 13.
    PokePark13,
    /// POKÉPARK 14.
    PokePark14,
    /// POKÉPARK 15.
    PokePark15,
    /// POKÉPARK 16.
    PokePark16,
    /// POKÉPARK 17.
    PokePark17,
    /// POKÉPARK 18.
    PokePark18,
    /// POKÉPARK 19.
    PokePark19,
    /// POKÉPARK 20.
    PokePark20,
    /// Event Site.
    EventSite,
    /// Stranger.
    Stranger = 0xEA61,
    /// Day-Care Couple.
    DayCareCouple,
    /// Treasure Hunter (B/W) / Pokémon Breeder (B2/W2).
    TreasureHunterOrPKMNBreeder,
}

// Aliases for reused IDs across gen 5 locations:
#[allow(non_upper_case_globals)]
impl Gen5Location {
    pub const ColdStorage: Self = Self::ColdStorageOrPWT;
    pub const PWT: Self = Self::ColdStorageOrPWT;
    pub const TreasureHunter: Self = Self::TreasureHunterOrPKMNBreeder;
    pub const PKMNBreeder: Self = Self::TreasureHunterOrPKMNBreeder;
}

/// Enum that identifies the different locations in the Gen 4 and Gen 5 games.
#[derive(Clone, Copy, Debug)]
pub enum Location {
    /// A location in a Gen 4 game.
    Gen4(Gen4Location),
    /// A location in a Gen 5 game.
    Gen5(Gen5Location),
}

impl Default for Location {
    fn default() -> Self {
        Location::Gen4(Gen4Location::default())
    }
}

impl std::fmt::Display for Location {
    /// Formats the `Location` as a printed string.
    ///
    /// This function will return the `String` representation of the specific location
    /// variant, either from `Gen4Location` or `Gen5Location`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Gen4(loc) => write!(f, "{}", loc),
            Location::Gen5(loc) => write!(f, "{}", loc),
        }
    }
}

impl Into<u16> for Location {
    /// Converts the `Location` into a `u16` representation.
    ///
    /// This function will return the `u16` value corresponding to the specific location
    /// variant, either from `Gen4Location` or `Gen5Location`.
    fn into(self) -> u16 {
        match self {
            Location::Gen4(loc) => loc as u16,
            Location::Gen5(loc) => loc as u16,
        }
    }
}

/// Enum identifying the different languages in the Gen 4 and Gen 5 games.
#[derive(Clone, Copy, Debug, Default, Display, TryFromPrimitive)]
#[repr(u8)]
pub enum Language {
    /// Japanese.
    Japanese = 0x01,
    /// English.
    #[default]
    English,
    /// French.
    French,
    /// Italian.
    Italian,
    /// German.
    German,
    /// Spanish.
    Spanish = 0x07,
    /// Korean.
    Korean,
}

/// Enum identifying the Trainer Class / sprite of GTS deposits in Generation 5.
#[derive(Clone, Copy, Debug, Default, Display, TryFromPrimitive)]
#[repr(u8)]
pub enum TrainerClass {
    /// Youngster.
    #[default]
    Youngster,
    /// Ace Trainer (male).
    MaleAceTrainer,
    /// Pokémon Ranger (male).
    MalePokemonRanger,
    /// Pokémon Breeder (male).
    MalePokemonBreeder,
    /// Researcher (male).
    MaleResearcher,
    /// Hiker.
    Hiker,
    /// Skinhead.
    Skinhead,
    /// Day-Care student (male).
    MaleDayCareStudent,
    /// Lass.
    Lass,
    /// Ace Trainer (female).
    FemaleAceTrainer,
    /// Pokémon Ranger (female).
    FemalePokemonRanger,
    /// Pokémon Breeder (female).
    FemalePokemonBreeder,
    /// Researcher (female).
    FemaleResearcher,
    /// Parasol Lady.
    ParasolLady,
    /// Nurse.
    Nurse,
    /// Day-Care student (female).
    FemaleDayCareStudent,
}

impl TrainerClass {
    pub const COUNT: u8 = (Self::FemaleDayCareStudent as u8 + 1);
}
