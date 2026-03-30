# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-03-29

### Added

#### `pk_edit`

- `Trainer` struct exposing trainer name, gender, ID, time played, money, game version, and security key
- `TrainerID` type with `public`/`private` field split; implements `From<[u8; 4]>`, `Into<Vec<u8>>`, and `Display`
- `TimePlayed` struct with `from_bytes` / `to_bytes`
- `GymBadges` bitfield with `Display`
- `GameVersion` enum: `RubySapphire`, `FireRedLeafGreen`, `Emerald`
- `SaveFile::get_trainer()` — read full trainer metadata from the save
- `SaveFile::get_game_version()` — detect which Gen III game the save belongs to
- `SaveFile::get_money()` / `set_money()` — read and write trainer money
- `SaveFile::swap_pokemon()` — atomically swap two Pokémon between any party/PC slots
- `Pokemon::set_nature()` — set nature by re-rolling PID/IVs while preserving gender and ability
- `Pokemon::set_level()` — set level (clamped 1–100)
- `Pokemon::set_pokeball_caught()` — set Pokéball used during capture
- `Pokemon::set_friendship()` — set friendship value
- `gen_pokemon_from_species()` factory function — create a fresh `Pokemon` from a species name, OT name, and OT ID
- `Pokerus`, `Language`, and `Evolution` types now part of the public API
- `Stats::highest_stat()` — return the name and value of the highest calculated stat
- `Stats::update_ivs()` / `update_evs()` — mutate individual IV/EV values by stat name

#### `pk_editor`

- Drag-and-drop Pokémon swapping between party slots and PC boxes
- `DragState` struct tracking in-flight drag (storage origin, cursor position, sprite handle, index)
- `Message::DragStart` / `DragMoved` / `DragReleased` / `DragDrop` variants
- `State::subscription()` — mouse tracking active only while a drag is in progress
- Sprite/icon image preloading at startup via `load_images()`; `Message::ImagesListed` delivers the result
- `icon::plus()` and `icon::minus()` icon functions
- Native error dialogs (`rfd`) for all async failure paths; inline `State::error` field removed
- Bag screen: per-pocket quantity increment/decrement controls; `Operation::Increment` / `Decrement` / `Change` enum replacing raw text-input quantity editing
- `Error::InvalidItem` and `Error::MissingDirectory` variants added

### Changed

#### `pk_editor`

- `bag::update()` parameters narrowed from `Vec<(String, u16)>` to `&mut [(String, u16)]` for all non-TM pockets
- `iced::application` call updated to builder style (`.title()`, `.subscription()`, `.run()`)
- `icon()` helper lifetime annotation simplified to `'_`

## [0.2.0] - 2026-03-28

### Changed (Breaking)

#### `pk_edit`

- Restructured public modules: `data_structure::pokemon` → `pokemon`; `Pocket` and `StorageType` moved to `save::storage`
- Renamed `Pokemon::give_item()` → `set_item()`, `Pokemon::held_item()` → `item()`
- Converted getter methods to public fields: `Pokemon::offset()` → `Pokemon::offset`, `Pokemon::personality_value()` → `Pokemon::personality_value`, `Pokemon::stats()` → `Pokemon::stats`

#### `pk_editor`

- Updated all import paths to reflect `pk_edit` module restructure
- Replaced `Vec::clone()` with `slice::to_owned()` at bag save boundaries
- Simplified closure-style message mapping to function-pointer style throughout view functions

### Fixed

- Bag update function signature narrowed from `Vec` to slice (`&mut [(String, u16)]`) for `tm_bag`

## [0.1.2] - (previous release)

_No changelog entries recorded prior to 0.2.0._
