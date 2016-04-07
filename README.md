# Architecture
## Flow
Event -> Geometry -> Render

      -> Style  -> Geometry -> Render
                -> Render

`Event -> Geometry`
- Uncommon, Window changed size

`Style -> Geometry`
- Common, Change size of element

## Styles (Styles = Appearance + Geometry + CustomVars)
Multiple merged maps, with later maps overriding earlier ones.

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