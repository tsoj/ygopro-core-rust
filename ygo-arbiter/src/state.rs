//! Full (god view) state snapshots of a duel and their per-player projection.

use serde::{Deserialize, Serialize};
use ygo_core::api::QueryInfo;
use ygo_core::constants::*;
use ygo_core::duel::Duel;

use crate::messages::Loc;
use crate::observe::{canon_pos, li_visible};

fn is_none<T>(v: &Option<T>) -> bool {
    v.is_none()
}

/// A field that is present (even as null) is Some: Some(None) for null.
fn present<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<Option<Option<T>>, D::Error> {
    Option::<T>::deserialize(d).map(Some)
}

pub const QUERY_ALL: u32 = QUERY_CODE
    | QUERY_POSITION
    | QUERY_ALIAS
    | QUERY_TYPE
    | QUERY_LEVEL
    | QUERY_RANK
    | QUERY_ATTRIBUTE
    | QUERY_RACE
    | QUERY_ATTACK
    | QUERY_DEFENSE
    | QUERY_BASE_ATTACK
    | QUERY_BASE_DEFENSE
    | QUERY_REASON
    | QUERY_REASON_CARD
    | QUERY_EQUIP_CARD
    | QUERY_TARGET_CARD
    | QUERY_OVERLAY_CARD
    | QUERY_COUNTERS
    | QUERY_OWNER
    | QUERY_STATUS
    | QUERY_IS_PUBLIC
    | QUERY_LSCALE
    | QUERY_RSCALE
    | QUERY_LINK
    | QUERY_IS_HIDDEN
    | QUERY_COVER;

/// Cheaper flag set for piles where only identity and position matter.
pub const QUERY_BASIC: u32 = QUERY_CODE | QUERY_POSITION | QUERY_OWNER | QUERY_IS_PUBLIC;

/// The locations of a snapshot, under their names.
pub const LOCATIONS: [(&str, u32); 7] = [
    ("deck", LOCATION_DECK),
    ("hand", LOCATION_HAND),
    ("mzone", LOCATION_MZONE),
    ("szone", LOCATION_SZONE),
    ("grave", LOCATION_GRAVE),
    ("removed", LOCATION_REMOVED),
    ("extra", LOCATION_EXTRA),
];

/// Internal status bits that record hidden decisions: choosing an attacker clears
/// ATTACK_CANCELED even if the player then cancels the target selection.
pub const PRIVATE_STATUS: u32 = STATUS_ATTACK_CANCELED;

/// One card of a location query (the fields the query had), plus its card id in a capture.
/// `reason_card` / `equip_card` are null (Some(None)) for no card.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CardQuery {
    #[serde(default, skip_serializing_if = "is_none")]
    pub code: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub position: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub alias: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none", rename = "type")]
    pub type_: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub level: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub rank: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub attribute: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub race: Option<u64>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub attack: Option<i32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub defense: Option<i32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub base_attack: Option<i32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub base_defense: Option<i32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub reason: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none", deserialize_with = "present")]
    pub reason_card: Option<Option<Loc>>,
    #[serde(default, skip_serializing_if = "is_none", deserialize_with = "present")]
    pub equip_card: Option<Option<Loc>>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub target_cards: Option<Vec<Option<Loc>>>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub overlays: Option<Vec<u32>>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub counters: Option<Vec<u32>>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub owner: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub status: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub is_public: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub lscale: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub rscale: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub link: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub link_marker: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub is_hidden: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub cover: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub id: Option<u32>,
    #[serde(default, skip_serializing_if = "is_none")]
    pub overlay_ids: Option<Vec<u32>>,
}

/// A link of the current chain (OCG_DuelQueryField). In a capture `related` tells whether the
/// link's card is still the object that was activated; a view leaves it out and has no
/// `handler` (null) for a card that is a new object.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SnapChainLink {
    pub code: u32,
    pub handler: Option<Loc>,
    pub trigger_con: u32,
    pub trigger_loc: u32,
    pub trigger_seq: u32,
    pub desc: u64,
    #[serde(default, skip_serializing_if = "is_none")]
    pub related: Option<bool>,
}

pub type Pile = Vec<Option<CardQuery>>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlayerCards {
    pub deck: Pile,
    pub hand: Pile,
    pub mzone: Pile,
    pub szone: Pile,
    pub grave: Pile,
    pub removed: Pile,
    pub extra: Pile,
}

impl PlayerCards {
    pub fn pile(&self, loc: u32) -> &Pile {
        match loc {
            LOCATION_DECK => &self.deck,
            LOCATION_HAND => &self.hand,
            LOCATION_MZONE => &self.mzone,
            LOCATION_SZONE => &self.szone,
            LOCATION_GRAVE => &self.grave,
            LOCATION_REMOVED => &self.removed,
            _ => &self.extra,
        }
    }
    pub fn pile_mut(&mut self, loc: u32) -> &mut Pile {
        match loc {
            LOCATION_DECK => &mut self.deck,
            LOCATION_HAND => &mut self.hand,
            LOCATION_MZONE => &mut self.mzone,
            LOCATION_SZONE => &mut self.szone,
            LOCATION_GRAVE => &mut self.grave,
            LOCATION_REMOVED => &mut self.removed,
            _ => &mut self.extra,
        }
    }
}

/// A god view snapshot ([`capture`]) or a player's view of one ([`view`]).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Snapshot {
    pub turn: u32,
    pub turn_player: u32,
    pub phase: u32,
    pub lp: [i32; 2],
    pub chain: Vec<SnapChainLink>,
    pub players: Vec<PlayerCards>,
}

fn u32_at(buf: &[u8], p: usize) -> u32 {
    u32::from_le_bytes(buf[p..p + 4].try_into().unwrap())
}

fn loc_info(buf: &[u8], p: usize) -> Option<Loc> {
    let loc = buf[p + 1] as u32;
    (loc != 0).then(|| Loc { con: buf[p] as u32, loc, seq: u32_at(buf, p + 2), pos: u32_at(buf, p + 6) })
}

/// Parses one card query starting at `pos`: (the card or None for an empty slot, the new pos).
pub fn parse_card_query(buf: &[u8], mut pos: usize) -> (Option<CardQuery>, usize) {
    if buf[pos] == 0 && buf[pos + 1] == 0 {
        return (None, pos + 2);
    }
    let mut c = CardQuery::default();
    loop {
        let size = u16::from_le_bytes([buf[pos], buf[pos + 1]]) as usize;
        let flag = u32_at(buf, pos + 2);
        let p = pos + 6;
        pos += 2 + size;
        let v = || u32_at(buf, p);
        let list = || (0..u32_at(buf, p) as usize).map(|i| u32_at(buf, p + 4 + 4 * i)).collect::<Vec<_>>();
        match flag {
            QUERY_CODE => c.code = Some(v()),
            QUERY_POSITION => c.position = Some(v()),
            QUERY_ALIAS => c.alias = Some(v()),
            QUERY_TYPE => c.type_ = Some(v()),
            QUERY_LEVEL => c.level = Some(v()),
            QUERY_RANK => c.rank = Some(v()),
            QUERY_ATTRIBUTE => c.attribute = Some(v()),
            QUERY_RACE => c.race = Some(u64::from_le_bytes(buf[p..p + 8].try_into().unwrap())),
            QUERY_ATTACK => c.attack = Some(v() as i32),
            QUERY_DEFENSE => c.defense = Some(v() as i32),
            QUERY_BASE_ATTACK => c.base_attack = Some(v() as i32),
            QUERY_BASE_DEFENSE => c.base_defense = Some(v() as i32),
            QUERY_REASON => c.reason = Some(v()),
            QUERY_OWNER => c.owner = Some(buf[p] as u32),
            QUERY_STATUS => c.status = Some(v()),
            QUERY_IS_PUBLIC => c.is_public = Some(buf[p] as u32),
            QUERY_LSCALE => c.lscale = Some(v()),
            QUERY_RSCALE => c.rscale = Some(v()),
            QUERY_IS_HIDDEN => c.is_hidden = Some(buf[p] as u32),
            QUERY_COVER => c.cover = Some(v()),
            QUERY_END => return (Some(c), pos),
            QUERY_REASON_CARD => c.reason_card = Some(loc_info(buf, p)),
            QUERY_EQUIP_CARD => c.equip_card = Some(loc_info(buf, p)),
            QUERY_TARGET_CARD => c.target_cards = Some((0..v() as usize).map(|i| loc_info(buf, p + 4 + 10 * i)).collect()),
            QUERY_OVERLAY_CARD => c.overlays = Some(list()),
            QUERY_COUNTERS => c.counters = Some(list()),
            QUERY_LINK => {
                c.link = Some(v());
                c.link_marker = Some(u32_at(buf, p + 4));
            }
            _ => {}
        }
    }
}

pub fn parse_location_query(buf: &[u8]) -> Pile {
    if buf.is_empty() {
        return Vec::new();
    }
    let end = 4 + u32_at(buf, 0) as usize;
    let (mut pos, mut out) = (4, Vec::new());
    while pos < end {
        let (card, p) = parse_card_query(buf, pos);
        out.push(card);
        pos = p;
    }
    out
}

/// The chain of an OCG_DuelQueryField buffer (the rest is in the snapshot).
pub fn parse_field_chain(buf: &[u8]) -> Vec<SnapChainLink> {
    let mut pos = 4;
    for _ in 0..2 {
        pos += 4;
        for _ in 0..7 + 8 {
            let present = buf[pos];
            pos += 1;
            if present != 0 {
                pos += 1 + 4;
            }
        }
        pos += 6 * 4;
    }
    let n = u32_at(buf, pos);
    pos += 4;
    (0..n)
        .map(|_| {
            let l = SnapChainLink {
                code: u32_at(buf, pos),
                handler: Some(Loc { con: buf[pos + 4] as u32, loc: buf[pos + 5] as u32, seq: u32_at(buf, pos + 6), pos: u32_at(buf, pos + 10) }),
                trigger_con: buf[pos + 14] as u32,
                trigger_loc: buf[pos + 15] as u32,
                trigger_seq: u32_at(buf, pos + 16),
                desc: u64::from_le_bytes(buf[pos + 20..pos + 28].try_into().unwrap()),
                related: None,
            };
            pos += 28;
            l
        })
        .collect()
}

fn header(d: &mut Duel) -> Snapshot {
    let [turn, turn_player, phase, lp0, lp1] = d.ext_get_info();
    let mut chain = parse_field_chain(&d.ocg_query_field());
    for (link, related) in chain.iter_mut().zip(d.ext_chain_related()) {
        link.related = Some(related);
    }
    Snapshot { turn, turn_player, phase, lp: [lp0 as i32, lp1 as i32], chain, players: Vec::new() }
}

fn query_location(d: &mut Duel, p: u32, loc: u32) -> Pile {
    let flags = if loc == LOCATION_DECK { QUERY_BASIC } else { QUERY_ALL };
    parse_location_query(&d.ocg_query_location(&QueryInfo { flags, con: p as u8, loc, seq: 0, overlay_seq: 0 }))
}

/// God view snapshot: every card of every location with full query data and card id.
pub fn capture(d: &mut Duel) -> Snapshot {
    let mut snap = header(d);
    let all = d.ext_get_all_cards();
    for p in 0..2u32 {
        let mut pl = PlayerCards::default();
        for (_, loc) in LOCATIONS {
            let mut cards = query_location(d, p, loc);
            let entries: Vec<&[u32; 6]> = all.iter().filter(|e| e[0] == p && e[1] == loc).collect();
            let slots: Vec<&&[u32; 6]> = entries.iter().filter(|e| e[3] == 0).collect();
            for (i, card) in cards.iter_mut().enumerate() {
                if let Some(card) = card {
                    card.id = Some(slots[i][5]);
                    if card.overlays.as_ref().is_some_and(|o| !o.is_empty()) {
                        card.overlay_ids = Some(entries.iter().filter(|e| e[2] == i as u32 && e[3] != 0).map(|e| e[5]).collect());
                    }
                }
            }
            *pl.pile_mut(loc) = cards;
        }
        snap.players.push(pl);
    }
    snap
}

/// What `viewer` can see of the duel now (view(capture(duel), viewer), without looking up the
/// card ids).
pub fn viewer_view(d: &mut Duel, viewer: u32) -> Snapshot {
    let snap = header(d);
    let mut out = view_header(&snap, viewer);
    for p in 0..2u32 {
        let mut vp = PlayerCards::default();
        for (_, loc) in LOCATIONS {
            *vp.pile_mut(loc) = view_location(&query_location(d, p, loc), p, loc, viewer);
        }
        out.players.push(vp);
    }
    out
}

pub fn card_visible(card: &CardQuery, controller: u32, location: u32, viewer: u32) -> bool {
    if location & (LOCATION_GRAVE | LOCATION_OVERLAY) != 0 {
        return true;
    }
    if card.is_public.unwrap_or(0) != 0 || card.position.unwrap_or(0) & POS_FACEUP != 0 {
        return true;
    }
    controller == viewer && location != LOCATION_DECK
}

/// References to cards in hidden piles must not reveal the card's position there.
fn mask_ref(li: Option<Loc>, viewer: u32) -> Option<Loc> {
    let li = li?;
    if li_visible(&li, viewer) && li.loc & LOCATION_DECK == 0 {
        return Some(li);
    }
    Some(Loc { con: li.con, loc: li.loc, seq: 0, pos: canon_pos(li.loc, li.pos) })
}

/// What `viewer` can see of a god view snapshot.
pub fn view(snap: &Snapshot, viewer: u32) -> Snapshot {
    let mut out = view_header(snap, viewer);
    for (p, pl) in snap.players.iter().enumerate() {
        let mut vp = PlayerCards::default();
        for (_, loc) in LOCATIONS {
            *vp.pile_mut(loc) = view_location(pl.pile(loc), p as u32, loc, viewer);
        }
        out.players.push(vp);
    }
    out
}

fn view_header(snap: &Snapshot, viewer: u32) -> Snapshot {
    // (a link's card that left its place since, e.g. shuffled into the deck, is a new object:
    // which physical copy it is now cannot be known, even when a copy of it is back on the field)
    let chain = snap
        .chain
        .iter()
        .map(|c| SnapChainLink {
            handler: if c.related.unwrap_or(true) { mask_ref(c.handler, viewer) } else { None },
            related: None,
            ..c.clone()
        })
        .collect();
    Snapshot { turn: snap.turn, turn_player: snap.turn_player, phase: snap.phase, lp: snap.lp, chain, players: Vec::new() }
}

/// What `viewer` can see of player `p`'s cards in location `loc`.
pub fn view_location(pile: &Pile, p: u32, loc: u32, viewer: u32) -> Pile {
    pile.iter()
        .map(|card| {
            let card = card.as_ref()?;
            let mask_refs = |c: &mut CardQuery| {
                if let Some(e) = c.equip_card.as_mut() {
                    *e = mask_ref(*e, viewer);
                }
                if let Some(t) = c.target_cards.as_mut() {
                    t.iter_mut().for_each(|x| *x = mask_ref(*x, viewer));
                }
            };
            if card_visible(card, p, loc, viewer) {
                // (reason_card is left out entirely: it points at whichever copy caused the move,
                // wherever that copy is hidden now)
                let mut shown = CardQuery { reason_card: None, id: None, overlay_ids: None, ..card.clone() };
                if loc & (LOCATION_GRAVE | LOCATION_REMOVED) == 0 {
                    // cards in the hand or on the field keep the stale reason of how they got
                    // there earlier; it would reveal hidden history and make identical copies
                    // distinguishable
                    shown.reason = None;
                }
                if loc & (LOCATION_MZONE | LOCATION_SZONE) == 0 {
                    // status bits of cards in piles are stale leftovers from earlier trips to the
                    // field; they would make identical copies distinguishable after a shuffle
                    shown.status = None;
                } else if let Some(s) = shown.status.as_mut() {
                    *s &= !PRIVATE_STATUS;
                }
                mask_refs(&mut shown);
                Some(shown)
            } else {
                let mut hidden = CardQuery {
                    position: card.position.map(|pos| canon_pos(loc, pos)),
                    owner: card.owner,
                    counters: card.counters.clone(),
                    equip_card: card.equip_card,
                    target_cards: card.target_cards.clone(),
                    overlays: card.overlays.clone(),
                    ..CardQuery::default()
                };
                mask_refs(&mut hidden);
                Some(hidden)
            }
        })
        .collect()
}
