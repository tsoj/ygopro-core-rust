//! The C++ core (`target/oracle/libocgcore.so`, ygopro-core with the patches in oracle/) through its C API,
//! with the same card data and scripts as ygoai/core.py.

use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void, CStr};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use ygo_core::api::{NewCardInfo, QueryInfo};
use ygo_core::carddb::CardDb;

#[repr(C)]
pub struct OcgCardData {
    code: u32,
    alias: u32,
    setcodes: *const u16,
    type_: u32,
    level: u32,
    attribute: u32,
    race: u64,
    attack: i32,
    defense: i32,
    lscale: u32,
    rscale: u32,
    link_marker: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OcgPlayer {
    pub starting_lp: u32,
    pub starting_draw_count: u32,
    pub draw_count_per_turn: u32,
}

type DataReader = extern "C" fn(*mut c_void, u32, *mut OcgCardData);
type DataReaderDone = extern "C" fn(*mut c_void, *mut OcgCardData);
type ScriptReader = extern "C" fn(*mut c_void, *mut c_void, *const c_char) -> c_int;
type LogHandler = extern "C" fn(*mut c_void, *const c_char, c_int);

#[repr(C)]
struct OcgDuelOptions {
    seed: [u64; 4],
    flags: u64,
    team1: OcgPlayer,
    team2: OcgPlayer,
    card_reader: DataReader,
    payload1: *mut c_void,
    script_reader: ScriptReader,
    payload2: *mut c_void,
    log_handler: LogHandler,
    payload3: *mut c_void,
    card_reader_done: DataReaderDone,
    payload4: *mut c_void,
    enable_unsafe_libraries: u8,
}

#[repr(C)]
struct OcgNewCardInfo {
    team: u8,
    duelist: u8,
    code: u32,
    con: u8,
    loc: u32,
    seq: u32,
    pos: u32,
}

#[repr(C)]
struct OcgQueryInfo {
    flags: u32,
    con: u8,
    loc: u32,
    seq: u32,
    overlay_seq: u32,
}

struct Api {
    _lib: libloading::Library,
    create: unsafe extern "C" fn(*mut *mut c_void, *const OcgDuelOptions) -> c_int,
    destroy: unsafe extern "C" fn(*mut c_void),
    new_card: unsafe extern "C" fn(*mut c_void, *const OcgNewCardInfo),
    start: unsafe extern "C" fn(*mut c_void),
    process: unsafe extern "C" fn(*mut c_void) -> c_int,
    get_message: unsafe extern "C" fn(*mut c_void, *mut u32) -> *const u8,
    set_response: unsafe extern "C" fn(*mut c_void, *const c_void, u32),
    load_script: unsafe extern "C" fn(*mut c_void, *const c_char, u32, *const c_char) -> c_int,
    query: unsafe extern "C" fn(*mut c_void, *mut u32, *const OcgQueryInfo) -> *const u8,
    query_location: unsafe extern "C" fn(*mut c_void, *mut u32, *const OcgQueryInfo) -> *const u8,
    query_field: unsafe extern "C" fn(*mut c_void, *mut u32) -> *const u8,
    set_location_order_by_id: unsafe extern "C" fn(*mut c_void, u8, u32, *const u32, u32) -> c_int,
}

static API: OnceLock<Api> = OnceLock::new();
static DB: OnceLock<Arc<CardDb>> = OnceLock::new();
static ROOT: OnceLock<PathBuf> = OnceLock::new();
static SETCODES: OnceLock<Mutex<HashMap<u32, Box<[u16]>>>> = OnceLock::new();
static SCRIPTS: OnceLock<Mutex<HashMap<String, Option<Vec<u8>>>>> = OnceLock::new();

/// Loads the library (once) and sets the card data and the repository root for the readers.
/// The oracle's library: `$YGO_CORE_LIB`, else `target/oracle/libocgcore.so` (built by oracle/build.sh).
pub fn oracle_lib() -> PathBuf {
    std::env::var("YGO_CORE_LIB").map(PathBuf::from).unwrap_or_else(|_| oracle_dir().join("../target/oracle/libocgcore.so"))
}

/// This repository's oracle/ directory (the patch, the extension, the Lua overrides).
fn oracle_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../oracle")
}

/// The card data and scripts; the oracle library too unless `lib_path` is `None` (Rust only).
pub fn init(lib_path: Option<&Path>, root: &Path, db: Arc<CardDb>) {
    let _ = DB.set(db);
    let _ = ROOT.set(root.to_path_buf());
    let _ = SETCODES.set(Mutex::new(HashMap::new()));
    let _ = SCRIPTS.set(Mutex::new(HashMap::new()));
    let Some(lib_path) = lib_path else { return };
    if API.get().is_some() {
        return;
    }
    unsafe {
        let lib = libloading::Library::new(lib_path).expect("load libocgcore");
        macro_rules! sym {
            ($name:expr) => {
                *lib.get($name).expect(std::str::from_utf8($name).unwrap())
            };
        }
        let api = Api {
            create: sym!(b"OCG_CreateDuel"),
            destroy: sym!(b"OCG_DestroyDuel"),
            new_card: sym!(b"OCG_DuelNewCard"),
            start: sym!(b"OCG_StartDuel"),
            process: sym!(b"OCG_DuelProcess"),
            get_message: sym!(b"OCG_DuelGetMessage"),
            set_response: sym!(b"OCG_DuelSetResponse"),
            load_script: sym!(b"OCG_LoadScript"),
            query: sym!(b"OCG_DuelQuery"),
            query_location: sym!(b"OCG_DuelQueryLocation"),
            query_field: sym!(b"OCG_DuelQueryField"),
            set_location_order_by_id: sym!(b"OCG_ExtSetLocationOrderById"),
            _lib: lib,
        };
        let _ = API.set(api);
    }
}

fn api() -> &'static Api {
    API.get().expect("core_ffi::init")
}

extern "C" fn card_reader(_payload: *mut c_void, code: u32, out: *mut OcgCardData) {
    let db = DB.get().unwrap();
    let out = unsafe { &mut *out };
    let mut sc = SETCODES.get().unwrap().lock().unwrap();
    let Some(c) = db.cards.get(&code) else {
        let empty = sc.entry(0).or_insert_with(|| vec![0u16].into_boxed_slice());
        *out = OcgCardData {
            code,
            alias: 0,
            setcodes: empty.as_ptr(),
            type_: 0,
            level: 0,
            attribute: 0,
            race: 0,
            attack: 0,
            defense: 0,
            lscale: 0,
            rscale: 0,
            link_marker: 0,
        };
        return;
    };
    let arr = sc.entry(code).or_insert_with(|| {
        let mut v: Vec<u16> = c.setcodes.iter().collect();
        v.push(0);
        v.into_boxed_slice()
    });
    *out = OcgCardData {
        code,
        alias: c.alias,
        setcodes: arr.as_ptr(),
        type_: c.type_,
        level: c.level,
        attribute: c.attribute,
        race: c.race,
        attack: c.attack,
        defense: c.defense,
        lscale: c.lscale,
        rscale: c.rscale,
        link_marker: c.link_marker,
    };
}

extern "C" fn card_reader_done(_payload: *mut c_void, _data: *mut OcgCardData) {}

/// Set when the core logs that a script ran out of its instruction budget (target/oracle/src
/// interpreter.cpp `LUA_STEP_BUDGET`): the core then treats the script as failed, which the Rust
/// engine (no budget) does not reproduce, so the game is skipped from there on.
pub static SCRIPT_BUDGET_EXCEEDED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

extern "C" fn log_handler(_payload: *mut c_void, text: *const c_char, _typ: c_int) {
    if std::env::var_os("YGO_DIFF_LOG").is_some() && !text.is_null() { eprintln!("core log: {}", unsafe { CStr::from_ptr(text) }.to_string_lossy()); }
    if !text.is_null() && unsafe { CStr::from_ptr(text) }.to_string_lossy().contains("instruction budget exceeded") {
        SCRIPT_BUDGET_EXCEEDED.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

/// `_find_script` of ygoai/core.py (source, not bytecode).
pub fn find_script(name: &str) -> Option<Vec<u8>> {
    let mut cache = SCRIPTS.get().unwrap().lock().unwrap();
    if let Some(d) = cache.get(name) {
        return d.clone();
    }
    let root = ROOT.get().unwrap();
    let scripts = root.join("external").join("CardScripts");
    let base = Path::new(name).file_name().unwrap().to_string_lossy().to_string();
    let mut candidates = vec![];
    if base == "ygoai_overrides.lua" {
        candidates.push(oracle_dir().join("ygoai_overrides.lua"));
    } else {
        if base.starts_with('c') && base.len() > 5 && base[1..base.len() - 4].chars().all(|c| c.is_ascii_digit()) {
            candidates.push(root.join("lua_fixes").join(&base));
            for d in ["official", "pre-release", "pre-errata", "goat"] {
                candidates.push(scripts.join(d).join(&base));
            }
        }
        candidates.push(scripts.join(&base));
    }
    let data = candidates.iter().find(|p| p.exists()).map(|p| std::fs::read(p).unwrap());
    cache.insert(name.to_string(), data.clone());
    data
}

extern "C" fn script_reader(_payload: *mut c_void, duel: *mut c_void, name: *const c_char) -> c_int {
    let name = unsafe { CStr::from_ptr(name) }.to_string_lossy().to_string();
    match find_script(&name) {
        None => 0,
        Some(data) => {
            let cname = std::ffi::CString::new(name).unwrap();
            let ok = unsafe { (api().load_script)(duel, data.as_ptr() as *const c_char, data.len() as u32, cname.as_ptr()) };
            (ok != 0) as c_int
        }
    }
}

/// One duel of the C++ core.
pub struct CoreDuel {
    handle: *mut c_void,
}

impl CoreDuel {
    pub fn new(seed: [u64; 4], flags: u64, player: OcgPlayer) -> CoreDuel {
        let opts = OcgDuelOptions {
            seed,
            flags,
            team1: player,
            team2: player,
            card_reader,
            payload1: std::ptr::null_mut(),
            script_reader,
            payload2: std::ptr::null_mut(),
            log_handler,
            payload3: std::ptr::null_mut(),
            card_reader_done,
            payload4: std::ptr::null_mut(),
            enable_unsafe_libraries: 0,
        };
        let mut handle = std::ptr::null_mut();
        let status = unsafe { (api().create)(&mut handle, &opts) };
        assert_eq!(status, 0, "OCG_CreateDuel");
        for script in ["constant.lua", "utility.lua", "ygoai_overrides.lua"] {
            let cname = std::ffi::CString::new(script).unwrap();
            assert!(script_reader(std::ptr::null_mut(), handle, cname.as_ptr()) != 0, "load {script}");
        }
        CoreDuel { handle }
    }
    pub fn new_card(&mut self, info: &NewCardInfo) {
        let i = OcgNewCardInfo { team: info.team, duelist: info.duelist, code: info.code, con: info.con, loc: info.loc, seq: info.seq, pos: info.pos };
        unsafe { (api().new_card)(self.handle, &i) }
    }
    pub fn start(&mut self) {
        unsafe { (api().start)(self.handle) }
    }
    /// `OCG_ExtSetLocationOrderById` (oracle/ocgext.cpp)
    pub fn set_location_order_by_id(&mut self, playerid: u8, location: u32, ids: &[u32]) -> bool {
        unsafe { (api().set_location_order_by_id)(self.handle, playerid, location, ids.as_ptr(), ids.len() as u32) != 0 }
    }
    pub fn process(&mut self) -> i32 {
        unsafe { (api().process)(self.handle) }
    }
    pub fn get_message(&mut self) -> Vec<u8> {
        let mut len = 0u32;
        let p = unsafe { (api().get_message)(self.handle, &mut len) };
        if len == 0 {
            return Vec::new();
        }
        unsafe { std::slice::from_raw_parts(p, len as usize) }.to_vec()
    }
    pub fn set_response(&mut self, r: &[u8]) {
        unsafe { (api().set_response)(self.handle, r.as_ptr() as *const c_void, r.len() as u32) }
    }
    fn qi(info: &QueryInfo) -> OcgQueryInfo {
        OcgQueryInfo { flags: info.flags, con: info.con, loc: info.loc, seq: info.seq, overlay_seq: info.overlay_seq }
    }
    pub fn query(&mut self, info: &QueryInfo) -> Vec<u8> {
        let mut len = 0u32;
        let q = Self::qi(info);
        let p = unsafe { (api().query)(self.handle, &mut len, &q) };
        if len == 0 || p.is_null() {
            return Vec::new();
        }
        unsafe { std::slice::from_raw_parts(p, len as usize) }.to_vec()
    }
    pub fn query_location(&mut self, info: &QueryInfo) -> Vec<u8> {
        let mut len = 0u32;
        let q = Self::qi(info);
        let p = unsafe { (api().query_location)(self.handle, &mut len, &q) };
        if len == 0 || p.is_null() {
            return Vec::new();
        }
        unsafe { std::slice::from_raw_parts(p, len as usize) }.to_vec()
    }
    pub fn query_field(&mut self) -> Vec<u8> {
        let mut len = 0u32;
        let p = unsafe { (api().query_field)(self.handle, &mut len) };
        if len == 0 {
            return Vec::new();
        }
        unsafe { std::slice::from_raw_parts(p, len as usize) }.to_vec()
    }
}

impl Drop for CoreDuel {
    fn drop(&mut self) {
        unsafe { (api().destroy)(self.handle) }
    }
}
