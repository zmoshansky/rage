## RAGE - "Rusty Application Gui Extravaganza"
This is very WIP, do not use yet.

Does basic flowing, %, absolute, and relative positioning/layout with borders, margin, padding.

`rage/screenshots` has a comparison between "RageFox" and FireFox (RageFox just renders a .png for the content, no interest in rendering html).
This layout is defined in `src/test_fixture.rs`.

# Dependencies
SDL, Piston

### Built With
rustc 1.8.0-beta.2 (2879d940a 2016-03-22)
cargo 0.9.0-nightly (8fc3fd8 2016-02-29)

# Performance
So far pretty good (Tested on Fedora 23 x64 Laptop - i7-4700HQ, 8GB).
`renderer` is set to re-render everything every frame until Z depth, change heirarchies, and occlusion are worked out.
2-4% CPU, ~9MB RAM (remove images and this drops to ~1-2%).

With "forced rendering" off, it will display if you don't interact with anything (hover incl.)
0% CPU, ~9MB RAM

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
