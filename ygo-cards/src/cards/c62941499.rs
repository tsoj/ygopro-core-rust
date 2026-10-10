//! Springans Ship - Exblowrer
use super::prelude::*;

pub const ID: u32 = 62941499;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,8,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Destroy cards adjacent to the selected zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish itself
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_START | TIMING_BATTLE_END));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.bancon)
    effect::set_condition(d, e2, Some(condition(bancon)));
    // e2:SetTarget(s.bantg)
    effect::set_target(d, e2, Some(cost_target(bantg)));
    // e2:SetOperation(s.banop)
    effect::set_operation(d, e2, Some(operation(banop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// Lua `a << n` on a 64-bit integer: a negative `n` shifts right.
fn lshift(x: i64, n: i64) -> i64 {
    if n >= 0 {
        if n >= 64 {
            0
        } else {
            x << n
        }
    } else if -n >= 64 {
        0
    } else {
        x >> (-n)
    }
}
/// `adjzone(loc,seq)`: the bits of the place and its vertically and horizontally adjacent zones.
fn adjzone(loc: u32, seq: i64) -> i64 {
    if loc == LOCATION_MZONE {
        if seq < 5 {
            // Own zone and horizontally adjancent | Vertical adjancent zone
            (lshift(7, seq - 1) & ZONES_MMZ as i64) | lshift(1, seq + 8)
        } else {
            // Own zone | vertical adjancent main monster zone
            lshift(1, seq) | (2 + (6 * (seq - 5)))
        }
    } else {
        // loc == LOCATION_SZONE
        // Own zone and horizontally adjancent | Vertical adjancent zone
        (lshift(7, seq + 7) & 0x1F00) | lshift(1, seq)
    }
}
/// `math.floor(math.log(x,2))` for the single-bit zone values used here.
fn floor_log2(x: i64) -> i64 {
    if x <= 0 {
        -1
    } else {
        63 - (x as u64).leading_zeros() as i64
    }
}
/// `optadd(loc,seq)` of `groupfrombit`.
fn optadd(d: &mut Duel, g: &mut CardSet, p: u8, loc: u32, seq: i64) {
    if seq < 0 {
        return;
    }
    // local c=Duel.GetFieldCard(p,loc,seq)
    // if c then g:AddCard(c) end
    if let Some(c) = duel::get_field_card(d, p, loc, seq as u32) {
        group::add_card(g, &CardSet::from_card(c));
    }
}
/// `groupfrombit(bit,p)`: the cards of a zone bit and the cards adjacent to it.
fn groupfrombit(d: &mut Duel, bit: i64, p: u8) -> CardSet {
    // local loc=(bit&0x7F>0) and LOCATION_MZONE or LOCATION_SZONE
    let loc = if (bit & 0x7F) > 0 { LOCATION_MZONE } else { LOCATION_SZONE };
    // local seq=(loc==LOCATION_MZONE) and bit or bit>>8
    let raw = if loc == LOCATION_MZONE { bit } else { bit >> 8 };
    // seq = math.floor(math.log(seq,2))
    let seq = floor_log2(raw);
    let mut g = CardSet::new();
    optadd(d, &mut g, p, loc, seq);
    // if seq<=4 then --No EMZ
    if seq <= 4 {
        // if seq+1<=4 then optadd(loc,seq+1) end
        if seq + 1 <= 4 {
            optadd(d, &mut g, p, loc, seq + 1);
        }
        // if seq-1>=0 then optadd(loc,seq-1) end
        if seq - 1 >= 0 {
            optadd(d, &mut g, p, loc, seq - 1);
        }
    }
    if loc == LOCATION_MZONE {
        if seq < 5 {
            optadd(d, &mut g, p, LOCATION_SZONE, seq);
            if seq == 1 {
                optadd(d, &mut g, p, LOCATION_MZONE, 5);
            }
            if seq == 3 {
                optadd(d, &mut g, p, LOCATION_MZONE, 6);
            }
        } else if seq == 5 {
            optadd(d, &mut g, p, LOCATION_MZONE, 1);
        } else if seq == 6 {
            optadd(d, &mut g, p, LOCATION_MZONE, 3);
        }
    } else {
        // loc == LOCATION_SZONE
        optadd(d, &mut g, p, LOCATION_MZONE, seq);
    }
    g
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsLocation(LOCATION_FZONE) and not (Duel.IsDuelType(DUEL_SEPARATE_PZONE) and c:IsLocation(LOCATION_PZONE))
    !card::is_location(d, c, LOCATION_FZONE)
        && !(duel::is_duel_type(d, DUEL_SEPARATE_PZONE) && card::is_location(d, c, LOCATION_PZONE))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return e:GetHandler():GetOverlayCount()>0 and #g>0 end
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_overlay_count(d, c) > 0 && !g.is_empty();
    }
    // local filter=0
    let mut bits: i64 = 0;
    // for oc in g:Iter() do filter=filter|adjzone(oc:GetLocation(),oc:GetSequence()) end
    for oc in group::iter(d, &g) {
        let loc = card::get_location(d, oc);
        let seq = card::get_sequence(d, oc) as i64;
        bits |= adjzone(loc, seq);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    // local zone=Duel.SelectFieldZone(tp,1,0,LOCATION_ONFIELD,~filter<<16)
    let zone = duel::select_field_zone(d, tp, 1, 0, LOCATION_ONFIELD, Some((!(bits as u32)) << 16));
    // Duel.Hint(HINT_ZONE,tp,zone)
    duel::hint(d, HINT_ZONE as u8, tp, zone as u64);
    // Duel.Hint(HINT_ZONE,1-tp,zone>>16)
    duel::hint(d, HINT_ZONE as u8, 1 - tp, (zone >> 16) as u64);
    // e:SetLabel(zone)
    effect::set_label(d, a.e, &[zone as i64]);
    // local sg=groupfrombit(zone>>16,1-tp)
    let sg = groupfrombit(d, (zone >> 16) as i64, 1 - tp);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=groupfrombit(e:GetLabel()>>16,1-tp)
    let label = effect::get_label(d, a.e);
    let g = groupfrombit(d, label >> 16, 1 - tp);
    // if #g==0 or c:GetOverlayCount()==0 or c:RemoveOverlayCard(tp,1,#g,REASON_EFFECT)<0 then return end
    if g.is_empty() || card::get_overlay_count(d, c) == 0 {
        return;
    }
    if card::remove_overlay_card(d, c, tp, 1, g.len() as u16, REASON_EFFECT) < 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local sg=g:Select(tp,1,#Duel.GetOperatedGroup(),false)
    let max = duel::get_operated_group(d).len() as u16;
    let sg = group::select(d, &g, tp, 1, max, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
/// `s.bancon(e,tp,eg,ep,ev,re,r,rp)`
fn bancon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let _ph = duel::get_current_phase(d);
    let c = effect::get_handler(d, a.e).nil();
    // return not e:GetHandler():IsStatus(STATUS_CHAINING) and Duel.IsTurnPlayer(1-tp)
    //     and (Duel.IsMainPhase() or Duel.IsBattlePhase())
    !card::is_status(d, c, STATUS_CHAINING)
        && duel::is_turn_player(d, 1 - a.tp)
        && (duel::is_main_phase(d, None) || duel::is_battle_phase(d, None))
}
/// `s.bantg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToRemove() end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::is_able_to_remove(d, c, None, None, None);
    }
    true
}
/// `s.banop(e,tp,eg,ep,ev,re,r,rp)`
fn banop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.Remove(c,POS_FACEUP,REASON_EFFECT|REASON_TEMPORARY)
        duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT | REASON_TEMPORARY, None, None);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetLabelObject(c)
        effect::set_label_object(d, e1, Value::Card(c));
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetLabelObject())
    let c = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, c, None, None);
}
