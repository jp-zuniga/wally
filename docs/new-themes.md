# Adding New Themes

To create a new theme, it must be added to the [`Themes`](../src/wally/themes/mod.rs) enum.

```rust
pub(crate) enum Themes {
    // ...
    Dracula,
    // ...
}
```

Then, a new file for the theme's implementation must be created. A theme is represented by a struct which stores its color palette.

```rust
pub(crate) struct DraculaFlavor {
    pub(crate) background: Color,
    pub(crate) foreground: Color,
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
impl DraculaFlavor {
    pub(crate) fn default() -> DraculaFlavor {
        DraculaFlavor {
            background: Color::from_u8(0x28, 0x2a, 0x36), // #282a36
            foreground: Color::from_u8(0xf8, 0xf8, 0xf2), // #f8f8f2
            // ...
        }
    }

    // constructors for other color palettes...
}
```

Now, all that's left is matching the new enum variant in the [`mk_palette()`](../src/wally/cli.rs)!

```rust
match self.palette {
    // ...
    Themes::Dracula => Box::new(DraculaFlavor::default()),
    // ...
};
```
