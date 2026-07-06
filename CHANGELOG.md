# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2026-07-06

### Added

#### `pk_editor`

- Gender editing support (previously read-only)
- Ability editing support (previously read-only)
- UI scaling — all slot sizes, spacing, and widget dimensions scale with window width
- `itertools` dependency for `chunks()` iterator in PC box grid rendering

### Changed

#### `pk_edit`

- Upgraded to `pk_edit` v0.5.0 — see its changelog for full details
- `pk_edit::PokemonTrait::lowest_level()` removed; use `GameData::lowest_level(dex_num)` instead
- `pokemon_info::update()` now takes `&AnyGameData` for level/clamp queries
- `pokemon_info::view()` takes `scale: f32` for responsive layout
- `PokemonFactory::gen_pokemon_from_species()` now takes `&AnyPokemon` (the existing slot) as first arg
- `match_wild_err_arm` lint removed from `Cargo.toml`

#### `pk_editor`

- `pokemon_info::update()` signature expanded with `game_data` parameter
- `PartyBoxes` screen passes `scale` factor to `party()`, `pc_box()`, and `pokemon_info()` views
- `pc_box()` uses `itertools::Itertools::chunks()` for cleaner 6-column grid rendering
- All hardcoded pixel widths replaced with `width * scale` throughout PC, party, and info widgets
- `WINDOW_WIDTH`/`WINDOW_HEIGHT` removed from `pokemon_info.rs` imports
- `PokemonInfo` panel height changed from `WINDOW_HEIGHT` to `Length::Fill`
- Various `as u8` casts replaced with `u8::try_from(...).unwrap_or(0)` for clippy compliance

### Fixed

- `gen_pokemon_from_species` now passes `&selected_pokemon` to preserve slot offset when generating a new Pokémon in an existing slot

## [0.4.0] - 2026-04-04

### Added

#### `pk_edit`

- `OpenSave` enum replaces `SaveFile` as the public save-file handle, enabling dispatch across multiple game generations
- `pk_edit::open()` top-level entry point returns `OpenSave` from raw save bytes
- `AnyPokemon` type alias / enum replaces the concrete `Pokemon` type throughout the public API
- `PokemonTrait` trait for generation-agnostic Pokémon access
- `GameData` / `AnyGameData` trait for generation-aware game metadata
- `ComputedStats` struct holding fully computed stat values at a given level
- `StatBlock` struct holding raw IV/EV values per stat (replaces flat fields on `Stats`)
- `AnyGameData::species()` — load species name list from the active game's data
- `AnyGameData::balls_sprite_ids()` — load Pokéball sprite ID list (replaces hardcoded `balls_id()`)
- `PokemonTrait::update_iv()` / `update_ev()` — replace `Stats::update_ivs()` / `update_evs()`
- `Gen3Pocket` type alias exported for pocket selection in Gen III saves

#### `pk_editor`

- `State::save_file` changed from `SaveFile` to `Option<OpenSave>` — no save is held on startup
- Species combo-box populated lazily from `save_file.game_data().species()` on file open
- Move slots receive full move list via `game_data` for in-slot move selection UI
- `DragStart` message variant drops the cursor-offset parameter (simplified drag state)
- `DragDrop` message variant drops the redundant first parameter

### Changed

#### `pk_edit`

- `Pokemon::set_item()` renamed to `set_held_item()`
- `Stats::update_ivs()` / `update_evs()` superseded by `PokemonTrait::update_iv()` / `update_ev()`
- `Stats::highest_stat()` removed; stat maximum computed directly from `ComputedStats` fields
- `StorageType` re-exported from `pk_edit` crate root (previously `pk_edit::save::storage::StorageType`)

#### `pk_editor`

- All `expect("REASON")` / `unwrap()` call-sites replaced with proper `?` propagation or `unwrap_or_default()`
- `bag::update()` now receives `&mut Gen3SaveFile` directly instead of the opaque `SaveFile`
- `pokemon_info::update()` now receives a `pokemon_factory` and explicit OT fields instead of `&SaveFile`
- `info_label()` and `stats()` view helpers updated to accept `AnyPokemon` + `AnyGameData`
- `State::party` / `current_pc` initialised as empty `Vec` (no dummy defaults on startup)

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
