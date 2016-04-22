## RAGE - "Rusty Application Gui Extravaganza"
This is very WIP, do not use yet.

Does basic flowing, %, absolute, and relative positioning/layout with borders, margin, padding.

`rage/screenshots` has a comparison between "RageFox" and FireFox (RageFox just renders a .png for the content, no interest in rendering html).
This layout is defined in `src/test_fixture.rs`.

# Dependencies
SDL, Piston

### Built With
This is currently only needed due to a reliance on the unstable `rc_counts` (see scene_graph) for debugging purposes.
rustc 1.10.0-nightly (a0c325980 2016-04-20)
cargo 0.11.0-nightly (bf3f531 2016-04-20)

# Performance
So far pretty good (Tested on Fedora 23 x64 Laptop - i7-4700HQ, 8GB). This is just intended for a very rough idea of performance, actual benchmarking needed later.

The entire tree is [rendered|layed out] if a single node needs [rendering|lay out].
Needs Z depth, change heirarchies, and occlusion fixed to handle this.

#### Idle Usage
- Idle being defined as application running, with focus, and the mouse hovering without moving.
- `Baseline` is a piston application that does nothing (launches window, no renders).
- `Rage` is the rage::web_browser test fixture.
- Reference `gnome-calculator` - 5.3% CPU, ~12.9MiB RAM

##### Debug
Baseline  - 1.3% CPU, ~9.9MiB RAM
Rage      - 1.3% CPU, ~10.3MiB RAM

##### Release
Baseline  - 0.7% CPU, ~9.9MiB RAM
Rage      - 0.7% CPU, ~10.3MiB RAM



# Architecture
This is likely out of date since so much is changing, but it gives the rough idea.
Style is completely Unimplemented. Event's can be seen in the console (Hover, Tap, Drag...).

## WorkFlow
Event -> State -> Style -> Layout -> Geometry -> Render

## Styles (Styles = Appearance + Layout + CustomVars)
Conditional Rules that are evaluated to create the Appearance and Layout.

### Rules
```
let rules: Vec<StyleMaps> = Selectors |> Classes
```

### Scoping priority
```
let computed_styles = Defaults |> Global |> Scoped
```

### What gets applied
```
let applied_styles = computed_styles |> Overrides
```
