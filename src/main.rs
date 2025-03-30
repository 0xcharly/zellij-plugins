use zellij_tile::prelude::*;

mod core;
mod frame;
mod plugin;
mod renderer;

register_plugin!(plugin::UltraCompactBar);
