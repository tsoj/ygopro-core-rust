//! Parsing of ygopro-core binary messages.
//!
//! Every message becomes a [`Msg`], which serializes to the dict of the Python parser: a "type"
//! key holding the message name without the MSG_ prefix (e.g. "MOVE") and the fields under the
//! same names. Locations have the keys con/loc/seq/pos exactly as the core writes them. Parsing
//! is strict: every byte of a message must be consumed, otherwise a [`ParseError`] is returned.

use serde::{Deserialize, Serialize};
use ygo_core::constants::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ParseError {}

type Result<T> = std::result::Result<T, ParseError>;

fn is_none<T>(v: &Option<T>) -> bool {
    v.is_none()
}

/// A card's place (loc_info): controller, location, sequence, position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Loc {
    pub con: u32,
    pub loc: u32,
    pub seq: u32,
    pub pos: u32,
}

/// A card in a listing: code and place, plus the fields of the listing (an option of a prompt has
/// its effect description, a tribute candidate its release parameter, ...). `pos` is absent
/// where the core does not send it.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Card {
    pub code: u32,
    pub con: u32,
    pub loc: u32,
    pub seq: u32,
    #[serde(default, skip_serializing_if = "is_none")]
    pub pos: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub desc: Option<u64>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub client_mode: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub direct: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub release_param: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub counters: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub param: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CodePos {
    pub code: u32,
    pub pos: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ZoneCard {
    pub pos: u32,
    pub overlays: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ReloadPlayer {
    pub lp: u32,
    pub mzone: Vec<Option<ZoneCard>>,
    pub szone: Vec<Option<ZoneCard>>,
    pub deck: u32,
    pub hand: u32,
    pub grave: u32,
    pub removed: u32,
    pub extra: u32,
    pub extra_p: u32,
}

/// A chain link as the core lists it (MSG_RELOAD_FIELD, OCG_DuelQueryField).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ChainLink {
    pub code: u32,
    pub handler: Loc,
    pub trigger_con: u32,
    pub trigger_loc: u32,
    pub trigger_seq: u32,
    pub desc: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Msg {
    Retry {},
    Hint { hint_type: u32, player: u32, data: u64 },
    Win { player: u32, reason: u32 },
    SelectBattlecmd { player: u32, activatable: Vec<Card>, attackable: Vec<Card>, to_m2: u32, to_ep: u32 },
    SelectIdlecmd {
        player: u32,
        summonable: Vec<Card>,
        spsummonable: Vec<Card>,
        repositionable: Vec<Card>,
        msetable: Vec<Card>,
        ssetable: Vec<Card>,
        activatable: Vec<Card>,
        to_bp: u32,
        to_ep: u32,
        can_shuffle: u32,
    },
    SelectEffectyn { player: u32, code: u32, con: u32, loc: u32, seq: u32, pos: u32, desc: u64 },
    SelectYesno { player: u32, desc: u64 },
    SelectOption { player: u32, options: Vec<u64> },
    SelectCard { player: u32, cancelable: u32, min: u32, max: u32, cards: Vec<Card> },
    SelectChain {
        player: u32,
        #[serde(default, skip_serializing_if = "is_none")]
        spe_count: Option<u32>,
        forced: u32,
        #[serde(default, skip_serializing_if = "is_none")]
        hint_timing: Option<u32>,
        #[serde(default, skip_serializing_if = "is_none")]
        hint_timing_other: Option<u32>,
        chains: Vec<Card>,
    },
    SelectPlace { player: u32, count: u32, flag: u32 },
    SelectPosition { player: u32, code: u32, positions: u32 },
    SelectTribute { player: u32, cancelable: u32, min: u32, max: u32, cards: Vec<Card> },
    SortChain { player: u32, cards: Vec<Card> },
    SelectCounter { player: u32, counter_type: u32, count: u32, cards: Vec<Card> },
    SelectSum { player: u32, mode: u32, acc: u32, min: u32, max: u32, must_cards: Vec<Card>, cards: Vec<Card> },
    SelectDisfield { player: u32, count: u32, flag: u32 },
    SortCard { player: u32, cards: Vec<Card> },
    SelectUnselectCard {
        player: u32,
        finishable: u32,
        cancelable: u32,
        min: u32,
        max: u32,
        select_cards: Vec<Card>,
        unselect_cards: Vec<Card>,
    },
    ConfirmDecktop { player: u32, cards: Vec<Card> },
    ConfirmCards { player: u32, cards: Vec<Card> },
    ShuffleDeck { player: u32 },
    ShuffleHand {
        player: u32,
        codes: Vec<u32>,
        public: Vec<bool>,
        #[serde(default, skip_serializing_if = "is_none")]
        prev: Option<Vec<u32>>,
    },
    SwapGraveDeck { player: u32, main_count: u32, extra_mask: String },
    ShuffleSetCard {
        location: u32,
        old: Vec<Loc>,
        overlay: Vec<Loc>,
        #[serde(default, skip_serializing_if = "is_none")]
        new: Option<Vec<u32>>,
    },
    ReverseDeck {},
    DeckTop { player: u32, seq: u32, code: u32, pos: u32 },
    ShuffleExtra { player: u32, codes: Vec<u32>, public: Vec<bool> },
    NewTurn { player: u32 },
    NewPhase { phase: u32 },
    ConfirmExtratop { player: u32, cards: Vec<Card> },
    Move {
        code: u32,
        #[serde(rename = "from")]
        from: Loc,
        to: Loc,
        reason: u32,
    },
    PosChange { code: u32, con: u32, loc: u32, seq: u32, prev_pos: u32, pos: u32 },
    Set { code: u32, con: u32, loc: u32, seq: u32, pos: u32 },
    Swap { code1: u32, loc1: Loc, code2: u32, loc2: Loc },
    FieldDisabled { flag: u32 },
    Summoning { code: u32, con: u32, loc: u32, seq: u32, pos: u32 },
    Summoned {},
    Spsummoning { code: u32, con: u32, loc: u32, seq: u32, pos: u32 },
    Spsummoned {},
    Flipsummoning { code: u32, con: u32, loc: u32, seq: u32, pos: u32 },
    Flipsummoned {},
    Chaining {
        code: u32,
        con: u32,
        loc: u32,
        seq: u32,
        pos: u32,
        trigger_con: u32,
        trigger_loc: u32,
        trigger_seq: u32,
        desc: u64,
        chain_count: u32,
    },
    Chained { chain_count: u32 },
    ChainSolving { chain_count: u32 },
    ChainSolved { chain_count: u32 },
    ChainEnd {},
    ChainNegated { chain_count: u32 },
    ChainDisabled { chain_count: u32 },
    CardSelected { cards: Vec<Loc> },
    RandomSelected { player: u32, cards: Vec<Loc> },
    BecomeTarget { cards: Vec<Loc> },
    Draw { player: u32, cards: Vec<CodePos> },
    Damage { player: u32, amount: u32 },
    Recover { player: u32, amount: u32 },
    Equip { card: Loc, target: Loc },
    Lpupdate { player: u32, lp: u32 },
    CardTarget { card: Loc, target: Loc },
    CancelTarget { card: Loc, target: Loc },
    PayLpcost { player: u32, amount: u32 },
    AddCounter { counter_type: u32, con: u32, loc: u32, seq: u32, count: u32 },
    RemoveCounter { counter_type: u32, con: u32, loc: u32, seq: u32, count: u32 },
    Attack { attacker: Loc, target: Loc },
    Battle {
        attacker: Loc,
        attacker_atk: i32,
        attacker_def: i32,
        attacker_destroyed: u32,
        target: Loc,
        target_atk: i32,
        target_def: i32,
        target_destroyed: u32,
    },
    AttackDisabled {},
    DamageStepStart {},
    DamageStepEnd {},
    MissedEffect { con: u32, loc: u32, seq: u32, pos: u32, code: u32 },
    TossCoin { player: u32, results: Vec<u32> },
    TossDice { player: u32, results: Vec<u32> },
    RockPaperScissors { player: u32 },
    HandRes { result: u32 },
    AnnounceRace { player: u32, count: u32, available: u64 },
    AnnounceAttrib { player: u32, count: u32, available: u32 },
    AnnounceCard { player: u32, values: Vec<u64> },
    AnnounceNumber { player: u32, values: Vec<u64> },
    CardHint { con: u32, loc: u32, seq: u32, pos: u32, hint_type: u32, value: u64 },
    TagSwap { player: u32, main_count: u32, extra_p_count: u32, top_code: u32, hand: Vec<CodePos>, extra: Vec<CodePos> },
    ReloadField { options: u32, players: Vec<ReloadPlayer>, chains: Vec<ChainLink> },
    AiName { text: String },
    ShowHint { text: String },
    PlayerHint { player: u32, hint_type: u32, value: u64 },
    MatchKill { code: u32 },
    RemoveCards { cards: Vec<Loc> },
}

/// The prompts: messages that require a response from a player (constants.SELECT_MESSAGES).
pub const SELECT_MESSAGES: &[u32] = &[
    MSG_SELECT_BATTLECMD,
    MSG_SELECT_IDLECMD,
    MSG_SELECT_EFFECTYN,
    MSG_SELECT_YESNO,
    MSG_SELECT_OPTION,
    MSG_SELECT_CARD,
    MSG_SELECT_CHAIN,
    MSG_SELECT_PLACE,
    MSG_SELECT_POSITION,
    MSG_SELECT_TRIBUTE,
    MSG_SORT_CHAIN,
    MSG_SELECT_COUNTER,
    MSG_SELECT_SUM,
    MSG_SELECT_DISFIELD,
    MSG_SORT_CARD,
    MSG_SELECT_UNSELECT_CARD,
    MSG_ROCK_PAPER_SCISSORS,
    MSG_ANNOUNCE_RACE,
    MSG_ANNOUNCE_ATTRIB,
    MSG_ANNOUNCE_CARD,
    MSG_ANNOUNCE_NUMBER,
];

impl Msg {
    /// The message name without MSG_ (the "type" of the dict).
    pub fn name(&self) -> &'static str {
        use Msg::*;
        match self {
            Retry {} => "RETRY",
            Hint { .. } => "HINT",
            Win { .. } => "WIN",
            SelectBattlecmd { .. } => "SELECT_BATTLECMD",
            SelectIdlecmd { .. } => "SELECT_IDLECMD",
            SelectEffectyn { .. } => "SELECT_EFFECTYN",
            SelectYesno { .. } => "SELECT_YESNO",
            SelectOption { .. } => "SELECT_OPTION",
            SelectCard { .. } => "SELECT_CARD",
            SelectChain { .. } => "SELECT_CHAIN",
            SelectPlace { .. } => "SELECT_PLACE",
            SelectPosition { .. } => "SELECT_POSITION",
            SelectTribute { .. } => "SELECT_TRIBUTE",
            SortChain { .. } => "SORT_CHAIN",
            SelectCounter { .. } => "SELECT_COUNTER",
            SelectSum { .. } => "SELECT_SUM",
            SelectDisfield { .. } => "SELECT_DISFIELD",
            SortCard { .. } => "SORT_CARD",
            SelectUnselectCard { .. } => "SELECT_UNSELECT_CARD",
            ConfirmDecktop { .. } => "CONFIRM_DECKTOP",
            ConfirmCards { .. } => "CONFIRM_CARDS",
            ShuffleDeck { .. } => "SHUFFLE_DECK",
            ShuffleHand { .. } => "SHUFFLE_HAND",
            SwapGraveDeck { .. } => "SWAP_GRAVE_DECK",
            ShuffleSetCard { .. } => "SHUFFLE_SET_CARD",
            ReverseDeck {} => "REVERSE_DECK",
            DeckTop { .. } => "DECK_TOP",
            ShuffleExtra { .. } => "SHUFFLE_EXTRA",
            NewTurn { .. } => "NEW_TURN",
            NewPhase { .. } => "NEW_PHASE",
            ConfirmExtratop { .. } => "CONFIRM_EXTRATOP",
            Move { .. } => "MOVE",
            PosChange { .. } => "POS_CHANGE",
            Set { .. } => "SET",
            Swap { .. } => "SWAP",
            FieldDisabled { .. } => "FIELD_DISABLED",
            Summoning { .. } => "SUMMONING",
            Summoned {} => "SUMMONED",
            Spsummoning { .. } => "SPSUMMONING",
            Spsummoned {} => "SPSUMMONED",
            Flipsummoning { .. } => "FLIPSUMMONING",
            Flipsummoned {} => "FLIPSUMMONED",
            Chaining { .. } => "CHAINING",
            Chained { .. } => "CHAINED",
            ChainSolving { .. } => "CHAIN_SOLVING",
            ChainSolved { .. } => "CHAIN_SOLVED",
            ChainEnd {} => "CHAIN_END",
            ChainNegated { .. } => "CHAIN_NEGATED",
            ChainDisabled { .. } => "CHAIN_DISABLED",
            CardSelected { .. } => "CARD_SELECTED",
            RandomSelected { .. } => "RANDOM_SELECTED",
            BecomeTarget { .. } => "BECOME_TARGET",
            Draw { .. } => "DRAW",
            Damage { .. } => "DAMAGE",
            Recover { .. } => "RECOVER",
            Equip { .. } => "EQUIP",
            Lpupdate { .. } => "LPUPDATE",
            CardTarget { .. } => "CARD_TARGET",
            CancelTarget { .. } => "CANCEL_TARGET",
            PayLpcost { .. } => "PAY_LPCOST",
            AddCounter { .. } => "ADD_COUNTER",
            RemoveCounter { .. } => "REMOVE_COUNTER",
            Attack { .. } => "ATTACK",
            Battle { .. } => "BATTLE",
            AttackDisabled {} => "ATTACK_DISABLED",
            DamageStepStart {} => "DAMAGE_STEP_START",
            DamageStepEnd {} => "DAMAGE_STEP_END",
            MissedEffect { .. } => "MISSED_EFFECT",
            TossCoin { .. } => "TOSS_COIN",
            TossDice { .. } => "TOSS_DICE",
            RockPaperScissors { .. } => "ROCK_PAPER_SCISSORS",
            HandRes { .. } => "HAND_RES",
            AnnounceRace { .. } => "ANNOUNCE_RACE",
            AnnounceAttrib { .. } => "ANNOUNCE_ATTRIB",
            AnnounceCard { .. } => "ANNOUNCE_CARD",
            AnnounceNumber { .. } => "ANNOUNCE_NUMBER",
            CardHint { .. } => "CARD_HINT",
            TagSwap { .. } => "TAG_SWAP",
            ReloadField { .. } => "RELOAD_FIELD",
            AiName { .. } => "AI_NAME",
            ShowHint { .. } => "SHOW_HINT",
            PlayerHint { .. } => "PLAYER_HINT",
            MatchKill { .. } => "MATCH_KILL",
            RemoveCards { .. } => "REMOVE_CARDS",
        }
    }

    /// Whether a player has to respond to the message (messages.is_select).
    pub fn is_select(&self) -> bool {
        use Msg::*;
        matches!(
            self,
            SelectBattlecmd { .. }
                | SelectIdlecmd { .. }
                | SelectEffectyn { .. }
                | SelectYesno { .. }
                | SelectOption { .. }
                | SelectCard { .. }
                | SelectChain { .. }
                | SelectPlace { .. }
                | SelectPosition { .. }
                | SelectTribute { .. }
                | SortChain { .. }
                | SelectCounter { .. }
                | SelectSum { .. }
                | SelectDisfield { .. }
                | SortCard { .. }
                | SelectUnselectCard { .. }
                | RockPaperScissors { .. }
                | AnnounceRace { .. }
                | AnnounceAttrib { .. }
                | AnnounceCard { .. }
                | AnnounceNumber { .. }
        )
    }

    /// The player that has to answer a prompt (the "player" field of the prompts).
    pub fn player(&self) -> Option<u32> {
        use Msg::*;
        Some(match self {
            SelectBattlecmd { player, .. }
            | SelectIdlecmd { player, .. }
            | SelectEffectyn { player, .. }
            | SelectYesno { player, .. }
            | SelectOption { player, .. }
            | SelectCard { player, .. }
            | SelectChain { player, .. }
            | SelectPlace { player, .. }
            | SelectPosition { player, .. }
            | SelectTribute { player, .. }
            | SortChain { player, .. }
            | SelectCounter { player, .. }
            | SelectSum { player, .. }
            | SelectDisfield { player, .. }
            | SortCard { player, .. }
            | SelectUnselectCard { player, .. }
            | RockPaperScissors { player, .. }
            | AnnounceRace { player, .. }
            | AnnounceAttrib { player, .. }
            | AnnounceCard { player, .. }
            | AnnounceNumber { player, .. }
            | Hint { player, .. }
            | Win { player, .. }
            | ConfirmDecktop { player, .. }
            | ConfirmCards { player, .. }
            | ConfirmExtratop { player, .. }
            | ShuffleDeck { player, .. }
            | ShuffleHand { player, .. }
            | ShuffleExtra { player, .. }
            | SwapGraveDeck { player, .. }
            | DeckTop { player, .. }
            | NewTurn { player, .. }
            | RandomSelected { player, .. }
            | Draw { player, .. }
            | Damage { player, .. }
            | Recover { player, .. }
            | Lpupdate { player, .. }
            | PayLpcost { player, .. }
            | TossCoin { player, .. }
            | TossDice { player, .. }
            | TagSwap { player, .. }
            | PlayerHint { player, .. } => *player,
            _ => return None,
        })
    }
}

struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn take<const N: usize>(&mut self) -> Result<[u8; N]> {
        let end = self.pos + N;
        let b = self.buf.get(self.pos..end).ok_or_else(|| ParseError("message truncated".into()))?;
        self.pos = end;
        Ok(b.try_into().unwrap())
    }
    fn u8(&mut self) -> Result<u32> {
        Ok(self.take::<1>()?[0] as u32)
    }
    fn u16(&mut self) -> Result<u32> {
        Ok(u16::from_le_bytes(self.take()?) as u32)
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.take()?))
    }
    fn i32(&mut self) -> Result<i32> {
        Ok(i32::from_le_bytes(self.take()?))
    }
    fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.take()?))
    }
    fn raw(&mut self, n: usize) -> Result<&'a [u8]> {
        let b = self.buf.get(self.pos..self.pos + n).ok_or_else(|| ParseError("message truncated".into()))?;
        self.pos += n;
        Ok(b)
    }
    fn loc(&mut self) -> Result<Loc> {
        Ok(Loc { con: self.u8()?, loc: self.u8()?, seq: self.u32()?, pos: self.u32()? })
    }
    /// A u32 count, then that many items.
    fn list<T>(&mut self, mut f: impl FnMut(&mut Self) -> Result<T>) -> Result<Vec<T>> {
        let n = self.u32()?;
        (0..n).map(|_| f(self)).collect()
    }
    /// code u32 followed by a full loc_info.
    fn card_loc(&mut self) -> Result<Card> {
        let code = self.u32()?;
        let l = self.loc()?;
        Ok(Card { code, con: l.con, loc: l.loc, seq: l.seq, pos: Some(l.pos), ..Card::default() })
    }
    /// code u32, con u8, loc u8, seq u32 (or u8).
    fn card_short(&mut self, seq_u8: bool) -> Result<Card> {
        let (code, con, loc) = (self.u32()?, self.u8()?, self.u8()?);
        let seq = if seq_u8 { self.u8()? } else { self.u32()? };
        Ok(Card { code, con, loc, seq, ..Card::default() })
    }
    fn chain_option_short(&mut self) -> Result<Card> {
        let mut c = self.card_short(false)?;
        c.desc = Some(self.u64()?);
        c.client_mode = Some(self.u8()?);
        Ok(c)
    }
    fn code_pos(&mut self) -> Result<CodePos> {
        Ok(CodePos { code: self.u32()?, pos: self.u32()? })
    }
    fn chain_link(&mut self) -> Result<ChainLink> {
        Ok(ChainLink {
            code: self.u32()?,
            handler: self.loc()?,
            trigger_con: self.u8()?,
            trigger_loc: self.u8()?,
            trigger_seq: self.u32()?,
            desc: self.u64()?,
        })
    }
    fn sort_cards(&mut self) -> Result<Vec<Card>> {
        self.list(|r| Ok(Card { code: r.u32()?, con: r.u8()?, loc: r.u32()?, seq: r.u32()?, ..Card::default() }))
    }
    fn confirm(&mut self) -> Result<(u32, Vec<Card>)> {
        Ok((self.u8()?, self.list(|r| r.card_short(false))?))
    }
    /// (the patched core marks face-up cards with the top bit)
    fn shuffle_codes(&mut self) -> Result<(u32, Vec<u32>, Vec<bool>)> {
        let player = self.u8()?;
        let raw = self.list(|r| r.u32())?;
        Ok((player, raw.iter().map(|c| c & 0x7fff_ffff).collect(), raw.iter().map(|c| c & 0x8000_0000 != 0).collect()))
    }
    fn string(&mut self) -> Result<String> {
        let n = self.u16()? as usize;
        let s = String::from_utf8_lossy(self.raw(n)?).into_owned();
        self.u8()?;
        Ok(s)
    }
}

fn parse_body(mtype: u32, r: &mut Reader) -> Result<Msg> {
    use Msg::*;
    Ok(match mtype {
        MSG_RETRY => Retry {},
        MSG_HINT => Hint { hint_type: r.u8()?, player: r.u8()?, data: r.u64()? },
        MSG_WIN => Win { player: r.u8()?, reason: r.u8()? },
        MSG_SELECT_BATTLECMD => {
            let player = r.u8()?;
            let activatable = r.list(|r| r.chain_option_short())?;
            let attackable = r.list(|r| {
                let mut a = r.card_short(true)?;
                a.direct = Some(r.u8()?);
                Ok(a)
            })?;
            SelectBattlecmd { player, activatable, attackable, to_m2: r.u8()?, to_ep: r.u8()? }
        }
        MSG_SELECT_IDLECMD => SelectIdlecmd {
            player: r.u8()?,
            summonable: r.list(|r| r.card_short(false))?,
            spsummonable: r.list(|r| r.card_short(false))?,
            repositionable: r.list(|r| r.card_short(true))?,
            msetable: r.list(|r| r.card_short(false))?,
            ssetable: r.list(|r| r.card_short(false))?,
            activatable: r.list(|r| r.chain_option_short())?,
            to_bp: r.u8()?,
            to_ep: r.u8()?,
            can_shuffle: r.u8()?,
        },
        MSG_SELECT_EFFECTYN => {
            let (player, code, l) = (r.u8()?, r.u32()?, r.loc()?);
            SelectEffectyn { player, code, con: l.con, loc: l.loc, seq: l.seq, pos: l.pos, desc: r.u64()? }
        }
        MSG_SELECT_YESNO => SelectYesno { player: r.u8()?, desc: r.u64()? },
        MSG_SELECT_OPTION => {
            let player = r.u8()?;
            let n = r.u8()?;
            SelectOption { player, options: (0..n).map(|_| r.u64()).collect::<Result<_>>()? }
        }
        MSG_SELECT_CARD => SelectCard {
            player: r.u8()?,
            cancelable: r.u8()?,
            min: r.u32()?,
            max: r.u32()?,
            cards: r.list(|r| r.card_loc())?,
        },
        MSG_SELECT_CHAIN => SelectChain {
            player: r.u8()?,
            spe_count: Some(r.u8()?),
            forced: r.u8()?,
            hint_timing: Some(r.u32()?),
            hint_timing_other: Some(r.u32()?),
            chains: r.list(|r| {
                let mut o = r.card_loc()?;
                o.desc = Some(r.u64()?);
                o.client_mode = Some(r.u8()?);
                Ok(o)
            })?,
        },
        MSG_SELECT_PLACE => SelectPlace { player: r.u8()?, count: r.u8()?, flag: r.u32()? },
        MSG_SELECT_DISFIELD => SelectDisfield { player: r.u8()?, count: r.u8()?, flag: r.u32()? },
        MSG_SELECT_POSITION => SelectPosition { player: r.u8()?, code: r.u32()?, positions: r.u8()? },
        MSG_SELECT_TRIBUTE => SelectTribute {
            player: r.u8()?,
            cancelable: r.u8()?,
            min: r.u32()?,
            max: r.u32()?,
            cards: r.list(|r| {
                let mut t = r.card_short(false)?;
                t.release_param = Some(r.u8()?);
                Ok(t)
            })?,
        },
        MSG_SORT_CHAIN => SortChain { player: r.u8()?, cards: r.sort_cards()? },
        MSG_SORT_CARD => SortCard { player: r.u8()?, cards: r.sort_cards()? },
        MSG_SELECT_COUNTER => SelectCounter {
            player: r.u8()?,
            counter_type: r.u16()?,
            count: r.u16()?,
            cards: r.list(|r| {
                let mut e = r.card_short(true)?;
                e.counters = Some(r.u16()?);
                Ok(e)
            })?,
        },
        MSG_SELECT_SUM => {
            let (player, mode, acc, min, max) = (r.u8()?, r.u8()?, r.u32()?, r.u32()?, r.u32()?);
            let mut c = |r: &mut Reader| {
                let mut e = r.card_loc()?;
                e.param = Some(r.u32()?);
                Ok(e)
            };
            let must_cards = r.list(&mut c)?;
            SelectSum { player, mode, acc, min, max, must_cards, cards: r.list(&mut c)? }
        }
        MSG_SELECT_UNSELECT_CARD => SelectUnselectCard {
            player: r.u8()?,
            finishable: r.u8()?,
            cancelable: r.u8()?,
            min: r.u32()?,
            max: r.u32()?,
            select_cards: r.list(|r| r.card_loc())?,
            unselect_cards: r.list(|r| r.card_loc())?,
        },
        MSG_CONFIRM_DECKTOP => {
            let (player, cards) = r.confirm()?;
            ConfirmDecktop { player, cards }
        }
        MSG_CONFIRM_CARDS => {
            let (player, cards) = r.confirm()?;
            ConfirmCards { player, cards }
        }
        MSG_CONFIRM_EXTRATOP => {
            let (player, cards) = r.confirm()?;
            ConfirmExtratop { player, cards }
        }
        MSG_SHUFFLE_DECK => ShuffleDeck { player: r.u8()? },
        MSG_SHUFFLE_HAND => {
            let (player, codes, public) = r.shuffle_codes()?;
            // (the patched core adds each card's position before the shuffle: its owner knows
            // where each of its cards went)
            let prev = codes.iter().map(|_| r.u8()).collect::<Result<_>>()?;
            ShuffleHand { player, codes, public, prev: Some(prev) }
        }
        MSG_SHUFFLE_EXTRA => {
            let (player, codes, public) = r.shuffle_codes()?;
            ShuffleExtra { player, codes, public }
        }
        MSG_SWAP_GRAVE_DECK => {
            let (player, main_count) = (r.u8()?, r.u32()?);
            let n = r.u32()? as usize;
            let extra_mask = r.raw(n)?.iter().map(|b| format!("{b:02x}")).collect();
            SwapGraveDeck { player, main_count, extra_mask }
        }
        MSG_SHUFFLE_SET_CARD => {
            let location = r.u8()?;
            let n = r.u8()?;
            let old = (0..n).map(|_| r.loc()).collect::<Result<_>>()?;
            let overlay = (0..n).map(|_| r.loc()).collect::<Result<_>>()?;
            // (ocgext: the zone each card went to)
            let new = (0..n).map(|_| r.u8()).collect::<Result<_>>()?;
            ShuffleSetCard { location, old, overlay, new: Some(new) }
        }
        MSG_REVERSE_DECK => ReverseDeck {},
        MSG_DECK_TOP => DeckTop { player: r.u8()?, seq: r.u32()?, code: r.u32()?, pos: r.u32()? },
        MSG_NEW_TURN => NewTurn { player: r.u8()? },
        MSG_NEW_PHASE => NewPhase { phase: r.u16()? },
        MSG_MOVE => Move { code: r.u32()?, from: r.loc()?, to: r.loc()?, reason: r.u32()? },
        MSG_POS_CHANGE => PosChange { code: r.u32()?, con: r.u8()?, loc: r.u8()?, seq: r.u8()?, prev_pos: r.u8()?, pos: r.u8()? },
        MSG_SET | MSG_SUMMONING | MSG_SPSUMMONING | MSG_FLIPSUMMONING => {
            let (code, l) = (r.u32()?, r.loc()?);
            let (con, loc, seq, pos) = (l.con, l.loc, l.seq, l.pos);
            match mtype {
                MSG_SET => Set { code, con, loc, seq, pos },
                MSG_SUMMONING => Summoning { code, con, loc, seq, pos },
                MSG_SPSUMMONING => Spsummoning { code, con, loc, seq, pos },
                _ => Flipsummoning { code, con, loc, seq, pos },
            }
        }
        MSG_SWAP => Swap { code1: r.u32()?, loc1: r.loc()?, code2: r.u32()?, loc2: r.loc()? },
        MSG_FIELD_DISABLED => FieldDisabled { flag: r.u32()? },
        MSG_SUMMONED => Summoned {},
        MSG_SPSUMMONED => Spsummoned {},
        MSG_FLIPSUMMONED => Flipsummoned {},
        MSG_CHAINING => {
            let (code, l) = (r.u32()?, r.loc()?);
            Chaining {
                code,
                con: l.con,
                loc: l.loc,
                seq: l.seq,
                pos: l.pos,
                trigger_con: r.u8()?,
                trigger_loc: r.u8()?,
                trigger_seq: r.u32()?,
                desc: r.u64()?,
                chain_count: r.u32()?,
            }
        }
        MSG_CHAINED => Chained { chain_count: r.u8()? },
        MSG_CHAIN_SOLVING => ChainSolving { chain_count: r.u8()? },
        MSG_CHAIN_SOLVED => ChainSolved { chain_count: r.u8()? },
        MSG_CHAIN_END => ChainEnd {},
        MSG_CHAIN_NEGATED => ChainNegated { chain_count: r.u8()? },
        MSG_CHAIN_DISABLED => ChainDisabled { chain_count: r.u8()? },
        MSG_CARD_SELECTED => CardSelected { cards: r.list(|r| r.loc())? },
        MSG_BECOME_TARGET => BecomeTarget { cards: r.list(|r| r.loc())? },
        MSG_REMOVE_CARDS => RemoveCards { cards: r.list(|r| r.loc())? },
        MSG_RANDOM_SELECTED => RandomSelected { player: r.u8()?, cards: r.list(|r| r.loc())? },
        MSG_DRAW => Draw { player: r.u8()?, cards: r.list(|r| r.code_pos())? },
        MSG_DAMAGE => Damage { player: r.u8()?, amount: r.u32()? },
        MSG_RECOVER => Recover { player: r.u8()?, amount: r.u32()? },
        MSG_PAY_LPCOST => PayLpcost { player: r.u8()?, amount: r.u32()? },
        MSG_EQUIP => Equip { card: r.loc()?, target: r.loc()? },
        MSG_CARD_TARGET => CardTarget { card: r.loc()?, target: r.loc()? },
        MSG_CANCEL_TARGET => CancelTarget { card: r.loc()?, target: r.loc()? },
        MSG_LPUPDATE => Lpupdate { player: r.u8()?, lp: r.u32()? },
        MSG_ADD_COUNTER => AddCounter { counter_type: r.u16()?, con: r.u8()?, loc: r.u8()?, seq: r.u8()?, count: r.u16()? },
        MSG_REMOVE_COUNTER => RemoveCounter { counter_type: r.u16()?, con: r.u8()?, loc: r.u8()?, seq: r.u8()?, count: r.u16()? },
        MSG_ATTACK => Attack { attacker: r.loc()?, target: r.loc()? },
        MSG_BATTLE => Battle {
            attacker: r.loc()?,
            attacker_atk: r.i32()?,
            attacker_def: r.i32()?,
            attacker_destroyed: r.u8()?,
            target: r.loc()?,
            target_atk: r.i32()?,
            target_def: r.i32()?,
            target_destroyed: r.u8()?,
        },
        MSG_ATTACK_DISABLED => AttackDisabled {},
        MSG_DAMAGE_STEP_START => DamageStepStart {},
        MSG_DAMAGE_STEP_END => DamageStepEnd {},
        MSG_MISSED_EFFECT => {
            let l = r.loc()?;
            MissedEffect { con: l.con, loc: l.loc, seq: l.seq, pos: l.pos, code: r.u32()? }
        }
        MSG_TOSS_COIN | MSG_TOSS_DICE => {
            let player = r.u8()?;
            let n = r.u8()?;
            let results = (0..n).map(|_| r.u8()).collect::<Result<_>>()?;
            if mtype == MSG_TOSS_COIN { TossCoin { player, results } } else { TossDice { player, results } }
        }
        MSG_ROCK_PAPER_SCISSORS => RockPaperScissors { player: r.u8()? },
        MSG_HAND_RES => HandRes { result: r.u8()? },
        MSG_ANNOUNCE_RACE => AnnounceRace { player: r.u8()?, count: r.u8()?, available: r.u64()? },
        MSG_ANNOUNCE_ATTRIB => AnnounceAttrib { player: r.u8()?, count: r.u8()?, available: r.u32()? },
        MSG_ANNOUNCE_CARD | MSG_ANNOUNCE_NUMBER => {
            let player = r.u8()?;
            let n = r.u8()?;
            let values = (0..n).map(|_| r.u64()).collect::<Result<_>>()?;
            if mtype == MSG_ANNOUNCE_CARD { AnnounceCard { player, values } } else { AnnounceNumber { player, values } }
        }
        MSG_CARD_HINT => {
            let l = r.loc()?;
            CardHint { con: l.con, loc: l.loc, seq: l.seq, pos: l.pos, hint_type: r.u8()?, value: r.u64()? }
        }
        MSG_PLAYER_HINT => PlayerHint { player: r.u8()?, hint_type: r.u8()?, value: r.u64()? },
        MSG_MATCH_KILL => MatchKill { code: r.u32()? },
        MSG_AI_NAME => AiName { text: r.string()? },
        MSG_SHOW_HINT => ShowHint { text: r.string()? },
        MSG_TAG_SWAP => {
            let (player, main_count) = (r.u8()?, r.u32()?);
            let extra_count = r.u32()?;
            let extra_p_count = r.u32()?;
            let hand_count = r.u32()?;
            let top_code = r.u32()?;
            let hand = (0..hand_count).map(|_| r.code_pos()).collect::<Result<_>>()?;
            let extra = (0..extra_count).map(|_| r.code_pos()).collect::<Result<_>>()?;
            TagSwap { player, main_count, extra_p_count, top_code, hand, extra }
        }
        MSG_RELOAD_FIELD => {
            let options = r.u32()?;
            let mut players = Vec::new();
            for _ in 0..2 {
                let mut p = ReloadPlayer { lp: r.u32()?, ..ReloadPlayer::default() };
                for n in [7, 8] {
                    for _ in 0..n {
                        let z = if r.u8()? != 0 { Some(ZoneCard { pos: r.u8()?, overlays: r.u32()? }) } else { None };
                        if n == 7 { p.mzone.push(z) } else { p.szone.push(z) }
                    }
                }
                (p.deck, p.hand, p.grave, p.removed, p.extra, p.extra_p) = (r.u32()?, r.u32()?, r.u32()?, r.u32()?, r.u32()?, r.u32()?);
                players.push(p);
            }
            let chains = r.list(|r| r.chain_link())?;
            ReloadField { options, players, chains }
        }
        _ => return Err(ParseError(format!("unknown message type {mtype}"))),
    })
}

/// Parses one message (its type byte, then the body).
pub fn parse(msg: &[u8]) -> Result<Msg> {
    let Some(&mtype) = msg.first() else { return Err(ParseError("empty message".into())) };
    let mut r = Reader { buf: msg, pos: 1 };
    let m = parse_body(mtype as u32, &mut r)?;
    if r.pos < msg.len() {
        return Err(ParseError(format!("{}: {} trailing bytes", m.name(), msg.len() - r.pos)));
    }
    Ok(m)
}

/// The messages of an `OCG_DuelGetMessage` buffer (each a u32 length, then the message).
pub fn split_messages(buf: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut pos = 0;
    while pos + 4 <= buf.len() {
        let n = u32::from_le_bytes(buf[pos..pos + 4].try_into().unwrap()) as usize;
        pos += 4;
        out.push(&buf[pos..pos + n]);
        pos += n;
    }
    out
}
