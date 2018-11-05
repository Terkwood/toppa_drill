<img align="left" width="64px" src="Assets/Textures/Drill.png" />

# Toppa Drill

## What is Toppa Drill?!

Toppa Drill is a space mining simulation developed in Amethyst, a game engine written in Rust.

## Game Mechanics

Currently not playable, since it's under heavy development.

Some progress has been made:
- Hotloadable chunks of tiles, allowing for basically endless worlds
- Saving and loading maps
- Force-based player movement (as long as he has fuel)

Next on the list:
- Collision (gravity will be re-enabled after this is done)
- Drilling-mechanism (requires collision)

Other features left for pre-release:
- Selling ores
- Buying items
- Upgrading the ship
- A 2D lighting system
- Fog of war (player sees only what is close to him, and only the top-most layer of ores. Can be extended by buying a "scanner-module")
- Damage-system (in case the player drills into a lava- or gas-pocket, or hits the walls too hard)
- World-setup in NewGame-Menu (setting the planet and chunk sizes, name of the game, etc)
- Render-setup in Options-Menu (different resolutions, render distances for different PC tiers)

## License

Toppa Drill is a free and open source game distributed under the terms of
a no-selling-permit version of the [MIT License][lm].

[lm]: LICENSE.md

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, is licensed as above, without any additional terms or conditions.
