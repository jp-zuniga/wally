use colored::control::{set_override, unset_override};

pub(crate) fn set_color_output(flag_state: Option<bool>) {
    match flag_state {
        Some(true) => {
            set_override(true);
        }
        Some(false) => {
            set_override(false);
        }
        None => {
            unset_override();
        }
    }
}
