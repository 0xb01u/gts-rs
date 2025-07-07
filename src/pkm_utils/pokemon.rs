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
use chrono::{Datelike, Local as LocalTime, NaiveDate};
use getset::{CopyGetters, Getters};
use std::{
    collections::HashSet,
    convert::{TryFrom, TryInto},
    fs::{self, File},
    io::{Error, ErrorKind, Result, Write},
    path::{Path, PathBuf},
};

use crate::{data_maps::*, internal_types::*, should_be_ok, should_be_some, should_not_happen};

// Games' internal representation constats:
const BOXED_PKM_LEN: usize = 0x88;
const GEN4_PKM_LEN: usize = 0xEC;
const GEN5_PKM_LEN: usize = 0xDC;

// Gen 4 Pokémon structure documentation: https://projectpokemon.org/docs/gen-4/pkm-structure-r65/
// Gen 5 Pokémon structure documentation: https://projectpokemon.org/home/docs/gen-5/bw-save-structure-r60/
/// Structure representing a Generation 4 or Generation 5 Pokémon.
///
/// Some of the fields are abstracted from their internal raw byte representation, using
/// higher-level data structures. For example, the Pokémon or Trainer's names (fields `name` and
/// `trainer_name`), which are stored as `String`.
///
/// Other fields, however, are kept stored as their internal raw byte representation, and thus are
/// devoid of their semantical meaning. For example, the Pokémon form (`form_id`), which is of
/// `u8` type. The reason behind this is just convenience for this application.
///
/// All fields which can be safely modified have public visibility. Other fields, whose
/// modification would alter other data in the Pokémon, are accessible through getters and setters.
#[rustfmt::skip]
#[derive(Clone, Default, Debug, Getters, CopyGetters)]
pub struct Pokemon {
    // Re-ordered these first two fields so that they are printed first when debug-printing the
    // Pokémon.
    pub species: IdFeature,                 // 0x08 - 0x09
    #[get = "pub"]
    name: String,                           // 0x48 - 0x5D, 0x5E = null terminator(?)

    #[get_copy = "pub"]
    pid: u32,                               // 0x00 - 0x03
    #[get = "pub"]
    nature: Nature,                         // Derived from PID.
    pub encryption_bypass: bool,            // 0x04
    pub bad_egg_flag: bool,                 // 0x04
    original_checksum: u16,                 // 0x06 - 0x07
    pub held_item: IdFeature,               // 0x0A - 0x0B
    pub trainer_id: u16,                    // 0x0C - 0x0D
    pub trainer_secret_id: u16,             // 0x0E - 0x0F
    #[get_copy = "pub"]
    experience: u32,                        // 0x10 - 0x13
    #[get_copy = "pub"]
    level: u8,                              // 0x8C
    pub friendship: u8,                     // 0x14
    pub ability: IdFeature,                 // 0x15
    pub markings: u8,                       // 0x16
    pub language: Language,                 // 0x17
    pub evs: StatsFeature,                  // 0x18 - 0x1D
    pub contest_stats: ContestStatsFeature, // 0x1E - 0x23
    pub sinnoh_ribbons: [u8; 8],            // 0x24 - 0x28, and 0x60 - 0x64
    pub moves: [IdFeature; 4],              // 0x28 - 0x2F
    pub move_pps: [u8; 4],                  // 0x30 - 0x33
    pub move_pp_ups: [u8; 4],               // 0x34 - 0x37
    pub ivs: StatsFeature,                  // 0x38 - 0x3C
    pub is_egg: bool,                       // 0x3B, bit 6
    pub is_nicknamed: bool,                 // 0x3B, bit 7
    pub hoenn_ribbons: [u8; 4],             // 0x3C - 0x3F
    pub fateful: bool,                      // 0x40, bit 0
    pub gender: Gender,                     // 0x40, bits 1-2
    pub form_id: u8,                        // 0x40, bits 3-6
    pub shiny_leaves: HashSet<ShinyLeaf>,   // 0x41, only in gen 4 (HGSS)
    pub egg_location: Location,             // 0x44 - 0x45 (Plat); 0x7E - 0x7F (DP)
    pub met_location: Location,             // 0x46 - 0x47 (Plat); 0x80 - 0x81 (DP)
    pub origin_game: Game,                  // 0x5F
    #[get = "pub"]
    trainer_name: String,                   // 0x68 - 0x77
    pub egg_date: Option<NaiveDate>,        // 0x78 - 0x7A
    pub met_date: NaiveDate,                // 0x7B - 0x7D
    pub pokerus: u8,                        // 0x82
    pub ball: Pokeball,                     // 0x83 (redundant in 0x86 in HGSS)
    pub met_level: u8,                      // 0x84, bits 0-6
    pub trainer_gender: Gender,             // 0x84, bit 7
    pub encounter_type: u8,                 // 0x85
    pub performance: u8,                    // 0x87
    // Pokémon in boxes have data up until here (0x88).

    pub stats: Option<StatsFeature>, // 0x90 - 0x9E

    // There are other fields that are used in game but are irrelevant for Pokémon transference,
    // including GTS services.
    // These include:
    //  * Max HP (0x8E - 0x8F)
    //  * Status conditions (0x88)
    //  * Capsule/seal indeces (0x8D)
    //  * [Gen 4] Seal coordinates (0xD4 - 0xEB)
    //  * Mail message + OT Name (0x9C - 0xD3)
    //  * Unknown fields and flags (0x89, 0x8A - 0x8B)
    //  * [Gen 5] More unknown fields (0xD4 - 0xDB)

    /// Meta-data storing whether this Pokémon is shiny, for printing purposes.
    is_shiny: bool,
    /// Meta-data storing whether this Pokémon is of Generation 5, for convenience reasons.
    #[get_copy = "pub"]
    is_gen5: bool,
}

impl Pokemon {
    /// Sets the PID of the Pokémon.
    ///
    /// The PID determines the Pokémon's nature. Therefore, it cannot be set directly.
    ///
    /// **This function modifies the Pokémon's nature.** See `self.nature()`.
    pub fn set_pid(&mut self, pid: u32) {
        // Set the PID:
        self.pid = pid;
        // Update the nature based on the new PID:
        self.nature = should_be_some!(
            Nature::from_id((pid % 25) as u16),
            "Invalid nature derived from PID: {}",
            pid
        );

        // Update shininess status:
        self.is_shiny = self.is_shiny();
    }

    /// Sets the nature for the Pokémon.
    ///
    /// The nature is determined by the Pokémon's PID. Therefore, it cannot be set directly.
    ///
    /// **This function modifies the Pokémon's PID.** See `self.pid()`.
    pub fn set_nature(&mut self, nature: Nature) {
        // Set the nature:
        self.nature = nature;

        // Update the PID to reflect the new nature:
        self.pid = self.pid.wrapping_sub(self.nature.id_and_name.id() as u32);

        // Update shininess status:
        self.is_shiny = self.is_shiny();
    }

    /// Sets the experience points of the Pokémon.
    ///
    /// The experience points are used to calculate the Pokémon's level. Therefore, it cannot be
    /// set directly.
    ///
    /// **This function modifies the Pokémon's level.** See `self.level()`.
    pub fn set_experience(&mut self, experience: u32) {
        // Set the experience:
        self.experience = experience;
        // Update the level based on the new experience:
        self.level = self.level_from_xp();
    }

    /// Calculates the level of the Pokémon from its experience points.
    ///
    /// The level might not be stored in the Pokémon data, so it is calculated from its current
    /// experience points. See `self.experience`.
    fn level_from_xp(&self) -> u8 {
        // Retrieve the species, experience type, and current experience:
        let pkm_id = self.species.id() as usize;
        let exp_type = should_be_some!(BASE_STATS.get(pkm_id), "Invalid species ID")[0];
        let exp = self.experience;

        // Iteratively check what level corresponds to the current experience:
        for i in 1..100 {
            let xp_needed = LEVEL_CURVES[i][exp_type as usize];
            if xp_needed > exp {
                return i as u8;
            }
        }

        100
    }

    /// Sets the name of the Pokémon.
    ///
    /// Pokemon names have to be limited to 10 characters.
    ///
    /// # Arguments
    /// * `name` - The name to set for the Pokémon.
    ///
    /// Returns an error of kind `InvalidData` if the name is longer than 10 characters, or if it
    /// contains characters not representable in the Gen 4 character encoding.
    pub fn set_name(&mut self, name: String) -> Result<()> {
        // Ensure the name is not longer than 10 characters:
        if name.len() > 10 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Pokémon names cannot be longer than 10 characters",
            ));
        }

        // Assert the name does not contain invalid characters by encoding it:
        let _ = Self::encode_name_gen4(&name)?;

        // Set the name:
        self.name = name;

        Ok(())
    }

    /// Returns the sequence of bytes corresponding to the internal Gen 4 representation of a name,
    /// be it the Pokémon's name, or the Trainer's name.
    ///
    /// The Gen 4 uses a custom character encoding for names.
    ///
    /// # Arguments
    /// * `name` - The name to encode.
    ///
    /// Returns an error of kind `InvalidData` if the name contains characters not representable in
    /// the Gen 4 character encoding.
    pub fn encode_name_gen4(name: &String) -> Result<Vec<u8>> {
        // Create a vector to hold the encoded name:
        let mut encoded_name = Vec::with_capacity(20);

        // Encode each character in the name using the charmap:
        for c in name.chars() {
            let &chr = CHARMAP
                .get_by_right(&c)
                .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid character in name"))?;
            encoded_name.extend(chr.to_le_bytes());
        }
        encoded_name.push(0xFF);
        encoded_name.push(0xFF);

        Ok(encoded_name)
    }

    /// Returns the String corresponding to the internal Gen 4 representation of a name,
    /// be it the Pokémon's name, or the Trainer's name.
    ///
    /// The Gen 4 uses a custom character encoding for names.
    pub fn decode_name_gen4(name: &[u8]) -> Result<String> {
        // Create a String to hold the decoded name:
        let mut decoded_name = String::with_capacity(10);

        // Decode each character in the name using the charmap:
        for chunk in name.chunks(2) {
            // Get the code for the chatacter:
            let char_code = u16::from_le_bytes([chunk[0], chunk[1]]);
            // Check for the terminator:
            if char_code == 0xffff {
                return Ok(decoded_name);
            }
            // Convert the chunk to a char:
            let chr = CHARMAP.get_by_left(&char_code);
            // If the character is not found, return an error; else, add to the decoded string:
            match chr {
                Some(&chr) => decoded_name.push(chr),
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Invalid character in name",
                    ))
                }
            };
        }

        Ok(decoded_name)
    }

    /// Returns the sequence of bytes corresponding to the internal Gen 5 representation of a name,
    /// be it the Pokémon's name, or the Trainer's name.
    ///
    /// The Gen 5 uses UTF-16 encoding for names, with 0xffff as null terminator.
    ///
    /// # Arguments
    /// * `name` - The name to encode.
    pub fn encode_name_gen5(name: &String) -> Vec<u8> {
        // Encode characters to UTF-16:
        let mut encoded_name = name
            .encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect::<Vec<u8>>();

        // Append the null terminator (0xffff):
        encoded_name.extend([0xff, 0xff]);

        encoded_name
    }

    /// Returns the String corresponding to the internal Gen 5 representation of a name,
    /// be it the Pokémon's name, or the Trainer's name.
    ///
    /// The Gen 5 uses UTF-16 encoding for names, with 0xffff as null terminator.
    ///
    /// # Arguments
    /// * `name` - The name to decode.
    ///
    /// Returns an error of kind `InvalidData` if the name is not encodeable in UTF-16, or if it
    /// does not end in the null terminator.
    pub fn decode_name_gen5(name: &[u8]) -> Result<String> {
        // Transform bytes to UTF-16 characters and try to decode:
        let byte_chars = name
            .chunks(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect::<Vec<u16>>();
        let decoding_result = String::from_utf16(&byte_chars);

        // Return an error if decoding fails:
        if let Err(e) = decoding_result {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid UTF-16 encoding: {}", e),
            ));
        }

        // Unwrap the decoded string:
        let decoded_name = should_be_ok!(decoding_result, "Invalid UTF-16 encoding");

        // Check the name ends with a null terminator (0xffff):
        let split = decoded_name.split("\u{ffff}").collect::<Vec<&str>>();
        if split.len() < 2 || split[0].is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Gen 5 name does not end with null terminator",
            ));
        }

        Ok(split[0].to_string())
    }

    /// Gets whether the Pokémon is shiny.
    ///
    /// Shininess is dependent on the Pokémon's PID, Trainer ID, and Trainer Secret ID.
    pub fn is_shiny(&self) -> bool {
        // Get the IDs:
        let pid = self.pid();
        let tid = self.trainer_id;
        let sid = self.trainer_secret_id;

        // Compute the shiny formula:
        let pid_high = (pid >> 16) as u16;
        let pid_low = (pid & 0xFFFF) as u16;
        let tid_xor = tid ^ sid;
        let pid_xor = pid_high ^ pid_low;
        tid_xor ^ pid_xor < 8
    }

    /// Gets the hidden power type and power of the Pokémon.
    ///
    /// The hidden power is determined by the Pokémon's IVs.
    pub fn get_hidden_power(&self) -> (String, u8) {
        // Declare and initialize result variables.
        let mut type_ = 0;
        let mut power = 0;

        // Compute type and power from IVs:
        let ivs = self.ivs;
        for (i, iv) in [
            Stat::Hp,
            Stat::Atk,
            Stat::Def,
            Stat::Spe,
            Stat::SpA,
            Stat::SpD,
        ]
        .iter()
        .enumerate()
        {
            type_ += (ivs.get(iv) % 2) * (1 << i);
            if 2 <= ivs.get(iv) % 4 {
                power += 1 << i;
            }
        }
        type_ = type_ * 15 / 63; // Scale to 0-15.
        power = power * 40 / 63 + 30; // Scale to 30-70.

        // Get the hidden power type String, and return result tuple:
        (
            should_be_some!(
                HIDDEN_POWERS.get(type_ as usize),
                "Invalid hidden power index: {}",
                type_
            )
            .clone(),
            power as u8,
        )
    }

    /// Returns whether this Pokémon's data corresponds to that of a Pokémon stored in a PC box (a
    /// "boxed" Pokémon).
    ///
    /// In Gen 4 and Gen 5 games, Pokémon placed in boxes internally do not store data for their
    /// stats (HP, Attack, Defene, Special Attack, Sepcial Defense, and Speed). Those are calculated
    /// when the Pokémon is put into the party.
    ///
    /// In other words, this function returns whether the Pokémon has data for its stats.  That is
    /// equal to checking whether the `stats` field is `None`.
    ///
    /// Pokémon that are not "boxed" might be called "party" Pokémon.
    pub fn is_boxed(&self) -> bool {
        self.stats.is_none()
    }

    /// Generates the stats for the Pokémon, considering its species, level, IVs, EVs, and
    /// nature.
    ///
    /// The stats are the Pokémon's HP, Attack, Defense, Special Attack, Special Defense, and
    /// Speed.
    ///
    /// This is useful for converting boxed Pokémon into party Pokémon.
    fn generate_stats(&self) -> StatsFeature {
        // Get the base stats for the species:
        let id = self.species.id() as usize;
        let base_stats = should_be_some!(
            BASE_STATS.get(id as usize),
            "Invalid species ID: {}",
            self.species.id()
        );

        // Get other variables used to calculate the stats:
        let lv = self.level;
        let ivs = self.ivs;
        let evs = self.evs;

        // Calculate the stats (disregarding the nature for now):
        let hp = (ivs.hp + 2 * (base_stats[0] as u16) + evs.hp / 4) * (lv as u16) / 100 + 10;
        let atk = (ivs.atk + 2 * (base_stats[1] as u16) + evs.atk) / 4 * (lv as u16) / 100 + 5;
        let def = (ivs.def + 2 * (base_stats[2] as u16) + evs.def) / 4 * (lv as u16) / 100 + 5;
        let spa = (ivs.spa + 2 * (base_stats[3] as u16) + evs.spa) / 4 * (lv as u16) / 100 + 5;
        let spd = (ivs.spd + 2 * (base_stats[4] as u16) + evs.spd) / 4 * (lv as u16) / 100 + 5;
        let spe = (ivs.spe + 2 * (base_stats[5] as u16) + evs.spe) / 4 * (lv as u16) / 100 + 5;
        let mut stats = StatsFeature {
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
        };

        // Add the nature modifiers:
        let nature = self.nature();
        match nature.increased_stat {
            Stat::Atk => stats.atk = (stats.atk as f32 * 1.1) as u16,
            Stat::Def => stats.def = (stats.def as f32 * 1.1) as u16,
            Stat::SpA => stats.spa = (stats.spa as f32 * 1.1) as u16,
            Stat::SpD => stats.spd = (stats.spd as f32 * 1.1) as u16,
            Stat::Spe => stats.spe = (stats.spe as f32 * 1.1) as u16,
            Stat::Hp => {
                // HP cannot be increased by nature.
                should_not_happen!("HP stat cannot be increased by nature");
            }
        }
        match nature.decreased_stat {
            Stat::Atk => stats.atk = (stats.atk as f32 * 0.9) as u16,
            Stat::Def => stats.def = (stats.def as f32 * 0.9) as u16,
            Stat::SpA => stats.spa = (stats.spa as f32 * 0.9) as u16,
            Stat::SpD => stats.spd = (stats.spd as f32 * 0.9) as u16,
            Stat::Spe => stats.spe = (stats.spe as f32 * 0.9) as u16,
            Stat::Hp => {
                // HP cannot be decreased by nature.
                should_not_happen!("HP stat cannot be decreased by nature");
            }
        }

        stats
    }

    /// Serializes the Pokémon into a vector of bytes, complying with the internal format used in
    /// the games.
    pub fn serialize(&self) -> Vec<u8> {
        // Create a vector of bytes with the maximum possible size:
        let mut bytes = if !self.is_gen5 {
            vec![0x00; GEN4_PKM_LEN]
        } else {
            vec![0x00; GEN5_PKM_LEN]
        };

        // Fill the bytes vector with the Pokémon's data:
        // Block A: 0x00 - 0x28
        bytes[0x00..0x04].copy_from_slice(&self.pid.to_le_bytes());
        bytes[0x04] = (self.encryption_bypass as u8) | ((self.bad_egg_flag as u8) << 1);
        // Checksum is computed at the end.
        bytes[0x08..0x0A].copy_from_slice(&self.species.id().to_le_bytes());
        bytes[0x0A..0x0C].copy_from_slice(&self.held_item.id().to_le_bytes());
        bytes[0x0C..0x0E].copy_from_slice(&self.trainer_id.to_le_bytes());
        bytes[0x0E..0x10].copy_from_slice(&self.trainer_secret_id.to_le_bytes());
        bytes[0x10..0x14].copy_from_slice(&self.experience.to_le_bytes());
        bytes[0x14] = self.friendship;
        bytes[0x15] = self.ability.id() as u8;
        bytes[0x16] = self.markings;
        bytes[0x17] = self.language as u8;
        bytes[0x18..0x1E].copy_from_slice(&[
            self.evs.hp as u8,
            self.evs.atk as u8,
            self.evs.def as u8,
            self.evs.spa as u8,
            self.evs.spd as u8,
            self.evs.spe as u8,
        ]);
        bytes[0x1E..0x24].copy_from_slice(&[
            self.contest_stats.cool,
            self.contest_stats.beauty,
            self.contest_stats.cute,
            self.contest_stats.smart,
            self.contest_stats.tough,
            self.contest_stats.sheen,
        ]);
        bytes[0x24..0x28].copy_from_slice(&self.sinnoh_ribbons[0..4]);
        // Block B: 0x28 - 0x48
        bytes[0x28..0x30].copy_from_slice(
            &[
                self.moves[0].id().to_le_bytes(),
                self.moves[1].id().to_le_bytes(),
                self.moves[2].id().to_le_bytes(),
                self.moves[3].id().to_le_bytes(),
            ]
            .concat(),
        );
        bytes[0x30..0x34].copy_from_slice(&self.move_pps);
        bytes[0x34..0x38].copy_from_slice(&self.move_pp_ups);
        let mut iv_bytes = 0;
        for (i, iv) in [
            Stat::Hp,
            Stat::Atk,
            Stat::Def,
            Stat::Spe,
            Stat::SpA,
            Stat::SpD,
        ]
        .iter()
        .enumerate()
        {
            iv_bytes |= (self.ivs.get(iv) as u32) << (5 * i)
        }
        bytes[0x38..0x3C].copy_from_slice(&iv_bytes.to_le_bytes());
        bytes[0x3B] |= (self.is_egg as u8) << 6 | (self.is_nicknamed as u8) << 7;
        bytes[0x3C..0x40].copy_from_slice(&self.hoenn_ribbons);
        bytes[0x40] = (self.fateful as u8) | ((self.gender as u8) << 1) | (self.form_id << 3);
        if !self.is_gen5 {
            // Gen 4 stores in 0x41 the shiny leaves:
            let mut leaf_bytes = 0u8;
            for leaf in self.shiny_leaves.iter() {
                leaf_bytes |= match leaf {
                    ShinyLeaf::A => 1 << 0x00,
                    ShinyLeaf::B => 1 << 0x01,
                    ShinyLeaf::C => 1 << 0x02,
                    ShinyLeaf::D => 1 << 0x03,
                    ShinyLeaf::E => 1 << 0x04,
                    ShinyLeaf::Crown => 1 << 0x05,
                };
            }
            bytes[0x41] = leaf_bytes;
        } else {
            // Gen 5 stores in 0x41 the nature ID:
            bytes[0x41] = self.nature.id_and_name.id() as u8;
        }
        if !self.is_gen5 {
            let egg_location: u16 = self.egg_location.into();
            bytes[0x44..0x46].copy_from_slice(&egg_location.to_le_bytes());
            let met_location: u16 = self.met_location.into();
            bytes[0x46..0x48].copy_from_slice(&met_location.to_le_bytes());
        }
        // Block C: 0x48 - 0x68
        if !self.is_gen5 {
            let mut encoded_name = should_be_ok!(
                Self::encode_name_gen4(&self.name),
                "The Pokémon has invalid name: {}",
                self.name
            );
            encoded_name.resize(0x5E - 0x48, 0);
            bytes[0x48..0x5E].copy_from_slice(&encoded_name);
        } else {
            let mut encoded_name = Self::encode_name_gen5(&self.name);
            encoded_name.resize(0x5E - 0x48, 0);
            bytes[0x48..0x5E].copy_from_slice(&encoded_name);
        }
        bytes[0x5F] = self.origin_game as u8;
        bytes[0x60..0x64].copy_from_slice(&self.sinnoh_ribbons[4..8]);
        // Block D: 0x68 - 0x82
        if !self.is_gen5 {
            let mut encoded_name = should_be_ok!(
                Self::encode_name_gen4(&self.trainer_name),
                "The Trainer has invalid name: {}",
                self.trainer_name
            );
            encoded_name.resize(0x78 - 0x68, 0);

            bytes[0x68..0x78].copy_from_slice(&encoded_name);
        } else {
            let mut encoded_name = Self::encode_name_gen5(&self.trainer_name);
            encoded_name.resize(0x78 - 0x68, 0);
            bytes[0x68..0x78].copy_from_slice(&encoded_name);
        }
        if let Some(egg_date) = self.egg_date {
            bytes[0x78..0x7B].copy_from_slice(&[
                (egg_date.year() - 2000) as u8,
                egg_date.month() as u8,
                egg_date.day() as u8,
            ]);
        }
        bytes[0x7B..0x7E].copy_from_slice(&[
            (self.met_date.year() - 2000) as u8,
            self.met_date.month() as u8,
            self.met_date.day() as u8,
        ]);
        if !self.is_gen5 {
            // Handle location particularities of Diamond and Pearl:
            // TODO: Check in-game representation of corner cases, e.g. Distortion World, Battle
            // Frontier, and HGSS locations.

            let egg_location = match self.egg_location {
                Location::Gen4(loc) => loc,
                _ => should_not_happen!(
                    "Egg location for Gen 4 Pokémon is not a Gen 4 location: {:?}",
                    self.egg_location
                ),
            };
            let met_location = match self.met_location {
                Location::Gen4(loc) => loc,
                _ => should_not_happen!(
                    "Met location for Gen 4 Pokémon is not a Gen 4 location: {:?}",
                    self.met_location
                ),
            };
            let dp_egg_location = if egg_location <= Gen4Location::DP_LAST_LOCATION {
                egg_location
            } else {
                Gen4Location::FarawayPlace
            };
            let dp_met_location = if met_location <= Gen4Location::DP_LAST_LOCATION {
                met_location
            } else {
                Gen4Location::FarawayPlace
            };

            bytes[0x7E..0x80].copy_from_slice(&(dp_egg_location as u16).to_le_bytes());
            bytes[0x80..0x82].copy_from_slice(&(dp_met_location as u16).to_le_bytes());
        } else {
            let egg_location: u16 = self.egg_location.into();
            bytes[0x7E..0x80].copy_from_slice(&egg_location.to_le_bytes());
            let met_location: u16 = self.met_location.into();
            bytes[0x80..0x82].copy_from_slice(&met_location.to_le_bytes());
        }
        bytes[0x82] = self.pokerus;
        // Handle HGSS ball particularities:
        bytes[0x83] = if !self.is_gen5 && self.ball >= Pokeball::FIRST_HGSS_BALL {
            Pokeball::PokeBall as u8
        } else {
            self.ball as u8
        };
        bytes[0x84] = self.met_level | (self.trainer_gender as u8) << 7;
        bytes[0x85] = self.encounter_type;
        bytes[0x86] = if !self.is_gen5 { self.ball as u8 } else { 0 };
        bytes[0x87] = self.performance;
        // 0x88 - End of "boxed" Pokémon data.

        bytes[0x8C] = self.level;
        // Check if the Pokémon has stats:
        let stats = match self.stats {
            Some(stats) => stats,
            None => self.generate_stats(),
        };
        // Set the current HP from the maximum HP:
        bytes[0x8E..0x90].copy_from_slice(&stats.hp.to_le_bytes());
        // Copy the stats.
        bytes[0x90..0x9C].copy_from_slice(
            &[
                stats.hp.to_le_bytes(),
                stats.atk.to_le_bytes(),
                stats.def.to_le_bytes(),
                stats.spe.to_le_bytes(),
                stats.spa.to_le_bytes(),
                stats.spd.to_le_bytes(),
            ]
            .concat(),
        );
        // We don't care about the other data after the stats (for now).

        // Compute and store the checksum:
        let checksum = bytes[0x08..BOXED_PKM_LEN]
            .chunks(2)
            .fold(0u16, |acc, chunk| {
                acc.wrapping_add(u16::from_le_bytes([chunk[0], chunk[1]]))
            });
        bytes[0x06..0x08].copy_from_slice(&checksum.to_le_bytes());

        bytes
    }

    /// Deserializes a Pokémon from a byte slice, complying with the internal format used in the
    /// games.
    pub fn deserialize(bytes: &[u8]) -> Pokemon {
        // Ensure the data is the correct size:
        assert!(
            bytes.len() == BOXED_PKM_LEN
                || bytes.len() == GEN4_PKM_LEN
                || bytes.len() == GEN5_PKM_LEN,
            "Invalid Pokémon data size",
        );

        // Create pokemon structure:
        let mut pkm = Pokemon::default();

        // Fill the Pokémon structure with the data:
        let species_id = u16::from_le_bytes([bytes[0x08], bytes[0x09]]);
        pkm.is_gen5 = bytes.len() == GEN5_PKM_LEN || species_id > 493;
        // Block A: 0x00 - 0x28
        pkm.set_pid(u32::from_le_bytes([
            bytes[0x00],
            bytes[0x01],
            bytes[0x02],
            bytes[0x03],
        ])); // Also sets the nature.
        pkm.encryption_bypass = bytes[0x04] & 0x03 != 0;
        pkm.bad_egg_flag = (bytes[0x04] & 0x02) != 0;
        pkm.original_checksum = u16::from_le_bytes([bytes[0x06], bytes[0x07]]);
        pkm.species = should_be_some!(
            IdFeature::from_species_id(species_id),
            "Invalid species ID: {}",
            species_id
        );
        let item_id = u16::from_le_bytes([bytes[0x0A], bytes[0x0B]]);
        if pkm.is_gen5 {
            pkm.held_item = should_be_some!(
                IdFeature::from_gen5_item_id(item_id),
                "Invalid item ID: {}",
                item_id
            );
        } else {
            pkm.held_item = should_be_some!(
                IdFeature::from_gen4_item_id(item_id),
                "Invalid item ID: {}",
                item_id
            );
        }
        pkm.trainer_id = u16::from_le_bytes([bytes[0x0C], bytes[0x0D]]);
        pkm.trainer_secret_id = u16::from_le_bytes([bytes[0x0E], bytes[0x0F]]);
        pkm.experience = u32::from_le_bytes([bytes[0x10], bytes[0x11], bytes[0x12], bytes[0x13]]);
        pkm.friendship = bytes[0x14];
        pkm.ability = should_be_some!(
            IdFeature::from_ability_id(bytes[0x15] as u16),
            "Invalid ability ID: {}",
            bytes[0x15]
        );
        pkm.markings = bytes[0x16];
        pkm.language = should_be_ok!(
            Language::try_from(bytes[0x17]),
            "Invalid language ID: {}",
            bytes[0x17]
        );
        pkm.evs = StatsFeature {
            hp: bytes[0x18] as u16,
            atk: bytes[0x19] as u16,
            def: bytes[0x1A] as u16,
            spa: bytes[0x1B] as u16,
            spd: bytes[0x1C] as u16,
            spe: bytes[0x1D] as u16,
        };
        pkm.contest_stats = ContestStatsFeature {
            cool: bytes[0x1E],
            beauty: bytes[0x1F],
            cute: bytes[0x20],
            smart: bytes[0x21],
            tough: bytes[0x22],
            sheen: bytes[0x23],
        };
        pkm.sinnoh_ribbons = [
            bytes[0x24],
            bytes[0x25],
            bytes[0x26],
            bytes[0x27],
            bytes[0x60],
            bytes[0x61],
            bytes[0x62],
            bytes[0x63],
        ];
        // Block B: 0x28 - 0x48
        let move1_id = u16::from_le_bytes([bytes[0x28], bytes[0x29]]);
        let move2_id = u16::from_le_bytes([bytes[0x2A], bytes[0x2B]]);
        let move3_id = u16::from_le_bytes([bytes[0x2C], bytes[0x2D]]);
        let move4_id = u16::from_le_bytes([bytes[0x2E], bytes[0x2F]]);
        pkm.moves = [
            should_be_some!(
                IdFeature::from_move_id(move1_id),
                "Invalid move ID: {}",
                move1_id
            ),
            should_be_some!(
                IdFeature::from_move_id(move2_id),
                "Invalid move ID: {}",
                move2_id
            ),
            should_be_some!(
                IdFeature::from_move_id(move3_id),
                "Invalid move ID: {}",
                move3_id
            ),
            should_be_some!(
                IdFeature::from_move_id(move4_id),
                "Invalid move ID: {}",
                move4_id
            ),
        ];
        pkm.move_pps = [bytes[0x30], bytes[0x31], bytes[0x32], bytes[0x33]];
        pkm.move_pp_ups = [bytes[0x34], bytes[0x35], bytes[0x36], bytes[0x37]];
        let iv_bytes = u32::from_le_bytes([bytes[0x38], bytes[0x39], bytes[0x3A], bytes[0x3B]]);
        pkm.ivs = StatsFeature {
            hp: (iv_bytes & 0x1F) as u16,
            atk: ((iv_bytes >> 5) & 0x1F) as u16,
            def: ((iv_bytes >> 10) & 0x1F) as u16,
            spe: ((iv_bytes >> 15) & 0x1F) as u16,
            spa: ((iv_bytes >> 20) & 0x1F) as u16,
            spd: ((iv_bytes >> 25) & 0x1F) as u16,
        };
        pkm.is_egg = (bytes[0x3C] & 0x40) != 0;
        pkm.is_nicknamed = (bytes[0x3B] & 0x80) != 0;
        pkm.hoenn_ribbons = [bytes[0x3C] & 0x3F, bytes[0x3D], bytes[0x3E], bytes[0x3f]];
        pkm.fateful = (bytes[0x40] & 0x01) != 0;
        pkm.gender = should_be_ok!(
            Gender::try_from((bytes[0x40] >> 1) & 0x03),
            "Invalid gender ID: {}",
            bytes[0x40] >> 1
        );
        pkm.form_id = bytes[0x40] >> 3;
        if !pkm.is_gen5 {
            // Gen 4 stores shiny leaves in 0x41:
            let leaf_bytes = bytes[0x41];
            pkm.shiny_leaves = HashSet::from_iter(
                [
                    (leaf_bytes & 0x01 != 0).then_some(ShinyLeaf::A),
                    (leaf_bytes & 0x02 != 0).then_some(ShinyLeaf::B),
                    (leaf_bytes & 0x04 != 0).then_some(ShinyLeaf::C),
                    (leaf_bytes & 0x08 != 0).then_some(ShinyLeaf::D),
                    (leaf_bytes & 0x10 != 0).then_some(ShinyLeaf::E),
                    (leaf_bytes & 0x20 != 0).then_some(ShinyLeaf::Crown),
                ]
                .into_iter()
                .flatten(),
            );
        } else {
            // Gen 5 stores nature ID in 0x41:
            pkm.nature = should_be_some!(
                Nature::from_id(bytes[0x41] as u16),
                "Invalid nature ID: {}",
                bytes[0x41]
            );
        }
        // Transform egg location to correct enum type:
        let egg_loc_plathgss = u16::from_le_bytes([bytes[0x44], bytes[0x45]]);
        let egg_loc_others = u16::from_le_bytes([bytes[0x7E], bytes[0x7F]]);

        pkm.egg_location = if egg_loc_plathgss != 0 {
            // If the Plat/HG/SS egg loc offset is non-zero, this is always a Gen 4 location.
            Location::Gen4(should_be_ok!(
                Gen4Location::try_from(egg_loc_plathgss),
                "Invalid egg location ID: {}",
                egg_loc_plathgss
            ))
        } else {
            // If the Plat/HG/SS egg location offset is zero, this can be a Gen 4 or Gen 5 location.
            if !pkm.is_gen5 {
                Location::Gen4(should_be_ok!(
                    Gen4Location::try_from(egg_loc_others),
                    "Invalid egg location ID: {}",
                    egg_loc_others
                ))
            } else {
                Location::Gen5(should_be_ok!(
                    Gen5Location::try_from(egg_loc_others),
                    "Invalid egg location ID: {}",
                    egg_loc_others
                ))
            }
        };
        // Transform met location to correct enum type:
        let met_loc_plathgss = u16::from_le_bytes([bytes[0x46], bytes[0x47]]);
        let met_loc_others = u16::from_le_bytes([bytes[0x80], bytes[0x81]]);

        pkm.met_location = if met_loc_plathgss != 0 {
            // If the Plat/HG/SS met location offset is non-zero, this is always a Gen 4 location.
            Location::Gen4(should_be_ok!(
                Gen4Location::try_from(met_loc_plathgss),
                "Invalid met location ID: {}",
                met_loc_plathgss
            ))
        } else {
            // If the Plat/HG/SS met location offset is zero, this can be a Gen 4 or Gen 5 location.
            if !pkm.is_gen5 {
                Location::Gen4(should_be_ok!(
                    Gen4Location::try_from(met_loc_others),
                    "Invalid met location ID: {}",
                    met_loc_others
                ))
            } else {
                Location::Gen5(should_be_ok!(
                    Gen5Location::try_from(met_loc_others),
                    "Invalid met location ID: {}",
                    met_loc_others
                ))
            }
        };
        // Block C: 0x48 - 0x68
        if !pkm.is_gen5 {
            pkm.name = should_be_ok!(
                Self::decode_name_gen4(&bytes[0x48..0x5E]),
                "Invalid Pokémon name"
            );
        } else {
            pkm.name = should_be_ok!(
                Self::decode_name_gen5(&bytes[0x48..0x5E]),
                "Invalid Pokémon name"
            );
        }
        pkm.origin_game = should_be_ok!(
            bytes[0x5F].try_into(),
            "Invalid origin game ID: {}",
            bytes[0x5F]
        );
        pkm.sinnoh_ribbons[4..8].copy_from_slice(&bytes[0x60..0x64]);
        // Block D: 0x68 - 0x82
        if !pkm.is_gen5 {
            pkm.trainer_name = should_be_ok!(
                Self::decode_name_gen4(&bytes[0x68..0x78]),
                "Invalid Trainer name"
            );
        } else {
            pkm.trainer_name = should_be_ok!(
                Self::decode_name_gen5(&bytes[0x68..0x78]),
                "Invalid Trainer name"
            );
        }
        pkm.egg_date = NaiveDate::from_ymd_opt(
            bytes[0x78] as i32 + 2000,
            bytes[0x79] as u32,
            bytes[0x7A] as u32,
        );
        pkm.met_date = should_be_some!(
            NaiveDate::from_ymd_opt(
                bytes[0x7B] as i32 + 2000,
                bytes[0x7C] as u32,
                bytes[0x7D] as u32
            ),
            "Invalid met date"
        );
        pkm.pokerus = bytes[0x82];
        // Handle HGSS ball particularities:
        let ball = bytes[0x83];
        let hgss_ball = bytes[0x86];
        pkm.ball = if !pkm.is_gen5 && hgss_ball != 0 {
            should_be_ok!(
                Pokeball::try_from(hgss_ball),
                "Invalid Pokéball ID: {}",
                hgss_ball
            )
        } else {
            should_be_ok!(Pokeball::try_from(ball), "Invalid Pokéball ID: {}", ball)
        };
        pkm.met_level = bytes[0x84] & 0x7F;
        pkm.trainer_gender = should_be_ok!(
            Gender::try_from((bytes[0x84] >> 7) & 0x01),
            "Invalid trainer gender ID"
        );
        pkm.encounter_type = bytes[0x85];
        pkm.performance = bytes[0x87];
        // 0x88 - End of "boxed" Pokémon data.

        pkm.level = bytes[0x8C];
        // Check if the Pokémon has stats:
        if bytes.len() == GEN4_PKM_LEN || bytes.len() == GEN5_PKM_LEN {
            // Ignore current HP.
            pkm.stats = Some(StatsFeature {
                hp: u16::from_le_bytes([bytes[0x90], bytes[0x91]]),
                atk: u16::from_le_bytes([bytes[0x92], bytes[0x93]]),
                def: u16::from_le_bytes([bytes[0x94], bytes[0x95]]),
                spe: u16::from_le_bytes([bytes[0x96], bytes[0x97]]),
                spa: u16::from_le_bytes([bytes[0x98], bytes[0x99]]),
                spd: u16::from_le_bytes([bytes[0x9A], bytes[0x9B]]),
            });
        } else {
            pkm.stats = None;
        }

        pkm
    }

    // Raw data processing methods:

    /// Converts the decrypted serialized data for a Pokémon into a valid game representation that
    /// bypasses the encryption.
    ///
    /// # Arguments
    /// * `decrypted_data` - The decrypted serialized Pokémon data, using the game's internal
    ///   representation, as a vector of bytes.
    pub fn to_encryption_bypass_data(decrypted_data: &[u8]) -> Vec<u8> {
        // Check the data has the correct size:
        assert!(
            decrypted_data.len() >= BOXED_PKM_LEN,
            "Invalid Pokémon data length: {}",
            decrypted_data.len()
        );

        // Decode the PID:
        let pid = u32::from_le_bytes(
            decrypted_data[0x00..0x04]
                .try_into()
                .expect("Failed to convert PID slice to array"),
        );

        // Create the result buffer, copy the data, and shuffle the data blocks:
        let mut encrypted_data = decrypted_data.to_vec();
        Self::shuffle_blocks(&mut encrypted_data, pid);

        // Set the encryption bypass flag:
        encrypted_data[0x04] |= 0x03;

        encrypted_data
    }

    /// Converts the decrypted serialized data for a Pokémon into a valid encrypted game
    /// representation.
    ///
    /// # Arguments
    /// * `decrypted_data` - The decrypted serialized Pokémon data, using the game's internal
    ///   representation, as a vector of bytes.
    pub fn to_encrypted_data(decrypted_data: &[u8]) -> Vec<u8> {
        // Check the data has the correct size:
        assert!(
            decrypted_data.len() >= BOXED_PKM_LEN,
            "Invalid Pokémon data length: {}",
            decrypted_data.len()
        );

        // Decode the PID:
        let pid = u32::from_le_bytes(
            decrypted_data[0x00..0x04]
                .try_into()
                .expect("Failed to convert PID slice to array"),
        );
        // Decode the checksum:
        let checksum = (decrypted_data[0x06] as u16) + ((decrypted_data[0x07] as u16) << 8);

        // Create the result buffer, copy the data, and shuffle the data blocks:
        let mut encrypted_data = decrypted_data.to_vec();
        Self::shuffle_blocks(&mut encrypted_data, pid);

        // Encrypt the data:
        Self::crypt_data(&mut encrypted_data, pid, checksum);

        encrypted_data
    }

    /// Decrypts the encrypted serialized data for a Pokémon into a readable valid game
    /// representation.
    ///
    /// # Arguments
    /// * `encrypted_data` - The encrypted serialized Pokémon data, as a vector of bytes.
    pub fn to_decrypted_data(encrypted_data: &[u8]) -> Vec<u8> {
        // Check the data has the correct size:
        assert!(
            encrypted_data.len() >= BOXED_PKM_LEN,
            "Invalid Pokémon data length: {}",
            encrypted_data.len()
        );

        // Decode the PID:
        let pid = u32::from_le_bytes(
            encrypted_data[0x00..0x04]
                .try_into()
                .expect("Failed to convert PID slice to array"),
        );
        // Decode the checksum:
        let checksum = (encrypted_data[0x06] as u16) + ((encrypted_data[0x07] as u16) << 8);

        // Create the result buffer, and copy the data:
        let mut decrypted_data = encrypted_data.to_vec();

        // Decrypt the data:
        Self::crypt_data(&mut decrypted_data, pid, checksum);

        // Unshuffle the data blocks:
        Self::unshuffle_blocks(&mut decrypted_data, pid);

        decrypted_data
    }

    /// Encrypts or decrypts the whole serialized Pokémon data using the game's encryption
    /// algorithm.
    ///
    /// **Encryption/decryption is performed in-place**: The provided vector with the original data
    /// `pokemon_data` will be overwritten.
    ///
    /// # Arguments
    /// * `pokemon_data` - The serialized Pokémon data to encrypt or decrypt.
    /// * `pid` - The Pokémon's PID, used as a seed for the encryption algorithm.
    /// * `checksum` - The Pokémon's checksum, used as a seed for the encryption algorithm.
    fn crypt_data(pokemon_data: &mut Vec<u8>, pid: u32, checksum: u16) {
        Self::encryption_step(&mut pokemon_data[0x8..BOXED_PKM_LEN], checksum as u32);
        if pokemon_data.len() > BOXED_PKM_LEN {
            Self::encryption_step(&mut pokemon_data[BOXED_PKM_LEN..], pid);
        }
    }

    /// Encrypts or decrypts a segment of the Pokémon data using the game's encryption algorithm.
    ///
    /// The encryption algorithm is a simple XOR-based stream cipher that uses a linear
    /// congruential generator to produce a pseudo-random sequence of bytes based on a seed.
    ///
    /// The algorithm is symmetric. In other words, the same XOR encryption algorithm is used for
    /// both encryption and decryption.
    ///
    /// **Encryption/decryption is performed in-place**: The provided slice with the original data
    /// `data` will be overwritten.
    ///
    /// # Arguments
    /// * `data` - The segment of the Pokémon data to encrypt or decrypt, as a mutable slice of
    ///   bytes.
    /// * `seed` - The seed used for the encryption algorithm.
    fn encryption_step(data: &mut [u8], seed: u32) {
        // Unpack the byte array into u16 values:
        let mut data_as_u16 = data
            .chunks(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect::<Vec<u16>>();

        // Perform the encryption, using a XOR operation with a linear congruential generator:
        let mut state = seed;
        for i in 0..data_as_u16.len() {
            let byte = &mut data_as_u16[i];
            state = (state.wrapping_mul(0x41C64E6D) + 0x6073) & 0xFFFFFFFF;
            *byte ^= (state >> 16) as u16;
        }

        // Copy back the data to the original buffer:
        data.copy_from_slice(
            &data_as_u16
                .iter()
                .flat_map(|&x| x.to_le_bytes())
                .collect::<Vec<u8>>(),
        );
    }

    /// Shuffles the "blocks"" of the serialized Pokémon data, as to comply with the game's
    /// internal representation.
    ///
    /// A part of the Pokémon's internal representation is divided into blocks of 32 bytes each.
    /// Internally, the games shuffle these blocks in an attempt to obfuscate the data before
    /// encryption.
    ///
    /// The blocks are defined as follows:
    /// - Block A: offsets 0x08 to 0x27
    /// - Block B: offsets 0x28 to 0x47
    /// - Block C: offsets 0x48 to 0x67
    /// - Block D: offsets 0x68 to 0x87
    ///
    /// This operation can be reverted with `unshuffle_blocks`.
    ///
    /// The order of the block shuffle is based on the Pokémon's PID.
    ///
    /// **Shuffling is performed in-place**: The provided vector with the original data
    /// `pokemon_data` will be overwritten.
    ///
    /// # Arguments
    /// * `pokemon_data` - The serialized Pokémon data to shuffle, as a mutable vector of bytes.
    /// * `pid` - The Pokémon's PID, used to determine the shuffle order.
    fn shuffle_blocks(pokemon_data: &mut Vec<u8>, pid: u32) {
        // Get the slice to shuffle:
        let shuffle_data = &mut pokemon_data[0x08..BOXED_PKM_LEN];

        // Get the shuffle order:
        let shuffle_order = Self::determine_shuffle_block_order(pid);

        // Shuffle the blocks:
        let mut tmp = [0u8; 0x80];
        for (i, &block_id) in shuffle_order.iter().enumerate() {
            tmp[i * 0x20..(i + 1) * 0x20]
                .copy_from_slice(&shuffle_data[block_id * 0x20..(block_id + 1) * 0x20]);
        }
        shuffle_data.copy_from_slice(&tmp);
    }

    /// Unshuffles the "blocks" of the serialized Pokémon data, to restore the standard order.
    ///
    /// This operation is the opposite to `shuffle_blocks`.
    ///
    /// The order of the block unshuffle is based on the Pokémon's PID.
    ///
    /// **Unshuffling is performed in-place**: The provided vector with the original data
    /// `pokemon_data` will be overwritten.
    ///
    /// # Arguments
    /// * `pokemon_data` - The serialized Pokémon data to unshuffle, as a mutable vector of bytes.
    /// * `pid` - The Pokémon's PID, used to determine the unshuffle order.
    fn unshuffle_blocks(pokemon_data: &mut Vec<u8>, pid: u32) {
        // Get the shuffled slice:
        let shuffle_data = &mut pokemon_data[0x08..BOXED_PKM_LEN];

        // Get the shuffle order:
        let shuffle_order = Self::determine_shuffle_block_order(pid);

        // Unshuffle the blocks, performing the opposite memcpy to shuffle_blocks:
        let mut tmp = [0u8; 0x80];
        for (i, &block_id) in shuffle_order.iter().enumerate() {
            tmp[block_id * 0x20..(block_id + 1) * 0x20]
                .copy_from_slice(&shuffle_data[i * 0x20..(i + 1) * 0x20]);
        }
        shuffle_data.copy_from_slice(&tmp);
    }

    /// Returns the order in which the blocks of a Pokémon's serialized data should be
    /// shuffled, based on the Pokémon's PID.
    ///
    /// # Arguments
    /// * `pid` - The Pokémon's PID, used to determine the shuffle order.
    fn determine_shuffle_block_order(pid: u32) -> [usize; 4] {
        // List of all the possible block orders:
        let possible_orders = [
            // [A block, B block, C block, D block]
            [0, 1, 2, 3],
            [0, 1, 3, 2],
            [0, 2, 1, 3],
            [0, 2, 3, 1],
            [0, 3, 1, 2],
            [0, 3, 2, 1],
            [1, 0, 2, 3],
            [1, 0, 3, 2],
            [1, 2, 0, 3],
            [1, 2, 3, 0],
            [1, 3, 0, 2],
            [1, 3, 2, 0],
            [2, 0, 1, 3],
            [2, 0, 3, 1],
            [2, 1, 0, 3],
            [2, 1, 3, 0],
            [2, 3, 0, 1],
            [2, 3, 1, 0],
            [3, 0, 1, 2],
            [3, 0, 2, 1],
            [3, 1, 0, 2],
            [3, 1, 2, 0],
            [3, 2, 0, 1],
            [3, 2, 1, 0],
        ];
        // Choose the order based on some specific middle bits of the PID:
        let order = ((pid >> 13) & 0x1F) % 24;

        possible_orders[order as usize]
    }

    // Convenience functions:

    /// Saves the Pokémon to binary a file in the specified directory, with the specified extension.
    ///
    /// The resulting file will contain the Pokémon's serialized data. See `serialize`.
    ///
    /// # Arguments
    /// * `dir_path` - The directory where the file will be saved. If `None`, defaults to
    ///   "pokemon".
    /// * `extension` - The file extension to use. If `None`, defaults to "pk4" for Gen 4 Pokémon
    ///   and "pk5" for Gen 5 Pokémon.
    ///
    /// # Returns
    /// `Ok(true)` if the pokémon was saved successfully, `Ok(false)` if the pokémon was not
    /// saved due to the resulting file already existing, or the corresponding error if there was
    /// an error during saving.
    pub fn save(&self, dir_path: Option<&Path>, extension: Option<String>) -> Result<bool> {
        // Retrieve save directory path and extenion:
        let dir_path = match dir_path {
            Some(p) => p.to_path_buf(),
            None => Path::new("pokemon").to_path_buf(),
        };
        let extension = match extension {
            Some(ext) => ext,
            None => {
                if self.is_gen5 {
                    "pk5".to_string()
                } else {
                    "pk4".to_string()
                }
            }
        };

        // Create the save directory:
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)?;
            // Make all users own the directory:
            // (The gts-rs app must be run as superuser, which messes with permissions.)
            let mut dir_permissions = fs::metadata(&dir_path)?.permissions();
            dir_permissions.set_readonly(false);
            fs::set_permissions(&dir_path, dir_permissions)?;
        }

        // Generate save file name:
        let current_time_str = LocalTime::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let shiny_mark = if self.is_shiny() { "!" } else { "" };
        let base_name = format!("{}_{}{}", self.species.name(), self.name, shiny_mark);

        // Generate the binary data to save:
        let data = self.serialize();

        // Check if the resulting file already exists, not saving self if it does:
        let (file_exists, _maybe_file_path) =
            Self::save_file_exists(&data, &dir_path, &base_name, &extension)?;
        if file_exists {
            return Ok(false);
        }

        // Save to disk:
        let file_path = dir_path.join(format!("{}_{}.{}", base_name, current_time_str, extension));
        let mut file = File::create(&file_path)?;
        file.write_all(&data)?;
        // Let all users own the saved file:
        // (The gts-rs app must be run as superuser, which messes with permissions.)
        let mut file_permissions = fs::metadata(&file_path)?.permissions();
        file_permissions.set_readonly(false);
        fs::set_permissions(&file_path, file_permissions)?;

        Ok(true)
    }

    /// Checks if a file with the same Pokémon data already exists in the specified path.
    ///
    /// # Arguments
    /// * `data` - The serialized Pokémon data as a vector of bytes.
    /// * `path` - The path of the directory where the Pokémon data files are stored.
    /// * `base_name` - The base name of the Pokémon file, without extension.
    /// * `extension` - The file extension to check for.
    ///
    /// # Returns
    /// * `Ok((true, Some(file_path)))` if a file with the same Pokémon data exists,
    ///   where `file_path` is the path to the existing file.
    /// * `Ok((false, None))` if no such file exists.
    /// * `Err(error)` if there was an error reading the directory or file.
    fn save_file_exists(
        data: &Vec<u8>,
        path: &Path,
        base_name: &String,
        extension: &String,
    ) -> Result<(bool, Option<PathBuf>)> {
        // Check if there is any file in the provided path that begins with the base name and ends
        // in the extension.
        for file in fs::read_dir(path)? {
            if let Ok(file) = file {
                let file_name = file.file_name().to_string_lossy().into_owned();
                if file_name.starts_with(base_name) && file_name.ends_with(extension) {
                    let file_path = file.path();
                    let other = fs::read(&file_path)?;
                    if data == &other {
                        // The file exists and is identical to the current Pokémon data.
                        return Ok((true, Some(file_path)));
                    }
                }
            }
        }

        Ok((false, None))
    }

    /// Loads a Pokémon from a binary file at the specified path.
    ///
    /// The file must be in the game's internal format, either Gen 4 or Gen 5, and representing
    /// either boxed or party Pokémon.
    ///
    /// # Arguments
    /// * `file_path` - The path to the file containing the Pokémon data.
    ///
    /// # Returns
    ///
    /// `Ok(pokemon)`, where `pokemon` is the loaded Pokémon, if the pokémon was loaded
    /// successfully, or the corresponding error if there was an error during loading.
    pub fn load(file_path: &Path) -> Result<Pokemon> {
        // Check the extension is correct:
        let file_path_str = file_path.to_string_lossy();
        if !file_path_str.ends_with(".pkm")
            && !file_path_str.ends_with(".pk4")
            && !file_path_str.ends_with(".pk5")
        {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "File must be a .pkm, .pk4, or .pk5 file",
            ));
        }

        // Read the file:
        let data = fs::read(file_path)?;
        // Check the file size is valid:
        assert!(
            data.len() == BOXED_PKM_LEN || data.len() == GEN4_PKM_LEN || data.len() == GEN5_PKM_LEN,
            "Invalid Pokémon file size: {}",
            data.len(),
        );

        // Deserialize the Pokémon from the data:
        let pokemon = Pokemon::deserialize(&data);

        Ok(pokemon)
    }
}
