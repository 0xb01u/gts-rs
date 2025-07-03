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
use bimap::{BiHashMap, BiMap};
use serde_json::from_str;
use std::{collections::HashMap, fs::read_to_string, sync::LazyLock};

use crate::gts::Geonet;

/// Character map for Gen4 and Gen5 Pokémon games, for character encoding.
///
/// Maps character IDs to their corresponding UTF-16 characters, and inversely.
pub static CHARMAP: LazyLock<BiMap<u16, char>> = LazyLock::new(|| {
    // Create BiHashMap:
    let mut character_map = BiHashMap::new();

    // Read character map from JSON:
    let map = from_str::<HashMap<String, HashMap<String, String>>>(
        read_to_string("data/char_map.json")
            .expect("Failed to read the character map file")
            .as_str(),
    )
    .expect("Couldn't parse charmap as valid JSON for a `HashMap<String, HashMap<u16, char>>>`");
    let charmap = map
        .get("characters")
        .expect("Failed to load character map: no 'characters' key found");

    // Convert the HashMap to a BiHashMap:
    for (id, character) in charmap.iter() {
        // Convert the character from String to char:
        character_map.insert(
            u16::from_str_radix(id, 16).expect(format!("Invalid u16 number: {}", id).as_str()),
            character.chars().collect::<Vec<char>>()[0],
        );
    }

    character_map
});

pub static NATURES: LazyLock<BiMap<u16, String>> = LazyLock::new(|| {
    // Create BiHashMap:
    let mut natures_map = BiHashMap::new();

    // Read nature names from JSON:
    let nature_names = from_str::<Vec<String>>(
        read_to_string("data/natures.json")
            .expect("Failed to read natures.json file")
            .as_str(),
    )
    .expect("Couldn't parse natures.json as valid JSON for a `Vec<String>>`");

    // Populate the BiHashMap with the nature IDs and names:
    for (i, name) in nature_names.iter().enumerate() {
        natures_map.insert(i as u16, name.clone());
    }

    natures_map
});

pub static NATURE_MODIFIERS: LazyLock<Vec<[f32; 5]>> = LazyLock::new(|| {
    // Read nature modifiers from JSON:
    from_str::<Vec<[f32; 5]>>(
        read_to_string("data/nature_modifiers.json")
            .expect("Failed to read nature_modifiers.json file")
            .as_str(),
    )
    .expect("Couldn't parse nature_modifiers.json as valid JSON for a `Vec<[f32; 5]>>`")
});

pub static SPECIES: LazyLock<BiMap<u16, String>> = LazyLock::new(|| {
    // Create BiHashMap:
    let mut species_map = BiHashMap::new();

    // Read species names from JSON:
    let species_names = from_str::<Vec<String>>(
        read_to_string("data/species.json")
            .expect("Failed to read species.json file")
            .as_str(),
    )
    .expect("Couldn't parse species.json as valid JSON for a `Vec<String>>`");

    // Populate the BiHashMap with the species IDs and names:
    for (i, name) in species_names.iter().enumerate() {
        species_map.insert(i as u16, name.clone());
    }

    species_map
});

pub static ITEMS_GEN4: LazyLock<BiMap<u16, String>> = LazyLock::new(|| {
    // Create BiHashMap:
    let mut item_map = BiHashMap::new();

    // Read item names from JSON:
    let item_names = from_str::<Vec<String>>(
        read_to_string("data/items.json")
            .expect("Failed to read items.json file")
            .as_str(),
    )
    .expect("Couldn't parse items.json as valid JSON for a `Vec<String>>`");

    // Populate the BiHashMap with the item IDs and names:
    for (i, name) in item_names.iter().enumerate() {
        item_map.insert(i as u16, name.clone());
    }

    item_map
});

pub static ITEMS_GEN5: LazyLock<BiMap<u16, String>> = LazyLock::new(|| {
    // Create BiHashMap:
    let mut item_map = BiHashMap::new();

    // Read item names from JSON:
    let item_names = from_str::<Vec<String>>(
        read_to_string("data/itemsg5.json")
            .expect("Failed to read items_gen5.json file")
            .as_str(),
    )
    .expect("Couldn't parse itemsg5.json as valid JSON for a `Vec<String>>`");

    // Populate the BiHashMap with the item IDs and names:
    for (i, name) in item_names.iter().enumerate() {
        item_map.insert(i as u16, name.clone());
    }

    item_map
});

pub static ABILITIES: LazyLock<BiMap<u16, String>> = LazyLock::new(|| {
    // Create BiHashMap:
    let mut ability_map = BiHashMap::new();

    // Read ability names from JSON:
    let ability_names = from_str::<Vec<String>>(
        read_to_string("data/abilities.json")
            .expect("Failed to read abilities.json file")
            .as_str(),
    )
    .expect("Couldn't parse abilities.json as valid JSON for a `Vec<String>>`");

    // Populate the BiHashMap with the ability IDs and names:
    for (i, name) in ability_names.iter().enumerate() {
        ability_map.insert(i as u16, name.clone());
    }

    ability_map
});

pub static MOVES: LazyLock<Vec<String>> = LazyLock::new(|| {
    from_str::<Vec<String>>(
        read_to_string("data/moves.json")
            .expect("Failed to read moves.json file")
            .as_str(),
    )
    .expect("Couldn't parse moves.json as valid JSON for a `Vec<String>>`")
});

pub static HIDDEN_POWERS: LazyLock<Vec<String>> = LazyLock::new(|| {
    from_str::<Vec<String>>(
        read_to_string("data/hidden_power.json")
            .expect("Failed to read hidden_power.json file")
            .as_str(),
    )
    .expect("Couldn't parse hidden_power.json as valid JSON for a `Vec<String>>`")
});

pub static GAMES: LazyLock<BiMap<u8, String>> = LazyLock::new(|| {
    // Create BiHashMap:
    let mut games_map = BiMap::new();

    // Read game names from JSON:
    let game_names = from_str::<Vec<String>>(
        read_to_string("data/games.json")
            .expect("Failed to read games.json file")
            .as_str(),
    )
    .expect("Couldn't parse games.json as valid JSON for a `Vec<String>>`");

    // Populate the BiHashMap with the game IDs and names:
    for (i, name) in game_names.iter().enumerate() {
        games_map.insert(i as u8, name.clone());
    }

    games_map
});

pub static BASE_STATS: LazyLock<Vec<[u8; 7]>> = LazyLock::new(|| {
    from_str::<Vec<[u8; 7]>>(
        read_to_string("data/base_stats.json")
            .expect("Failed to read base_stats.json file")
            .as_str(),
    )
    .expect("Couldn't parse base_stats.json as valid JSON for a `Vec<[u8; 7]>>`")
});

pub static LEVEL_CURVES: LazyLock<[[u32; 6]; 101]> = LazyLock::new(|| {
    from_str::<Vec<[u32; 6]>>(
        read_to_string("data/level_curves.json")
            .expect("Failed to read level_curves.json file")
            .as_str(),
    )
    .expect("Couldn't parse level_curves.json as valid JSON for a `Vec<[u8; 6]>>`")
    .try_into()
    .expect("Couldn't convert level_curves.json to a fixed-size array of 101 elements")
});

pub static GEONET_GEN5: LazyLock<Geonet> = LazyLock::new(|| {
    from_str::<Geonet>(
        read_to_string("data/geonet5.json")
            .expect("Failed to read geonet5.json file")
            .as_str(),
    )
    .expect("Couldn't parse geonet_gen5.json as valid JSON for a `Geonet`")
});
