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
use std::{
    convert::{TryFrom, TryInto},
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Error, ErrorKind, Result},
};

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
#[derive(Clone, Debug, Default)]
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

impl TryFrom<usize> for Stat {
    type Error = Error;

    /// Converts a `usize` value to a `Stat`.
    ///
    /// The conversion is the following:
    /// * `0` - `Stat::Hp`
    /// * `1` - `Stat::Atk`
    /// * `2` - `Stat::Def`
    /// * `3` - `Stat::SpA`
    /// * `4` - `Stat::SpD`
    /// * `5` - `Stat::Spe`
    /// * Any other value - `Err(Error::InvalidData)`
    fn try_from(value: usize) -> Result<Self> {
        match value {
            0 => Ok(Stat::Hp),
            1 => Ok(Stat::Atk),
            2 => Ok(Stat::Def),
            3 => Ok(Stat::SpA),
            4 => Ok(Stat::SpD),
            5 => Ok(Stat::Spe),

            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid stat value")),
        }
    }
}

impl TryFrom<u8> for Stat {
    type Error = Error;

    /// Converts a `u8` value to a `Stat`.
    ///
    /// The conversion is the following:
    /// * `0` - `Stat::Hp`
    /// * `1` - `Stat::Atk`
    /// * `2` - `Stat::Def`
    /// * `3` - `Stat::SpA`
    /// * `4` - `Stat::SpD`
    /// * `5` - `Stat::Spe`
    /// * Any other value - `Err(Error::InvalidData)`
    ///
    /// Internally, this function just calls `Stat::try_from::<usize>` with the value cast to
    /// `usize` type.
    fn try_from(value: u8) -> Result<Self> {
        Stat::try_from(value as usize)
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
pub enum ContestStat {
    /// Cool contest stat.
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

impl TryFrom<usize> for ContestStat {
    type Error = Error;

    /// Converts a `usize` value to a `ContestStat`.
    ///
    /// The conversion is the following:
    /// * `0` - `ContestStat::Cool`
    /// * `1` - `ContestStat::Beauty`
    /// * `2` - `ContestStat::Cute`
    /// * `3` - `ContestStat::Smart`
    /// * `4` - `ContestStat::Tough`
    /// * `5` - `ContestStat::Sheen`
    /// * Any other value - `Err(Error::InvalidData)`
    fn try_from(value: usize) -> Result<Self> {
        match value {
            0 => Ok(ContestStat::Cool),
            1 => Ok(ContestStat::Beauty),
            2 => Ok(ContestStat::Cute),
            3 => Ok(ContestStat::Smart),
            4 => Ok(ContestStat::Tough),
            5 => Ok(ContestStat::Sheen),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid contest stat value",
            )),
        }
    }
}

impl TryFrom<u8> for ContestStat {
    type Error = Error;

    /// Converts a `u8` value to a `ContestStat`.
    ///
    /// The conversion is the following:
    /// * `0` - `ContestStat::Cool`
    /// * `1` - `ContestStat::Beauty`
    /// * `2` - `ContestStat::Cute`
    /// * `3` - `ContestStat::Smart`
    /// * `4` - `ContestStat::Tough`
    /// * `5` - `ContestStat::Sheen`
    /// * Any other value - `Err(Error::InvalidData)`
    ///
    /// Internally, this function just calls `ContestStat::try_from::<usize>` with the value cast
    /// to `usize`.
    fn try_from(value: u8) -> Result<Self> {
        ContestStat::try_from(value as usize)
    }
}

impl TryInto<usize> for ContestStat {
    type Error = Error;

    /// Converts a `ContestStat` to a `usize` value.
    ///
    /// The conversion is the following:
    /// * `ContestStat::Cool` - `0`
    /// * `ContestStat::Beauty` - `1`
    /// * `ContestStat::Cute` - `2`
    /// * `ContestStat::Smart` - `3`
    /// * `ContestStat::Tough` - `4`
    /// * `ContestStat::Sheen` - `5`
    fn try_into(self) -> Result<usize> {
        match self {
            ContestStat::Cool => Ok(0),
            ContestStat::Beauty => Ok(1),
            ContestStat::Cute => Ok(2),
            ContestStat::Smart => Ok(3),
            ContestStat::Tough => Ok(4),
            ContestStat::Sheen => Ok(5),
        }
    }
}

/// Enum the represent the different Pokémon and trainer genders.
///
/// A trainer cannot be genderless.
#[derive(Clone, Copy, Debug, Default)]
pub enum Gender {
    /// Male gender.
    Male,
    /// Female gender.
    Female,
    /// Genderless, only for certain special Pokémon.
    #[default]
    Genderless,
}

impl TryFrom<usize> for Gender {
    type Error = Error;

    /// Converts a `usize` value to a `Gender`.
    ///
    /// The conversion is the following:
    /// * `0x0` - `Gender::Male`
    /// * `0x1` - `Gender::Female`
    /// * `0x2` - `Gender::Genderless`
    /// * Any other value - `Err(Error::InvalidData)`
    fn try_from(value: usize) -> Result<Self> {
        match value {
            0x0 => Ok(Gender::Male),
            0x1 => Ok(Gender::Female),
            0x2 => Ok(Gender::Genderless),

            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid stat value")),
        }
    }
}

impl TryFrom<u8> for Gender {
    type Error = Error;

    /// Converts a `usize` value to a `Gender`.
    ///
    /// The conversion is the following:
    /// * `0x0` - `Gender::Male`
    /// * `0x1` - `Gender::Female`
    /// * `0x2` - `Gender::Genderless`
    /// * Any other value - `Err(Error::InvalidData)`
    ///
    /// Internally, this function just calls `Gender::try_from::<usize>` with the value cast to
    fn try_from(value: u8) -> Result<Self> {
        Self::try_from(value as usize)
    }
}

impl Display for Gender {
    /// Converts a `Gender` to a printed string of characters.
    ///
    /// The conversion is the following:
    /// * `Gender::Male` - `"Male"`
    /// * `Gender::Female` - `"Female"`
    /// * `Gender::Genderless` - `"Genderless"`
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let name = match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::Genderless => "Genderless",
        };

        write!(f, "{name}")
    }
}

/// Enum that identifies the different shiny leaves a Pokémon can have in HeartGold and SoulSilver.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

impl TryFrom<usize> for Game {
    type Error = Error;

    /// Converts a `usize` value to a `Game`.
    ///
    /// The conversion is the following:
    /// * `1` - `Game::Sapphire`
    /// * `2` - `Game::Ruby`
    /// * `3` - `Game::Emerald`
    /// * `4` - `Game::FireRed`
    /// * `5` - `Game::LeafGreen`
    /// * `7` - `Game::HeartGold`
    /// * `8` - `Game::SoulSilver`
    /// * `10` - `Game::Diamond`
    /// * `11` - `Game::Pearl`
    /// * `12` - `Game::Platinum`
    /// * `15` - `Game::ColosseumXD`
    /// * `20` - `Game::White`
    /// * `21` - `Game::Black`
    /// * `22` - `Game::White2`
    /// * `23` - `Game::Black2`
    ///
    /// Any other value will return an error.
    fn try_from(value: usize) -> Result<Self> {
        match value {
            1 => Ok(Game::Sapphire),
            2 => Ok(Game::Ruby),
            3 => Ok(Game::Emerald),
            4 => Ok(Game::FireRed),
            5 => Ok(Game::LeafGreen),
            7 => Ok(Game::HeartGold),
            8 => Ok(Game::SoulSilver),
            10 => Ok(Game::Diamond),
            11 => Ok(Game::Pearl),
            12 => Ok(Game::Platinum),
            15 => Ok(Game::ColosseumXD),
            20 => Ok(Game::White),
            21 => Ok(Game::Black),
            22 => Ok(Game::White2),
            23 => Ok(Game::Black2),

            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid game value")),
        }
    }
}

impl TryFrom<u8> for Game {
    type Error = Error;

    /// Converts a `u8` value to a `Game`.
    ///
    /// The conversion is the following:
    /// * `1` - `Game::Sapphire`
    /// * `2` - `Game::Ruby`
    /// * `3` - `Game::Emerald`
    /// * `4` - `Game::FireRed`
    /// * `5` - `Game::LeafGreen`
    /// * `7` - `Game::HeartGold`
    /// * `8` - `Game::SoulSilver`
    /// * `10` - `Game::Diamond`
    /// * `11` - `Game::Pearl`
    /// * `12` - `Game::Platinum`
    /// * `15` - `Game::ColosseumXD`
    /// * `20` - `Game::White`
    /// * `21` - `Game::Black`
    /// * `22` - `Game::White2`
    /// * `23` - `Game::Black2`
    ///
    /// Any other value will return an error.
    ///
    /// Internally, this function just calls `Game::try_from::<usize>` with the value cast to
    /// `usize`.
    fn try_from(value: u8) -> Result<Self> {
        Self::try_from(value as usize)
    }
}

impl TryFrom<&String> for Game {
    type Error = Error;

    /// Converts a `String` value to a `Game`.
    ///
    /// The conversion is the following:
    /// * `"Sapphire"` - `Game::Sapphire`
    /// * `"Ruby"` - `Game::Ruby`
    /// * `"Emerald"` - `Game::Emerald`
    /// * `"FireRed"` - `Game::FireRed`
    /// * `"LeafGreen"` - `Game::LeafGreen`
    /// * `"HeartGold"` - `Game::HeartGold`
    /// * `"SoulSilver"` - `Game::SoulSilver`
    /// * `"Diamond"` - `Game::Diamond`
    /// * `"Pearl"` - `Game::Pearl`
    /// * `"Platinum"` - `Game::Platinum`
    /// * `"ColosseumXD"` - `Game::ColosseumXD`
    /// * `"White"` - `Game::White`
    /// * `"Black"` - `Game::Black`
    /// * `"White2"` - `Game::White2`
    /// * `"Black2"` - `Game::Black2`
    ///
    /// Any other value will return an error.
    fn try_from(value: &String) -> Result<Self> {
        match value.as_str() {
            "Sapphire" => Ok(Game::Sapphire),
            "Ruby" => Ok(Game::Ruby),
            "Emerald" => Ok(Game::Emerald),
            "FireRed" => Ok(Game::FireRed),
            "LeafGreen" => Ok(Game::LeafGreen),
            "HeartGold" => Ok(Game::HeartGold),
            "SoulSilver" => Ok(Game::SoulSilver),
            "Diamond" => Ok(Game::Diamond),
            "Pearl" => Ok(Game::Pearl),
            "Platinum" => Ok(Game::Platinum),
            "ColosseumXD" => Ok(Game::ColosseumXD),
            "White" => Ok(Game::White),
            "Black" => Ok(Game::Black),
            "White 2" => Ok(Game::White2),
            "Black 2" => Ok(Game::Black2),

            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid game name")),
        }
    }
}

impl Display for Game {
    /// Converts a `Game` to a printed string of characters.
    ///
    /// The conversion is the following:
    /// * `Game::Sapphire` - `"Sapphire"`
    /// * `Game::Ruby` - `"Ruby"`
    /// * `Game::Emerald` - `"Emerald"`
    /// * `Game::FireRed` - `"FireRed"`
    /// * `Game::LeafGreen` - `"LeafGreen"`
    /// * `Game::HeartGold` - `"HeartGold"`
    /// * `Game::SoulSilver` - `"SoulSilver"`
    /// * `Game::Diamond` - `"Diamond"`
    /// * `Game::Pearl` - `"Pearl"`
    /// * `Game::Platinum` - `"Platinum"`
    /// * `Game::ColosseumXD` - `"ColosseumXD"`
    /// * `Game::White` - `"White"`
    /// * `Game::Black` - `"Black"`
    /// * `Game::White2` - `"White 2"`
    /// * `Game::Black2` - `"Black 2"`
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let name = match self {
            Game::Sapphire => "Sapphire",
            Game::Ruby => "Ruby",
            Game::Emerald => "Emerald",
            Game::FireRed => "FireRed",
            Game::LeafGreen => "LeafGreen",
            Game::HeartGold => "HeartGold",
            Game::SoulSilver => "SoulSilver",
            Game::Diamond => "Diamond",
            Game::Pearl => "Pearl",
            Game::Platinum => "Platinum",
            Game::ColosseumXD => "ColosseumXD",
            Game::White => "White",
            Game::Black => "Black",
            Game::White2 => "White 2",
            Game::Black2 => "Black 2",
        };

        write!(f, "{name}")
    }
}
