//! The input plugin: translates raw input (PC mouse today) into
//! display-independent core intents. Replaceable as a whole — e.g. with
//! touch input — without touching the core. `handle_click` joins the
//! cross-domain chain assembled in main.rs, so this module holds no
//! register of its own.

pub mod systems;
