# pk_editor

A Pokémon Generation III save file editor built with [Iced](https://docs.rs/iced/latest/iced/).
Provides utilities to view and modify save files from Pokémon games: **Ruby**, **Sapphire**, **Emerald**, **FireRed**, and **LeafGreen**.

Supports **Linux** and **Windows**. Work in progress — unexpected crashes may occur.

---

## Features

### Pokémon Editing
- [x] Generate new Pokémon (produces illegal Pokémon — use at your own risk)
- [x] Edit species
- [x] Edit level
- [x] Edit nature
- [x] Edit friendship
- [x] Edit held item
- [x] Edit moves (up to 4)
- [x] Edit IVs and EVs for all stats
- [x] Edit Pokérus status (None / Infected / Cured)
- [x] Edit Pokéball
- [ ] Edit nickname
- [x] Edit gender
- [x] Edit ability

### Trainer & Bag
- [x] Edit trainer bag (Items, Pokéballs, Berries, TMs, Key Items)
- [ ] Edit trainer info (name, ID, time played)

### Storage
- [x] View and select from party (up to 6 Pokémon)
- [x] View and select from PC boxes (14 boxes × 30 slots)
- [x] Navigate between PC boxes

---

## Screenshots

![Party & Boxes screen](https://github.com/CMIW/pk_editor/blob/main/Screenshots/Screenshot01.png)
![Pokémon info panel](https://github.com/CMIW/pk_editor/blob/main/Screenshots/Screenshot02.png)
![Bag & Trainer screen](https://github.com/CMIW/pk_editor/blob/main/Screenshots/Screenshot03.png)

---

## Building

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or later)
- A C compiler (required by `rusqlite` with the `bundled` feature)

### Steps

```bash
git clone https://github.com/CMIW/pk_editor.git
cd pk_editor
cargo build --release
```

The compiled binary will be at `target/release/pk_editor`.

> **Note:** The build script (`build.rs`) generates an icon font from `fonts/icons.toml` using
> [`iced_fontello`](https://docs.rs/iced_fontello). This runs automatically during `cargo build`.

---

## Usage

1. Launch the application.
2. Click the **Open** icon (folder) in the menu bar to open a `.sav` file.
3. Select a Pokémon from the **Party** or a **PC Box** on the left to view and edit its details on the right panel.
4. Use the **Bag & Trainer** tab to manage bag pockets.
5. Click the **Save** icon (floppy disk) to write the modified save file back to disk.

> **Tip:** Always keep a backup of your original save file before editing.

---

## Architecture

The project is split into two crates:

```
pk_editor/
├── src/           # GUI application (binary)
└── core/pk_edit/  # Save file parsing library
```

### GUI (`src/`)

Built on [Iced](https://docs.rs/iced), following the Elm architecture (Model–Update–View).

| File / Module | Responsibility |
|---|---|
| `main.rs` | Application entry point, `State`, top-level `update` / `view` |
| `message.rs` | Root `Message` enum for all UI events |
| `error.rs` | Application-level `Error` type |
| `theme.rs` | Styling functions for containers, buttons, inputs, and pick lists |
| `icon.rs` | Auto-generated icon font helpers (`open`, `save`, `plus`, `minus`, …) |
| `misc.rs` | Shared constants (`WINDOW_WIDTH`, `WINDOW_HEIGHT`, `PROJECT_DIR`) |
| `screen/party_box.rs` | Party & Boxes screen layout |
| `screen/bag.rs` | Bag & Trainer screen layout and `Message` / `update` |
| `widgets/` | Custom and composite Iced widgets (see below) |

#### Screens

- **PartyBoxes** — shows the trainer's party (left) alongside a navigable PC box grid (right). Selecting any slot opens the Pokémon info panel.
- **BagTrainer** — shows five bag pockets (Items, Pokéballs, Berries, TMs, Key Items) each with item pickers and quantity controls.

#### Custom Widgets

| Widget | Description |
|---|---|
| `party_slot` | 240 × 80 interactive slot showing sprite, nickname, level, and gender |
| `pc_slot` | 80 × 80 interactive slot showing a sprite (or empty) |
| `tab` | Selectable tab with a coloured status bar indicator |
| `gender` | 26 × 26 badge displaying ♀ (pink) or ♂ (blue) |
| `level` | 80 × 26 pill displaying "Lv. N" |
| `stat_bar` | Colour-coded progress bar for Pokémon stats |
| `menu_bar` | Top bar with Open / Save buttons and screen tabs |
| `party` | Composite widget rendering all six party slots |
| `pc` | Composite widget rendering a full 6 × 5 PC box grid with navigation |
| `pokemon_info` | Full editing panel: species, stats, moves, nature, item, Pokérus, OT info |

#### Theming

The application uses the **Dracula** built-in Iced theme with custom semi-transparent dark
overlay containers (`color!(0x000000, 0.5)`), rounded borders, and drop shadows throughout.

### Core Library (`core/pk_edit/`)

A standalone Rust library for reading and writing Generation III save data.

```
core/pk_edit/src/
├── lib.rs              # Public API re-exports
├── error.rs            # PokemonError / SaveDataError types
├── misc.rs             # SQLite database helpers and game data lookups
├── common/
│   └── character_set.rs  # Gen III character encoding ↔ Unicode
├── pokemon/
│   ├── entity.rs       # Pokemon struct (100-byte party / 80-byte PC format)
│   ├── data.rs         # Sub-structures: Growth, Attacks, EVs, Misc blocks + bitfields
│   ├── factory.rs      # Pokemon generation (Method 1 RNG, gen_pokemon_from_species)
│   ├── crypto.rs       # XOR encryption and 16-bit checksum
│   └── stats.rs        # Stat calculation (base + IV + EV + nature modifier)
└── save/
    ├── mod.rs          # SaveFile struct — top-level save management
    ├── section.rs      # 14-section save structure, SectionID enum, checksums
    ├── trainer.rs      # Trainer, TrainerID, TimePlayed, GameVersion, GymBadges
    ├── storage.rs      # StorageType (Party/PC), Pocket enum, pocket encryption
    └── pc.rs           # PCBuffer — 9 sections stitched into 14 × 30-slot PC
```

#### Save File Format

Gen III saves contain two 57,344-byte blocks (A and B) each divided into **14 sections** of
4,096 bytes. The active block is determined by the save index counter. Sections are identified
by `SectionID` and hold trainer info, party data, PC buffers, and bag pockets. Pocket data and
Pokémon sub-structures are XOR-encrypted using a 32-bit security key derived from the trainer ID.

#### Pokémon Data Layout

Each Pokémon is stored as 100 bytes (party) or 80 bytes (PC):

| Bytes | Content |
|---|---|
| 0–3 | Personality value (PID) |
| 4–7 | Original trainer ID (public + secret) |
| 8–17 | Nickname (Gen III character encoding) |
| 18–19 | Language |
| 20–27 | OT name |
| 28 | Pokérus status |
| 29 | Pokéball / met location |
| 30–31 | Origins info (bitfield) |
| 32–35 | IVs / egg / ability (bitfield) |
| 36–47 | Ribbons and obedience (bitfield) |
| 48–79 | Encrypted sub-structure (Growth + Attacks + EVs + Misc) |
| 80–99 | Party data (stats, current HP, condition — not encrypted) |

The order of the four sub-structures is determined by `PID % 24` (one of 24 permutations).

#### Game Data

Species stats, move data, item names, abilities, and experience tables are stored in a bundled
SQLite database (`pk_edit.db`) extracted at startup via `pk_edit::misc::extract_db`.

---

## Dependencies

| Crate | Purpose |
|---|---|
| [iced 0.14](https://docs.rs/iced) | GUI framework (Elm architecture) |
| [tokio](https://docs.rs/tokio) | Async file I/O |
| [rfd](https://docs.rs/rfd) | Native file open / save dialogs |
| [thiserror](https://docs.rs/thiserror) | Ergonomic error type derivation |
| [include_dir](https://docs.rs/include_dir) | Embed assets folder at compile time |
| [pk_edit](core/pk_edit) | Gen III save file parsing (local) |
| [byteorder](https://docs.rs/byteorder) | Endian-aware integer I/O |
| [rusqlite](https://docs.rs/rusqlite) | Bundled SQLite for game data |
| [modular-bitfield](https://docs.rs/modular-bitfield) | Bitfield structs for Pokémon data |
| [lcg-rand](https://docs.rs/lcg-rand) | LCG RNG for Method 1 PID/IV generation |

---

## Documentation

```bash
cargo doc --open               # GUI crate
cargo doc --open --manifest-path core/pk_edit/Cargo.toml   # core library
```

---

## License

This project does not currently include a license file. All rights reserved by the author unless stated otherwise.
