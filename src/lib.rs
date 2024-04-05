pub mod client;
pub mod const_vars;
pub mod local_macro;
pub mod tests;
pub mod types;
pub mod utils;

pub use client::BingClient;
pub use types::{
    chat_msg_type::EasyMsg,
    chat_type::Chat,
    cookie_type::Cookie,
    plugin_type::Plugin,
    user_input_type::{Tone, UserInput},
};
pub use utils::image_base64::Image;
