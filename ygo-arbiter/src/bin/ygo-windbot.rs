//! WindBot (ProjectIgnis' bot for EDOPro, github.com/ProjectIgnis/windbot) as a bot of the
//! protocol (PROTOCOL.md): each game starts a WindBot process that joins this program as its
//! EDOPro server on a local port. It gets the player's observations and prompts in EDOPro's
//! binary form (`edopro`), before each prompt the player's view of both sides' cards
//! (MSG_UPDATE_DATA), and its responses go back to the arbiter. The deck's name (the book's) is
//! WindBot's deck (its executor, e.g. "Blue-Eyes"); WindBot's deck list must be the game's.
//! The arbiter hides a SELECT_POSITION's card (it can be an opponent's face-down card); EDOPro
//! names it, and WindBot needs it: the card the hint before the prompt names, if one does.
//!
//!   ygo-windbot                       (the bot; option WindBot: the directory of WindBot's
//!                                      build, with the card databases (all its *.cdb, e.g.
//!                                      BabelCDB's); Debug: its log on stderr)
//!   ygo-windbot --check BOOK [GAMES]  (the encodings: every message and response of random-bot
//!                                      games on the book's first openings, encoded and read back)

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::fd::AsFd;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::{Map, Value};
use ygo_arbiter::actions::{Response, encode as encode_response};
use ygo_arbiter::bot::serve;
use ygo_arbiter::edopro::{decode_response, encode, start, update_data};
use ygo_arbiter::game::{Bot, BotError, DuelConfig, InitInfo, Observer, PlayOptions, card_db, init, new_record, play};
use ygo_arbiter::matchrun::load_book;
use ygo_arbiter::messages::{Msg, parse};
use ygo_arbiter::observe::filter_prompt;
use ygo_arbiter::protocol::{BotOption, Identity};
use ygo_arbiter::random::RandomBot;
use ygo_arbiter::state::Snapshot;
use ygo_core::constants::*;
use ygo_core::duel::Duel;

// (EDOPro's packets: a u16 length, then the type and its data)
const CTOS_RESPONSE: u8 = 0x1;
const CTOS_UPDATE_DECK: u8 = 0x2;
const STOC_GAME_MSG: u8 = 0x1;
const STOC_JOIN_GAME: u8 = 0x12;
const STOC_TYPE_CHANGE: u8 = 0x13;
/// The handshake of an EDOPro server's STOC_JOIN_GAME.
const SERVER_HANDSHAKE: u32 = 4043399681;
/// The locations the server refreshes (MSG_UPDATE_DATA) before a prompt.
const REFRESHED: [u32; 6] = [LOCATION_HAND, LOCATION_MZONE, LOCATION_SZONE, LOCATION_GRAVE, LOCATION_REMOVED, LOCATION_EXTRA];

fn send(s: &mut TcpStream, typ: u8, data: &[u8]) -> std::io::Result<()> {
    let mut p = ((data.len() + 1) as u16).to_le_bytes().to_vec();
    p.push(typ);
    p.extend(data);
    s.write_all(&p)
}

fn recv(s: &mut TcpStream) -> std::io::Result<(u8, Vec<u8>)> {
    let mut n = [0u8; 2];
    s.read_exact(&mut n)?;
    let mut p = vec![0u8; u16::from_le_bytes(n) as usize];
    s.read_exact(&mut p)?;
    Ok((p.first().copied().unwrap_or(0), p.get(1..).unwrap_or_default().to_vec()))
}

struct WindBot {
    dir: PathBuf,
    debug: bool,
    child: Option<Child>,
    stream: Option<TcpStream>,
    /// the last HINT_SELECTMSG's card, if it named one
    hinted: u32,
}

/// Base64 (RFC 4648, padded).
fn base64(b: &[u8]) -> String {
    const A: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut s = String::new();
    for c in b.chunks(3) {
        let n = c.iter().enumerate().fold(0u32, |n, (i, &x)| n | (x as u32) << (16 - 8 * i));
        for i in 0..4 {
            s.push(if i <= c.len() { A[(n >> (18 - 6 * i) & 63) as usize] as char } else { '=' });
        }
    }
    s
}

fn failed(e: impl std::fmt::Display) -> BotError {
    BotError::Failed(format!("windbot: {e}"))
}

impl WindBot {
    /// WindBot as the game's player: started, joined and dealt its deck.
    fn connect(&mut self, player: u32, init: &InitInfo) -> Result<(), String> {
        let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
        let port = listener.local_addr().map_err(|e| e.to_string())?.port();
        let log = || match self.debug {
            true => std::io::stderr().as_fd().try_clone_to_owned().map(Stdio::from).unwrap_or(Stdio::null()),
            false => Stdio::null(),
        };
        // (WindBot's card databases: all of the directory's, as a base64 JSON list)
        let mut cdbs: Vec<String> = std::fs::read_dir(&self.dir)
            .map_err(|e| format!("{}: {e}", self.dir.display()))?
            .filter_map(|e| e.ok()?.path().canonicalize().ok())
            .filter(|p| p.extension().is_some_and(|x| x == "cdb"))
            .map(|p| p.to_string_lossy().into_owned())
            .collect();
        cdbs.sort();
        let db_paths = base64(serde_json::to_string(&cdbs).unwrap().as_bytes());
        let mut child = Command::new(self.dir.join("WindBot"))
            .args(["Name=WindBot", &format!("Deck={}", init.deck.name), "Host=127.0.0.1", &format!("Port={port}"), "Chat=false"])
            .arg(format!("DbPaths={db_paths}"))
            .arg(format!("Debug={}", self.debug))
            .current_dir(&self.dir)
            .stdin(Stdio::null())
            .stdout(log())
            .stderr(log())
            .spawn()
            .map_err(|e| format!("{}: {e}", self.dir.join("WindBot").display()))?;
        listener.set_nonblocking(true).map_err(|e| e.to_string())?;
        let t0 = Instant::now();
        let mut s = loop {
            match listener.accept() {
                Ok((s, _)) => break s,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if let Ok(Some(status)) = child.try_wait() {
                        return Err(format!("WindBot exited ({status}) before it joined"));
                    }
                    if t0.elapsed() > Duration::from_secs(30) {
                        return Err("WindBot did not join within 30 s".into());
                    }
                    std::thread::sleep(Duration::from_millis(20));
                }
                Err(e) => return Err(e.to_string()),
            }
        };
        self.child = Some(child);
        s.set_nonblocking(false).and_then(|_| s.set_nodelay(true)).map_err(|e| e.to_string())?;
        // (its CTOS_PLAYER_INFO and CTOS_JOIN_GAME)
        for _ in 0..2 {
            recv(&mut s).map_err(|e| e.to_string())?;
        }
        let c = &init.config;
        let mut j: Vec<u8> = Vec::new();
        j.extend(0u32.to_le_bytes()); // (banlist)
        j.extend([0, 0, 0, 1, 1, 0, 0, 0]); // (rule, mode, duel rule, no deck check, no shuffle, align)
        j.extend((c.start_lp as u32).to_le_bytes());
        j.extend([c.start_hand as u8, c.draw_per_turn as u8]);
        j.extend(0u16.to_le_bytes()); // (no time limit)
        j.extend([0; 4]);
        j.extend(SERVER_HANDSHAKE.to_le_bytes());
        j.extend(0u32.to_le_bytes()); // (version)
        for v in [1i32, 1, 1, (c.flags & 0xffff_ffff) as i32, 0, 0] {
            j.extend(v.to_le_bytes()); // (team sizes, best of, duel flags, forbidden types, extra rules)
        }
        send(&mut s, STOC_JOIN_GAME, &j).map_err(|e| e.to_string())?;
        let deck = loop {
            let (typ, data) = recv(&mut s).map_err(|e| e.to_string())?;
            if typ == CTOS_UPDATE_DECK {
                break data;
            }
        };
        let n = u32::from_le_bytes(deck[..4].try_into().unwrap()) as usize;
        let mut theirs: Vec<u32> = deck[8..8 + 4 * n].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
        let mut ours: Vec<u32> = init.deck.main.iter().chain(&init.deck.extra).copied().collect();
        theirs.sort_unstable();
        ours.sort_unstable();
        if theirs != ours {
            return Err(format!("WindBot's deck {:?} is not the game's", init.deck.name));
        }
        send(&mut s, STOC_TYPE_CHANGE, &[0]).map_err(|e| e.to_string())?;
        let opp = (init.opponent_deck_size, init.opponent_extra_size);
        let own = (init.deck.main.len(), init.deck.extra.len());
        let decks = if player == 0 { [own, opp] } else { [opp, own] };
        let lp = c.start_lp as u32;
        send(&mut s, STOC_GAME_MSG, &start(player, [lp, lp], decks)).map_err(|e| e.to_string())?;
        self.stream = Some(s);
        Ok(())
    }

    fn game_msg(&mut self, msg: &[u8]) -> Result<(), BotError> {
        let s = self.stream.as_mut().ok_or_else(|| failed("not connected"))?;
        send(s, STOC_GAME_MSG, msg).map_err(failed)
    }
}

impl Bot for WindBot {
    fn name(&self) -> String {
        "windbot".into()
    }
    fn wants_state(&self) -> bool {
        true
    }
    fn start(&mut self, player: u32, init: &InitInfo) -> Result<(), BotError> {
        self.connect(player, init).map_err(failed)
    }
    fn observe(&mut self, msg: &Msg) {
        if let Msg::Hint { hint_type: HINT_SELECTMSG, data, .. } = *msg {
            self.hinted = u32::try_from(data).ok().filter(|&c| card_db().get(c).type_ != 0).unwrap_or(0);
        }
        // (a lost connection shows at the next prompt)
        let _ = self.game_msg(&encode(msg));
    }
    fn respond(&mut self, prompt: &Msg, state: Option<&Snapshot>, _clock: Option<(f64, f64)>) -> Result<Response, BotError> {
        let view = state.ok_or_else(|| failed("no state"))?;
        for p in 0..2 {
            for loc in REFRESHED {
                self.game_msg(&update_data(view, p, loc))?;
            }
        }
        let prompt = &match prompt {
            Msg::SelectPosition { player, code: 0, positions } => Msg::SelectPosition { player: *player, code: self.hinted, positions: *positions },
            p => p.clone(),
        };
        self.game_msg(&encode(prompt))?;
        let s = self.stream.as_mut().ok_or_else(|| failed("not connected"))?;
        // (a WindBot that does not answer: its prompt on stderr, once, for the arbiter's log)
        s.set_read_timeout(Some(Duration::from_secs(20))).map_err(failed)?;
        let mut warned = false;
        loop {
            match recv(s) {
                Ok((CTOS_RESPONSE, data)) => return decode_response(prompt, &data).map_err(failed),
                Ok((typ, data)) => eprintln!("ygo-windbot: packet {typ:#x} ({} bytes) at {}", data.len(), prompt.name()),
                Err(e) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => {
                    if !warned {
                        eprintln!("ygo-windbot: no response after 20 s to {}", serde_json::to_string(prompt).unwrap_or_default());
                        warned = true;
                    }
                }
                Err(e) => return Err(failed(e)),
            }
        }
    }
}

impl Drop for WindBot {
    fn drop(&mut self) {
        if let Some(c) = &mut self.child {
            let _ = c.kill();
            let _ = c.wait();
        }
    }
}

/// Checks the encodings on the god view of games: each message read back, each response read back
/// as one with the same bytes.
#[derive(Default)]
struct Check {
    prompt: Option<Msg>,
    messages: usize,
    responses: usize,
    errors: Vec<String>,
}

impl Observer for Check {
    fn message(&mut self, m: &Msg) {
        // (this core's additions are not in EDOPro's form)
        if matches!(m, Msg::ShuffleHand { .. } | Msg::ShuffleExtra { .. } | Msg::ShuffleSetCard { .. }) {
            return;
        }
        self.messages += 1;
        match parse(&encode(m)) {
            Ok(back) if back == *m => {}
            r => self.errors.push(format!("{m:?} -> {r:?}")),
        }
    }
    fn prompt(&mut self, prompt: &Msg, _duel: &mut Duel) {
        self.message(prompt);
        self.prompt = prompt.player().map(|p| filter_prompt(prompt, p));
    }
    fn response(&mut self, _player: u32, r: &Response) {
        let Some(m) = &self.prompt else { return };
        let Ok(bytes) = encode_response(m, r) else { return };
        self.responses += 1;
        match decode_response(m, &bytes).map(|back| encode_response(m, &back)) {
            Ok(Ok(b)) if b == bytes => {}
            x => self.errors.push(format!("{}: {r:?} -> {x:?}", m.name())),
        }
    }
}

fn check(book: &str, games: usize) -> Result<(), String> {
    let mut c = Check::default();
    for (i, o) in load_book(book.as_ref(), usize::MAX, games)?.iter().enumerate() {
        let mut record = new_record(&o.decks, o.seed, &DuelConfig::default());
        let (mut b0, mut b1) = (RandomBot::new(2 * i as i64), RandomBot::new(2 * i as i64 + 1));
        play(&mut record, &mut [&mut b0, &mut b1], &mut [&mut c], &mut PlayOptions::default()).map_err(|e| e.0)?;
    }
    for e in c.errors.iter().take(10) {
        eprintln!("{e}");
    }
    println!("{} messages, {} responses: {} errors", c.messages, c.responses, c.errors.len());
    if c.errors.is_empty() { Ok(()) } else { Err("the encodings do not read back".into()) }
}

fn main() {
    if let Err(e) = init(None) {
        eprintln!("ygo-windbot: {e}");
        std::process::exit(1);
    }
    let args: Vec<String> = std::env::args().collect();
    let result = if args.get(1).map(String::as_str) == Some("--check") {
        match args.get(2) {
            Some(book) => check(book, args.get(3).and_then(|n| n.parse().ok()).unwrap_or(20)),
            None => Err("usage: ygo-windbot --check BOOK [GAMES]".into()),
        }
    } else {
        let id = Identity { name: "windbot".into(), more: Map::from_iter([("version".into(), env!("CARGO_PKG_VERSION").into())]) };
        let options = vec![
            BotOption::String { name: "WindBot".into(), default: "windbot".into() },
            BotOption::Check { name: "Debug".into(), default: false },
        ];
        serve(id, options, |values: &Map<String, Value>| {
            Ok(Box::new(WindBot {
                dir: PathBuf::from(values["WindBot"].as_str().unwrap_or("windbot")),
                debug: values["Debug"].as_bool().unwrap_or(false),
                child: None,
                stream: None,
                hinted: 0,
            }) as Box<dyn Bot>)
        })
    };
    if let Err(e) = result {
        eprintln!("ygo-windbot: {e}");
        std::process::exit(1);
    }
}
