# Custom GTS Server in Rust 🦀

This Rust application allows the hosting of a HTTP server to which retail Nintendo DS cartridges can connect. It makes use of the PokemonClassic network to get the required certificates. It enables transferring Pokémon to and from retail Nintendo DS cartridges. It is compatible with all Generation IV and V Pokémon games (Diamond, Pearl, Platinum, HeartGold, SoulSilver, Black, White, Black 2, and White 2).

This is a Rust re-implementation of the [original IR-GTS-MG Python script](https://github.com/ScottehMax/IR-GTS-MG/tree/gen-5). You can read more about its implementation details in the [associated documentation](docs/app.md). You can read more about the differences in functionality with the Python script in the [changes](docs/changes.md) file.

## Requirements

 - Rust and Cargo, version 1.87.0 or later ([available here](https://www.rust-lang.org/tools/install))
 - Generation IV or V Pokémon game
 - A supported device to run the game on, either:
   - DS family console (DS, DSi, 3DS, or variants)
   - melonDS emulator (not BizHawk)
 - Wireless network (WEP or passwordless for console, any for emulator)
 - Administrator privileges

## Installation

 1. Clone the repository to your local machine:
```
git clone https://github.com/0xb01u/gts-rs.git
```
 * You can also download the repository as a ZIP and extract it, instead.

 2. Navigate to the cloned directory:
```
cd <path/to/project/root>
```

 3. Build the project using Cargo:
```
cargo build --release
```
 * The resulting program binary (`gts-rs`) will be located in the `target/release` directory.
 * You may omit the `--release` flag to build in debug mode, which will produce a binary in the `target/debug` directory instead.

## Setting up the network: Console

Before proceeding, it's important to ensure that your computer/server hosting the application can be accessed from the network the Nintendo DS is connected to. **Generation IV and V Pokémon games only support connecting to networks with WEP encryption or no password at all**. This can be tricky, as modern routers do not all support this insecure protocol. Additionally, Windows 11 has removed the ability to connect to insecure networks, so both devices can't be on the same network. Nevertheless, there are still ways to let the two devices connect.

### Option 1: Router configuration

If possible, configure your router to host a separate network that uses WEP encryption or has no password. Ensure that both your host machine and the Nintendo DS are connected to the same router. The host machine does not necessarily need to connect to the unsecure network, as long as it's a network on the same router.

### Option 2: Use a mobile hotspot

If your router doesn't support creating a second subnet or lacks WEP/passwordless options, you can try to use an (old) phone to create a Hotspot. Some modern phones still allow creating insecure Hotspots while on a network too.

 1. Connect the phone to the same network your host machine is on.

 2. Create a hotspot with WEP encryption or no security/password.

 3. Connect the Nintendo DS to this hotspot.

Since the phone is on the same subnet as the host machine, it should be able to route traffic to it.

### Option 3: Using a Hotspot with data (Linux/Windows 10 and below)

Some modern phones allow creating insecure Hotspots, but not while connected to a network. This can still be used for the Nintendo DS, but Windows 11 generally won't let you connect to it. If you're using certain Linux distributions or Windows 10 and below, you could connect both the host machine and ds to the phone.

Just be aware that this will use data.

### Option 4: Port-forwarding

If you're unable to have your host machine and Nintendo DS on the same network but can connect the DS to an insecure network (such as using a hotspot with data, but the host machine uses Windows 11), you have the option to port-forward the host machine. This allows the public IP of the host machine to be reached from the Nintendo DS.

The exact steps to perform this are highly dependent on the router/provider you have, so this won't be explained here.

## Setting up the network: Emulator

The melonDS emulator has a built-in WIFI network, which uses the same network as the machine the emulator is running on.

In the simplest scenario, you can run the application on the same machine as the emulator. If you are unable to run the application on the same machine, it can be run on another machine in the same network or in an outside network, as long as your machine is able to reach it.

# Usage

You can run the application from the root of the project, as follows:
```
target/release/gts-rs
```
or, for a debug build:
```
target/debug/gts-rs
```
Remember that you need to run the application with administrator/superuser privileges. Therefore, on Linux, you may need to use `sudo` (e.g., `sudo target/release/gts-rs`).

If you get an `"Address already in use"` error when running the application, you may need to turn off the default domain name resolver on your machine, if it creates a local DNS server. Otherwise, the application will fail to create the DNS server for incoming requests, as another application is already bound to the DNS address of the machine. For example, for Ubuntu, you should turn off the `systemd-resolved` service, like this:
```
sudo systemctl stop systemd-resolved
```
**Do not forget to turn it back on after you stop the application!** For example, for Ubuntu:
```
sudo systemctl start systemd-resolved
```

After running the application, a message will be displayed indicating the IP used for the servers, similar to the following:
```
GTS-RS servers running on IP: XXX.XXX.XXX.XXX
```
Make note of the IP address displayed in the message.

On your emulator or console of choice, you have to set the network configuration as follows:
 1. Boot up a game and navigate to `NINTENDO WFC SETTINGS` in the startup menu, then `Nintendo Wi-Fi Connection Settings`.
 2. Create a new connection and connect to the insecure network (console), or edit the existing connection (emulator).
 3. Set the Primary DNS to the IP address noted earlier. The Secondary should be left blank/the same as the Primary.

### Send a Pokémon to the game

To send a Pokémon file using the GTS (Global Trade Station), follow these steps:

 1. Enter the GTS within the Pokémon game.
 2. When prompted, drag the `.pkm`/`.pk4`/`.pk5` file you want to send into the prompt window, or type/copy-and-paste the path to the file. After a short time, the Pokémon will appear on the DS and be placed in either an empty spot in your party or the first available PC box. This can take a few seconds, as for some reason the connection for this command is rather slow.

Note: Sending more than one Pokémon at a time is not possible. You'll need to exit and re-enter the GTS to send another Pokémon.

### Receive a Pokémon from the game

Whenever you offer a Pokémon in the GTS, its data will be received on the host machine automatically. You will receive an error on the DS stating that the Pokemon cannot be offered for trade - this ensures the Pokémon remains in your game. The application will automatically save the Pokémon under the `pokemon/` directory in the root of the project. It will check if the Pokémon's data has been saved before, to prevent creating duplicates (this will be warned by the application).

## Support

If you encounter an error, please take a screenshot or copy the script output, describe the state of the DS and any associated error codes, and add an issue to Github's issue tracker.

### Credits

 * LordLandon: Original inspiration and groundwork laid by their SendPKM script.
 * ProjectPokemon Community: Extensive documentation on GTS/encryption protocols.
 * Infinite Recursion: Initial development of the script.
 * Shutterbug: For their development of the nds-constraint exploit, without which this project would be impossible.
 * PokeClassic: For the hosting/directing to unofficial servers that provide the necessary certificates.
 * jamiejquinn: Update to function after the shutdown of Nintendo's online services.
 * rebrunner: Updating the source code from Python 2 to Python 3.
 * RETIREglitch/Jorik Devreese: Complete rewrite of the source code, and new server architecture.
 * ScottehMax: Support for Generation V games.
 * Bolu: Re-implementation of the original Python app in Rust, with a significant refactoring of the logic/codebase.
