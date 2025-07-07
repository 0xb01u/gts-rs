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
// GTS protocol documentation: https://projectpokemon.org/home/docs/gen-5/gts-protocol-r19/
// WARNING: That documentation is at times somewhat unclear.
use base64::{engine::general_purpose::URL_SAFE as URL_SAFE_B64, Engine as _};
use chrono::{DateTime, Local as LocalTime, NaiveDateTime};
use getset::Getters;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Result},
};

use crate::{
    data_maps::GEONET_GEN5, internal_types::*, pokemon::Pokemon, should_be_ok, should_be_some,
};

/// Struct representing a location for a Pokémon in the GTS.
///
/// A GTS location is composed of a country and a region.
#[derive(Serialize, Deserialize)]
pub struct Geonet {
    countries: Vec<String>,
    states: HashMap<String, Vec<String>>,
}

/// Gets the country and state names, for the respective country and state codes.
///
/// The state might be an empty string.
///
/// Returns `None` if the contry code or state code is invalid.
///
/// # Arguments
/// * `country_code` - The code of the country to get the name of.
/// * `state_code` - The code of the state to get the name of.
pub fn country(country_code: u8, state_code: u8) -> Option<(String, String)> {
    let country = GEONET_GEN5.countries.get(country_code as usize)?;
    let state = GEONET_GEN5.states.get(country)?.get(state_code as usize)?;

    Some((country.clone(), state.clone()))
}

/// Gets the country and state codes, for the respective country and state names.
///
/// The state might be an empty string, if the country has no states.
///
/// Returns `None` if the country or state name is invalid.
///
/// # Arguments
/// * `country_name` - The name of the country to get the code of.
/// * `state_name` - The name of the state to get the code of.
pub fn country_code(country_name: &String, state_name: &String) -> Option<(u8, u8)> {
    let country_index = GEONET_GEN5
        .countries
        .iter()
        .position(|c| c == country_name)?;
    let state_index = GEONET_GEN5
        .states
        .get(country_name)?
        .iter()
        .position(|s| s == state_name)?;

    Some((country_index as u8, state_index as u8))
}

/// Struct representing the extra data sent from the GTS servers in Pokémon receptions, alongside
/// the Pokémon itself.
pub struct GTSData {
    pkm_id: u16,
    gender: Gender,
    lvl: u8,
    req_pkm_id: u16,
    req_gender: Gender,
    req_min_lvl: u8,
    req_max_lvl: u8,
    trainer_gender: Gender,
    deposited_time: NaiveDateTime,
    traded_time: NaiveDateTime,
    profile_id: u32,
    trainer_id: u16,
    trainer_secret_id: Option<u16>, // Only present in Gen 5.
    trainer_name: String,           // Goes before the trainer ID in Gen 4.
    country: String,
    region: String,
    trainer_class: TrainerClass,
    is_exchanged: bool, // Always `false` for deposits, `true` for receptions.
    game: Game,
    language: Language,
    unity_tower_floors: Option<u8>, // Only present in Gen 5.
}

impl GTSData {
    /// Constructs a `GTSData` struct from a `Pokemon`.
    ///
    /// All fields which can be derived from the Pokémon data will be filled in using that data.
    /// Other fields will be filled in with random values or default values.
    ///
    /// # Arguments
    /// * `pokemon` - The Pokémon to construct the GTS data from.
    fn from_pokemon(pokemon: &Pokemon) -> Self {
        // Create RNG object for generating some fields for which I cannot think of any other more
        // creative way of generating lol:
        let mut rng = SmallRng::from_rng(&mut rand::rng());

        // Generate random GTS PID:
        let profile_id = rng.random();

        // Fill in the fields for the deposited Pokémon (from the Pokémon data when available,
        // randomly, or with some default values otherwise):
        let pkm_id = pokemon.species.id();
        let gender = pokemon.gender;
        let lvl = pokemon.level();
        let trainer_gender = pokemon.trainer_gender;
        let deposited_time = NaiveDateTime::from(pokemon.met_date);
        let traded_time = LocalTime::now().naive_utc();
        let trainer_id = pokemon.trainer_id;
        let trainer_secret_id = Some(pokemon.trainer_secret_id);
        let trainer_name = pokemon.trainer_name().clone();
        let game = pokemon.origin_game;
        let country = if !pokemon.is_gen5() {
            "Japan".to_string()
        } else {
            "United States of America".to_string()
        };
        let region = if !pokemon.is_gen5() {
            // Twinleaf Town's closest match:
            "Hokkaido (Sapporo)".to_string()
        } else {
            if game == Game::White2 || game == Game::Black2 {
                // Asperia City's closest match:
                "New Jersey".to_string()
            } else {
                // Nuvema Town's closest match:
                "New York".to_string()
            }
            // Information on real-world locations extracted from:
            // https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_world_in_relation_to_the_real_world
        };
        let trainer_class = should_be_ok!(
            TrainerClass::try_from(
                ((trainer_id ^ pokemon.trainer_secret_id) as u8) % TrainerClass::COUNT
            ),
            "Tried to create a Trainer Class with an invalid ID"
        );
        let is_exchanged = true;
        let language = pokemon.language;
        let unity_tower_floors = if pokemon.is_gen5() { Some(0) } else { None };

        // Fill the data for the requested Pokémon:
        let req_pkm_id = if !pokemon.is_gen5() {
            rng.random_range(1..=493)
        } else {
            rng.random_range(1..=649)
        };
        let req_min_lvl = 1;
        let req_max_lvl = 100;
        let req_gender = Gender::Genderless;

        Self {
            pkm_id,
            gender,
            lvl,
            req_pkm_id,
            req_gender,
            req_min_lvl,
            req_max_lvl,
            trainer_gender,
            deposited_time,
            traded_time,
            profile_id,
            trainer_id,
            trainer_secret_id,
            trainer_name,
            country,
            region,
            trainer_class,
            is_exchanged,
            game,
            language,
            unity_tower_floors,
        }

        // Unknown data (offsets):
        // 0x09
        // 0x0B
        // [Gen 5] 0x3A - 0x3B?
    }

    /// Serializes the GTS data into a vector of bytes.
    ///
    /// # Arguments
    /// * `is_gen5` - Whether the data is from a Gen 5 GTS reception or not.
    fn serialize(&self, is_gen5: bool) -> Vec<u8> {
        // Create the data vector to fill:
        let mut data = vec![0; if !is_gen5 { 0x38 } else { 0x3C }];

        // Fill in with the data:
        data[0x00..0x02].copy_from_slice(&self.pkm_id.to_le_bytes());
        data[0x02] = self.gender as u8;
        data[0x03] = self.lvl;
        data[0x04..0x06].copy_from_slice(&self.req_pkm_id.to_le_bytes());
        data[0x06] = self.req_gender as u8 + 1;
        data[0x07] = self.req_min_lvl;
        data[0x08] = self.req_max_lvl;
        data[0x0A] = self.trainer_gender as u8;
        data[0x0C..0x14].copy_from_slice(&self.deposited_time.and_utc().timestamp().to_be_bytes());
        data[0x14..0x1C].copy_from_slice(&self.traded_time.and_utc().timestamp().to_be_bytes());
        data[0x1C..0x20].copy_from_slice(&self.profile_id.to_le_bytes());
        if !is_gen5 {
            data[0x30..0x32].copy_from_slice(&self.trainer_id.to_le_bytes());
        } else {
            data[0x20..0x22].copy_from_slice(&self.trainer_id.to_le_bytes());
            let secret_id = should_be_some!(
                self.trainer_secret_id,
                "Trainer secret ID is not present in Gen 5 GTS data"
            );
            data[0x22..0x24].copy_from_slice(&secret_id.to_le_bytes());
        }
        if !is_gen5 {
            let mut encoded_name = Pokemon::encode_name_gen4(&self.trainer_name)
                .expect("Failed to encode Gen 4 trainer name for GTS data");
            encoded_name.resize(0x30 - 0x20, 0);
            data[0x20..0x30].copy_from_slice(&encoded_name);
        } else {
            let mut encoded_name = Pokemon::encode_name_gen5(&self.trainer_name);
            encoded_name.resize(0x30 - 0x20, 0);
            data[0x24..0x34].copy_from_slice(&encoded_name);
        }
        let extra_offset = if is_gen5 { 2 } else { 0 };
        let (country, region) = should_be_some!(
            country_code(&self.country, &self.region),
            "Invalid country or region: {} {}",
            self.country,
            self.region
        );
        data[0x32 + extra_offset] = country;
        data[0x33 + extra_offset] = region;
        data[0x34 + extra_offset] = self.trainer_class as u8;
        data[0x35 + extra_offset] = if self.is_exchanged { 1 } else { 0 };
        data[0x36 + extra_offset] = self.game as u8;
        data[0x37 + extra_offset] = self.language as u8;
        if is_gen5 {
            data[0x3B] = should_be_some!(
                self.unity_tower_floors,
                "Unity Tower floors are not present in Gen 5 GTS data"
            );
        }

        data
    }

    // This was intended to be used for client deposits, but then I realized the data send in those
    // is not exacty the same as the one managed here.
    #[allow(dead_code)]
    /// Deserializes the GTS data from a byte slice.
    ///
    /// This is useful when attempting to send a Pokémon to a Gen 4 or Gen 5 game using the GTS
    /// services.
    ///
    /// # Arguments
    /// * `data` - The byte slice containing the GTS data to deserialize.
    /// * `is_gen5` - Whether the data is from a Gen 5 GTS reception.
    fn deserialize(data: &[u8], is_gen5: bool) -> Self {
        // Check the data has the correct size:
        if !is_gen5 {
            assert!(
                data.len() == 0x124,
                "Invalid GTS data length for Gen 4: {}",
                data.len()
            );
        } else {
            assert!(
                data.len() == 0x128,
                "Invalid GTS data length for Gen 5: {}",
                data.len()
            );
        }

        // Fill in the fields:
        let pkm_id = u16::from_le_bytes([data[0x00], data[0x01]]);
        let gender = should_be_ok!(
            Gender::try_from(data[0x02] - 1),
            "Invalid gender: {}",
            data[0x02]
        );
        let lvl = data[0x03];
        let req_pkm_id = u16::from_le_bytes([data[0x04], data[0x05]]);
        let req_gender = should_be_ok!(
            Gender::try_from(data[0x06] - 1),
            "Invalid requested gender: {}",
            data[0x06]
        );
        let req_min_lvl = data[0x07];
        let req_max_lvl = data[0x08];
        let trainer_gender = should_be_ok!(
            Gender::try_from(data[0x0A]),
            "Invalid trainer gender: {}",
            data[0x0A]
        );
        let deposited_time = DateTime::from_timestamp(
            i64::from_be_bytes(
                data[0x0C..0x14]
                    .try_into()
                    .expect("Failed to convert deposited time slice to array"),
            ),
            0,
        )
        .unwrap_or(LocalTime::now().to_utc())
        .naive_utc();
        let traded_time = DateTime::from_timestamp(
            i64::from_be_bytes(
                data[0x14..0x1C]
                    .try_into()
                    .expect("Failed to convert traded time slice to array"),
            ),
            0,
        )
        .unwrap_or(LocalTime::now().to_utc())
        .naive_utc();
        let profile_id = u32::from_le_bytes(
            data[0x1C..0x20]
                .try_into()
                .expect("Failed to convert GTS PID slice to array"),
        );
        let trainer_id = if !is_gen5 {
            u16::from_le_bytes([data[0x30], data[0x31]])
        } else {
            u16::from_le_bytes([data[0x20], data[0x21]])
        };
        let trainer_secret_id = if is_gen5 {
            Some(u16::from_le_bytes([data[0x22], data[0x23]]))
        } else {
            None
        };
        let trainer_name = if !is_gen5 {
            should_be_ok!(
                Pokemon::decode_name_gen4(&data[0x20..0x30]),
                "Failed to decode Gen 4 trainer name from GTS data"
            )
        } else {
            should_be_ok!(
                Pokemon::decode_name_gen5(&data[0x24..0x34]),
                "Failed to decode Gen 5 trainer name from GTS data"
            )
        };
        let extra_offset = if is_gen5 { 2 } else { 0 };
        let (country, region) = should_be_some!(
            country(data[0x32 + extra_offset], data[0x33 + extra_offset]),
            "Invalid country or region code: {} {}",
            data[0x32 + extra_offset],
            data[0x33 + extra_offset]
        );
        let trainer_class = should_be_ok!(
            TrainerClass::try_from(data[0x34 + extra_offset]),
            "Invalid trainer class ID: {}",
            data[0x34 + extra_offset]
        );
        let is_exchanged = data[0x35 + extra_offset] != 0;
        let game = should_be_ok!(
            data[0x36 + extra_offset].try_into(),
            "Invalid game ID: {}",
            data[0x36 + extra_offset]
        );
        let language = should_be_ok!(
            Language::try_from(data[0x37 + extra_offset]),
            "Invalid language ID: {}",
            data[0x37 + extra_offset]
        );
        let unity_tower_floors = if is_gen5 { Some(data[0x3B]) } else { None };

        Self {
            pkm_id,
            gender,
            lvl,
            req_pkm_id,
            req_gender,
            req_min_lvl,
            req_max_lvl,
            trainer_gender,
            deposited_time,
            traded_time,
            profile_id,
            trainer_id,
            trainer_secret_id,
            trainer_name,
            country,
            region,
            trainer_class,
            is_exchanged,
            game,
            language,
            unity_tower_floors,
        }
    }
}

#[allow(dead_code)]
/// Struct representing a Pokémon deposit to the GTS.
///
/// This struct is used to parse the data received by the GTS when a Pokémon is deposited.
///
/// This struct is mainly useful to retrieve the Pokémon sent to the GTS via the `pokemon`
/// function, after parsing the raw data received with the `from_base64` function.
#[derive(Getters)]
pub struct GTSDeposit {
    gts_checksum: u32, // Unused field; kept for reference.
    profile_id: u32,   // Unused field; kept for reference.
    #[get = "pub"]
    pokemon: Pokemon,
    // Ignore everything else received, as it is not useful for this app.
}

impl GTSDeposit {
    /// Constructs a `GTSDeposit` from the base64-encoded data received from the game.
    ///
    /// This function decodes de bas64 data and deserializes it completely into the struct.
    ///
    /// All encrypted data is decrypted in the process.
    ///
    ///
    /// # Arguments
    /// * `base64_data` - The base64-encoded data received, as a reference to a String.
    /// * `is_gen5` - Whether the received data is from a Gen 5 game.
    ///
    /// # Returns
    /// Returns Ok(`GTSDeposit`) on correct execution, or an error if the data could not be decoded
    /// as base64.
    pub fn from_base64(base64_data: &String, is_gen5: bool) -> Result<Self> {
        // Decode the base64 data:
        let data = match URL_SAFE_B64.decode(base64_data) {
            Ok(decoded) => decoded,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Failed to decode base64 data: {}", e),
                ));
            }
        };

        // Retrive encryption key:
        let xor_constant = if !is_gen5 { 0x4A3B2C1D } else { 0x2DB842B2 };
        let gts_checksum = u32::from_be_bytes(
            data[0x00..0x04]
                .try_into()
                .expect("Failed to convert GTS checksum slice to array"),
        ) ^ xor_constant;

        // [Gen 4] Decrypt the profile ID and Pokémon data:
        let decrypted_data = if !is_gen5 {
            Self::decrypt_stream_cipher_data(&data[0x04..0xF4], gts_checksum | gts_checksum << 16)
        } else {
            data
        };

        // Fill in the fields:
        let profile_id = u32::from_le_bytes(
            decrypted_data[0x00..0x04]
                .try_into()
                .expect("Failed to convert profile ID slice to array"),
        );
        let pkm_offset = if !is_gen5 { 0x04 } else { 0x0C };
        let pkm_end = if !is_gen5 { 0xF0 } else { 0xE8 };
        let pokemon = Pokemon::deserialize(&Pokemon::to_decrypted_data(
            &decrypted_data[pkm_offset..pkm_end],
        ));

        Ok(Self {
            gts_checksum,
            profile_id,
            pokemon,
        })
    }

    /// Decrypts the received GTS data using a stream cipher algorithm.
    ///
    /// This is intended to be performed for Gen 4 receptions, on the data after the checksum, once
    /// it is decoded from base64.
    ///
    /// # Arguments
    /// * `encrypted_data` - The encrypted data to decrypt, as a byte slice.
    /// * `mut state` - The initial state of the stream cipher, i.e., the key used to decrypt the
    ///   data.
    fn decrypt_stream_cipher_data(encrypted_data: &[u8], mut state: u32) -> Vec<u8> {
        let mut decrypted_data = Vec::with_capacity(encrypted_data.len());

        for byte in encrypted_data.iter() {
            state = (state.wrapping_mul(0x45) + 0x1111) & 0x7FFFFFFF;
            let keybyte = (state >> 16) as u8;
            decrypted_data.push(byte ^ keybyte);
        }

        decrypted_data
    }
}

/// Struct representing a Pokémon reception from the GTS.
///
/// This struct is used to send a Pokémon to a game from the GTS.
pub struct GTSReception {
    pokemon: Pokemon,
    gts_data: GTSData,
    is_gen5: bool,
}

impl GTSReception {
    /// Creates a new `GTSReception` from a Pokémon.
    ///
    /// # Arguments
    /// * `pokemon` - The Pokémon with which to create the reception.
    pub fn from_pokemon(pokemon: &Pokemon) -> Self {
        // Create the GTS data from the Pokémon:
        let gts_data = GTSData::from_pokemon(pokemon);

        Self {
            pokemon: pokemon.clone(),
            gts_data,
            is_gen5: pokemon.is_gen5(),
        }
    }

    /// Serializes the GTS reception data into a byte vector.
    pub fn serialize(&self) -> Vec<u8> {
        // Create the result vector:
        let mut data;

        // Serialize and append the Pokémon data:
        let pokemon_data = self.pokemon.serialize();
        let pokemon_encrypted_data = Pokemon::to_encrypted_data(&pokemon_data);
        data = pokemon_encrypted_data;

        // Append the Gen 5 padding:
        if self.is_gen5 {
            data.extend(vec![0; 0x10]);
        }

        // Serialize and append the GTS data:
        let gts_data = self.gts_data.serialize(self.is_gen5);
        data.extend(gts_data);

        data
    }
}
