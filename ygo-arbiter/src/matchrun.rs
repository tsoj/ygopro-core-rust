//! Bot-vs-bot matches in the spirit of fastchess.
//!
//! An "opening" is a pair of decks plus a seed (which fixes both starting deck orders and the
//! core's RNG). Every opening is played twice with the bots swapping seats, so both bots
//! get exactly the same draws with the same deck. Results are W/D/L, an Elo estimate with a 95%
//! interval from the pentanomial statistics of the game pairs, and optionally a sequential
//! probability ratio test (GSPRT) between two Elo hypotheses.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use serde_json::{Value, json};
use ygo_core::constants::{TYPE_FUSION, TYPE_LINK, TYPE_SYNCHRO, TYPE_XYZ};

use crate::game::{Bot, BotError, Clock, Deck, DuelConfig, GameError, GameRecord, PlayOptions, SearchInfo, card_db, new_record, play};
use crate::protocol::{BotProcess, shlex_split};

/// A bot: a command speaking the bot protocol, and the options to set (name and value as
/// text).
#[derive(Debug, Clone)]
pub struct BotSpec {
    pub name: String,
    pub command: String,
    pub options: Vec<(String, String)>,
}

impl BotSpec {
    /// NAME=COMMAND
    pub fn parse(text: &str) -> Option<BotSpec> {
        let (name, cmd) = text.split_once('=')?;
        (!cmd.is_empty()).then(|| BotSpec { name: name.into(), command: cmd.into(), options: Vec::new() })
    }

    /// The bot as a process (started, its options set, ready).
    pub fn start(&self) -> Result<BotProcess, BotError> {
        BotProcess::start(&shlex_split(&self.command), &self.name, &self.options)
    }
}

/// A fresh bot process (one per game: games stay independent, and a bot's memory does not
/// grow over a match).
pub fn get_bot(spec: &BotSpec) -> Result<Box<dyn Bot>, BotError> {
    Ok(Box::new(spec.start()?))
}

/// An NPS-adjusted node limit (`--nps-nodes`): bot b (the base) searches `nodes` per decision,
/// bot a that many times the ratio of their nodes per second, both measured over the match's
/// searches so far (their own seconds: without the reconstruction); the ratio is 1 until both have
/// searched NPS_MIN_SECS.
pub struct NpsNodes {
    pub nodes: u64,
    /// per bot (a, b): the nodes and seconds searched
    searched: Mutex<[(u64, f64); 2]>,
}

pub const NPS_MIN_SECS: f64 = 5.0;

impl NpsNodes {
    pub fn new(nodes: u64) -> NpsNodes {
        NpsNodes { nodes, searched: Mutex::new([(0, 0.0); 2]) }
    }

    /// The bots' nodes per second so far (a, b; 0: none yet).
    pub fn nps(&self) -> [f64; 2] {
        let s = self.searched.lock().unwrap();
        s.map(|(n, t)| if t > 0.0 { n as f64 / t } else { 0.0 })
    }

    /// a's nodes per second over b's (1 before both searched NPS_MIN_SECS).
    pub fn ratio(&self) -> f64 {
        let s = *self.searched.lock().unwrap();
        if s.iter().any(|&(n, t)| t < NPS_MIN_SECS || n == 0) {
            return 1.0;
        }
        (s[0].0 as f64 / s[0].1) / (s[1].0 as f64 / s[1].1)
    }

    /// The node limit of bot `side` (0: a, 1: b).
    pub fn limit(&self, side: usize) -> u64 {
        if side == 1 { self.nodes } else { ((self.nodes as f64 * self.ratio()).round() as u64).max(1) }
    }

    fn add(&self, side: usize, info: &SearchInfo) {
        if info.secs > 0.0 {
            let mut s = self.searched.lock().unwrap();
            s[side].0 += info.nodes;
            s[side].1 += info.secs;
        }
    }
}

/// A bot, with `nps` under an NPS-adjusted node limit: its limit set before each decision, its
/// searches counted (and the limits it had, for the record).
struct NpsLimited<'a> {
    inner: Box<dyn Bot>,
    nps: Option<&'a NpsNodes>,
    side: usize,
    info: Option<SearchInfo>,
    limits: (u64, u64),
}

impl Bot for NpsLimited<'_> {
    fn name(&self) -> String {
        self.inner.name()
    }
    fn wants_state(&self) -> bool {
        self.inner.wants_state()
    }
    fn start(&mut self, player: u32, init: &crate::game::InitInfo) -> Result<(), BotError> {
        self.inner.start(player, init)
    }
    fn observe(&mut self, msg: &crate::messages::Msg) {
        self.inner.observe(msg)
    }
    fn respond(
        &mut self,
        prompt: &crate::messages::Msg,
        state: Option<&crate::state::Snapshot>,
        clock: Option<(f64, f64)>,
    ) -> Result<crate::actions::Response, BotError> {
        let Some(nps) = self.nps else {
            let r = self.inner.respond(prompt, state, clock);
            self.info = self.inner.search_info();
            return r;
        };
        let limit = nps.limit(self.side);
        self.inner.set_node_limit(Some(limit));
        let r = self.inner.respond(prompt, state, clock);
        self.info = self.inner.search_info();
        if let Some(i) = &self.info {
            nps.add(self.side, i);
            self.limits = (self.limits.0 + limit, self.limits.1 + 1);
        }
        r
    }
    fn end(&mut self, result: &serde_json::Map<String, Value>) {
        self.inner.end(result)
    }
    fn search_info(&mut self) -> Option<SearchInfo> {
        self.info.take()
    }
}

#[derive(Debug, Clone)]
pub struct Opening {
    pub seed: i64,
    pub decks: Vec<Deck>,
}

/// The openings of a book (README.md: "Opening books"), in order: of its first `lines` lines, at
/// most `limit` openings, those in which each seat won at least `book_min` test games. The error
/// names the first line that is not a valid opening.
pub fn load_book(path: &Path, book_min: i64, lines: usize, limit: usize) -> Result<Vec<Opening>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()).take(lines) {
        if out.len() >= limit {
            break;
        }
        let (opening, wins) = parse_opening(line).map_err(|e| format!("{} line {}: {e}", path.display(), i + 1))?;
        if wins >= book_min {
            out.push(opening);
        }
    }
    Ok(out)
}

/// An opening of a book, and the fewer test wins of its two seats (i64::MAX without "wins").
fn parse_opening(line: &str) -> Result<(Opening, i64), String> {
    let d: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
    let seed = d.get("seed").and_then(Value::as_i64).ok_or("no integer \"seed\"")?;
    let decks: Vec<Deck> = serde_json::from_value(d.get("decks").cloned().ok_or("no \"decks\"")?).map_err(|e| format!("decks: {e}"))?;
    if decks.len() != 2 {
        return Err(format!("{} decks instead of 2", decks.len()));
    }
    for (p, deck) in decks.iter().enumerate() {
        check_deck(deck).map_err(|e| format!("deck {p}: {e}"))?;
    }
    let wins = match d.get("wins") {
        None => i64::MAX,
        Some(w) => match w.as_array().map(|a| a.iter().map(Value::as_i64).collect::<Option<Vec<_>>>()) {
            Some(Some(w)) if w.len() == 2 => w[0].min(w[1]),
            _ => return Err(format!("\"wins\" is {w}, not two integers")),
        },
    };
    Ok((Opening { seed, decks }, wins))
}

/// Whether a deck is legal: 40 to 60 cards in the Main Deck, at most 15 in the Extra Deck, each card
/// in the card database and in the right one of them, at most 3 copies of a card (an alternate
/// artwork counts as its original).
pub fn check_deck(deck: &Deck) -> Result<(), String> {
    let db = card_db();
    if !(40..=60).contains(&deck.main.len()) {
        return Err(format!("{} cards in the Main Deck (40 to 60)", deck.main.len()));
    }
    if deck.extra.len() > 15 {
        return Err(format!("{} cards in the Extra Deck (at most 15)", deck.extra.len()));
    }
    let mut copies: HashMap<u32, u32> = HashMap::new();
    for (&code, extra) in deck.main.iter().map(|c| (c, false)).chain(deck.extra.iter().map(|c| (c, true))) {
        let Some(card) = db.cards.get(&code) else { return Err(format!("card {code} is not in the card database")) };
        if (card.type_ & (TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK) != 0) != extra {
            return Err(format!("card {code} belongs in the {}", if extra { "Main Deck" } else { "Extra Deck" }));
        }
        let original = if card.alias != 0 && card.alias.abs_diff(code) < 10 { card.alias } else { code };
        let n = copies.entry(original).or_default();
        *n += 1;
        if *n > 3 {
            return Err(format!("more than 3 copies of card {original}"));
        }
    }
    Ok(())
}

/// One game of a match: the record and a's score.
pub struct GameResult {
    pub score: f64,
    pub a_seat: u32,
    pub record: GameRecord,
}

/// Plays the opening twice with swapped seats; returns the results from a's point of view. With
/// `nps`, the bots search under its node limits (the result records each seat's mean limit:
/// "node_limits").
pub fn play_opening(opening: &Opening, a: &BotSpec, b: &BotSpec, tc: Option<(f64, f64)>, max_turns: u32, nps: Option<&NpsNodes>) -> Result<Vec<GameResult>, String> {
    let mut out = Vec::new();
    for a_seat in 0..2u32 {
        let mut record = new_record(&opening.decks, opening.seed, &DuelConfig::default());
        let bot = |spec: &BotSpec, side: usize| -> Result<NpsLimited, String> {
            Ok(NpsLimited { inner: get_bot(spec).map_err(|e| format!("{e:?}"))?, nps, side, info: None, limits: (0, 0) })
        };
        let (mut ea, mut eb) = (bot(a, 0)?, bot(b, 1)?);
        let mut opts = PlayOptions {
            max_turns,
            clocks: tc.map(|(base, inc)| [Clock::new(base, inc), Clock::new(base, inc)]),
            illegal_is_loss: true,
            ..PlayOptions::default()
        };
        let r: Result<(), GameError> =
            if a_seat == 0 { play(&mut record, &mut [&mut ea, &mut eb], &mut [], &mut opts) } else { play(&mut record, &mut [&mut eb, &mut ea], &mut [], &mut opts) };
        r.map_err(|e| e.0)?;
        if nps.is_some() {
            let mean = |l: (u64, u64)| if l.1 > 0 { l.0 as f64 / l.1 as f64 } else { 0.0 };
            let by_seat = if a_seat == 0 { [mean(ea.limits), mean(eb.limits)] } else { [mean(eb.limits), mean(ea.limits)] };
            record.result.insert("node_limits".into(), json!(by_seat));
        }
        drop((ea, eb));
        let score = match record.result.get("winner").and_then(|w| w.as_u64()) {
            None => 0.5,
            Some(w) if w as u32 == a_seat => 1.0,
            Some(_) => 0.0,
        };
        out.push(GameResult { score, a_seat, record });
    }
    Ok(out)
}

pub fn score_to_elo(score: f64) -> f64 {
    let score = score.clamp(1e-6, 1.0 - 1e-6);
    -400.0 * (1.0 / score - 1.0).log10()
}

pub fn elo_to_score(elo: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf(-elo / 400.0))
}

#[derive(Debug, Clone, Default)]
pub struct MatchStats {
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    /// pentanomial counts of pair scores 0, 0.5, 1, 1.5, 2
    pub penta: [u32; 5],
}

impl MatchStats {
    pub fn add_pair(&mut self, scores: &[f64]) {
        for &s in scores {
            if s == 1.0 {
                self.wins += 1;
            } else if s == 0.0 {
                self.losses += 1;
            } else {
                self.draws += 1;
            }
        }
        self.penta[(scores.iter().sum::<f64>() * 2.0).round() as usize] += 1;
    }

    pub fn games(&self) -> u32 {
        self.wins + self.draws + self.losses
    }

    fn pair_mean_var(&self) -> (f64, f64, u32) {
        let n: u32 = self.penta.iter().sum();
        if n == 0 {
            return (0.5, 0.0, 0);
        }
        let vals = [0.0, 0.25, 0.5, 0.75, 1.0];
        let mean = vals.iter().zip(self.penta).map(|(v, c)| v * c as f64).sum::<f64>() / n as f64;
        let var = vals.iter().zip(self.penta).map(|(v, c)| c as f64 * (v - mean).powi(2)).sum::<f64>() / n as f64;
        (mean, var, n)
    }

    /// Elo difference and half width of the 95% confidence interval.
    pub fn elo(&self) -> (f64, f64) {
        let (mean, var, n) = self.pair_mean_var();
        if n == 0 {
            return (0.0, f64::INFINITY);
        }
        let se = if n > 1 { (var / n as f64).sqrt() } else { 0.5 };
        let (lo, hi) = (score_to_elo(mean - 1.96 * se), score_to_elo(mean + 1.96 * se));
        (score_to_elo(mean), (hi - lo) / 2.0)
    }

    /// Generalized SPRT log-likelihood ratio (normal approximation, pentanomial).
    pub fn llr(&self, elo0: f64, elo1: f64) -> f64 {
        let (mean, var, n) = self.pair_mean_var();
        if n < 2 || var <= 0.0 {
            return 0.0;
        }
        let (s0, s1) = (elo_to_score(elo0), elo_to_score(elo1));
        n as f64 * ((mean - s0).powi(2) - (mean - s1).powi(2)) / (2.0 * var)
    }

    pub fn summary(&self, sprt: Option<(f64, f64)>) -> String {
        let (e, err) = self.elo();
        let p = self.penta;
        let mut text = format!(
            "games {}: +{} ={} -{}  elo {e:+.1} +/- {err:.1}  penta [{}, {}, {}, {}, {}]",
            self.games(),
            self.wins,
            self.draws,
            self.losses,
            p[0],
            p[1],
            p[2],
            p[3],
            p[4]
        );
        if let Some((e0, e1)) = sprt {
            text += &format!("  LLR {:+.2}", self.llr(e0, e1));
        }
        text
    }
}

pub fn sprt_bounds(alpha: f64, beta: f64) -> (f64, f64) {
    ((beta / (1.0 - alpha)).ln(), ((1.0 - beta) / alpha).ln())
}
