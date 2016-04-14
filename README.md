# RAGE - `Rusty Application Gui Extravaganza`
#####

This is very WIP, do not use.
`rage/screenshots` has a comparison between RageFox and FireFox (RageFox just renders a .png for the content, no interest in rendering html).

# Dependencies
SDL, Piston

### Built With
rustc 1.8.0-beta.2 (2879d940a 2016-03-22)
cargo 0.9.0-nightly (8fc3fd8 2016-02-29)

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
