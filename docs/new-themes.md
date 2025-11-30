# Adding New Themes

To create a new theme, it must first be added to the [`WallyPalettes`](../src/wally/themes/mod.rs#L29) enum. If it has several variants or color palettes, they should all be added as enum variants.

```rust
pub(crate) enum WallyPalettes {
    // ...
    Nord,
    // ...
    SolarizedDark,
    SolarizedLight,
    // ...
}
```

Then, a new file for the theme's implementation must be created. There, the `Palette` struct needs to be extended with methods that instantiate the new theme.

> `Palette` defines a standardized, terminal-like suite of color names (e.g., `foreground`, `base`, `yellow`, etc.) to simplify different themes' implementations.

If the theme has several color palettes, there should be methods that instantiate each one.

```rust
impl Palette {
    pub(crate) fn sol_dark() -> Self {
        Palette {
            foreground: Color::from_u8(0x83, 0x94, 0x96), // #839496
            accent: Color::from_u8(0x07, 0x36, 0x42),     // #073642
            // ...
        }
    }

    pub(crate) fn sol_light() -> Self {
        // ...
    }
}
```

Lastly, the new enum variant must be matched in [`WallyArgs::mk_palette()`](../src/wally/cli/args.rs#L85).

```rust
match self.palette {
    // ...
    WallyPalettes::Nord => Palette::nord(),
    // ...
    WallyPalettes::SolarizedDark => Palette::sol_dark(),
    WallyPalettes::SolarizedLight => Palette::sol_light(),
    // ...
};
```

And that's it! Once these changes are merged into the next release, the new theme will appear to users when running `wally themes`
