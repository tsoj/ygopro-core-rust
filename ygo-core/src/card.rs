//! Cards (`card.h`, `card.cpp` part 1: the card, its state and its properties).

use crate::constants::*;
use crate::duel::{Duel, EffectContainer};
use crate::effect::sort_effects_by_id;
use crate::types::*;

/// `card_state`
#[derive(Clone, Copy, Debug, Default)]
pub struct CardState {
    pub code: u32,
    pub code2: u32,
    pub setcodes: SetCodes,
    pub type_: u32,
    pub level: u32,
    pub rank: u32,
    pub link: u32,
    pub link_marker: u32,
    pub lscale: u32,
    pub rscale: u32,
    pub attribute: u32,
    pub race: u64,
    pub attack: i32,
    pub defense: i32,
    pub base_attack: i32,
    pub base_defense: i32,
    pub controler: u8,
    pub location: u8,
    pub sequence: u32,
    pub position: u32,
    pub reason: u32,
    pub pzone: bool,
    pub reason_card: Option<CardId>,
    pub reason_player: u8,
    pub reason_effect: Option<EffectId>,
}

/// The values of a card being computed (`card::temp`, for the stats): kept out of `Card` so that
/// these short-lived writes do not copy a card shared with a copy of the duel.
#[derive(Clone, Copy, Debug)]
pub struct StatTemp {
    pub code: u32,
    pub type_: u32,
    pub level: u32,
    pub rank: u32,
    pub link: u32,
    pub link_marker: u32,
    pub lscale: u32,
    pub rscale: u32,
    pub attribute: u32,
    pub race: u64,
    pub attack: i32,
    pub defense: i32,
    pub base_attack: i32,
    pub base_defense: i32,
}

impl Default for StatTemp {
    fn default() -> Self {
        StatTemp {
            code: u32::MAX,
            type_: u32::MAX,
            level: u32::MAX,
            rank: u32::MAX,
            link: u32::MAX,
            link_marker: u32::MAX,
            lscale: u32::MAX,
            rscale: u32::MAX,
            attribute: u32::MAX,
            race: u64::MAX,
            attack: -1,
            defense: -1,
            base_attack: -1,
            base_defense: -1,
        }
    }
}

/// The location part of a card state (`card_state::is_location`).
#[derive(Clone, Copy, Debug)]
pub struct LocState {
    pub location: u8,
    pub sequence: u32,
    pub pzone: bool,
}

impl LocState {
    pub fn is_location(&self, loc: u32) -> bool {
        if self.location as u32 & (loc & 0xff) != 0 {
            return true;
        }
        if loc & LOCATION_EMZONE != 0 && self.location as u32 == LOCATION_MZONE && self.sequence >= 5 {
            return true;
        }
        if loc & LOCATION_MMZONE != 0 && self.location as u32 == LOCATION_MZONE && self.sequence < 5 {
            return true;
        }
        if loc & LOCATION_STZONE != 0 && self.location as u32 == LOCATION_SZONE && self.sequence < 5 {
            return true;
        }
        if loc & LOCATION_FZONE != 0 && self.location as u32 == LOCATION_SZONE && self.sequence == 5 {
            return true;
        }
        if loc & LOCATION_PZONE != 0 && self.location as u32 == LOCATION_SZONE && self.pzone {
            return true;
        }
        false
    }
}

impl CardState {
    pub fn is_location(&self, loc: u32) -> bool {
        self.clone_loc().is_location(loc)
    }
    pub fn clone_loc(&self) -> LocState {
        LocState { location: self.location, sequence: self.sequence, pzone: self.pzone }
    }
    pub fn set0xff(&mut self) {
        self.code = u32::MAX;
        self.code2 = u32::MAX;
        self.type_ = u32::MAX;
        self.level = u32::MAX;
        self.rank = u32::MAX;
        self.link = u32::MAX;
        self.link_marker = u32::MAX;
        self.lscale = u32::MAX;
        self.rscale = u32::MAX;
        self.attribute = u32::MAX;
        self.race = u64::MAX;
        self.attack = -1;
        self.defense = -1;
        self.base_attack = -1;
        self.base_defense = -1;
        self.controler = u8::MAX;
        self.location = u8::MAX;
        self.sequence = u32::MAX;
        self.position = u32::MAX;
        self.reason = u32::MAX;
        self.reason_player = u8::MAX;
    }
}

/// `card::summon_info`
#[derive(Clone, Copy, Debug, Default)]
pub struct SummonInfo {
    pub type_: u32,
    pub player: u8,
    pub location: u8,
    pub sequence: u8,
    pub pzone: bool,
}

/// `card::sendto_param_t`
#[derive(Clone, Copy, Debug, Default)]
pub struct SendtoParam {
    pub playerid: u8,
    pub position: u8,
    pub location: u8,
    pub sequence: u8,
}

impl SendtoParam {
    pub fn set(&mut self, p: u8, pos: u8, loc: u8, seq: u8) {
        self.playerid = p;
        self.position = pos;
        self.location = loc;
        self.sequence = seq;
    }
    pub fn clear(&mut self) {
        *self = SendtoParam::default();
    }
}

/// `card::attacker_map`: fieldid_r -> (card, count)
#[derive(Clone, Debug, Default)]
pub struct AttackerMap {
    pub items: Vec<(u32, Option<CardId>, u32)>,
}

impl AttackerMap {
    pub fn addcard(&mut self, d_fieldid_r: Option<(CardId, u32)>) {
        let (pcard, fid) = match d_fieldid_r {
            Some((c, f)) => (Some(c), f),
            None => (None, 0),
        };
        if let Some(e) = self.items.iter_mut().find(|e| e.0 == fid) {
            e.2 += 1;
        } else {
            self.items.push((fid, pcard, 1));
        }
    }
    pub fn findcard(&self, fid: u32) -> u32 {
        self.items.iter().find(|e| e.0 == fid).map_or(0, |e| e.2)
    }
    pub fn clear(&mut self) {
        self.items.clear();
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// `card`
#[derive(Clone, Debug)]
pub struct Card {
    pub data: CardData,
    pub previous: CardState,
    pub temp: CardState,
    pub current: CardState,
    pub owner: u8,
    pub summon: SummonInfo,
    pub status: u32,
    pub cover: u32,
    pub sendto_param: SendtoParam,
    pub release_param: u32,
    pub sum_param: u32,
    pub position_param: u32,
    pub spsummon_param: u32,
    pub to_field_param: u32,
    pub attack_announce_count: u8,
    pub direct_attackable: u8,
    pub announce_count: u8,
    pub attacked_count: u8,
    pub attack_all_target: u8,
    pub attack_controler: u8,
    pub cardid: u32,
    /// The code whose script the card uses (`interpreter::register_card`: alternative artworks use the original's).
    pub script_code: u32,
    pub fieldid: u32,
    pub fieldid_r: u32,
    pub turnid: u16,
    pub turn_counter: u16,
    pub unique_pos: [u8; 2],
    pub unique_fieldid: u32,
    pub unique_code: u32,
    pub unique_location: u32,
    pub unique_function: Option<LuaFn>,
    pub unique_effect: Option<EffectId>,
    pub spsummon_code: u32,
    pub spsummon_counter: [u16; 2],
    pub spsummon_counter_rst: [u16; 2],
    /// `std::map<uint32_t, uint64_t>`
    pub assume: Vec<(u32, u64)>,
    pub equiping_target: Option<CardId>,
    pub pre_equip_target: Option<CardId>,
    pub overlay_target: Option<CardId>,
    pub pre_overlay_target: Option<CardId>,
    /// `relation_map` (card -> reset flags)
    pub relations: Vec<(CardId, u32)>,
    /// `counter_map` (sorted by counter type)
    pub counters: Vec<(u16, [u16; 2])>,
    /// `effect_count` (sorted)
    pub indestructable_effects: Vec<(u32, i32)>,
    pub announced_cards: AttackerMap,
    pub attacked_cards: AttackerMap,
    pub battled_cards: AttackerMap,
    pub equiping_cards: CardSet,
    pub material_cards: CardSet,
    pub effect_target_owner: CardSet,
    pub effect_target_cards: CardSet,
    pub xyz_materials: Vec<CardId>,
    pub single_effect: EffectContainer,
    pub field_effect: EffectContainer,
    pub equip_effect: EffectContainer,
    pub target_effect: EffectContainer,
    pub xmaterial_effect: EffectContainer,
    /// `indexer`: the effects of the card and the container they are in (1..5 as above)
    pub indexer: Vec<(EffectId, u8)>,
    /// `effect_relation`
    pub relate_effect: Vec<(EffectId, u16)>,
    pub immune_effect: Vec<EffectId>,
}

impl Card {
    pub fn new() -> Card {
        let mut temp = CardState::default();
        temp.set0xff();
        let current = CardState { controler: PLAYER_NONE as u8, ..Default::default() };
        Card {
            data: CardData::default(),
            previous: CardState::default(),
            temp,
            current,
            owner: PLAYER_NONE as u8,
            summon: SummonInfo::default(),
            status: 0,
            cover: 0,
            sendto_param: SendtoParam::default(),
            release_param: 0,
            sum_param: 0,
            position_param: 0,
            spsummon_param: 0,
            to_field_param: 0,
            attack_announce_count: 0,
            direct_attackable: 0,
            announce_count: 0,
            attacked_count: 0,
            attack_all_target: 0,
            attack_controler: 0,
            cardid: 0,
            script_code: 0,
            fieldid: 0,
            fieldid_r: 0,
            turnid: 0,
            turn_counter: 0,
            unique_pos: [0, 0],
            unique_fieldid: 0,
            unique_code: 0,
            unique_location: 0,
            unique_function: None,
            unique_effect: None,
            spsummon_code: 0,
            spsummon_counter: [0, 0],
            spsummon_counter_rst: [0, 0],
            assume: Vec::new(),
            equiping_target: None,
            pre_equip_target: None,
            overlay_target: None,
            pre_overlay_target: None,
            relations: Vec::new(),
            counters: Vec::new(),
            indestructable_effects: Vec::new(),
            announced_cards: AttackerMap::default(),
            attacked_cards: AttackerMap::default(),
            battled_cards: AttackerMap::default(),
            equiping_cards: CardSet::new(),
            material_cards: CardSet::new(),
            effect_target_owner: CardSet::new(),
            effect_target_cards: CardSet::new(),
            xyz_materials: Vec::new(),
            single_effect: EffectContainer::default(),
            field_effect: EffectContainer::default(),
            equip_effect: EffectContainer::default(),
            target_effect: EffectContainer::default(),
            xmaterial_effect: EffectContainer::default(),
            indexer: Vec::new(),
            relate_effect: Vec::new(),
            immune_effect: Vec::new(),
        }
    }
    #[inline]
    pub fn is_position(&self, pos: u32) -> u32 {
        self.current.position & pos
    }
    #[inline]
    pub fn set_status(&mut self, s: u32, enabled: bool) {
        if enabled {
            self.status |= s;
        } else {
            self.status &= !s;
        }
    }
    /// match at least 1 status
    #[inline]
    pub fn get_status(&self, s: u32) -> u32 {
        self.status & s
    }
    /// match all statuses
    #[inline]
    pub fn is_status(&self, s: u32) -> bool {
        self.status & s == s
    }
    pub fn get_assumed(&self, t: u32) -> Option<u64> {
        self.assume.iter().find(|a| a.0 == t).map(|a| a.1)
    }
    pub fn is_link_marker_of(dir: u32, marker: u32) -> bool {
        marker & dir != 0
    }
    pub fn match_setcode(set_code: u16, to_match: u16) -> bool {
        (set_code & 0xfff) == (to_match & 0xfff) && (set_code & to_match) == set_code
    }
}

impl Default for Card {
    fn default() -> Self {
        Card::new()
    }
}

/// mapping of double-name cards
pub fn second_code(code: u32) -> u32 {
    match code {
        CARD_MARINE_DOLPHIN => 17955766,
        CARD_TWINKLE_MOSS => 17732278,
        _ => 0,
    }
}

/// `card_operation_sort`: the order of cards in operations (`std::sort` comparator).
pub fn card_operation_sort(d: &Duel, c1: CardId, c2: CardId) -> bool {
    let a = d.card(c1);
    let b = d.card(c2);
    let cp1 = match a.overlay_target {
        Some(o) => d.card(o).current.controler as i32,
        None => a.current.controler as i32,
    };
    let cp2 = match b.overlay_target {
        Some(o) => d.card(o).current.controler as i32,
        None => b.current.controler as i32,
    };
    if cp1 != cp2 {
        if cp1 == PLAYER_NONE as i32 || cp2 == PLAYER_NONE as i32 {
            return cp1 < cp2;
        }
        if d.infos.turn_player == 0 {
            return cp1 < cp2;
        } else {
            return cp1 > cp2;
        }
    }
    if a.current.location != b.current.location {
        return a.current.location < b.current.location;
    }
    if a.current.location as u32 & LOCATION_OVERLAY != 0 {
        let s1 = d.card(a.overlay_target.unwrap()).current.sequence;
        let s2 = d.card(b.overlay_target.unwrap()).current.sequence;
        if s1 != s2 {
            s1 < s2
        } else {
            a.current.sequence < b.current.sequence
        }
    } else if a.current.location as u32 & (LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE | LOCATION_REMOVED) != 0 {
        a.current.sequence > b.current.sequence
    } else {
        a.current.sequence < b.current.sequence
    }
}

/// `card_public_sort`
pub fn card_public_sort(d: &Duel, c1: CardId, c2: CardId) -> bool {
    if card_operation_sort(d, c1, c2) {
        return true;
    }
    if card_operation_sort(d, c2, c1) {
        return false;
    }
    let a = d.card(c1);
    let b = d.card(c2);
    if a.current.location as u32 & LOCATION_OVERLAY != 0 && b.current.location as u32 & LOCATION_OVERLAY != 0 {
        if let (Some(o1), Some(o2)) = (a.overlay_target, b.overlay_target) {
            let l1 = d.card(o1).current.location;
            let l2 = d.card(o2).current.location;
            if l1 != l2 {
                return l1 < l2;
            }
        }
    }
    a.cardid < b.cardid
}

/// Sorts with a C++-style "less" comparator (a strict weak ordering).
pub fn sort_by_less<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], less: F) {
    v.sort_by(|&a, &b| {
        if less(a, b) {
            std::cmp::Ordering::Less
        } else if less(b, a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
}

/// `card::public_order`: the cards of a set in public order.
pub fn public_order(d: &Duel, cset: &CardSet) -> Vec<CardId> {
    if cset.is_empty() {
        return Vec::new();
    }
    let mut cv = cset.to_vec();
    if cv.len() > 1 {
        sort_by_less(&mut cv, |a, b| card_public_sort(d, a, b));
    }
    cv
}

fn query_insert_u32(buf: &mut Vec<u8>, query: u32, value: u32) {
    buf.extend_from_slice(&((4 + 4) as u16).to_le_bytes());
    buf.extend_from_slice(&query.to_le_bytes());
    buf.extend_from_slice(&value.to_le_bytes());
}

impl CardId {
    #[inline]
    fn c<'a>(&self, d: &'a Duel) -> &'a Card {
        d.card(*self)
    }

    pub fn is_extra_deck_monster(self, d: &Duel) -> bool {
        let t = self.c(d).data.type_;
        t & TYPE_MONSTER != 0 && t & d.get_extra_deck_types() != 0
    }

    /// `card::get_infos`: writes the queried properties to `d.query_buffer`.
    pub fn get_infos(self, d: &mut Duel, query_flag: u32) {
        let was = d.querying;
        d.querying = true;
        let mut buf = std::mem::take(&mut d.query_buffer);
        macro_rules! ins {
            ($q:expr, $v:expr) => {
                if query_flag & $q != 0 {
                    let v = $v;
                    query_insert_u32(&mut buf, $q, v as u32);
                }
            };
        }
        ins!(QUERY_CODE, self.c(d).data.code);
        ins!(QUERY_POSITION, self.get_info_location(d).position);
        ins!(QUERY_ALIAS, self.get_code(d));
        ins!(QUERY_TYPE, self.get_type(d, None, 0, 2));
        ins!(QUERY_LEVEL, self.get_level(d));
        ins!(QUERY_RANK, self.get_rank(d));
        ins!(QUERY_ATTRIBUTE, self.get_attribute(d, None, 0, 2));
        if query_flag & QUERY_RACE != 0 {
            let v = self.get_race(d, None, 0, 2);
            buf.extend_from_slice(&((4 + 8) as u16).to_le_bytes());
            buf.extend_from_slice(&QUERY_RACE.to_le_bytes());
            buf.extend_from_slice(&v.to_le_bytes());
        }
        ins!(QUERY_ATTACK, self.get_attack(d));
        ins!(QUERY_DEFENSE, self.get_defense(d));
        ins!(QUERY_BASE_ATTACK, self.get_base_attack(d));
        ins!(QUERY_BASE_DEFENSE, self.get_base_defense(d));
        ins!(QUERY_REASON, self.c(d).current.reason);
        ins!(QUERY_COVER, self.c(d).cover);
        let loc_or_zero = |d: &Duel, buf: &mut Vec<u8>, c: Option<CardId>| match c {
            Some(c) => {
                let info = c.get_info_location(d);
                buf.push(info.controler);
                buf.push(info.location);
                buf.extend_from_slice(&info.sequence.to_le_bytes());
                buf.extend_from_slice(&info.position.to_le_bytes());
            }
            None => {
                buf.extend_from_slice(&0u16.to_le_bytes());
                buf.extend_from_slice(&0u64.to_le_bytes());
            }
        };
        if query_flag & QUERY_REASON_CARD != 0 {
            buf.extend_from_slice(&((4 + 2 + 8) as u16).to_le_bytes());
            buf.extend_from_slice(&QUERY_REASON_CARD.to_le_bytes());
            loc_or_zero(d, &mut buf, self.c(d).current.reason_card);
        }
        if query_flag & QUERY_EQUIP_CARD != 0 {
            buf.extend_from_slice(&((4 + 2 + 8) as u16).to_le_bytes());
            buf.extend_from_slice(&QUERY_EQUIP_CARD.to_le_bytes());
            loc_or_zero(d, &mut buf, self.c(d).equiping_target);
        }
        if query_flag & QUERY_TARGET_CARD != 0 {
            let targets = public_order(d, &self.c(d).effect_target_cards);
            buf.extend_from_slice(&((4 + 4 + targets.len() * (2 + 8)) as u16).to_le_bytes());
            buf.extend_from_slice(&QUERY_TARGET_CARD.to_le_bytes());
            buf.extend_from_slice(&(targets.len() as u32).to_le_bytes());
            for t in targets {
                loc_or_zero(d, &mut buf, Some(t));
            }
        }
        if query_flag & QUERY_OVERLAY_CARD != 0 {
            let mats = self.c(d).xyz_materials.clone();
            buf.extend_from_slice(&((4 + 4 + mats.len() * 4) as u16).to_le_bytes());
            buf.extend_from_slice(&QUERY_OVERLAY_CARD.to_le_bytes());
            buf.extend_from_slice(&(mats.len() as u32).to_le_bytes());
            for m in mats {
                buf.extend_from_slice(&d.card(m).data.code.to_le_bytes());
            }
        }
        if query_flag & QUERY_COUNTERS != 0 {
            let counters = self.c(d).counters.clone();
            buf.extend_from_slice(&((4 + 4 + counters.len() * 4) as u16).to_le_bytes());
            buf.extend_from_slice(&QUERY_COUNTERS.to_le_bytes());
            buf.extend_from_slice(&(counters.len() as u32).to_le_bytes());
            for (t, v) in counters {
                let x = (t as u32).wrapping_add(((v[0] as u32 + v[1] as u32) << 16) as u32);
                buf.extend_from_slice(&x.to_le_bytes());
            }
        }
        if query_flag & QUERY_OWNER != 0 {
            buf.extend_from_slice(&((4 + 1) as u16).to_le_bytes());
            buf.extend_from_slice(&QUERY_OWNER.to_le_bytes());
            buf.push(self.c(d).owner);
        }
        ins!(QUERY_STATUS, self.c(d).status);
        {
            buf.extend_from_slice(&((4 + 1) as u16).to_le_bytes());
            buf.extend_from_slice(&QUERY_IS_PUBLIC.to_le_bytes());
            let is_public = self.c(d).is_position(POS_FACEUP) != 0
                || self.is_related_to_chains(d)
                || (self.c(d).current.is_location(LOCATION_HAND | LOCATION_ONFIELD) && self.is_affected_by_effect(d, EFFECT_PUBLIC).is_some());
            buf.push(is_public as u8);
        }
        ins!(QUERY_LSCALE, self.get_lscale(d));
        ins!(QUERY_RSCALE, self.get_rscale(d));
        if query_flag & QUERY_LINK != 0 {
            let link = self.get_link(d);
            let marker = self.get_link_marker(d);
            buf.extend_from_slice(&((4 + 4 + 4) as u16).to_le_bytes());
            buf.extend_from_slice(&QUERY_LINK.to_le_bytes());
            buf.extend_from_slice(&link.to_le_bytes());
            buf.extend_from_slice(&marker.to_le_bytes());
        }
        if query_flag & QUERY_IS_HIDDEN != 0 {
            let hidden = self.is_affected_by_effect(d, EFFECT_DARKNESS_HIDE).is_some();
            buf.extend_from_slice(&((4 + 1) as u16).to_le_bytes());
            buf.extend_from_slice(&QUERY_IS_HIDDEN.to_le_bytes());
            buf.push(hidden as u8);
        }
        buf.extend_from_slice(&(4u16).to_le_bytes());
        buf.extend_from_slice(&QUERY_END.to_le_bytes());
        d.query_buffer = buf;
        d.querying = was;
    }

    pub fn is_related_to_chains(self, d: &Duel) -> bool {
        if !self.c(d).is_status(STATUS_CHAINING) {
            return false;
        }
        for ch in &d.core.current_chain {
            let te = ch.triggering_effect.unwrap();
            if te.get_handler(d) == self && self.is_has_relation_effect(d, te) {
                return true;
            }
        }
        false
    }

    pub fn get_info_location(self, d: &Duel) -> LocInfo {
        let c = self.c(d);
        if let Some(o) = c.overlay_target {
            let oc = d.card(o);
            LocInfo {
                controler: oc.current.controler,
                location: ((oc.current.location as u32 | LOCATION_OVERLAY) & 0xff) as u8,
                sequence: oc.current.sequence,
                position: c.current.sequence,
            }
        } else {
            LocInfo {
                controler: c.current.controler,
                location: c.current.location,
                sequence: c.current.sequence,
                position: c.current.position,
            }
        }
    }

    /// the current card name; for double-name cards, the printed name
    pub fn get_code(self, d: &mut Duel) -> u32 {
        d.note_read(self, 1);
        if let Some(v) = self.c(d).get_assumed(ASSUME_CODE) {
            return v as u32;
        }
        if d.stat_temp[self.0 as usize].code != u32::MAX {
            return d.stat_temp[self.0 as usize].code;
        }
        let data_code = self.c(d).data.code;
        let mut code = data_code;
        d.stat_temp[self.0 as usize].code = data_code;
        let mut effects = Vec::new();
        self.filter_effect(d, EFFECT_CHANGE_CODE, &mut effects, true);
        if let Some(&last) = effects.last() {
            code = last.get_value_card(d, Some(self), 0) as u32;
        }
        d.stat_temp[self.0 as usize].code = u32::MAX;
        if code == data_code {
            effects.clear();
            self.filter_effect(d, EFFECT_ADD_CODE, &mut effects, true);
            let addcode = effects.iter().rev().find(|&&e| d.effect(e).operation.is_none()).copied();
            let alias = self.c(d).data.alias;
            if alias != 0 && addcode.is_none() {
                code = alias;
            }
        } else {
            let dat = d.read_card(code);
            if dat.alias != 0 && second_code(code) == 0 {
                code = dat.alias;
            }
        }
        code
    }
    /// the current second card name; for double-name cards, the name in the description
    pub fn get_another_code(self, d: &mut Duel) -> u32 {
        let code = self.get_code(d);
        if code != self.c(d).data.code {
            return second_code(code);
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_ADD_CODE, &mut eset, true);
        let Some(addcode) = eset.iter().rev().find(|&&e| d.effect(e).operation.is_none()).copied() else {
            return 0;
        };
        let otcode = addcode.get_value_card(d, Some(self), 0) as u32;
        if self.get_code(d) != otcode && code != otcode {
            return otcode;
        }
        0
    }
    pub fn get_summon_code(self, d: &mut Duel, codes: &mut std::collections::BTreeSet<u32>, scard: Option<CardId>, sumtype: u64, playerid: u8) {
        let mut eset = Vec::new();
        let mut changed = false;
        self.filter_effect(d, EFFECT_ADD_CODE, &mut eset, false);
        self.filter_effect(d, EFFECT_REMOVE_CODE, &mut eset, false);
        self.filter_effect(d, EFFECT_CHANGE_CODE, &mut eset, true);
        for peffect in eset {
            let Some(op) = d.effect(peffect).operation.clone() else { continue };
            d.add_param(Value::card(scard));
            d.add_param(Value::Int(sumtype as i64));
            d.add_param(Value::Int(playerid as i64));
            if !d.check_condition(Some(&op), 3) {
                continue;
            }
            let ecode = d.effect(peffect).code;
            if ecode == EFFECT_ADD_CODE {
                codes.insert(peffect.get_value_card(d, Some(self), 0) as u32);
            } else if ecode == EFFECT_REMOVE_CODE {
                let v = peffect.get_value_card(d, Some(self), 0) as u32;
                codes.remove(&v);
            } else {
                codes.clear();
                codes.insert(peffect.get_value_card(d, Some(self), 0) as u32);
                changed = true;
            }
        }
        if !changed {
            codes.insert(self.get_code(d));
            let code = self.get_another_code(d);
            if code != 0 {
                codes.insert(code);
            }
        }
    }
    pub fn get_set_card(self, d: &mut Duel, setcodes: &mut std::collections::BTreeSet<u16>) {
        d.note_read(self, 256);
        let code = self.get_code(d);
        let og = if code != self.c(d).data.code { d.read_card(code).setcodes } else { self.c(d).data.setcodes };
        setcodes.extend(og.iter());
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_ADD_SETCODE, &mut eset, true);
        for eff in eset {
            if d.effect(eff).operation.is_some() {
                continue;
            }
            let mut value = eff.get_value_card(d, Some(self), 0) as u32;
            while value > 0 {
                setcodes.insert((value & 0xffff) as u16);
                value >>= 16;
            }
        }
        let code2 = self.get_another_code(d);
        if code2 != 0 {
            setcodes.extend(d.read_card(code2).setcodes.iter());
        }
    }
    pub fn get_origin_set_card(self, d: &Duel) -> SetCodes {
        self.c(d).data.setcodes
    }
    pub fn get_pre_set_card(self, d: &Duel, setcodes: &mut std::collections::BTreeSet<u16>) {
        let c = self.c(d);
        let code = c.previous.code;
        let og = if code != c.data.code { d.read_card(code).setcodes } else { c.data.setcodes };
        setcodes.extend(og.iter());
        setcodes.extend(c.previous.setcodes.iter());
        if c.previous.code2 != 0 {
            setcodes.extend(d.read_card(c.previous.code2).setcodes.iter());
        }
    }
    pub fn get_summon_set_card(self, d: &mut Duel, setcodes: &mut std::collections::BTreeSet<u16>, scard: Option<CardId>, sumtype: u64, playerid: u8) {
        let mut eset = Vec::new();
        let mut codes = std::collections::BTreeSet::new();
        let mut changed = false;
        self.filter_effect(d, EFFECT_ADD_CODE, &mut eset, false);
        self.filter_effect(d, EFFECT_REMOVE_CODE, &mut eset, false);
        self.filter_effect(d, EFFECT_CHANGE_CODE, &mut eset, true);
        for peffect in eset {
            let Some(op) = d.effect(peffect).operation.clone() else { continue };
            d.add_param(Value::card(scard));
            d.add_param(Value::Int(sumtype as i64));
            d.add_param(Value::Int(playerid as i64));
            if !d.check_condition(Some(&op), 3) {
                continue;
            }
            let ecode = d.effect(peffect).code;
            if ecode == EFFECT_ADD_CODE {
                codes.insert(peffect.get_value_card(d, Some(self), 0) as u32);
            } else if ecode == EFFECT_REMOVE_CODE {
                let v = peffect.get_value_card(d, Some(self), 0) as u32;
                codes.remove(&v);
            } else {
                codes.clear();
                codes.insert(peffect.get_value_card(d, Some(self), 0) as u32);
                changed = true;
            }
        }
        for code in codes {
            setcodes.extend(d.read_card(code).setcodes.iter());
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_ADD_SETCODE, &mut eset, false);
        self.filter_effect(d, EFFECT_CHANGE_SETCODE, &mut eset, true);
        for peffect in eset {
            let Some(op) = d.effect(peffect).operation.clone() else { continue };
            d.add_param(Value::card(scard));
            d.add_param(Value::Int(sumtype as i64));
            d.add_param(Value::Int(playerid as i64));
            if !d.check_condition(Some(&op), 3) {
                continue;
            }
            let setcode = peffect.get_value_card(d, Some(self), 0) as u32;
            if d.effect(peffect).code == EFFECT_CHANGE_SETCODE {
                setcodes.clear();
                changed = true;
            }
            setcodes.insert((setcode & 0xffff) as u16);
        }
        if !changed {
            self.get_set_card(d, setcodes);
        }
    }
    pub fn get_type(self, d: &mut Duel, scard: Option<CardId>, sumtype: u64, playerid: u8) -> u32 {
        d.note_read(self, 2);
        if let Some(v) = self.c(d).get_assumed(ASSUME_TYPE) {
            return v as u32;
        }
        let c = self.c(d);
        if c.current.location as u32 & (LOCATION_ONFIELD | LOCATION_HAND | LOCATION_GRAVE) == 0 {
            return c.data.type_;
        }
        if c.current.is_location(LOCATION_PZONE) && sumtype == 0 {
            return TYPE_PENDULUM + TYPE_SPELL;
        }
        if d.stat_temp[self.0 as usize].type_ != u32::MAX {
            return d.stat_temp[self.0 as usize].type_;
        }
        let data_type = c.data.type_;
        let mut typ = data_type as i32;
        let mut alttype: i32 = 0;
        if sumtype != 0 && c.current.location as u32 == LOCATION_SZONE {
            alttype = data_type as i32;
        }
        let mut changed = false;
        d.stat_temp[self.0 as usize].type_ = data_type;
        let mut effects = Vec::new();
        self.filter_effect(d, EFFECT_ADD_TYPE, &mut effects, false);
        self.filter_effect(d, EFFECT_REMOVE_TYPE, &mut effects, false);
        self.filter_effect(d, EFFECT_CHANGE_TYPE, &mut effects, true);
        for peffect in effects {
            let op = d.effect(peffect).operation.clone();
            if op.is_some() && sumtype == 0 {
                continue;
            }
            let ecode = d.effect(peffect).code;
            if let Some(op) = op {
                d.add_param(Value::card(scard));
                d.add_param(Value::Int(sumtype as i64));
                d.add_param(Value::Int(playerid as i64));
                if !d.check_condition(Some(&op), 3) {
                    continue;
                }
                d.add_param(Value::Int(playerid as i64));
                let v = peffect.get_value_card(d, Some(self), 1) as i32;
                if ecode == EFFECT_ADD_TYPE {
                    alttype |= v;
                } else if ecode == EFFECT_REMOVE_TYPE {
                    alttype &= !v;
                } else {
                    alttype = v;
                    changed = true;
                }
            } else {
                d.add_param(Value::Int(playerid as i64));
                let v = peffect.get_value_card(d, Some(self), 1) as i32;
                if ecode == EFFECT_ADD_TYPE {
                    typ |= v;
                } else if ecode == EFFECT_REMOVE_TYPE {
                    typ &= !v;
                } else {
                    typ = v;
                }
                d.stat_temp[self.0 as usize].type_ = typ as u32;
            }
        }
        typ |= alttype;
        if changed {
            typ = alttype;
        }
        d.stat_temp[self.0 as usize].type_ = u32::MAX;
        typ as u32
    }
    fn is_monster_for_stats(self, d: &mut Duel) -> bool {
        self.c(d).data.type_ & TYPE_MONSTER != 0
            || self.get_type(d, None, 0, 2) & TYPE_MONSTER != 0
            || self.is_affected_by_effect(d, EFFECT_PRE_MONSTER).is_some()
    }
    /// Atk and def are special cases since text atk/def ? are involved.
    pub fn get_base_attack(self, d: &mut Duel) -> i32 {
        if !self.is_monster_for_stats(d) {
            return 0;
        }
        let c = self.c(d);
        if c.current.location as u32 != LOCATION_MZONE || c.get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP) != 0 {
            return c.data.attack;
        }
        if d.stat_temp[self.0 as usize].base_attack != -1 {
            return d.stat_temp[self.0 as usize].base_attack;
        }
        let mut batk = c.data.attack.max(0);
        let mut bdef = c.data.defense.max(0);
        let is_link = c.data.type_ & TYPE_LINK != 0;
        d.stat_temp[self.0 as usize].base_attack = batk;
        let mut eset = Vec::new();
        let mut swap = false;
        if !is_link {
            self.filter_effect(d, EFFECT_SWAP_BASE_AD, &mut eset, false);
            swap = !eset.is_empty();
        }
        self.filter_effect(d, EFFECT_SET_BASE_ATTACK, &mut eset, false);
        if swap {
            self.filter_effect(d, EFFECT_SET_BASE_DEFENSE, &mut eset, false);
        }
        sort_effects_by_id(d, &mut eset);
        // calculate continuous effects of this first
        let mut i = 0;
        while i < eset.len() {
            let e = eset[i];
            let (typ, single_range, code) = (d.effect(e).type_ as u32, d.effect(e).is_flag(EFFECT_FLAG_SINGLE_RANGE), d.effect(e).code);
            if typ & EFFECT_TYPE_SINGLE == 0 || single_range {
                match code {
                    EFFECT_SET_BASE_ATTACK => {
                        batk = (e.get_value_card(d, Some(self), 0) as i32).max(0);
                        d.stat_temp[self.0 as usize].base_attack = batk;
                        eset.remove(i);
                        continue;
                    }
                    EFFECT_SET_BASE_DEFENSE => {
                        bdef = (e.get_value_card(d, Some(self), 0) as i32).max(0);
                        eset.remove(i);
                        continue;
                    }
                    _ => {}
                }
            }
            i += 1;
        }
        for peffect in eset {
            match d.effect(peffect).code {
                EFFECT_SET_BASE_ATTACK => batk = (peffect.get_value_card(d, Some(self), 0) as i32).max(0),
                EFFECT_SET_BASE_DEFENSE => bdef = (peffect.get_value_card(d, Some(self), 0) as i32).max(0),
                EFFECT_SWAP_BASE_AD => std::mem::swap(&mut batk, &mut bdef),
                _ => {}
            }
            d.stat_temp[self.0 as usize].base_attack = batk;
        }
        d.stat_temp[self.0 as usize].base_attack = -1;
        batk
    }
    pub fn get_attack(self, d: &mut Duel) -> i32 {
        d.note_read(self, 64);
        if let Some(v) = self.c(d).get_assumed(ASSUME_ATTACK) {
            return v as i32;
        }
        if !self.is_monster_for_stats(d) {
            return 0;
        }
        let c = self.c(d);
        if c.current.location as u32 != LOCATION_MZONE || c.get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP) != 0 {
            return c.data.attack;
        }
        if d.stat_temp[self.0 as usize].attack != -1 {
            return d.stat_temp[self.0 as usize].attack;
        }
        let mut batk = c.data.attack.max(0);
        let mut bdef = c.data.defense.max(0);
        let is_link = c.data.type_ & TYPE_LINK != 0;
        d.stat_temp[self.0 as usize].base_attack = batk;
        d.stat_temp[self.0 as usize].attack = batk;
        let mut atk: i32 = -1;
        let mut up_atk: i32 = 0;
        let mut upc_atk: i32 = 0;
        let mut swap_final = false;
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_UPDATE_ATTACK, &mut eset, false);
        self.filter_effect(d, EFFECT_SET_ATTACK, &mut eset, false);
        self.filter_effect(d, EFFECT_SET_ATTACK_FINAL, &mut eset, false);
        self.filter_effect(d, EFFECT_SWAP_ATTACK_FINAL, &mut eset, false);
        self.filter_effect(d, EFFECT_SET_BASE_ATTACK, &mut eset, false);
        if !is_link {
            self.filter_effect(d, EFFECT_SWAP_AD, &mut eset, false);
            self.filter_effect(d, EFFECT_SWAP_BASE_AD, &mut eset, false);
            self.filter_effect(d, EFFECT_SET_BASE_DEFENSE, &mut eset, false);
        }
        sort_effects_by_id(d, &mut eset);
        let rev = self.is_affected_by_effect(d, EFFECT_REVERSE_UPDATE).is_some();
        let mut effects_atk = Vec::new();
        let mut effects_atk_r = Vec::new();
        let mut i = 0;
        while i < eset.len() {
            let e = eset[i];
            let (typ, single_range, code) = (d.effect(e).type_ as u32, d.effect(e).is_flag(EFFECT_FLAG_SINGLE_RANGE), d.effect(e).code);
            if typ & EFFECT_TYPE_SINGLE == 0 || single_range {
                match code {
                    EFFECT_SET_BASE_ATTACK => {
                        batk = (e.get_value_card(d, Some(self), 0) as i32).max(0);
                        d.stat_temp[self.0 as usize].base_attack = batk;
                        eset.remove(i);
                        continue;
                    }
                    EFFECT_SET_BASE_DEFENSE => {
                        bdef = (e.get_value_card(d, Some(self), 0) as i32).max(0);
                        eset.remove(i);
                        continue;
                    }
                    _ => {}
                }
            }
            i += 1;
        }
        d.stat_temp[self.0 as usize].attack = batk;
        for peffect in eset {
            let (typ, single_range, code, delay) = {
                let e = d.effect(peffect);
                (e.type_ as u32, e.is_flag(EFFECT_FLAG_SINGLE_RANGE), e.code, e.is_flag(EFFECT_FLAG_DELAY))
            };
            match code {
                EFFECT_UPDATE_ATTACK => {
                    let v = peffect.get_value_card(d, Some(self), 0) as i32;
                    if typ & EFFECT_TYPE_SINGLE != 0 && !single_range {
                        up_atk = up_atk.wrapping_add(v);
                    } else {
                        upc_atk = upc_atk.wrapping_add(v);
                    }
                }
                EFFECT_SET_ATTACK => {
                    atk = peffect.get_value_card(d, Some(self), 0) as i32;
                    if typ & EFFECT_TYPE_SINGLE == 0 {
                        up_atk = 0;
                    }
                }
                EFFECT_SET_ATTACK_FINAL => {
                    if typ & EFFECT_TYPE_SINGLE != 0 && !single_range {
                        atk = peffect.get_value_card(d, Some(self), 0) as i32;
                        up_atk = 0;
                        upc_atk = 0;
                    } else if !delay {
                        effects_atk.push(peffect);
                    } else {
                        effects_atk_r.push(peffect);
                    }
                }
                EFFECT_SET_BASE_ATTACK => {
                    batk = (peffect.get_value_card(d, Some(self), 0) as i32).max(0);
                    atk = -1;
                }
                EFFECT_SWAP_ATTACK_FINAL => {
                    atk = peffect.get_value_card(d, Some(self), 0) as i32;
                    up_atk = 0;
                    upc_atk = 0;
                }
                EFFECT_SET_BASE_DEFENSE => bdef = (peffect.get_value_card(d, Some(self), 0) as i32).max(0),
                EFFECT_SWAP_AD => swap_final = !swap_final,
                EFFECT_SWAP_BASE_AD => std::mem::swap(&mut batk, &mut bdef),
                _ => {}
            }
            let t = &mut d.stat_temp[self.0 as usize];
            t.base_attack = batk;
            let base = if atk < 0 { batk } else { atk };
            t.attack = if !rev {
                base.wrapping_add(up_atk).wrapping_add(upc_atk)
            } else {
                base.wrapping_sub(up_atk).wrapping_sub(upc_atk)
            };
            if t.attack < 0 {
                t.attack = 0;
            }
        }
        for peffect in effects_atk {
            let v = peffect.get_value_card(d, Some(self), 0) as i32;
            d.stat_temp[self.0 as usize].attack = v;
        }
        if d.stat_temp[self.0 as usize].defense == -1 {
            if swap_final {
                let v = self.get_defense(d);
                d.stat_temp[self.0 as usize].attack = v;
            }
            for peffect in effects_atk_r {
                let v = peffect.get_value_card(d, Some(self), 0) as i32;
                d.stat_temp[self.0 as usize].attack = v;
                if d.effect(peffect).is_flag(EFFECT_FLAG_REPEAT) {
                    let v = peffect.get_value_card(d, Some(self), 0) as i32;
                    d.stat_temp[self.0 as usize].attack = v;
                }
            }
        }
        let atk = d.stat_temp[self.0 as usize].attack.max(0);
        let t = &mut d.stat_temp[self.0 as usize];
        t.base_attack = -1;
        t.attack = -1;
        atk
    }
    pub fn get_base_defense(self, d: &mut Duel) -> i32 {
        if self.c(d).data.type_ & TYPE_LINK != 0 {
            return 0;
        }
        if !self.is_monster_for_stats(d) {
            return 0;
        }
        let c = self.c(d);
        if c.current.location as u32 != LOCATION_MZONE || c.get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP) != 0 {
            return c.data.defense;
        }
        if d.stat_temp[self.0 as usize].base_defense != -1 {
            return d.stat_temp[self.0 as usize].base_defense;
        }
        let mut batk = c.data.attack.max(0);
        let mut bdef = c.data.defense.max(0);
        d.stat_temp[self.0 as usize].base_defense = bdef;
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_SWAP_BASE_AD, &mut eset, false);
        let swap = !eset.is_empty();
        self.filter_effect(d, EFFECT_SET_BASE_DEFENSE, &mut eset, false);
        if swap {
            self.filter_effect(d, EFFECT_SET_BASE_ATTACK, &mut eset, false);
        }
        sort_effects_by_id(d, &mut eset);
        let mut i = 0;
        while i < eset.len() {
            let e = eset[i];
            let (typ, single_range, code) = (d.effect(e).type_ as u32, d.effect(e).is_flag(EFFECT_FLAG_SINGLE_RANGE), d.effect(e).code);
            if typ & EFFECT_TYPE_SINGLE == 0 || single_range {
                match code {
                    EFFECT_SET_BASE_ATTACK => {
                        batk = (e.get_value_card(d, Some(self), 0) as i32).max(0);
                        eset.remove(i);
                        continue;
                    }
                    EFFECT_SET_BASE_DEFENSE => {
                        bdef = (e.get_value_card(d, Some(self), 0) as i32).max(0);
                        d.stat_temp[self.0 as usize].base_defense = bdef;
                        eset.remove(i);
                        continue;
                    }
                    _ => {}
                }
            }
            i += 1;
        }
        for peffect in eset {
            match d.effect(peffect).code {
                EFFECT_SET_BASE_ATTACK => batk = (peffect.get_value_card(d, Some(self), 0) as i32).max(0),
                EFFECT_SET_BASE_DEFENSE => bdef = (peffect.get_value_card(d, Some(self), 0) as i32).max(0),
                EFFECT_SWAP_BASE_AD => std::mem::swap(&mut batk, &mut bdef),
                _ => {}
            }
            d.stat_temp[self.0 as usize].base_defense = bdef;
        }
        d.stat_temp[self.0 as usize].base_defense = -1;
        bdef
    }
    pub fn get_defense(self, d: &mut Duel) -> i32 {
        d.note_read(self, 128);
        if self.c(d).data.type_ & TYPE_LINK != 0 {
            return 0;
        }
        if let Some(v) = self.c(d).get_assumed(ASSUME_DEFENSE) {
            return v as i32;
        }
        if !self.is_monster_for_stats(d) {
            return 0;
        }
        let c = self.c(d);
        if c.current.location as u32 != LOCATION_MZONE || c.get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP) != 0 {
            return c.data.defense;
        }
        if d.stat_temp[self.0 as usize].defense != -1 {
            return d.stat_temp[self.0 as usize].defense;
        }
        let mut batk = c.data.attack.max(0);
        let mut bdef = c.data.defense.max(0);
        d.stat_temp[self.0 as usize].base_defense = bdef;
        d.stat_temp[self.0 as usize].defense = bdef;
        let mut def: i32 = -1;
        let mut up_def: i32 = 0;
        let mut upc_def: i32 = 0;
        let mut swap_final = false;
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_SWAP_AD, &mut eset, false);
        self.filter_effect(d, EFFECT_UPDATE_DEFENSE, &mut eset, false);
        self.filter_effect(d, EFFECT_SET_DEFENSE, &mut eset, false);
        self.filter_effect(d, EFFECT_SET_DEFENSE_FINAL, &mut eset, false);
        self.filter_effect(d, EFFECT_SWAP_DEFENSE_FINAL, &mut eset, false);
        self.filter_effect(d, EFFECT_SWAP_BASE_AD, &mut eset, false);
        self.filter_effect(d, EFFECT_SET_BASE_ATTACK, &mut eset, false);
        self.filter_effect(d, EFFECT_SET_BASE_DEFENSE, &mut eset, false);
        sort_effects_by_id(d, &mut eset);
        let rev = self.is_affected_by_effect(d, EFFECT_REVERSE_UPDATE).is_some();
        let mut effects_def = Vec::new();
        let mut effects_def_r = Vec::new();
        let mut i = 0;
        while i < eset.len() {
            let e = eset[i];
            let (typ, single_range, code) = (d.effect(e).type_ as u32, d.effect(e).is_flag(EFFECT_FLAG_SINGLE_RANGE), d.effect(e).code);
            if typ & EFFECT_TYPE_SINGLE == 0 || single_range {
                match code {
                    EFFECT_SET_BASE_ATTACK => {
                        batk = (e.get_value_card(d, Some(self), 0) as i32).max(0);
                        eset.remove(i);
                        continue;
                    }
                    EFFECT_SET_BASE_DEFENSE => {
                        bdef = (e.get_value_card(d, Some(self), 0) as i32).max(0);
                        d.stat_temp[self.0 as usize].base_defense = bdef;
                        eset.remove(i);
                        continue;
                    }
                    _ => {}
                }
            }
            i += 1;
        }
        d.stat_temp[self.0 as usize].defense = bdef;
        for peffect in eset {
            let (typ, single_range, code, delay) = {
                let e = d.effect(peffect);
                (e.type_ as u32, e.is_flag(EFFECT_FLAG_SINGLE_RANGE), e.code, e.is_flag(EFFECT_FLAG_DELAY))
            };
            match code {
                EFFECT_UPDATE_DEFENSE => {
                    let v = peffect.get_value_card(d, Some(self), 0) as i32;
                    if typ & EFFECT_TYPE_SINGLE != 0 && !single_range {
                        up_def = up_def.wrapping_add(v);
                    } else {
                        upc_def = upc_def.wrapping_add(v);
                    }
                }
                EFFECT_SET_DEFENSE => {
                    def = peffect.get_value_card(d, Some(self), 0) as i32;
                    if typ & EFFECT_TYPE_SINGLE == 0 {
                        up_def = 0;
                    }
                }
                EFFECT_SET_DEFENSE_FINAL => {
                    if typ & EFFECT_TYPE_SINGLE != 0 && !single_range {
                        def = peffect.get_value_card(d, Some(self), 0) as i32;
                        up_def = 0;
                        upc_def = 0;
                    } else if !delay {
                        effects_def.push(peffect);
                    } else {
                        effects_def_r.push(peffect);
                    }
                }
                EFFECT_SET_BASE_DEFENSE => {
                    bdef = (peffect.get_value_card(d, Some(self), 0) as i32).max(0);
                    def = -1;
                }
                EFFECT_SWAP_DEFENSE_FINAL => {
                    def = peffect.get_value_card(d, Some(self), 0) as i32;
                    up_def = 0;
                    upc_def = 0;
                }
                EFFECT_SET_BASE_ATTACK => batk = (peffect.get_value_card(d, Some(self), 0) as i32).max(0),
                EFFECT_SWAP_AD => swap_final = !swap_final,
                EFFECT_SWAP_BASE_AD => std::mem::swap(&mut batk, &mut bdef),
                _ => {}
            }
            let t = &mut d.stat_temp[self.0 as usize];
            t.base_defense = bdef;
            let base = if def < 0 { bdef } else { def };
            t.defense = if !rev {
                base.wrapping_add(up_def).wrapping_add(upc_def)
            } else {
                base.wrapping_sub(up_def).wrapping_sub(upc_def)
            };
            if t.defense < 0 {
                t.defense = 0;
            }
        }
        for peffect in effects_def {
            let v = peffect.get_value_card(d, Some(self), 0) as i32;
            d.stat_temp[self.0 as usize].defense = v;
        }
        if d.stat_temp[self.0 as usize].attack == -1 {
            if swap_final {
                let v = self.get_attack(d);
                d.stat_temp[self.0 as usize].defense = v;
            }
            for peffect in effects_def_r {
                let v = peffect.get_value_card(d, Some(self), 0) as i32;
                d.stat_temp[self.0 as usize].defense = v;
                if d.effect(peffect).is_flag(EFFECT_FLAG_REPEAT) {
                    let v = peffect.get_value_card(d, Some(self), 0) as i32;
                    d.stat_temp[self.0 as usize].defense = v;
                }
            }
        }
        let def = d.stat_temp[self.0 as usize].defense.max(0);
        let t = &mut d.stat_temp[self.0 as usize];
        t.base_defense = -1;
        t.defense = -1;
        def
    }
    /// Level/Attribute/Race is available for: cards with original type TYPE_MONSTER, cards with
    /// current type TYPE_MONSTER, cards with EFFECT_PRE_MONSTER
    pub fn get_level(self, d: &mut Duel) -> u32 {
        d.note_read(self, 4);
        let dt = self.c(d).data.type_;
        let status = self.c(d).status;
        if (dt & TYPE_XYZ != 0
            && !(self.is_affected_by_effect(d, EFFECT_RANK_LEVEL).is_some() || self.is_affected_by_effect(d, EFFECT_RANK_LEVEL_S).is_some()))
            || dt & TYPE_LINK != 0
            || status & STATUS_NO_LEVEL != 0
            || !self.is_monster_for_stats(d)
        {
            return 0;
        }
        if let Some(v) = self.c(d).get_assumed(ASSUME_LEVEL) {
            return v as u32;
        }
        if d.stat_temp[self.0 as usize].level != u32::MAX {
            return d.stat_temp[self.0 as usize].level;
        }
        let mut level = self.c(d).data.level as i32;
        d.stat_temp[self.0 as usize].level = level as u32;
        let mut up = 0i32;
        let mut upc = 0i32;
        let mut effects = Vec::new();
        if self.is_affected_by_effect(d, EFFECT_RANK_LEVEL_S).is_some() || self.is_affected_by_effect(d, EFFECT_LEVEL_RANK_S).is_some() {
            self.filter_effect(d, EFFECT_UPDATE_RANK, &mut effects, false);
            self.filter_effect(d, EFFECT_UPDATE_LEVEL, &mut effects, false);
            self.filter_effect(d, EFFECT_CHANGE_RANK, &mut effects, false);
            self.filter_effect(d, EFFECT_CHANGE_LEVEL, &mut effects, false);
            self.filter_effect(d, EFFECT_CHANGE_RANK_FINAL, &mut effects, false);
            self.filter_effect(d, EFFECT_CHANGE_LEVEL_FINAL, &mut effects, true);
        } else {
            self.filter_effect(d, EFFECT_UPDATE_LEVEL, &mut effects, false);
            self.filter_effect(d, EFFECT_CHANGE_LEVEL, &mut effects, false);
            self.filter_effect(d, EFFECT_CHANGE_LEVEL_FINAL, &mut effects, true);
        }
        for peffect in effects {
            let (code, typ, single_range) = {
                let e = d.effect(peffect);
                (e.code, e.type_ as u32, e.is_flag(EFFECT_FLAG_SINGLE_RANGE))
            };
            match code {
                EFFECT_UPDATE_RANK | EFFECT_UPDATE_LEVEL => {
                    let v = peffect.get_value_card(d, Some(self), 0) as i32;
                    if typ & EFFECT_TYPE_SINGLE != 0 && !single_range {
                        up = up.wrapping_add(v);
                    } else {
                        upc = upc.wrapping_add(v);
                    }
                }
                EFFECT_CHANGE_RANK | EFFECT_CHANGE_LEVEL => {
                    level = peffect.get_value_card(d, Some(self), 0) as i32;
                    up = 0;
                }
                EFFECT_CHANGE_RANK_FINAL | EFFECT_CHANGE_LEVEL_FINAL => {
                    level = peffect.get_value_card(d, Some(self), 0) as i32;
                    up = 0;
                    upc = 0;
                }
                _ => {}
            }
            d.stat_temp[self.0 as usize].level = level.wrapping_add(up).wrapping_add(upc) as u32;
        }
        level = level.wrapping_add(up).wrapping_add(upc);
        if level < 1 && self.get_type(d, None, 0, 2) & TYPE_MONSTER != 0 && self.is_affected_by_effect(d, EFFECT_ALLOW_NEGATIVE).is_none() {
            level = 1;
        }
        d.stat_temp[self.0 as usize].level = u32::MAX;
        level as u32
    }
    pub fn get_rank(self, d: &mut Duel) -> u32 {
        d.note_read(self, 8);
        let dt = self.c(d).data.type_;
        let status = self.c(d).status;
        if ((dt & TYPE_XYZ == 0 || status & STATUS_NO_LEVEL != 0)
            && !(self.is_affected_by_effect(d, EFFECT_LEVEL_RANK).is_some() || self.is_affected_by_effect(d, EFFECT_LEVEL_RANK_S).is_some()))
            || dt & TYPE_LINK != 0
        {
            return 0;
        }
        if let Some(v) = self.c(d).get_assumed(ASSUME_RANK) {
            return v as u32;
        }
        if self.c(d).current.location as u32 & LOCATION_MZONE == 0 {
            return self.c(d).data.level;
        }
        if d.stat_temp[self.0 as usize].rank != u32::MAX {
            return d.stat_temp[self.0 as usize].rank;
        }
        let mut rank = self.c(d).data.level as i32;
        d.stat_temp[self.0 as usize].rank = rank as u32;
        let mut up = 0i32;
        let mut upc = 0i32;
        let mut effects = Vec::new();
        if self.is_affected_by_effect(d, EFFECT_RANK_LEVEL_S).is_some() || self.is_affected_by_effect(d, EFFECT_LEVEL_RANK_S).is_some() {
            self.filter_effect(d, EFFECT_UPDATE_RANK, &mut effects, false);
            self.filter_effect(d, EFFECT_UPDATE_LEVEL, &mut effects, false);
            self.filter_effect(d, EFFECT_CHANGE_RANK, &mut effects, false);
            self.filter_effect(d, EFFECT_CHANGE_LEVEL, &mut effects, false);
            self.filter_effect(d, EFFECT_CHANGE_RANK_FINAL, &mut effects, false);
            self.filter_effect(d, EFFECT_CHANGE_LEVEL_FINAL, &mut effects, true);
        } else {
            self.filter_effect(d, EFFECT_UPDATE_RANK, &mut effects, false);
            self.filter_effect(d, EFFECT_CHANGE_RANK, &mut effects, false);
            self.filter_effect(d, EFFECT_CHANGE_RANK_FINAL, &mut effects, true);
        }
        for peffect in effects {
            let (code, typ, single_range) = {
                let e = d.effect(peffect);
                (e.code, e.type_ as u32, e.is_flag(EFFECT_FLAG_SINGLE_RANGE))
            };
            match code {
                EFFECT_UPDATE_RANK | EFFECT_UPDATE_LEVEL => {
                    let v = peffect.get_value_card(d, Some(self), 0) as i32;
                    if typ & EFFECT_TYPE_SINGLE != 0 && !single_range {
                        up = up.wrapping_add(v);
                    } else {
                        upc = upc.wrapping_add(v);
                    }
                }
                EFFECT_CHANGE_RANK | EFFECT_CHANGE_LEVEL => {
                    rank = peffect.get_value_card(d, Some(self), 0) as i32;
                    up = 0;
                }
                EFFECT_CHANGE_RANK_FINAL | EFFECT_CHANGE_LEVEL_FINAL => {
                    rank = peffect.get_value_card(d, Some(self), 0) as i32;
                    up = 0;
                    upc = 0;
                }
                _ => {}
            }
            d.stat_temp[self.0 as usize].rank = rank.wrapping_add(up).wrapping_add(upc) as u32;
        }
        rank = rank.wrapping_add(up).wrapping_add(upc);
        if rank < 1 && self.get_type(d, None, 0, 2) & TYPE_MONSTER != 0 && self.is_affected_by_effect(d, EFFECT_ALLOW_NEGATIVE).is_none() {
            rank = 1;
        }
        d.stat_temp[self.0 as usize].rank = u32::MAX;
        rank as u32
    }
    pub fn get_link(self, d: &mut Duel) -> u32 {
        d.note_read(self, 512);
        let c = self.c(d);
        if c.data.type_ & TYPE_LINK == 0 || c.status & STATUS_NO_LEVEL != 0 {
            return 0;
        }
        if let Some(v) = c.get_assumed(ASSUME_LINK) {
            return v as u32;
        }
        if c.current.location as u32 & LOCATION_MZONE == 0 {
            return c.data.level;
        }
        if d.stat_temp[self.0 as usize].link != u32::MAX {
            return d.stat_temp[self.0 as usize].link;
        }
        let mut link = c.data.level as i32;
        d.stat_temp[self.0 as usize].link = link as u32;
        let mut up = 0i32;
        let mut upc = 0i32;
        let mut effects = Vec::new();
        self.filter_effect(d, EFFECT_UPDATE_LINK, &mut effects, false);
        self.filter_effect(d, EFFECT_CHANGE_LINK, &mut effects, false);
        self.filter_effect(d, EFFECT_CHANGE_LINK_FINAL, &mut effects, true);
        for peffect in effects {
            let (code, typ, single_range) = {
                let e = d.effect(peffect);
                (e.code, e.type_ as u32, e.is_flag(EFFECT_FLAG_SINGLE_RANGE))
            };
            match code {
                EFFECT_UPDATE_LINK => {
                    let v = peffect.get_value_card(d, Some(self), 0) as i32;
                    if typ & EFFECT_TYPE_SINGLE != 0 && !single_range {
                        up = up.wrapping_add(v);
                    } else {
                        upc = upc.wrapping_add(v);
                    }
                }
                EFFECT_CHANGE_LINK => {
                    link = peffect.get_value_card(d, Some(self), 0) as i32;
                    up = 0;
                }
                EFFECT_CHANGE_LINK_FINAL => {
                    link = peffect.get_value_card(d, Some(self), 0) as i32;
                    up = 0;
                    upc = 0;
                }
                _ => {}
            }
            d.stat_temp[self.0 as usize].link = link.wrapping_add(up).wrapping_add(upc) as u32;
        }
        link = link.wrapping_add(up).wrapping_add(upc);
        if link < 1 && self.get_type(d, None, 0, 2) & TYPE_MONSTER != 0 && self.is_affected_by_effect(d, EFFECT_ALLOW_NEGATIVE).is_none() {
            link = 1;
        }
        d.stat_temp[self.0 as usize].link = u32::MAX;
        link as u32
    }
    pub fn get_synchro_level(self, d: &mut Duel, pcard: Option<CardId>) -> u32 {
        let dt = self.c(d).data.type_;
        if dt & TYPE_LINK != 0 && self.is_affected_by_effect(d, EFFECT_SYNCHRO_LEVEL).is_none() {
            return 0;
        }
        if (dt & TYPE_XYZ != 0 || self.c(d).status & STATUS_NO_LEVEL != 0)
            && !(self.is_affected_by_effect(d, EFFECT_RANK_LEVEL).is_some() || self.is_affected_by_effect(d, EFFECT_RANK_LEVEL_S).is_some())
        {
            return 0;
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_SYNCHRO_LEVEL, &mut eset, true);
        if let Some(&e) = eset.first() {
            e.get_value_card(d, pcard, 0) as u32
        } else {
            self.get_level(d)
        }
    }
    pub fn get_ritual_level(self, d: &mut Duel, pcard: Option<CardId>) -> u32 {
        let dt = self.c(d).data.type_;
        if dt & TYPE_LINK != 0 {
            return 0;
        }
        if (dt & TYPE_XYZ != 0 || self.c(d).status & STATUS_NO_LEVEL != 0)
            && !(self.is_affected_by_effect(d, EFFECT_RANK_LEVEL).is_some() || self.is_affected_by_effect(d, EFFECT_RANK_LEVEL_S).is_some())
        {
            return 0;
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_RITUAL_LEVEL, &mut eset, true);
        if let Some(&e) = eset.first() {
            e.get_value_card(d, pcard, 0) as u32
        } else {
            self.get_level(d)
        }
    }
    pub fn check_xyz_level(self, d: &mut Duel, pcard: CardId, lv: u32) -> bool {
        if self.c(d).status & STATUS_NO_LEVEL != 0 {
            return false;
        }
        let mut eset = Vec::new();
        self.filter_effect(d, EFFECT_XYZ_LEVEL, &mut eset, true);
        for eff in eset {
            let mut res = Vec::new();
            d.add_param(Value::Card(self));
            d.add_param(Value::Card(pcard));
            eff.get_values(d, 2, &mut res);
            if res.len() == 1 {
                let lev = res[0] as u32;
                if (lev & 0xfff) == lv || ((lev >> 16) & 0xfff) == lv {
                    return true;
                }
            } else if res.contains(&(lv as i64)) {
                return true;
            }
        }
        self.get_level(d) == lv
    }
    pub fn get_attribute(self, d: &mut Duel, scard: Option<CardId>, sumtype: u64, playerid: u8) -> u32 {
        d.note_read(self, 16);
        if let Some(v) = self.c(d).get_assumed(ASSUME_ATTRIBUTE) {
            return v as u32;
        }
        if !self.is_monster_for_stats(d) {
            return 0;
        }
        if d.stat_temp[self.0 as usize].attribute != u32::MAX {
            return d.stat_temp[self.0 as usize].attribute;
        }
        let mut attribute = self.c(d).data.attribute as i32;
        let mut altattribute = 0i32;
        let mut changed = false;
        d.stat_temp[self.0 as usize].attribute = attribute as u32;
        let mut effects = Vec::new();
        self.filter_effect(d, EFFECT_ADD_ATTRIBUTE, &mut effects, false);
        self.filter_effect(d, EFFECT_REMOVE_ATTRIBUTE, &mut effects, false);
        self.filter_effect(d, EFFECT_CHANGE_ATTRIBUTE, &mut effects, true);
        for peffect in effects {
            let op = d.effect(peffect).operation.clone();
            if op.is_some() && sumtype == 0 {
                continue;
            }
            let code = d.effect(peffect).code;
            if let Some(op) = op {
                d.add_param(Value::card(scard));
                d.add_param(Value::Int(sumtype as i64));
                d.add_param(Value::Int(playerid as i64));
                if !d.check_condition(Some(&op), 3) {
                    continue;
                }
                d.add_param(Value::Int(playerid as i64));
                let v = peffect.get_value_card(d, Some(self), 1) as i32;
                if code == EFFECT_ADD_ATTRIBUTE {
                    altattribute |= v;
                } else if code == EFFECT_REMOVE_ATTRIBUTE {
                    altattribute &= !v;
                } else if code == EFFECT_CHANGE_ATTRIBUTE {
                    altattribute = v;
                    changed = true;
                }
            } else {
                d.add_param(Value::Int(playerid as i64));
                let v = peffect.get_value_card(d, Some(self), 1) as i32;
                if code == EFFECT_ADD_ATTRIBUTE {
                    attribute |= v;
                } else if code == EFFECT_REMOVE_ATTRIBUTE {
                    attribute &= !v;
                } else if code == EFFECT_CHANGE_ATTRIBUTE {
                    attribute = v;
                }
                d.stat_temp[self.0 as usize].attribute = attribute as u32;
            }
        }
        attribute |= altattribute;
        if changed {
            attribute = altattribute;
        }
        d.stat_temp[self.0 as usize].attribute = u32::MAX;
        attribute as u32
    }
    pub fn get_race(self, d: &mut Duel, scard: Option<CardId>, sumtype: u64, playerid: u8) -> u64 {
        d.note_read(self, 32);
        if let Some(v) = self.c(d).get_assumed(ASSUME_RACE) {
            return v;
        }
        if !self.is_monster_for_stats(d) {
            return 0;
        }
        if d.stat_temp[self.0 as usize].race != u64::MAX {
            return d.stat_temp[self.0 as usize].race;
        }
        let mut race = self.c(d).data.race;
        let mut altrace = 0u64;
        let mut changed = false;
        d.stat_temp[self.0 as usize].race = race;
        let mut effects = Vec::new();
        self.filter_effect(d, EFFECT_ADD_RACE, &mut effects, false);
        self.filter_effect(d, EFFECT_REMOVE_RACE, &mut effects, false);
        self.filter_effect(d, EFFECT_CHANGE_RACE, &mut effects, true);
        for peffect in effects {
            let op = d.effect(peffect).operation.clone();
            if op.is_some() && sumtype == 0 {
                continue;
            }
            let code = d.effect(peffect).code;
            if let Some(op) = op {
                d.add_param(Value::card(scard));
                d.add_param(Value::Int(sumtype as i64));
                d.add_param(Value::Int(playerid as i64));
                if !d.check_condition(Some(&op), 3) {
                    continue;
                }
                d.add_param(Value::Int(playerid as i64));
                let v = peffect.get_value_card(d, Some(self), 1) as u64;
                if code == EFFECT_ADD_RACE {
                    altrace |= v;
                } else if code == EFFECT_REMOVE_RACE {
                    altrace &= !v;
                } else if code == EFFECT_CHANGE_RACE {
                    altrace = v;
                    changed = true;
                }
            } else {
                d.add_param(Value::Int(playerid as i64));
                let v = peffect.get_value_card(d, Some(self), 1) as u64;
                if code == EFFECT_ADD_RACE {
                    race |= v;
                } else if code == EFFECT_REMOVE_RACE {
                    race &= !v;
                } else if code == EFFECT_CHANGE_RACE {
                    race = v;
                }
                d.stat_temp[self.0 as usize].race = race;
            }
        }
        race |= altrace;
        if changed {
            race = altrace;
        }
        d.stat_temp[self.0 as usize].race = u64::MAX;
        race
    }
    fn get_scale(self, d: &mut Duel, right: bool) -> u32 {
        let c = self.c(d);
        let base = if right { c.data.rscale } else { c.data.lscale };
        if !c.current.is_location(LOCATION_PZONE) {
            return base;
        }
        let cached = if right { d.stat_temp[self.0 as usize].rscale } else { d.stat_temp[self.0 as usize].lscale };
        if cached != u32::MAX {
            return cached;
        }
        let set_temp = |d: &mut Duel, v: u32| {
            if right {
                d.stat_temp[self.0 as usize].rscale = v
            } else {
                d.stat_temp[self.0 as usize].lscale = v
            }
        };
        let mut scale = base as i32;
        set_temp(d, base);
        let mut up = 0i32;
        let mut upc = 0i32;
        let (upd, chg) = if right { (EFFECT_UPDATE_RSCALE, EFFECT_CHANGE_RSCALE) } else { (EFFECT_UPDATE_LSCALE, EFFECT_CHANGE_LSCALE) };
        let mut effects = Vec::new();
        self.filter_effect(d, upd, &mut effects, false);
        self.filter_effect(d, chg, &mut effects, true);
        for peffect in effects {
            let (code, typ, single_range) = {
                let e = d.effect(peffect);
                (e.code, e.type_ as u32, e.is_flag(EFFECT_FLAG_SINGLE_RANGE))
            };
            let v = peffect.get_value_card(d, Some(self), 0) as i32;
            if code == upd {
                if typ & EFFECT_TYPE_SINGLE != 0 && !single_range {
                    up = up.wrapping_add(v);
                } else {
                    upc = upc.wrapping_add(v);
                }
            } else {
                scale = v;
                up = 0;
            }
            set_temp(d, scale as u32);
        }
        scale = scale.wrapping_add(up).wrapping_add(upc);
        set_temp(d, u32::MAX);
        scale as u32
    }
    pub fn get_lscale(self, d: &mut Duel) -> u32 {
        self.get_scale(d, false)
    }
    pub fn get_rscale(self, d: &mut Duel) -> u32 {
        self.get_scale(d, true)
    }
    pub fn get_link_marker(self, d: &mut Duel) -> u32 {
        fn rotate(marker: u32) -> u32 {
            (if marker & LINK_MARKER_BOTTOM_LEFT != 0 { LINK_MARKER_BOTTOM_RIGHT } else { 0 })
                | (if marker & LINK_MARKER_BOTTOM != 0 { LINK_MARKER_RIGHT } else { 0 })
                | (if marker & LINK_MARKER_BOTTOM_RIGHT != 0 { LINK_MARKER_TOP_RIGHT } else { 0 })
                | (if marker & LINK_MARKER_RIGHT != 0 { LINK_MARKER_TOP } else { 0 })
                | (if marker & LINK_MARKER_TOP_RIGHT != 0 { LINK_MARKER_TOP_LEFT } else { 0 })
                | (if marker & LINK_MARKER_TOP != 0 { LINK_MARKER_LEFT } else { 0 })
                | (if marker & LINK_MARKER_TOP_LEFT != 0 { LINK_MARKER_BOTTOM_LEFT } else { 0 })
                | (if marker & LINK_MARKER_LEFT != 0 { LINK_MARKER_BOTTOM } else { 0 })
        }
        let c = self.c(d);
        if c.current.position & POS_FACEDOWN != 0 && c.current.is_location(LOCATION_ONFIELD) {
            return 0;
        }
        if let Some(v) = c.get_assumed(ASSUME_LINKMARKER) {
            return v as u32;
        }
        if self.get_type(d, None, 0, 2) & TYPE_LINK == 0 {
            return 0;
        }
        if d.stat_temp[self.0 as usize].link_marker != u32::MAX {
            return d.stat_temp[self.0 as usize].link_marker;
        }
        let mut link_marker = self.c(d).data.link_marker;
        d.stat_temp[self.0 as usize].link_marker = link_marker;
        let mut effects = Vec::new();
        self.filter_effect(d, EFFECT_ADD_LINKMARKER, &mut effects, false);
        self.filter_effect(d, EFFECT_REMOVE_LINKMARKER, &mut effects, false);
        self.filter_effect(d, EFFECT_CHANGE_LINKMARKER, &mut effects, true);
        for peffect in effects {
            let code = d.effect(peffect).code;
            let v = peffect.get_value_card(d, Some(self), 0) as u32;
            if code == EFFECT_ADD_LINKMARKER {
                link_marker |= v;
            } else if code == EFFECT_REMOVE_LINKMARKER {
                link_marker &= !v;
            } else if code == EFFECT_CHANGE_LINKMARKER {
                link_marker = v;
            }
            d.stat_temp[self.0 as usize].link_marker = link_marker;
        }
        d.stat_temp[self.0 as usize].link_marker = u32::MAX;
        let c = self.c(d);
        if c.current.position & POS_ATTACK == 0 && c.current.is_location(LOCATION_ONFIELD) {
            link_marker = rotate(link_marker);
        }
        link_marker
    }
    pub fn is_link_marker(self, d: &mut Duel, dir: u32) -> bool {
        self.get_link_marker(d) & dir != 0
    }
}
