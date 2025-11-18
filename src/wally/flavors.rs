use clap::ValueEnum;

use super::img::Color;

pub const COLOR_COUNT: usize = 14;

#[derive(Clone, Copy, Debug, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub(crate) enum RosePineFlavorNames {
    Default,
    Dawn,
    Moon,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RosePineFlavor {
    pub(crate) base: Color,
    pub(crate) surface: Color,
    pub(crate) overlay: Color,
    pub(crate) muted: Color,
    pub(crate) subtle: Color,
    pub(crate) text: Color,
    pub(crate) love: Color,
    pub(crate) gold: Color,
    pub(crate) rose: Color,
    pub(crate) pine: Color,
    pub(crate) foam: Color,
    pub(crate) iris: Color,
    pub(crate) h_low: Color,
    pub(crate) h_med: Color,
    pub(crate) h_high: Color,
}

impl RosePineFlavor {
    pub fn get(&self, idx: usize) -> Color {
        match idx {
            0 => self.base,
            1 => self.surface,
            2 => self.overlay,
            3 => self.muted,
            4 => self.subtle,
            5 => self.text,
            6 => self.love,
            7 => self.gold,
            8 => self.rose,
            9 => self.pine,
            10 => self.foam,
            11 => self.iris,
            12 => self.h_low,
            13 => self.h_med,
            14 => self.h_high,
            _ => unreachable!(),
        }
    }
}
