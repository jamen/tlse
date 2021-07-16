use std::os::raw::{c_float, c_long, c_uchar, c_ulong};

use winapi::shared::d3d9::{IDirect3D9, IDirect3DDevice9};
use winapi::shared::d3d9caps::D3DCAPS9;
use winapi::shared::d3d9types::{D3DDISPLAYMODE, D3DPRESENT_PARAMETERS};
use winapi::shared::windef::HWND;

use crate::{
    BoostScopedPtr, C2DBoxF, C2DBoxI, C2DExtentsI, CArray, CBaseClassNonCopyable, CCountedPointer,
    CDeviceResetCallback, CRenderManager, CSystemManager, CWideString, CxxList,
};

#[repr(C)]
pub struct CDisplayManager {
    pub vmt: *mut (),
    pub c_base_class_non_copyable: CBaseClassNonCopyable,
    pub system_manager: *mut CSystemManager,
    pub p_render_manager: CCountedPointer<CRenderManager>,
    pub render_target_surface: CSurface,
    pub render_target_depth_buffer: CSurface,
    pub back_buffer_surface: CSurface,
    pub depth_buffer_surface: CSurface,
    pub back_buffer_copy_texture: CTexture,
    pub d3d: *mut IDirect3D9,
    pub d3d_device: *mut IDirect3DDevice9,
    pub d3d_device_caps: D3DCAPS9,
    pub back_buffer_dimensions: C2DExtentsI,
    pub render_target_dimensions: C2DExtentsI,
    pub back_buffers_count: c_long,
    pub exclusive_mode: bool,
    pub multisample_type: ESurfaceMultisampleType,
    pub back_buffer_format: CPixelFormat,
    pub depth_format_type: CDepthBufferFormat,
    pub using_render_manager: bool,
    pub using_shader_render_manager: bool,
    pub window_handle: HWND,
    pub current_mode: D3DDISPLAYMODE,
    pub viewport_box: C2DBoxI,
    pub viewport_box_float: C2DBoxF,
    pub rendering: bool,
    pub can_begin_render: bool,
    pub current_screen_index: c_long,
    pub present_params: D3DPRESENT_PARAMETERS,
    pub virtual_coords_resolution_independent: bool,
    pub device_lost_flag: bool,
    pub shader_resource_path: CWideString,
    pub frame_counter: c_long,
    pub render_manager_init_flags: c_ulong,
    pub device_reset_callbacks: CxxList<*mut CDeviceResetCallback>,
    pub last_device_creation_parameters: self::CCreateDeviceParameters,
    pub vsync_callback_timer: BoostScopedPtr<CVsyncCallbackTimer>,
    pub back_buffer_formats: CArray<CPixelFormat>,
    pub multisample_types: CArray<ESurfaceMultisampleType>,
    pub busy_resource_manager: CBusyResourceManager,
    pub gamma: c_float,
    pub want_hi_res_screenshot: bool,
    pub taking_screenshot: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct CDisplayManagerInit {
    pub width: c_long,
    pub height: c_long,
    pub depth: c_long,
    pub refresh_rage: c_long,
    pub use_render_manager: bool,
    pub use_shader_render_manager: bool,
    pub window_handle: HWND,
    pub z_buffer_depth: c_uchar,
    pub back_buffer_count: c_uchar,
    pub multisample_type: ESurfaceMultisampleType,
    pub push_buffer_size: c_long,
    pub kick_off_size: c_long,
    pub present_immediate: bool,
    pub present_immediately_if_missed_vsync: bool,
    pub presentation_interval_vsync_count: c_long,
    pub skip_config_detection: bool,
    pub render_manager_init_flags: c_ulong,
    pub shader_resource_path: CWideString,
}

// TODO
#[derive(Debug)]
#[repr(C)]
pub struct CSurface {}

// TODO
#[derive(Debug)]
#[repr(C)]
pub struct CTexture {}

// TODO
#[derive(Debug)]
#[repr(C)]
pub struct CPixelFormat {}

// TODO
#[derive(Debug)]
#[repr(C)]
pub struct CDepthBufferFormat {}

// TODO
#[derive(Debug)]
#[repr(C)]
pub struct CBusyResourceManager {}

// TODO
#[derive(Debug)]
#[repr(C)]
pub struct CCreateDeviceParameters {}

// TODO
#[derive(Debug)]
#[repr(C)]
pub struct CVsyncCallbackTimer {}

#[derive(Debug)]
#[repr(C)]
pub enum ESurfaceMultisampleType {
    SURFACE_MULTISAMPLE_NONE = 0,
    SURFACE_MULTISAMPLE_2 = 2,
    SURFACE_MULTISAMPLE_4 = 4,
    SURFACE_MULTISAMPLE_9 = 9,
    SURFACE_MULTISAMPLE_3 = 3,
    SURFACE_MULTISAMPLE_5 = 5,
    SURFACE_MULTISAMPLE_6 = 6,
    SURFACE_MULTISAMPLE_7 = 7,
    SURFACE_MULTISAMPLE_8 = 8,
    SURFACE_MULTISAMPLE_10 = 10,
    SURFACE_MULTISAMPLE_11 = 11,
    SURFACE_MULTISAMPLE_12 = 12,
    SURFACE_MULTISAMPLE_13 = 13,
    SURFACE_MULTISAMPLE_14 = 14,
    SURFACE_MULTISAMPLE_15 = 15,
    SURFACE_MULTISAMPLE_16 = 16,
}
