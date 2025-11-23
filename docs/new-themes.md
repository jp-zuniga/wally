# Adding New Themes

To create a new theme, it must be added to the [`Themes`](../src/wally/themes/mod.rs) enum.

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
pub(crate) struct DraculaPalette {
    pub(crate) background: Color,
    pub(crate) line: Color,
    pub(crate) selection: Color,
    // ...
}
```

This new struct must implement the [`ColorPalette`](../src/wally/themes/mod.rs) trait.

```rust
pub(crate) trait ColorPalette {
    fn len(&self) -> usize;

    fn background(&self) -> Color;

    fn get_color(&self, idx: usize) -> Color;
}
```

It should also provide methods to instantiate itself. If the theme only has one color palette, a simple `::new()` will do. If it has several, there should be a `::<palette>()` method for each one.

```rust
impl DraculaPalette {
    pub(crate) fn new() -> DraculaPalette {
        DraculaPalette {
            background: Color::from_u8(0x28, 0x2a, 36), // #282a36
            line: Color::from_u8(0x62, 0x72, 0xa4),     // #6272A4
        }
    }

    // constructors for other color palettes...
}
```

Now, all that's left is matching the new enum variant in the [`run()`](../src/wally/mod.rs) function!

```rust
match &args.command {
    Commands::Dots {
        palette,
        dot_size,
        steps,
    } => match palette {
        // previous `match` arms...
        Themes::Dracula => mk_dots(&args, DraculaPalette::new(), *dot_size, *steps),
    },
};
```
