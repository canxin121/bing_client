pub mod client;
pub mod const_vars;
pub mod local_macro;
pub mod test;
pub mod types;
pub mod utils;

pub use client::BingClient;
pub use types::{
    plugin_type::Plugin,
    user_input_type::{Tone, UserInput},
};
pub use utils::image_base64::Image;
