pub mod client;
pub mod const_vars;
pub mod local_macro;
pub mod tests;
pub mod types;
pub mod utils;

pub use client::BingClient;
pub use types::chat_msg_type::EasyMsg;
pub use types::chat_type::Chat;
pub use types::cookie_type::Cookie;
pub use types::plugin_type::Plugin;
pub use types::user_input_type::Tone;
pub use types::user_input_type::UserInput;
pub use utils::image_base64::Image;
