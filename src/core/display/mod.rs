//! The core's protocol vocabulary for the display side: state the
//! display writes and the core reads. The display depends on the core,
//! never the other way around. Pure protocol — no systems, no
//! resources, nothing to register.

pub mod components;
