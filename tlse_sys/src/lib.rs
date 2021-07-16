//! Partial bindings for Fable.

#![feature(abi_thiscall)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
// #![allow(dead_code)]
// #![allow(unused)]

/// Defines a function pointer as a constant. This is very unsafe, the pointer is behind
/// `MaybeUninit`, and its a bit verbose to call: `unsafe { (example.assume_init())(123) }`.
#[macro_export]
macro_rules! bind {
    (
        $(
            $(#[$fn_attr:meta])*
            $v:vis $(extern $fn_abi:literal)? fn $fn_name:ident (
                $($param_name:ident : $param_type:ty),* $(,)?
            ) $(-> $ret_type:ty)? = $address:expr
        );* $(;)?
    ) => {
        $(
            #[allow(non_upper_case_globals)]
            $(#[$fn_attr])*
            $v const $fn_name: std::mem::MaybeUninit<unsafe $(extern $fn_abi)? fn (
                $($param_name : $param_type),*
            ) $(-> $ret_type)?> = unsafe { std::mem::transmute::<usize,_>($address) };
        )*
    };
}

#[path = "game_text/mod.rs"]
pub mod NGameText;

#[path = "entity_events/mod.rs"]
pub mod NEntityEvents;

#[path = "ui_system/mod.rs"]
pub mod NUISystem;

#[path = "display_view/mod.rs"]
pub mod NDisplayView;

mod array;
mod bank_file;
mod base_class;
mod basic_string;
mod bullet_time_manager;
mod c_3d_animation_manager;
mod camera;
mod char_string;
mod color;
mod counted_pointer;
mod cxx;
mod debug_manager;
mod def_class_base;
mod definition_manager;
mod device_reset_callback;
mod disk_file_win32;
mod display_engine;
mod display_manager;
mod display_view_manager;
mod engine_primitive_handle;
mod environment;
mod faction_manager;
mod font_bank;
mod font_manager;
mod frame_rate_smoother;
mod game;
mod game_camera_manager;
mod game_component;
mod game_definition_manager;
mod game_event;
mod game_player_interface;
mod game_script_interface;
mod game_time_manager;
mod geometry;
mod graphic_data_bank;
mod input_event;
mod input_manager;
mod input_process;
mod linked_list;
mod main_game_component;
mod mesh_bank;
mod music_manager;
mod navigation_manager;
mod network_client;
mod player;
mod player_def;
mod player_manager;
mod profile_manager;
mod render_manager;
mod runtime_data;
mod script_info_manager;
mod sound_bank;
mod system_manager;
mod tc_base;
mod tc_physics_base;
mod thing;
mod thing_creature_base;
mod thing_game_object;
mod thing_manager;
mod thing_physical;
mod thing_player_creature;
mod vector_map;
mod wide_string;
mod world;
mod world_map;

pub use array::*;
pub use bank_file::*;
pub use base_class::*;
pub use basic_string::*;
pub use bullet_time_manager::*;
pub use c_3d_animation_manager::*;
pub use camera::*;
pub use char_string::*;
pub use color::*;
pub use counted_pointer::*;
pub use cxx::*;
pub use debug_manager::*;
pub use def_class_base::*;
pub use definition_manager::*;
pub use device_reset_callback::*;
pub use disk_file_win32::*;
pub use display_engine::*;
pub use display_manager::*;
pub use display_view_manager::*;
pub use engine_primitive_handle::*;
pub use environment::*;
pub use faction_manager::*;
pub use font_bank::*;
pub use font_manager::*;
pub use frame_rate_smoother::*;
pub use game::*;
pub use game_camera_manager::*;
pub use game_component::*;
pub use game_definition_manager::*;
pub use game_event::*;
pub use game_player_interface::*;
pub use game_script_interface::*;
pub use game_time_manager::*;
pub use geometry::*;
pub use graphic_data_bank::*;
pub use input_event::*;
pub use input_manager::*;
pub use input_process::*;
pub use linked_list::*;
pub use main_game_component::*;
pub use mesh_bank::*;
pub use music_manager::*;
pub use navigation_manager::*;
pub use network_client::*;
pub use player::*;
pub use player_def::*;
pub use player_manager::*;
pub use profile_manager::*;
pub use render_manager::*;
pub use script_info_manager::*;
pub use sound_bank::*;
pub use system_manager::*;
pub use tc_base::*;
pub use tc_physics_base::*;
pub use thing::*;
pub use thing_creature_base::*;
pub use thing_game_object::*;
pub use thing_manager::*;
pub use thing_physical::*;
pub use thing_player_creature::*;
pub use vector_map::*;
pub use wide_string::*;
pub use world::*;
pub use world_map::*;

use std::os::raw::{c_char, c_float};

use winapi::shared::minwindef::HINSTANCE;

bind! {
    pub extern "fastcall" fn gf_main(p1: *mut HINSTANCE, p2: *mut c_char, p3: usize) = 0x00402510;
    pub extern "fastcall" fn gf_ay_to_vy(v: c_float) -> c_float = 0x009e1dd0;
}

// // Global values
// bind! {
//     0x0137544a, pub G_FULL_SCREEN, *mut bool;
//     0x013b8648, pub G_ONLY_BUILD_STATIC_MAPS, *mut bool;
//     0x013b8605, pub G_EDIT, *mut bool;
//     0x013b8610, pub G_DELAY_EACH_FRAME_MS, *mut c_ulong;
// }
