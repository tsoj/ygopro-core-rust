//! Responses to selection messages.
//!
//! A response is a small record whose shape depends on the prompt type (see [`encode`]). This
//! module turns responses into the binary format the core expects and enumerates the options of
//! prompts.

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize};
use ygo_core::constants::*;

use crate::messages::{Card, Msg};

/// Declares `Command`: per command its name in the Python protocol.
macro_rules! commands {
    ($($command:ident = $name:literal,)*) => {
        /// An idle or battle command (`Response::kind`).
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum Command {
            $(#[serde(rename = $name)] $command,)*
        }

        impl Command {
            pub fn name(self) -> &'static str {
                match self {
                    $(Command::$command => $name,)*
                }
            }
        }
    };
}

commands! {
    Summon = "summon",
    Spsummon = "spsummon",
    Repos = "repos",
    Mset = "mset",
    Sset = "sset",
    Activate = "activate",
    ToBp = "to_bp",
    ToEp = "to_ep",
    Shuffle = "shuffle",
    Attack = "attack",
    ToM2 = "to_m2",
}

impl Command {
    /// An idle command's commands in the core's order (a response's code is the position).
    pub const IDLE: [Command; 9] = [
        Command::Summon,
        Command::Spsummon,
        Command::Repos,
        Command::Mset,
        Command::Sset,
        Command::Activate,
        Command::ToBp,
        Command::ToEp,
        Command::Shuffle,
    ];
    /// A battle command's, the same way.
    pub const BATTLE: [Command; 4] = [Command::Activate, Command::Attack, Command::ToM2, Command::ToEp];

    /// The prompt's list of the cards this command takes one of (by `Response::index`).
    pub fn list_name(self) -> Option<&'static str> {
        Some(match self {
            Command::Summon => "summonable",
            Command::Spsummon => "spsummonable",
            Command::Repos => "repositionable",
            Command::Mset => "msetable",
            Command::Sset => "ssetable",
            Command::Activate => "activatable",
            Command::Attack => "attackable",
            Command::ToBp | Command::ToEp | Command::Shuffle | Command::ToM2 => return None,
        })
    }

    /// The cards of the prompt `m` this command takes one of (None: a command without a card, or
    /// not one of that prompt).
    pub fn cards(self, m: &Msg) -> Option<&[Card]> {
        Some(match (m, self) {
            (Msg::SelectIdlecmd { summonable, .. }, Command::Summon) => summonable,
            (Msg::SelectIdlecmd { spsummonable, .. }, Command::Spsummon) => spsummonable,
            (Msg::SelectIdlecmd { repositionable, .. }, Command::Repos) => repositionable,
            (Msg::SelectIdlecmd { msetable, .. }, Command::Mset) => msetable,
            (Msg::SelectIdlecmd { ssetable, .. }, Command::Sset) => ssetable,
            (Msg::SelectIdlecmd { activatable, .. } | Msg::SelectBattlecmd { activatable, .. }, Command::Activate) => activatable,
            (Msg::SelectBattlecmd { attackable, .. }, Command::Attack) => attackable,
            _ => return None,
        })
    }

    /// Whether the prompt `m` offers this command (one of a card: for any card).
    pub fn offered(self, m: &Msg) -> bool {
        let flag = match (m, self) {
            (Msg::SelectIdlecmd { to_bp, .. }, Command::ToBp) => to_bp,
            (Msg::SelectIdlecmd { to_ep, .. } | Msg::SelectBattlecmd { to_ep, .. }, Command::ToEp) => to_ep,
            (Msg::SelectIdlecmd { can_shuffle, .. }, Command::Shuffle) => can_shuffle,
            (Msg::SelectBattlecmd { to_m2, .. }, Command::ToM2) => to_m2,
            _ => return self.cards(m).is_some_and(|cards| !cards.is_empty()),
        };
        *flag != 0
    }
}

fn is_none<T>(v: &Option<T>) -> bool {
    v.is_none()
}

/// A JSON truth value: a bool, or a number (as Python's `if resp.get("cancel")`).
fn truthy<'de, D: Deserializer<'de>>(d: D) -> Result<Option<bool>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum B {
        Bool(bool),
        Int(i64),
        Null(()),
    }
    Ok(match B::deserialize(d)? {
        B::Bool(b) => Some(b),
        B::Int(i) => Some(i != 0),
        B::Null(()) => None,
    })
}

/// A response: the keys of the Python dict (only those of the prompt's type are set; other keys
/// a player sent are kept in `extra`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Response {
    #[serde(default, skip_serializing_if = "is_none")]
    pub kind: Option<Command>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub index: Option<i64>,
    #[serde(default, skip_serializing_if = "is_none", deserialize_with = "truthy")]
    pub yes: Option<bool>,
    #[serde(default, skip_serializing_if = "is_none", deserialize_with = "truthy")]
    pub cancel: Option<bool>,
    #[serde(default, skip_serializing_if = "is_none", deserialize_with = "truthy")]
    pub finish: Option<bool>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub indices: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub places: Option<Vec<[u32; 3]>>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub position: Option<i64>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub counts: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub order: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub mask: Option<u64>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub hand: Option<i64>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl Response {
    /// An idle or battle command, of the card at `index` of its list (`Command::cards`).
    pub fn command(command: Command, index: Option<usize>) -> Response {
        Response { kind: Some(command), index: index.map(|i| i as i64), ..Response::default() }
    }

    /// The option at `index`.
    pub fn index(index: usize) -> Response {
        Response { index: Some(index as i64), ..Response::default() }
    }

    /// Not chaining (a chain window's pass).
    pub fn pass() -> Response {
        Response { index: Some(-1), ..Response::default() }
    }

    pub fn yes(yes: bool) -> Response {
        Response { yes: Some(yes), ..Response::default() }
    }

    /// The cards at `indices` of a selection (sorted).
    pub fn cards(mut indices: Vec<usize>) -> Response {
        indices.sort();
        Response { indices: Some(indices.into_iter().map(|i| i as i64).collect()), ..Response::default() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidResponse(pub String);

impl std::fmt::Display for InvalidResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for InvalidResponse {}

fn need<T: Clone>(v: &Option<T>, key: &str) -> Result<T, InvalidResponse> {
    v.clone().ok_or_else(|| InvalidResponse(format!("response without \"{key}\"")))
}

fn i32le(v: i64) -> Vec<u8> {
    (v as i32).to_le_bytes().to_vec()
}

fn command_code(commands: &[Command], resp: &Response) -> Result<Vec<u8>, InvalidResponse> {
    let kind = need(&resp.kind, "kind")?;
    let k = commands.iter().position(|&x| x == kind).ok_or_else(|| InvalidResponse(format!("unknown kind {}", kind.name())))?;
    Ok(i32le((resp.index.unwrap_or(0) << 16) | k as i64))
}

/// The binary response to the prompt `m`.
pub fn encode(m: &Msg, resp: &Response) -> Result<Vec<u8>, InvalidResponse> {
    use Msg::*;
    Ok(match m {
        SelectIdlecmd { .. } => command_code(&Command::IDLE, resp)?,
        SelectBattlecmd { .. } => command_code(&Command::BATTLE, resp)?,
        SelectEffectyn { .. } | SelectYesno { .. } => i32le(need(&resp.yes, "yes")? as i64),
        SelectOption { .. } | AnnounceNumber { .. } | SelectChain { .. } => i32le(need(&resp.index, "index")?),
        SelectCard { .. } | SelectTribute { .. } | SelectSum { .. } => {
            if resp.cancel == Some(true) {
                return Ok(i32le(-1));
            }
            let idx = need(&resp.indices, "indices")?;
            let mut out = i32le(0);
            out.extend((idx.len() as u32).to_le_bytes());
            for i in idx {
                out.extend((i as u32).to_le_bytes());
            }
            out
        }
        SelectUnselectCard { .. } => {
            if resp.finish == Some(true) {
                return Ok(i32le(-1));
            }
            let mut out = i32le(1);
            out.extend(i32le(need(&resp.index, "index")?));
            out
        }
        SelectPlace { .. } | SelectDisfield { .. } => {
            need(&resp.places, "places")?.iter().flat_map(|&[p, loc, seq]| [p as u8, loc as u8, seq as u8]).collect()
        }
        SelectPosition { .. } => i32le(need(&resp.position, "position")?),
        SelectCounter { .. } => need(&resp.counts, "counts")?.iter().flat_map(|&c| (c as i16).to_le_bytes()).collect(),
        SortCard { .. } | SortChain { .. } => match &resp.order {
            None => vec![0xff],
            Some(order) => order.iter().map(|&o| o as i8 as u8).collect(),
        },
        AnnounceRace { .. } => need(&resp.mask, "mask")?.to_le_bytes().to_vec(),
        AnnounceAttrib { .. } => (need(&resp.mask, "mask")? as u32).to_le_bytes().to_vec(),
        AnnounceCard { .. } => i32le(need(&resp.code, "code")?),
        RockPaperScissors { .. } => i32le(need(&resp.hand, "hand")?),
        _ => return Err(InvalidResponse(format!("cannot encode response for {}", m.name()))),
    })
}

/// The options of an idle command prompt, in the Python order.
pub fn idle_options(m: &Msg) -> Vec<Response> {
    let Msg::SelectIdlecmd { summonable, spsummonable, repositionable, msetable, ssetable, activatable, to_bp, to_ep, can_shuffle, .. } = m
    else {
        return vec![];
    };
    let mut opts = Vec::new();
    for (command, list) in [
        (Command::Summon, summonable),
        (Command::Spsummon, spsummonable),
        (Command::Repos, repositionable),
        (Command::Mset, msetable),
        (Command::Sset, ssetable),
        (Command::Activate, activatable),
    ] {
        opts.extend((0..list.len()).map(|i| Response::command(command, Some(i))));
    }
    for (command, on) in [(Command::ToBp, to_bp), (Command::ToEp, to_ep), (Command::Shuffle, can_shuffle)] {
        if *on != 0 {
            opts.push(Response::command(command, None));
        }
    }
    opts
}

/// The options of a battle command prompt, in the Python order.
pub fn battle_options(m: &Msg) -> Vec<Response> {
    let Msg::SelectBattlecmd { activatable, attackable, to_m2, to_ep, .. } = m else { return vec![] };
    let mut opts = Vec::new();
    for (command, list) in [(Command::Activate, activatable), (Command::Attack, attackable)] {
        opts.extend((0..list.len()).map(|i| Response::command(command, Some(i))));
    }
    for (command, on) in [(Command::ToM2, to_m2), (Command::ToEp, to_ep)] {
        if *on != 0 {
            opts.push(Response::command(command, None));
        }
    }
    opts
}

/// The free zones of a SELECT_PLACE / SELECT_DISFIELD prompt: (player, location, sequence).
pub fn available_places(player: u32, flag: u32) -> Vec<[u32; 3]> {
    let mut places = Vec::new();
    for own in [true, false] {
        let p = if own { player } else { 1 - player };
        for (loc, nseq, shift) in [(LOCATION_MZONE, 7, 0), (LOCATION_SZONE, 8, 8)] {
            for seq in 0..nseq {
                let bit = 1u32 << (seq + shift + if own { 0 } else { 16 });
                if flag & bit == 0 {
                    places.push([p, loc, seq]);
                }
            }
        }
    }
    places
}

/// Mirror of select_sum_check1 in playerop.cpp.
fn check_exact(params: &[u32], acc: i64) -> bool {
    fn rec(params: &[u32], i: usize, acc: i64) -> bool {
        if acc == 0 || i == params.len() {
            return false;
        }
        let (o1, o2) = ((params[i] & 0xFFFF) as i64, (params[i] >> 16) as i64);
        if i == params.len() - 1 {
            return acc == o1 || acc == o2;
        }
        (acc > o1 && rec(params, i + 1, acc - o1)) || (o2 > 0 && acc > o2 && rec(params, i + 1, acc - o2))
    }
    rec(params, 0, acc)
}

fn check_atleast(params: &[u32], acc: i64) -> bool {
    let (mut total, mut mx, mut mn) = (0i64, 0i64, 0x7FFF_FFFFi64);
    for &op in params {
        let (o1, o2) = ((op & 0xFFFF) as i64, (op >> 16) as i64);
        let ms = if o2 != 0 && o2 < o1 { o2 } else { o1 };
        total += ms;
        mx += if o2 > o1 { o2 } else { o1 };
        mn = mn.min(ms);
    }
    !(mx < acc || total - mn >= acc)
}

/// Whether choosing `indices` answers a SELECT_SUM prompt.
pub fn check_sum(m: &Msg, indices: &[usize]) -> bool {
    let Msg::SelectSum { mode, acc, min, max, must_cards, cards, .. } = m else { return false };
    let mut params: Vec<u32> = must_cards.iter().map(|c| c.param.unwrap_or(0)).collect();
    params.extend(indices.iter().map(|&i| cards[i].param.unwrap_or(0)));
    if *mode == 0 {
        if !(*min as usize <= indices.len() && indices.len() <= *max as usize) {
            return false;
        }
        return check_exact(&params, *acc as i64);
    }
    check_atleast(&params, *acc as i64)
}

/// What [`is_declarable`] needs to know of a card.
#[derive(Debug, Clone, Default)]
pub struct DeclCard {
    pub code: u32,
    pub alias: u32,
    pub type_: u32,
    pub race: u64,
    pub attribute: u32,
    pub setcodes: Vec<u16>,
}

const CARD_MARINE_DOLPHIN: u32 = 78734254;
const CARD_TWINKLE_MOSS: u32 = 13857930;

/// Whether a card can be declared with an ANNOUNCE_CARD filter (mirror of is_declarable in
/// playerop.cpp; the values are Python ints there, here i128).
pub fn is_declarable(card: &DeclCard, opcodes: &[u64]) -> bool {
    let mut stack: Vec<i128> = Vec::new();
    let (mut alias, mut token) = (false, false);
    for &op in opcodes {
        if crate::observe::is_binary(op) {
            if stack.len() >= 2 {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(match op {
                    OPCODE_ADD => a.wrapping_add(b),
                    OPCODE_SUB => a.wrapping_sub(b),
                    OPCODE_MUL => a.wrapping_mul(b),
                    OPCODE_DIV => {
                        if b != 0 {
                            a / b
                        } else {
                            0
                        }
                    }
                    OPCODE_AND => (a != 0 && b != 0) as i128,
                    OPCODE_OR => (a != 0 || b != 0) as i128,
                    OPCODE_BAND => a & b,
                    OPCODE_BOR => a | b,
                    OPCODE_BXOR => a ^ b,
                    OPCODE_LSHIFT => a.checked_shl(b as u32).unwrap_or(0),
                    _ => a.checked_shr(b as u32).unwrap_or(if a < 0 { -1 } else { 0 }),
                });
            }
            continue;
        }
        let unary = |stack: &mut Vec<i128>, f: &dyn Fn(i128) -> i128| {
            if let Some(v) = stack.pop() {
                stack.push(f(v));
            }
        };
        match op {
            OPCODE_NEG => unary(&mut stack, &|v| -v),
            OPCODE_NOT => unary(&mut stack, &|v| (v == 0) as i128),
            OPCODE_BNOT => unary(&mut stack, &|v| !v),
            OPCODE_ISCODE => unary(&mut stack, &|v| (card.code as i128 == v & 0xFFFF_FFFF) as i128),
            OPCODE_ISTYPE => unary(&mut stack, &|v| card.type_ as i128 & v),
            OPCODE_ISRACE => unary(&mut stack, &|v| card.race as i128 & v),
            OPCODE_ISATTRIBUTE => unary(&mut stack, &|v| card.attribute as i128 & v),
            OPCODE_GETCODE => stack.push(card.code as i128),
            OPCODE_GETTYPE => stack.push(card.type_ as i128),
            OPCODE_GETRACE => stack.push(card.race as i128),
            OPCODE_GETATTRIBUTE => stack.push(card.attribute as i128),
            OPCODE_ISSETCARD => unary(&mut stack, &|v| {
                let set_code = (v & 0xFFFF_FFFF) as u32;
                let (settype, subtype) = (set_code & 0xFFF, set_code & 0xF000);
                card.setcodes.iter().any(|&sc| (sc as u32 & 0xFFF) == settype && (sc as u32 & 0xF000 & subtype) == subtype) as i128
            }),
            OPCODE_ALLOW_ALIASES => alias = true,
            OPCODE_ALLOW_TOKENS => token = true,
            _ => stack.push(op as i128),
        }
    }
    if stack.len() != 1 || stack[0] == 0 {
        return false;
    }
    if card.code == CARD_MARINE_DOLPHIN || card.code == CARD_TWINKLE_MOSS {
        return true;
    }
    let is_token_monster = card.type_ & (TYPE_MONSTER | TYPE_TOKEN) == TYPE_MONSTER | TYPE_TOKEN;
    (alias || card.alias == 0) && (token || !is_token_monster)
}
