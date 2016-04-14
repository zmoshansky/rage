# RAGE - `Rusty Application Gui Extravaganza`
#####

# Architecture
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
