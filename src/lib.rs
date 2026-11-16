// Copyright (C) 2026 Mai Ooizumi
// SPDX-License-Identifier: AGPL-3.0-or-later

mod config;

use plugify::{Arr, Str, register_plugin};
use rand::seq::IndexedRandom;
use std::{
    error::Error,
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Path, PathBuf},
    sync::Mutex,
};

const COMMAND: &str = "imomoe_chat_prefix_reload";
struct State {
    prefixes: Vec<String>,
    registered: bool,
    active: bool,
}

static STATE: Mutex<State> = Mutex::new(State {
    prefixes: Vec::new(),
    registered: false,
    active: false,
});

// These values match the S2SDK command callback ABI.
const SERVER_SLOT: i32 = -1;
const CONSOLE_CONTEXT: i32 = 0;
const HANDLED: i32 = 2;
const NO_FLAGS: i64 = 0;
const PRE_HOOK: u8 = 0;
type CommandCallback = extern "C" fn(i32, i32, &Arr<Str>) -> i32;
type AddCommand = unsafe extern "C" fn(&Str, &Str, i64, CommandCallback, u8) -> bool;
type RemoveCommand = unsafe extern "C" fn(&Str, CommandCallback) -> bool;

// Plugify's Rust module populates these from the S2SDK command exports.
#[unsafe(no_mangle)]
static mut __s2sdk_AddConsoleCommand: Option<AddCommand> = None;
#[unsafe(no_mangle)]
static mut __s2sdk_RemoveCommand: Option<RemoveCommand> = None;

fn config_path() -> Result<PathBuf, &'static str> {
    let dir = plugify::CONFIGS_DIR
        .get()
        .ok_or("Plugify config directory unavailable")?;
    Ok(PathBuf::from(dir).join("imomoe_chat_prefix/prefix.toml"))
}

fn reload(path: &Path) -> Result<(), String> {
    let candidate = config::load(path, false)?;
    let mut state = STATE.lock().unwrap_or_else(|e| e.into_inner());
    if !state.active {
        return Err("plugin is not active".into());
    }
    state.prefixes = candidate;
    Ok(())
}

extern "C" fn on_reload(slot: i32, context: i32, _: &Arr<Str>) -> i32 {
    if slot != SERVER_SLOT || context != CONSOLE_CONTEXT {
        return HANDLED;
    }
    if catch_unwind(|| {
        if !STATE.lock().unwrap_or_else(|e| e.into_inner()).active {
            return;
        }
        match config_path()
            .map_err(str::to_owned)
            .and_then(|path| reload(&path))
        {
            Ok(()) => {
                plugify::info!("imomoe_chat_prefix: prefix reloaded");
            }
            Err(error) => {
                plugify::error!("imomoe_chat_prefix: reload rejected: {}", error);
            }
        }
    })
    .is_err()
    {
        plugify::error!("imomoe_chat_prefix: reload panicked");
    }
    HANDLED
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn GetChatPrefix(out: &mut Str) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        let state = STATE.lock().unwrap_or_else(|e| e.into_inner());
        let prefix = match (state.active, state.prefixes.as_slice()) {
            (false, _) | (_, []) => return false,
            (_, [only]) => only,
            (_, choices) => choices
                .choose(&mut rand::rng())
                .expect("nonempty prefix list"),
        };
        out.set(prefix);
        true
    }))
    .unwrap_or(false)
}

fn start() -> Result<(), Box<dyn Error>> {
    let result = catch_unwind(|| -> Result<(), Box<dyn Error>> {
        let (Some(add), Some(_)) = (unsafe { __s2sdk_AddConsoleCommand }, unsafe {
            __s2sdk_RemoveCommand
        }) else {
            return Err("missing S2SDK command bindings".into());
        };
        let path = config_path()?;
        let initial = config::load(&path, true);
        // No FCVAR_CLIENT_CAN_EXECUTE; the callback also checks the caller.
        let registered = unsafe {
            add(
                &Str::from_str(COMMAND),
                &Str::from_str("Reload the shared chat prefix"),
                NO_FLAGS,
                on_reload,
                PRE_HOOK,
            )
        };
        if !registered {
            return Err(format!("could not register {COMMAND}").into());
        }
        STATE.lock().unwrap_or_else(|e| e.into_inner()).registered = true;
        let prefixes = match initial {
            Ok(prefixes) => prefixes,
            Err(error) => {
                plugify::error!(
                    "imomoe_chat_prefix: {}; prefix unavailable until a successful reload",
                    error
                );
                Vec::new()
            }
        };
        let mut state = STATE.lock().unwrap_or_else(|e| e.into_inner());
        state.prefixes = prefixes;
        state.active = true;
        Ok(())
    })
    .unwrap_or_else(|_| Err("startup panicked".into()));
    if let Err(error) = result {
        // A cosmetic dependency must not abort the Plugify host's startup.
        plugify::error!("imomoe_chat_prefix: {}; shared prefix unavailable", error);
    }
    Ok(())
}

fn end() -> Result<(), Box<dyn Error>> {
    catch_unwind(|| {
        {
            let mut state = STATE.lock().unwrap_or_else(|e| e.into_inner());
            state.active = false;
            state.prefixes.clear();
            if !state.registered {
                return Ok(());
            }
        }
        let remove = unsafe { __s2sdk_RemoveCommand }
            .ok_or("imomoe_chat_prefix: missing RemoveCommand binding")?;
        if unsafe { remove(&Str::from_str(COMMAND), on_reload) } {
            STATE.lock().unwrap_or_else(|e| e.into_inner()).registered = false;
            Ok(())
        } else {
            Err(format!("imomoe_chat_prefix: could not remove {COMMAND}").into())
        }
    })
    .unwrap_or_else(|_| Err("imomoe_chat_prefix: shutdown panicked".into()))
}
register_plugin!(start: start, end: end);

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    // An in-memory Plugify host for exercising the exported string and command ABI.
    unsafe extern "C" fn string_new(data: *const u8, size: usize) -> Str {
        let bytes = if size == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(data, size) }
        };
        let text = String::from_utf8(bytes.to_vec()).unwrap();
        unsafe {
            std::mem::transmute::<[usize; 3], Str>([Box::into_raw(Box::new(text)) as usize, 0, 0])
        }
    }

    unsafe fn text_ptr(value: *const Str) -> *mut String {
        unsafe { *value.cast::<usize>() as *mut String }
    }

    unsafe extern "C" fn string_drop(value: *mut Str) {
        drop(unsafe { Box::from_raw(text_ptr(value)) });
    }

    unsafe extern "C" fn string_data(value: *const Str) -> *mut u8 {
        unsafe { (&*text_ptr(value)).as_ptr().cast_mut() }
    }

    unsafe extern "C" fn string_len(value: *const Str) -> usize {
        unsafe { (&*text_ptr(value)).len() }
    }

    unsafe extern "C" fn string_assign(value: *mut Str, data: *const u8, size: usize) {
        let bytes = unsafe { std::slice::from_raw_parts(data, size) };
        *unsafe { &mut *text_ptr(value) } = String::from_utf8(bytes.to_vec()).unwrap();
    }

    unsafe extern "C" fn vector_new(_: *const Str, _: usize) -> Arr<Str> {
        unsafe { std::mem::transmute::<[usize; 3], Arr<Str>>([0; 3]) }
    }

    unsafe extern "C" fn vector_drop(_: *mut Arr<Str>) {}

    unsafe extern "C" fn log(
        _: plugify::StrView,
        _: plugify::Severity,
        _: &plugify::SourceLocation,
    ) {
    }

    static REMOVE_OK: AtomicBool = AtomicBool::new(false);

    unsafe extern "C" fn add(_: &Str, _: &Str, _: i64, _: CommandCallback, _: u8) -> bool {
        true
    }

    unsafe extern "C" fn remove(_: &Str, _: CommandCallback) -> bool {
        REMOVE_OK.load(Ordering::Relaxed)
    }

    #[test]
    fn startup_reload_and_shutdown_preserve_the_export_contract() {
        plugify::string::init_construct_string(string_new as *const () as usize);
        plugify::string::init_destroy_string(string_drop as *const () as usize);
        plugify::string::init_get_string_data(string_data as *const () as usize);
        plugify::string::init_get_string_length(string_len as *const () as usize);
        plugify::string::init_assign_string(string_assign as *const () as usize);
        plugify::vector::init_construct_vector_string(vector_new as *const () as usize);
        plugify::vector::init_destroy_vector_string(vector_drop as *const () as usize);
        plugify::plugin::init_log(log as *const () as usize);
        plugify::PLUGIN
            .set(plugify::PluginInfo {
                id: 0,
                name: "imomoe_chat_prefix".into(),
                description: String::new(),
                version: String::new(),
                author: String::new(),
                website: String::new(),
                license: String::new(),
                location: String::new(),
                dependencies: Vec::new(),
            })
            .unwrap();
        unsafe {
            __s2sdk_AddConsoleCommand = Some(add);
            __s2sdk_RemoveCommand = Some(remove);
        }

        let root = std::env::temp_dir().join(format!(
            "imomoe-prefix-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let dir = root.join("imomoe_chat_prefix");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("prefix.toml");
        plugify::CONFIGS_DIR
            .set(root.to_string_lossy().into_owned())
            .unwrap();
        let args = Arr::<Str>::new();
        let mut out = Str::from_str("unchanged");
        let current = || STATE.lock().unwrap().prefixes.clone();

        start().unwrap();
        assert!(GetChatPrefix(&mut out));
        assert_eq!(out.as_str(), " [Community]\x01 ");

        std::fs::write(&path, "prefix = '{green}[Community]'").unwrap();
        assert_eq!(on_reload(0, CONSOLE_CONTEXT, &args), HANDLED);
        assert_eq!(on_reload(SERVER_SLOT, 1, &args), HANDLED);
        assert!(GetChatPrefix(&mut out));
        assert_eq!(out.as_str(), " [Community]\x01 ");
        assert_eq!(on_reload(SERVER_SLOT, CONSOLE_CONTEXT, &args), HANDLED);
        assert_eq!(current(), [" \x04[Community]\x01 "]);
        assert!(GetChatPrefix(&mut out));
        assert_eq!(out.as_str(), " \x04[Community]\x01 ");

        std::fs::write(
            &path,
            "prefix = ['{bluegrey}[Community]', '{gold}【Community】']",
        )
        .unwrap();
        assert_eq!(on_reload(SERVER_SLOT, CONSOLE_CONTEXT, &args), HANDLED);
        let multiple = current();
        assert_eq!(
            multiple,
            [" \x0A[Community]\x01 ", " \x10【Community】\x01 "]
        );
        assert!(GetChatPrefix(&mut out));
        assert!(multiple.iter().any(|prefix| prefix == out.as_str()));

        std::fs::write(&path, "prefix = ['{red}[Community]', '{rgb}[Community]']").unwrap();
        assert_eq!(on_reload(SERVER_SLOT, CONSOLE_CONTEXT, &args), HANDLED);
        assert_eq!(current(), multiple);
        std::fs::remove_file(&path).unwrap();
        assert_eq!(on_reload(SERVER_SLOT, CONSOLE_CONTEXT, &args), HANDLED);
        assert_eq!(current(), multiple);

        std::fs::write(&path, "prefix = '{green}[Community]'").unwrap();
        REMOVE_OK.store(false, Ordering::Relaxed);
        assert!(end().is_err());
        out.set("unchanged");
        assert!(!GetChatPrefix(&mut out));
        assert_eq!(out.as_str(), "unchanged");
        assert_eq!(on_reload(SERVER_SLOT, CONSOLE_CONTEXT, &args), HANDLED);
        assert!(reload(&path).is_err());
        assert!(!GetChatPrefix(&mut out));
        assert!(STATE.lock().unwrap().registered);

        REMOVE_OK.store(true, Ordering::Relaxed);
        end().unwrap();
        assert!(!STATE.lock().unwrap().registered);
        start().unwrap();
        assert!(GetChatPrefix(&mut out));
        assert_eq!(out.as_str(), " \x04[Community]\x01 ");
        end().unwrap();

        std::fs::write(&path, "prefix = '{invalid}[Community]'").unwrap();
        start().unwrap();
        out.set("unchanged");
        assert!(!GetChatPrefix(&mut out));
        assert_eq!(out.as_str(), "unchanged");
        std::fs::write(&path, "prefix = '{green}[Community]'").unwrap();
        assert_eq!(on_reload(SERVER_SLOT, CONSOLE_CONTEXT, &args), HANDLED);
        assert!(GetChatPrefix(&mut out));
        end().unwrap();
        unsafe {
            __s2sdk_AddConsoleCommand = None;
            __s2sdk_RemoveCommand = None;
        }
        std::fs::remove_file(path).unwrap();
        std::fs::remove_dir(dir).unwrap();
        std::fs::remove_dir(root).unwrap();
    }
}
