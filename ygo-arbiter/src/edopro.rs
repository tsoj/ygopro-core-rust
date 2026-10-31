//! A player's side of the duel as an EDOPro server sends it to its clients: the core's messages
//! in binary (the inverse of `messages::parse`, without this core's additions: face-up flags
//! in shuffled codes, a shuffled hand's previous places, a set-card shuffle's new zones), card
//! queries (MSG_UPDATE_DATA) from a state view, and the clients' binary responses as the
//! arbiter's responses. For bots written against EDOPro (ygo-windbot: WindBot).

use ygo_core::constants::*;

use crate::actions::{Command, Response};
use crate::messages::{Card, ChainLink, Loc, Msg};
use crate::state::{CardQuery, Snapshot};

#[derive(Default)]
struct Writer(Vec<u8>);

impl Writer {
    fn u8(&mut self, v: u32) -> &mut Self {
        self.0.push(v as u8);
        self
    }
    fn u16(&mut self, v: u32) -> &mut Self {
        self.0.extend((v as u16).to_le_bytes());
        self
    }
    fn u32(&mut self, v: u32) -> &mut Self {
        self.0.extend(v.to_le_bytes());
        self
    }
    fn i32(&mut self, v: i32) -> &mut Self {
        self.0.extend(v.to_le_bytes());
        self
    }
    fn u64(&mut self, v: u64) -> &mut Self {
        self.0.extend(v.to_le_bytes());
        self
    }
    fn loc(&mut self, l: &Loc) -> &mut Self {
        self.u8(l.con).u8(l.loc).u32(l.seq).u32(l.pos)
    }
    /// A u32 count, then the items.
    fn list<T>(&mut self, items: &[T], mut f: impl FnMut(&mut Self, &T)) -> &mut Self {
        self.u32(items.len() as u32);
        for x in items {
            f(self, x);
        }
        self
    }
    fn card_loc(&mut self, c: &Card) -> &mut Self {
        self.u32(c.code).loc(&Loc { con: c.con, loc: c.loc, seq: c.seq, pos: c.pos.unwrap_or(0) })
    }
    fn card_short(&mut self, c: &Card, seq_u8: bool) -> &mut Self {
        self.u32(c.code).u8(c.con).u8(c.loc);
        if seq_u8 { self.u8(c.seq) } else { self.u32(c.seq) }
    }
    fn chain_option_short(&mut self, c: &Card) -> &mut Self {
        self.card_short(c, false).u64(c.desc.unwrap_or(0)).u8(c.client_mode.unwrap_or(0))
    }
    fn chain_link(&mut self, l: &ChainLink) -> &mut Self {
        self.u32(l.code).loc(&l.handler).u8(l.trigger_con).u8(l.trigger_loc).u32(l.trigger_seq).u64(l.desc)
    }
    fn sort_cards(&mut self, cards: &[Card]) -> &mut Self {
        self.list(cards, |w, c| {
            w.u32(c.code).u8(c.con).u32(c.loc).u32(c.seq);
        })
    }
    fn string(&mut self, s: &str) -> &mut Self {
        self.u16(s.len() as u32);
        self.0.extend(s.as_bytes());
        self.u8(0)
    }
}

/// The message as an EDOPro client receives it: its type byte, then the body.
pub fn encode(m: &Msg) -> Vec<u8> {
    use Msg::*;
    let mut w = Writer::default();
    let typ = match m {
        Retry {} => MSG_RETRY,
        Hint { hint_type, player, data } => {
            w.u8(*hint_type).u8(*player).u64(*data);
            MSG_HINT
        }
        Win { player, reason } => {
            w.u8(*player).u8(*reason);
            MSG_WIN
        }
        SelectBattlecmd { player, activatable, attackable, to_m2, to_ep } => {
            w.u8(*player).list(activatable, |w, c| {
                w.chain_option_short(c);
            });
            w.list(attackable, |w, c| {
                w.card_short(c, true).u8(c.direct.unwrap_or(0));
            });
            w.u8(*to_m2).u8(*to_ep);
            MSG_SELECT_BATTLECMD
        }
        SelectIdlecmd { player, summonable, spsummonable, repositionable, msetable, ssetable, activatable, to_bp, to_ep, can_shuffle } => {
            w.u8(*player);
            for (list, seq_u8) in [(summonable, false), (spsummonable, false), (repositionable, true), (msetable, false), (ssetable, false)] {
                w.list(list, |w, c| {
                    w.card_short(c, seq_u8);
                });
            }
            w.list(activatable, |w, c| {
                w.chain_option_short(c);
            });
            w.u8(*to_bp).u8(*to_ep).u8(*can_shuffle);
            MSG_SELECT_IDLECMD
        }
        SelectEffectyn { player, code, con, loc, seq, pos, desc } => {
            w.u8(*player).u32(*code).loc(&Loc { con: *con, loc: *loc, seq: *seq, pos: *pos }).u64(*desc);
            MSG_SELECT_EFFECTYN
        }
        SelectYesno { player, desc } => {
            w.u8(*player).u64(*desc);
            MSG_SELECT_YESNO
        }
        SelectOption { player, options } => {
            w.u8(*player).u8(options.len() as u32);
            options.iter().for_each(|&o| {
                w.u64(o);
            });
            MSG_SELECT_OPTION
        }
        SelectCard { player, cancelable, min, max, cards } => {
            w.u8(*player).u8(*cancelable).u32(*min).u32(*max).list(cards, |w, c| {
                w.card_loc(c);
            });
            MSG_SELECT_CARD
        }
        SelectChain { player, spe_count, forced, hint_timing, hint_timing_other, chains } => {
            w.u8(*player).u8(spe_count.unwrap_or(0)).u8(*forced).u32(hint_timing.unwrap_or(0)).u32(hint_timing_other.unwrap_or(0));
            w.list(chains, |w, c| {
                w.card_loc(c).u64(c.desc.unwrap_or(0)).u8(c.client_mode.unwrap_or(0));
            });
            MSG_SELECT_CHAIN
        }
        SelectPlace { player, count, flag } => {
            w.u8(*player).u8(*count).u32(*flag);
            MSG_SELECT_PLACE
        }
        SelectDisfield { player, count, flag } => {
            w.u8(*player).u8(*count).u32(*flag);
            MSG_SELECT_DISFIELD
        }
        SelectPosition { player, code, positions } => {
            w.u8(*player).u32(*code).u8(*positions);
            MSG_SELECT_POSITION
        }
        SelectTribute { player, cancelable, min, max, cards } => {
            w.u8(*player).u8(*cancelable).u32(*min).u32(*max).list(cards, |w, c| {
                w.card_short(c, false).u8(c.release_param.unwrap_or(0));
            });
            MSG_SELECT_TRIBUTE
        }
        SortChain { player, cards } => {
            w.u8(*player).sort_cards(cards);
            MSG_SORT_CHAIN
        }
        SortCard { player, cards } => {
            w.u8(*player).sort_cards(cards);
            MSG_SORT_CARD
        }
        SelectCounter { player, counter_type, count, cards } => {
            w.u8(*player).u16(*counter_type).u16(*count).list(cards, |w, c| {
                w.card_short(c, true).u16(c.counters.unwrap_or(0));
            });
            MSG_SELECT_COUNTER
        }
        SelectSum { player, mode, acc, min, max, must_cards, cards } => {
            w.u8(*player).u8(*mode).u32(*acc).u32(*min).u32(*max);
            for list in [must_cards, cards] {
                w.list(list, |w, c| {
                    w.card_loc(c).u32(c.param.unwrap_or(0));
                });
            }
            MSG_SELECT_SUM
        }
        SelectUnselectCard { player, finishable, cancelable, min, max, select_cards, unselect_cards } => {
            w.u8(*player).u8(*finishable).u8(*cancelable).u32(*min).u32(*max);
            for list in [select_cards, unselect_cards] {
                w.list(list, |w, c| {
                    w.card_loc(c);
                });
            }
            MSG_SELECT_UNSELECT_CARD
        }
        ConfirmDecktop { player, cards } | ConfirmCards { player, cards } | ConfirmExtratop { player, cards } => {
            w.u8(*player).list(cards, |w, c| {
                w.card_short(c, false);
            });
            match m {
                ConfirmDecktop { .. } => MSG_CONFIRM_DECKTOP,
                ConfirmCards { .. } => MSG_CONFIRM_CARDS,
                _ => MSG_CONFIRM_EXTRATOP,
            }
        }
        ShuffleDeck { player } => {
            w.u8(*player);
            MSG_SHUFFLE_DECK
        }
        ShuffleHand { player, codes, .. } => {
            w.u8(*player).list(codes, |w, &c| {
                w.u32(c);
            });
            MSG_SHUFFLE_HAND
        }
        ShuffleExtra { player, codes, .. } => {
            w.u8(*player).list(codes, |w, &c| {
                w.u32(c);
            });
            MSG_SHUFFLE_EXTRA
        }
        SwapGraveDeck { player, main_count, extra_mask } => {
            let mask: Vec<u8> = (0..extra_mask.len() / 2).map(|i| u8::from_str_radix(&extra_mask[2 * i..2 * i + 2], 16).unwrap_or(0)).collect();
            w.u8(*player).u32(*main_count).u32(mask.len() as u32);
            w.0.extend(mask);
            MSG_SWAP_GRAVE_DECK
        }
        ShuffleSetCard { location, old, overlay, .. } => {
            w.u8(*location).u8(old.len() as u32);
            old.iter().chain(overlay).for_each(|l| {
                w.loc(l);
            });
            MSG_SHUFFLE_SET_CARD
        }
        ReverseDeck {} => MSG_REVERSE_DECK,
        DeckTop { player, seq, code, pos } => {
            w.u8(*player).u32(*seq).u32(*code).u32(*pos);
            MSG_DECK_TOP
        }
        NewTurn { player } => {
            w.u8(*player);
            MSG_NEW_TURN
        }
        NewPhase { phase } => {
            w.u16(*phase);
            MSG_NEW_PHASE
        }
        Move { code, from, to, reason } => {
            w.u32(*code).loc(from).loc(to).u32(*reason);
            MSG_MOVE
        }
        PosChange { code, con, loc, seq, prev_pos, pos } => {
            w.u32(*code).u8(*con).u8(*loc).u8(*seq).u8(*prev_pos).u8(*pos);
            MSG_POS_CHANGE
        }
        Set { code, con, loc, seq, pos } | Summoning { code, con, loc, seq, pos } | Spsummoning { code, con, loc, seq, pos } | Flipsummoning { code, con, loc, seq, pos } => {
            w.u32(*code).loc(&Loc { con: *con, loc: *loc, seq: *seq, pos: *pos });
            match m {
                Set { .. } => MSG_SET,
                Summoning { .. } => MSG_SUMMONING,
                Spsummoning { .. } => MSG_SPSUMMONING,
                _ => MSG_FLIPSUMMONING,
            }
        }
        Swap { code1, loc1, code2, loc2 } => {
            w.u32(*code1).loc(loc1).u32(*code2).loc(loc2);
            MSG_SWAP
        }
        FieldDisabled { flag } => {
            w.u32(*flag);
            MSG_FIELD_DISABLED
        }
        Summoned {} => MSG_SUMMONED,
        Spsummoned {} => MSG_SPSUMMONED,
        Flipsummoned {} => MSG_FLIPSUMMONED,
        Chaining { code, con, loc, seq, pos, trigger_con, trigger_loc, trigger_seq, desc, chain_count } => {
            w.u32(*code).loc(&Loc { con: *con, loc: *loc, seq: *seq, pos: *pos }).u8(*trigger_con).u8(*trigger_loc).u32(*trigger_seq).u64(*desc).u32(*chain_count);
            MSG_CHAINING
        }
        Chained { chain_count } | ChainSolving { chain_count } | ChainSolved { chain_count } | ChainNegated { chain_count } | ChainDisabled { chain_count } => {
            w.u8(*chain_count);
            match m {
                Chained { .. } => MSG_CHAINED,
                ChainSolving { .. } => MSG_CHAIN_SOLVING,
                ChainSolved { .. } => MSG_CHAIN_SOLVED,
                ChainNegated { .. } => MSG_CHAIN_NEGATED,
                _ => MSG_CHAIN_DISABLED,
            }
        }
        ChainEnd {} => MSG_CHAIN_END,
        CardSelected { cards } | BecomeTarget { cards } | RemoveCards { cards } => {
            w.list(cards, |w, l| {
                w.loc(l);
            });
            match m {
                CardSelected { .. } => MSG_CARD_SELECTED,
                BecomeTarget { .. } => MSG_BECOME_TARGET,
                _ => MSG_REMOVE_CARDS,
            }
        }
        RandomSelected { player, cards } => {
            w.u8(*player).list(cards, |w, l| {
                w.loc(l);
            });
            MSG_RANDOM_SELECTED
        }
        Draw { player, cards } => {
            w.u8(*player).list(cards, |w, c| {
                w.u32(c.code).u32(c.pos);
            });
            MSG_DRAW
        }
        Damage { player, amount } | Recover { player, amount } | PayLpcost { player, amount } => {
            w.u8(*player).u32(*amount);
            match m {
                Damage { .. } => MSG_DAMAGE,
                Recover { .. } => MSG_RECOVER,
                _ => MSG_PAY_LPCOST,
            }
        }
        Equip { card, target } | CardTarget { card, target } | CancelTarget { card, target } => {
            w.loc(card).loc(target);
            match m {
                Equip { .. } => MSG_EQUIP,
                CardTarget { .. } => MSG_CARD_TARGET,
                _ => MSG_CANCEL_TARGET,
            }
        }
        Lpupdate { player, lp } => {
            w.u8(*player).u32(*lp);
            MSG_LPUPDATE
        }
        AddCounter { counter_type, con, loc, seq, count } | RemoveCounter { counter_type, con, loc, seq, count } => {
            w.u16(*counter_type).u8(*con).u8(*loc).u8(*seq).u16(*count);
            if matches!(m, AddCounter { .. }) { MSG_ADD_COUNTER } else { MSG_REMOVE_COUNTER }
        }
        Attack { attacker, target } => {
            w.loc(attacker).loc(target);
            MSG_ATTACK
        }
        Battle { attacker, attacker_atk, attacker_def, attacker_destroyed, target, target_atk, target_def, target_destroyed } => {
            w.loc(attacker).i32(*attacker_atk).i32(*attacker_def).u8(*attacker_destroyed);
            w.loc(target).i32(*target_atk).i32(*target_def).u8(*target_destroyed);
            MSG_BATTLE
        }
        AttackDisabled {} => MSG_ATTACK_DISABLED,
        DamageStepStart {} => MSG_DAMAGE_STEP_START,
        DamageStepEnd {} => MSG_DAMAGE_STEP_END,
        MissedEffect { con, loc, seq, pos, code } => {
            w.loc(&Loc { con: *con, loc: *loc, seq: *seq, pos: *pos }).u32(*code);
            MSG_MISSED_EFFECT
        }
        TossCoin { player, results } | TossDice { player, results } => {
            w.u8(*player).u8(results.len() as u32);
            results.iter().for_each(|&r| {
                w.u8(r);
            });
            if matches!(m, TossCoin { .. }) { MSG_TOSS_COIN } else { MSG_TOSS_DICE }
        }
        RockPaperScissors { player } => {
            w.u8(*player);
            MSG_ROCK_PAPER_SCISSORS
        }
        HandRes { result } => {
            w.u8(*result);
            MSG_HAND_RES
        }
        AnnounceRace { player, count, available } => {
            w.u8(*player).u8(*count).u64(*available);
            MSG_ANNOUNCE_RACE
        }
        AnnounceAttrib { player, count, available } => {
            w.u8(*player).u8(*count).u32(*available);
            MSG_ANNOUNCE_ATTRIB
        }
        AnnounceCard { player, values } | AnnounceNumber { player, values } => {
            w.u8(*player).u8(values.len() as u32);
            values.iter().for_each(|&v| {
                w.u64(v);
            });
            if matches!(m, AnnounceCard { .. }) { MSG_ANNOUNCE_CARD } else { MSG_ANNOUNCE_NUMBER }
        }
        CardHint { con, loc, seq, pos, hint_type, value } => {
            w.loc(&Loc { con: *con, loc: *loc, seq: *seq, pos: *pos }).u8(*hint_type).u64(*value);
            MSG_CARD_HINT
        }
        PlayerHint { player, hint_type, value } => {
            w.u8(*player).u8(*hint_type).u64(*value);
            MSG_PLAYER_HINT
        }
        MatchKill { code } => {
            w.u32(*code);
            MSG_MATCH_KILL
        }
        AiName { text } | ShowHint { text } => {
            w.string(text);
            if matches!(m, AiName { .. }) { MSG_AI_NAME } else { MSG_SHOW_HINT }
        }
        TagSwap { player, main_count, extra_p_count, top_code, hand, extra } => {
            w.u8(*player).u32(*main_count).u32(extra.len() as u32).u32(*extra_p_count).u32(hand.len() as u32).u32(*top_code);
            hand.iter().chain(extra).for_each(|c| {
                w.u32(c.code).u32(c.pos);
            });
            MSG_TAG_SWAP
        }
        ReloadField { options, players, chains } => {
            w.u32(*options);
            for p in players {
                w.u32(p.lp);
                for z in p.mzone.iter().chain(&p.szone) {
                    match z {
                        Some(z) => w.u8(1).u8(z.pos).u32(z.overlays),
                        None => w.u8(0),
                    };
                }
                for n in [p.deck, p.hand, p.grave, p.removed, p.extra, p.extra_p] {
                    w.u32(n);
                }
            }
            w.list(chains, |w, l| {
                w.chain_link(l);
            });
            MSG_RELOAD_FIELD
        }
    };
    let mut out = vec![typ as u8];
    out.extend(w.0);
    out
}

/// MSG_START as the server sends it to `player`: its type (0: the player starts), both
/// players' Life Points, then each player's Main and Extra Deck sizes.
pub fn start(player: u32, lp: [u32; 2], decks: [(usize, usize); 2]) -> Vec<u8> {
    let mut w = Writer::default();
    w.u8(MSG_START).u8(player).u32(lp[0]).u32(lp[1]);
    for (main, extra) in decks {
        w.u16(main as u32).u16(extra as u32);
    }
    w.0
}

/// A card's query as EDOPro sends it (the fields the view has, each a u16 size, its flag and
/// its value; QUERY_END last), or an empty zone (a size 0).
fn query(w: &mut Writer, q: Option<&CardQuery>) {
    let Some(q) = q else {
        w.u16(0);
        return;
    };
    let mut field = |flag: u32, value: &[u8]| {
        w.u16(4 + value.len() as u32).u32(flag);
        w.0.extend(value);
    };
    let u = |v: u32| v.to_le_bytes().to_vec();
    let i = |v: i32| v.to_le_bytes().to_vec();
    let fields: [(u32, Option<Vec<u8>>); 13] = [
        (QUERY_CODE, q.code.map(u)),
        (QUERY_POSITION, q.position.map(u)),
        (QUERY_ALIAS, q.alias.map(u)),
        (QUERY_TYPE, q.type_.map(u)),
        (QUERY_LEVEL, q.level.map(u)),
        (QUERY_RANK, q.rank.map(u)),
        (QUERY_ATTRIBUTE, q.attribute.map(u)),
        (QUERY_RACE, q.race.map(|r| r.to_le_bytes().to_vec())),
        (QUERY_ATTACK, q.attack.map(i)),
        (QUERY_DEFENSE, q.defense.map(i)),
        (QUERY_BASE_ATTACK, q.base_attack.map(i)),
        (QUERY_BASE_DEFENSE, q.base_defense.map(i)),
        (QUERY_OWNER, q.owner.map(|o| vec![o as u8])),
    ];
    for (flag, value) in fields {
        if let Some(v) = value {
            field(flag, &v);
        }
    }
    if let Some(o) = &q.overlays {
        let mut v = u(o.len() as u32);
        o.iter().for_each(|&c| v.extend(u(c)));
        field(QUERY_OVERLAY_CARD, &v);
    }
    for (flag, value) in [(QUERY_STATUS, q.status), (QUERY_LSCALE, q.lscale), (QUERY_RSCALE, q.rscale)] {
        if let Some(v) = value {
            field(flag, &u(v));
        }
    }
    if let Some(link) = q.link {
        let mut v = u(link);
        v.extend(u(q.link_marker.unwrap_or(0)));
        field(QUERY_LINK, &v);
    }
    field(QUERY_END, &[]);
}

/// MSG_UPDATE_DATA of a location of `player`'s cards in the view: the cards' queries (as EDOPro's
/// server refreshes a client's locations).
pub fn update_data(view: &Snapshot, player: u32, loc: u32) -> Vec<u8> {
    let mut body = Writer::default();
    for q in view.players[player as usize].pile(loc) {
        query(&mut body, q.as_ref());
    }
    let mut w = Writer::default();
    w.u8(MSG_UPDATE_DATA).u8(player).u8(loc).u32(body.0.len() as u32);
    w.0.extend(body.0);
    w.0
}

fn i32_at(b: &[u8], i: usize) -> i32 {
    // (as the core's response buffer: zeros beyond its end)
    let mut x = [0u8; 4];
    for (k, v) in x.iter_mut().enumerate() {
        *v = b.get(i + k).copied().unwrap_or(0);
    }
    i32::from_le_bytes(x)
}

/// The response the core reads from an EDOPro client's response bytes to `m` (its reading of
/// them: `playerop.rs`).
pub fn decode_response(m: &Msg, b: &[u8]) -> Result<Response, String> {
    use Msg::*;
    let v = i32_at(b, 0);
    let index = |v: i32| Response { index: Some(v as i64), ..Response::default() };
    Ok(match m {
        SelectIdlecmd { .. } | SelectBattlecmd { .. } => {
            let commands: &[Command] = if matches!(m, SelectIdlecmd { .. }) { &Command::IDLE } else { &Command::BATTLE };
            let kind = *commands.get((v & 0xffff) as usize).ok_or(format!("command {}", v & 0xffff))?;
            Response::command(kind, kind.list_name().map(|_| (v >> 16) as usize))
        }
        SelectEffectyn { .. } | SelectYesno { .. } => Response { yes: Some(v != 0), ..Response::default() },
        SelectOption { .. } | AnnounceNumber { .. } | SelectChain { .. } => index(v),
        SelectCard { .. } | SelectTribute { .. } | SelectSum { .. } => {
            if v == -1 {
                return Ok(Response { cancel: Some(true), ..Response::default() });
            }
            let n = match m {
                SelectCard { cards, .. } | SelectTribute { cards, .. } | SelectSum { cards, .. } => cards.len(),
                _ => unreachable!(),
            };
            let at = |k: usize| b.get(k).copied().unwrap_or(0) as i64;
            let indices = match v {
                3 => (0..n).filter(|&i| at(4 + i / 8) >> (i % 8) & 1 != 0).map(|i| i as i64).collect(),
                0..=2 => {
                    let count = i32_at(b, 4) as usize;
                    (0..count)
                        .map(|i| match v {
                            0 => i32_at(b, 8 + 4 * i) as i64,
                            1 => at(8 + 2 * i) | at(9 + 2 * i) << 8,
                            _ => at(8 + i),
                        })
                        .collect()
                }
                _ => return Err(format!("selection type {v}")),
            };
            Response { indices: Some(indices), ..Response::default() }
        }
        SelectUnselectCard { .. } => match v {
            -1 => Response { finish: Some(true), ..Response::default() },
            _ => index(i32_at(b, 4)),
        },
        SelectPlace { .. } | SelectDisfield { .. } => {
            Response { places: Some(b.chunks_exact(3).map(|p| [p[0] as u32, p[1] as u32, p[2] as u32]).collect()), ..Response::default() }
        }
        SelectPosition { .. } => Response { position: Some(v as i64), ..Response::default() },
        SelectCounter { .. } => Response { counts: Some(b.chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]]) as i64).collect()), ..Response::default() },
        SortCard { .. } | SortChain { .. } => match b.first() {
            None | Some(0xff) => Response::default(),
            Some(_) => Response { order: Some(b.iter().map(|&o| o as i8 as i64).collect()), ..Response::default() },
        },
        AnnounceRace { .. } => {
            let mut x = [0u8; 8];
            x.iter_mut().zip(b).for_each(|(d, s)| *d = *s);
            Response { mask: Some(u64::from_le_bytes(x)), ..Response::default() }
        }
        AnnounceAttrib { .. } => Response { mask: Some(v as u32 as u64), ..Response::default() },
        AnnounceCard { .. } => Response { code: Some(v as i64), ..Response::default() },
        RockPaperScissors { .. } => Response { hand: Some(v as i64), ..Response::default() },
        _ => return Err(format!("no response to {}", m.name())),
    })
}
