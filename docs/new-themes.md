# Adding New Themes

To create a new theme, it must be added to the [`Themes`](../wally/src/wally/themes/mod.rs) enum.

```rust
pub(crate) enum Themes {
    RosePineDefault,
    RosePineDawn,
    // ...
    Dracula,
}
```

Then, a new file for the theme's implementation must be created. There, create a struct for the new theme. This will store the theme's color palette.

```rust
pub(crate) struct CatppuccinFlavor {
    pub(crate) rosewater: Color,
    pub(crate) flamingo: Color,
    pub(crate) pink: Color,
    // ...
}
```

This new struct must implement the [`ColorPalette`](../wally/src/wally/themes/mod.rs) trait.

```rust
pub(crate) trait ColorPalette {
    fn len(&self) -> usize;

    fn background(&self) -> Color;

    fn get_color(&self, idx: usize) -> Color;
}
```

It should also provide methods to instantiate itself. If the theme only has one color palette, a simple `::new()` will do. If it has several, there should be a `::<palette>()` method for each one.

```rust
impl RosePineFlavor {
    pub(crate) fn default() -> RosePineFlavor {
        RosePineFlavor {
            base: Color::from_u8(0x19, 0x17, 0x24),    // #191724
            surface: Color::from_u8(0x1f, 0x1d, 0x2e), // #1f1d2e
            // ...
        }
    }

    // constructors for other color palettes...
}
```

Now, all that's left is matching the new enum variant in the [`run()`](../wally/src/wally/mod.rs) function!

```rust
match &args.command {
    Commands::Dots {
        palette,
        dot_size,
        steps,
    } => match palette {
        // previous `match` arms...
        Themes::Dracula => mk_dots(&args, Dracula::new(), *dot_size, *steps),
    },
};
```
