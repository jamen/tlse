use std::any::Any;
use std::convert::TryFrom;
use std::mem::{self, MaybeUninit};
use std::os::raw::{c_double, c_float, c_ulong};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi::AllocConsole;
use winapi::um::wincon::FreeConsole;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use ilhook::x86::Registers;

use lazy_static::lazy_static;

use tlse_sys::{
    gf_ay_to_vy, C2DBoxF, C2DVector, CBaseObject, CClass, CCountedPointer, CDisplayEngine,
    CFontBank, CFontManager, CGameCameraManager, CGameEventPackageSet, CGameScriptInterface,
    CInputEvent, CInputProcessMain, CInterpolationInfo, CMainGameComponent, CNetworkClient,
    CPlayer, CPlayerManager, CProcessedInput, CRGBColour, CRenderManager, CRenderManager2D,
    CScriptThing, CTCPhysicsBase, CThing, CThingCreatureBase, CThingPlayerCreature, CVectorMap,
    CWideString, CxxList, EInputDeviceType, EInputEventType, EInputKey, ETCInterfaceType,
    UInputEventData,
};

use hlua::Lua;

// mod fable_string_ext;
mod hook;
pub mod script;

// use fable_string_ext::FableStringExt;

static mut G_FULLSCREEN: *mut bool = 0x0137544a_usize as *mut bool;
static mut G_SHOW_INITIAL_MOVIES: *mut bool = 0x01375448_usize as *mut bool;
static mut SHOW_BUILD_NUMBER: *mut bool = 0x013b86e7_usize as *mut bool;
static mut P_MAIN_GAME_COMPONENT: *mut *mut CMainGameComponent =
    0x013b86a0_usize as *mut *mut CMainGameComponent;
static mut GP_RENDER_MANAGER: *mut *mut CRenderManager =
    0x013b8384_usize as *mut *mut CRenderManager;
static mut GP_FONT_MANAGER: *mut *mut CFontManager = 0x013b838c_usize as *mut *mut CFontManager;
static mut VIRTUAL_SCREEN_WIDTH: *mut c_double = 0x013961e8_usize as *mut c_double;
static mut VIRTUAL_SCREEN_HEIGHT: *mut c_double = 0x013961f0_usize as *mut c_double;
static mut VIRTUAL_PIXEL_WIDTH: *mut c_double = 0x013ca800_usize as *mut c_double;
static mut VIRTUAL_PIXEL_HEIGHT: *mut c_double = 0x013ca808_usize as *mut c_double;

static mut CMainGameComponent__ConstantFPS: *mut u32 = unsafe { 0x01375550usize as *mut u32 };
static mut CMainGameComponent__AlwaysUpdateGameFrame: *mut bool =
    unsafe { 0x013b8688usize as *mut bool };

lazy_static! {
    static ref TLSE: Tlse = Tlse::new();
}

pub struct Tlse {
    // This doesn't really work. Fix input handling some other way.
    dedupe_input: AtomicBool,
    dedupe_repl: AtomicBool,
    active: AtomicBool,
    active_time: AtomicUsize,
    input: Arc<Mutex<Vec<u16>>>,
    lua: Arc<Mutex<hlua::Lua<'static>>>,
}

impl Tlse {
    fn new() -> Self {
        Self {
            dedupe_input: AtomicBool::new(false),
            dedupe_repl: AtomicBool::new(false),
            active: AtomicBool::new(false),
            active_time: AtomicUsize::new(0.0f32.to_bits() as usize),
            input: Arc::new(Mutex::new(Vec::new())),
            lua: Arc::new(Mutex::new(Lua::new())),
        }
    }
}

unsafe fn start() {
    AllocConsole();

    let mut lua_lock = TLSE.lua.lock().unwrap();

    script::lua_fable_lib(&mut *lua_lock);

    *G_FULLSCREEN = false;
    *G_SHOW_INITIAL_MOVIES = false;
    // *SHOW_BUILD_NUMBER = true;

    println!(
        "CMainGameComponent__ConstantFPS {:?}",
        *CMainGameComponent__ConstantFPS
    );
    println!(
        "CMainGameComponent__AlwaysUpdateGameFrame {:?}",
        *CMainGameComponent__AlwaysUpdateGameFrame
    );

    println!();

    println!(
        "UInputEventData size {:?}",
        mem::size_of::<UInputEventData>()
    );
    println!("CInputEvent size {:?}", mem::size_of::<CInputEvent>());

    hook! {
        x86,
        // { 0x0042ec7c, JmpBack, title_screen_run_hook },
        // { 0x0042dc94, JmpBack, title_screen_update_hook },
        // { 0x004189c2, JmpBack, game_start_hook },
        // { 0x004162b5, JmpBack, game_update_regular_hook },
        // { 0x00435530, JmpBack, display_engine_do_render_hook },
        // { 0x009dd8f0, JmpBack, draw_2d_text_hook },
        // { 0x009da9f0, JmpBack, render_2d_draw_list_hook },
        // { 0x009d9c80, JmpBack, draw_buffered_polys_hook },
        // { 0x0069e980, JmpBack, view_3d_render_hook },
        { 0x00435000, JmpBack, render_hero_fuckshit_hook },
        { 0x0069b790, JmpBack, game_camera_manager_update_hook },
        { 0x00687fd0, JmpBack, input_process_main_process_input_hook },
        // { 0x004830a1, JmpBack, menu_open_hook },
        // { 0x0047d21c, JmpBack, menu_close_hook_1 },
        // { 0x006b3900, JmpBack, input_process_control_creature__process_input_hook },
    };
}

unsafe extern "cdecl" fn render_hero_fuckshit_hook(regs: *mut Registers, _addr: usize) {
    let mut p_main_game_component = *(P_MAIN_GAME_COMPONENT as *mut *mut CMainGameComponent);
    let mut p_player_manager = &mut *(*p_main_game_component).p_player_manager.0;
    let mut world = &mut *(*p_main_game_component).p_world.0;
    let mut game_camera_manager = &mut *(*world).p_game_camera_manager.0;
    let mut render_manager = (*GP_RENDER_MANAGER) as *mut CRenderManager2D;

    let mut p_main_player = (CPlayerManager::get_main_player.assume_init())(p_player_manager);

    // let mut p_thing_player_creature = CPlayer::get_p_controlled_creature(p_main_player);
    let p_thing = (CPlayer::get_p_player_character_thing.assume_init())(p_main_player);
    let physics_tc = ((p_thing as *mut u8).offset(112) as *mut CTCPhysicsBase);

    let p_cut_scene_main_font = (*p_main_game_component).p_cut_scene_main_font.data;

    let font_height = (gf_ay_to_vy.assume_init())((*p_cut_scene_main_font).font_height as f32);

    // println!(
    //     "console active {:?}",
    //     TLSE.active.load(Ordering::Relaxed)
    // );

    if TLSE.active.load(Ordering::Relaxed) {
        let mut input_lock = TLSE.input.lock().unwrap();

        let input_text = String::from_utf16(&*input_lock).unwrap();

        // Draw text
        (CRenderManager2D::draw_2d_text.assume_init())(
            render_manager,
            &mut C2DVector {
                x: 1000.0,
                y: (*VIRTUAL_SCREEN_HEIGHT as f32 - font_height) / 2.0,
            },
            input_text.into(),
            p_cut_scene_main_font,
            &mut CRGBColour(0xFF_FF_FF_FF),
            0,
        );
    }
}

unsafe extern "cdecl" fn game_camera_manager_update_hook(regs: *mut Registers, _addr: usize) {
    let this_ptr = (*regs).ecx as *mut CGameCameraManager;
}

unsafe extern "cdecl" fn input_process_main_process_input_hook(regs: *mut Registers, _addr: usize) {
    let this_arg = (*regs).ecx as usize as *mut CInputProcessMain;
    let mut param_1 = *(((*regs).esp as usize as *mut u8).offset(4) as *mut *mut CInputEvent);
    // let param_2 = ((*regs).esp as *mut u8).offset(8) as *mut CProcessedInput;

    // hex_table::HexTable::new(32, 0, false, true, false).format(
    //     std::slice::from_raw_parts(
    //         *(((*regs).esp as usize as *mut u8).offset(4) as *mut *mut u8),
    //         124,
    //     ),
    //     &mut std::io::stdout(),
    // );

    match (*param_1).typ {
        EInputEventType::IE_CHAR_PRESSED => {
            if TLSE.active.load(Ordering::Relaxed) {
                let c = char::from_u32((*param_1).data.character as u32);

                if let Some(c) = c {
                    if !c.is_control() {
                        if !TLSE.dedupe_input.fetch_xor(true, Ordering::Relaxed) {
                            let mut input_lock = TLSE.input.lock().unwrap();
                            input_lock.push((*param_1).data.character);
                        }
                    } else {
                        match (*param_1).data.character {
                            // Backspace
                            0x8 => {
                                if TLSE.dedupe_input.fetch_xor(true, Ordering::Relaxed) {
                                    let mut input_lock = TLSE.input.lock().unwrap();
                                    input_lock.pop();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            } else {
                let mut input_lock = TLSE.input.lock().unwrap();
                input_lock.drain(..);
            }
        }
        EInputEventType::IE_KEY_PRESSED => match (*param_1).data.key {
            EInputKey::KB_HASH => {
                if (*param_1).current_time
                    != f32::from_bits(TLSE.active_time.load(Ordering::Relaxed) as u32)
                {
                    TLSE.active_time.store(
                        (*param_1).current_time.to_bits() as usize,
                        Ordering::Relaxed,
                    );
                    let past = TLSE.active.fetch_xor(true, Ordering::Relaxed);
                }
            }
            EInputKey::KB_RETURN => {
                if !TLSE.dedupe_repl.fetch_xor(true, Ordering::Relaxed) {
                    let mut input_lock = TLSE.input.lock().unwrap();

                    let input_text = String::from_utf16(&*input_lock).unwrap();

                    let mut lua_lock = TLSE.lua.lock().unwrap();

                    let _ = lua_lock.execute::<()>(input_text.as_str());
                }
            }
            _ => {}
        },
        EInputEventType::IE_MOUSE_MOVEMENT => {
            println!("mouse {:?}", (*param_1).data.mouse);
        }
        _ => {}
    }

    if TLSE.active.load(Ordering::Relaxed) {
        (*param_1).device_type = EInputDeviceType::INPUT_DEVICE_TYPE_NULL;
        (*param_1).typ = EInputEventType::IE_NULL;
    }
}

// unsafe extern "cdecl" fn input_process_control_creature__process_input_hook(
//     regs: *mut Registers,
//     _addr: usize,
// ) {
//     let param_1 = ((*regs).esp as usize as *mut u8).offset(4) as *mut CInputEvent;

//     println!(
//         "input event {:?} {:?} {:?} {:?} {:?}",
//         (*param_1).device_type,
//         (*param_1).device_number,
//         (*param_1).typ,
//         (*param_1).current_time,
//         (*param_1).start_time,
//     );
// }

// unsafe extern "cdecl" fn menu_open_hook(regs: *mut Registers, _addr: usize) {
//     println!("menu opened");
//     *CMainGameComponent__ConstantFPS = 30;
//     *CMainGameComponent__AlwaysUpdateGameFrame = true;
// }

// unsafe extern "cdecl" fn menu_close_hook_1(regs: *mut Registers, _addr: usize) {
//     println!("menu closed");
//     *CMainGameComponent__ConstantFPS = 15;
//     *CMainGameComponent__AlwaysUpdateGameFrame = false;
// }

unsafe fn stop() {
    FreeConsole();
}

#[no_mangle]
unsafe extern "system" fn DllMain(_: HINSTANCE, msg: DWORD, _: LPVOID) -> BOOL {
    match msg {
        DLL_PROCESS_ATTACH => start(),
        DLL_PROCESS_DETACH => stop(),
        _ => {}
    }
    1 as BOOL
}
