//! Gozen Match
use super::prelude::*;

pub const ID: u32 = 53334471;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // s[0]=0
    // s[1]=0
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_STANDBY_PHASE,TIMING_STANDBY_PHASE|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, TIMING_STANDBY_PHASE, Some(TIMING_STANDBY_PHASE | TIMINGS_CHECK_MONSTER));
    // e1:SetTarget(s.acttg)
    effect::set_target(d, e1, Some(cost_target(acttg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // adjust
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EVENT_ADJUST)
    effect::set_code(d, e2, EVENT_ADJUST);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetOperation(s.adjustop)
    effect::set_operation(d, e2, Some(operation(adjustop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot summon,spsummon,flipsummon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EFFECT_FORCE_SPSUMMON_POSITION)
    effect::set_code(d, e4, EFFECT_FORCE_SPSUMMON_POSITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(1,1)
    effect::set_target_range(d, e4, 1, 1);
    // e4:SetTarget(s.sumlimit)
    effect::set_target(d, e4, Some(lua_fn(sumlimit)));
    // e4:SetValue(POS_FACEDOWN)
    effect::set_value(d, e4, POS_FACEDOWN as i64);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e5, EFFECT_CANNOT_SUMMON);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e4:Clone()
    let e6 = effect::clone(d, e4);
    // e6:SetCode(EFFECT_CANNOT_FLIP_SUMMON)
    effect::set_code(d, e6, EFFECT_CANNOT_FLIP_SUMMON);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // Cannot activate a monster's effect that flips itself face-up as cost if it's of a different Attribute than the other face-up monsters that player controls
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD);
    // e7:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e7, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e7:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e7, EFFECT_CANNOT_ACTIVATE);
    // e7:SetRange(LOCATION_SZONE)
    effect::set_range(d, e7, LOCATION_SZONE);
    // e7:SetTargetRange(1,1)
    effect::set_target_range(d, e7, 1, 1);
    // e7:SetValue(function(e,re,tp) return re:HasSelfChangePositionCost() and s[tp]>0 and re:GetHandler():IsAttributeExcept(s[tp]) end)
    effect::set_value_fn(d, e7, lua_fn(actlimit));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.acttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // s[0]=0
    set_s_var(d, ID, 0, Value::Int(0));
    // s[1]=0
    set_s_var(d, ID, 1, Value::Int(0));
    true
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // local at=s.getattribute(Duel.GetMatchingGroup(Card.IsFaceup,targetp or sump,LOCATION_MZONE,0,nil))
    let p = if a.v(5).truthy() { a.p(5) } else { a.p(2) };
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), p, LOCATION_MZONE, 0, NO_EX);
    let at = getattribute(d, &g);
    // if at==0 then return false end
    if at == 0 {
        return false;
    }
    // return c:GetAttribute()~=at
    let c = a.c(1);
    card::get_attribute(d, c) != at
}
/// `s.getattribute(g)`
fn getattribute(d: &mut Duel, g: &CardSet) -> u32 {
    // local aat=0
    let mut aat = 0;
    // for tc in g:Iter() do aat=(aat|tc:GetAttribute()) end
    for tc in group::iter(d, g) {
        aat |= card::get_attribute(d, tc);
    }
    // return aat
    aat
}
/// `function(e,re,tp) return re:HasSelfChangePositionCost() and s[tp]>0 and re:GetHandler():IsAttributeExcept(s[tp]) end`
fn actlimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // re:HasSelfChangePositionCost()
    if !effect::has_self_change_position_cost(d, re) {
        return false;
    }
    // s[tp]>0
    let st = s_var(d, ID, a.p(2) as u64).int();
    if st <= 0 {
        return false;
    }
    // re:GetHandler():IsAttributeExcept(s[tp])
    let h = effect::get_handler(d, re).nil();
    card::is_attribute_except(d, h, st as u32, None, 0, None)
}
/// One player's side of `s.adjustop`: `g` is the player's face-up monsters (`p` the player). Returns
/// the cards to send to the Graveyard.
fn adjust_side(d: &mut Duel, p: u8, mut g: CardSet) -> CardSet {
    // if #g==0 then s[p]=0
    if g.is_empty() {
        set_s_var(d, ID, p as u64, Value::Int(0));
        return g;
    }
    // local att=s.getattribute(g)
    let mut att = getattribute(d, &g);
    // if (att&att-1)~=0 then
    if (att & att.wrapping_sub(1)) != 0 {
        // if s[p]==0 or (s[p]&att)==0 then
        let st = s_var(d, ID, p as u64).int();
        if st == 0 || (st & att as i64) == 0 {
            // Duel.Hint(HINT_SELECTMSG,p,aux.Stringid(id,0))
            duel::hint(d, HINT_SELECTMSG as u8, p, aux::stringid(ID, 0));
            // att=Duel.AnnounceAttribute(p,1,att)
            att = duel::announce_attribute(d, p, 1, att);
        } else {
            // else att=s[p] end
            att = st as u32;
        }
    }
    // g:Remove(s.rmfilter,nil,att)
    group::remove(d, &mut g, &|d, c| card::get_attribute(d, c) == att, NO_EX);
    // s[p]=att
    set_s_var(d, ID, p as u64, Value::Int(att as i64));
    g
}
/// `s.adjustop(e,tp,eg,ep,ev,re,r,rp)`
fn adjustop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // if (phase==PHASE_DAMAGE and not Duel.IsDamageCalculated()) or phase==PHASE_DAMAGE_CAL then return end
    if (phase == PHASE_DAMAGE && !duel::is_damage_calculated(d)) || phase == PHASE_DAMAGE_CAL {
        return;
    }
    // local g1=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    // (the attribute choice of the player's own monsters, then the opponent's)
    let g1 = adjust_side(d, tp, g1);
    let g2 = adjust_side(d, 1 - tp, g2);
    // local readjust=false
    let mut readjust = false;
    // if #g1>0 then
    if !g1.is_empty() {
        // Duel.SendtoGrave(g1,REASON_RULE,PLAYER_NONE,tp)
        duel::send_to_grave(d, g1, REASON_RULE, Some(PLAYER_NONE as u8), Some(tp));
        // readjust=true
        readjust = true;
    }
    // if #g2>0 then
    if !g2.is_empty() {
        // Duel.SendtoGrave(g2,REASON_RULE,PLAYER_NONE,1-tp)
        duel::send_to_grave(d, g2, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - tp));
        // readjust=true
        readjust = true;
    }
    // if readjust then Duel.Readjust() end
    if readjust {
        duel::readjust(d);
    }
}
