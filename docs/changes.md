# Changes to the original Python implementation

## Changes to the representation of the data

The [original Python implementation](https://github.com/ScottehMax/IR-GTS-MG/tree/gen-5) handled Pokémon and GTS data using the games' [internal representation](https://projectpokemon.org/home/docs/gen-4/pkm-structure-r65/), stored mainly in byte arrays/lists. Although convenient for some GTS-related tasks, working with such raw format is cumbersome in other cases, harder to understand, and uncommon for high-level datatypes (e.g., a Pokémon class). Also, in the original implementation there was certain disconnect between the Generation IV and Generation V code, as support for the latter was added later in development. This made it harder to understand the code and maintain it.

Therefore, the choice was made for the Rust re-implementation to use a more high-level representation of the data to work with, including Pokémon and GTS-related structs (with per-feature fields) and enums. To translate from the internal game representations and these data types, serialization and deserialization methors are implemented.

It is important to note that the data types developed are not exhaustive of the information stored in the games, but rather focus on the data relevant to this application (GTS Pokémon trading).

## Changes to the Pokémon sent

However, due to the changes in the data representation for Pokémon, the Pokémon data sent from this application to the games is not exactly the same as the one sent by the original Python script. More generally, deserialization and serialization of Pokémon data are not _exactly_ reverse operations.

The main reasons for this is the following: The internal data representation presents certain discrepancies between games, that result in sort of reduncancies in the data. For example, Platinum having a different "Met Location" offset than Diamond and Pearl. Some of this redundant data is not included in the high-level representation, merging the redundancies into a single field.

Although initial testing suggest that the current implementation sends semantically equivalent Pokémon to the games (i.e., Pokémon that, even though internally different, are equivalent in all regards that matter for players), further testing is still needed with all the games to ensure that no meaningful information is lost, and that all legality checks are always passed.

Additionally, some useless data (e.g., the mail message) is discarded altogether and zeroed-out when serializing the data; although it is usually non-zero in the games. Nevertheless, this is a very minor issue, as the games also discard that information when storing the Pokémon in boxes, and regenerate it when withdrawing them.
