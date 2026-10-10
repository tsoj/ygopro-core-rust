//! Rivalry of Warlords
use super::prelude::*;

pub const ID: u32 = 90846359;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // s[0]=0
    // s[1]=0
    // (the script's variables, set when the script is loaded)
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_STANDBY_PHASE, Some(TIMING_STANDBY_PHASE | TIMINGS_CHECK_MONSTER));
    effect::set_target(d, e1, Some(cost_target(acttg)));
    card::register_effect(d, c, e1, false);
    // adjust
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e2, EVENT_ADJUST);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_operation(d, e2, Some(operation(adjustop)));
    card::register_effect(d, c, e2, false);
    // cannot summon,spsummon,flipsummon
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EFFECT_FORCE_SPSUMMON_POSITION);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e4, 1, 1);
    effect::set_target(d, e4, Some(lua_fn(sumlimit)));
    effect::set_value(d, e4, POS_FACEDOWN as i64);
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EFFECT_CANNOT_SUMMON);
    card::register_effect(d, c, e5, false);
    // local e6=e4:Clone()
    let e6 = effect::clone(d, e4);
    effect::set_code(d, e6, EFFECT_CANNOT_FLIP_SUMMON);
    card::register_effect(d, c, e6, false);
    // Cannot activate a monster's effect that flips itself face-up as cost if it's of a different Type than the other face-up monsters that player controls
    let e7 = effect::create_effect(d, c);
    effect::set_type(d, e7, EFFECT_TYPE_FIELD);
    effect::set_property(d, e7, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e7, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e7, LOCATION_SZONE);
    effect::set_target_range(d, e7, 1, 1);
    effect::set_value_fn(d, e7, lua_fn(e7_value));
    card::register_effect(d, c, e7, false);
}
/// `s.acttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // s[0]=0
    // s[1]=0
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
    true
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sump = a.p(2);
    // targetp or sump
    let targetp = match a.v(5) {
        Value::Nil => sump,
        v => v.int() as u8,
    };
    // local rc=s.getrace(Duel.GetMatchingGroup(Card.IsFaceup,targetp or sump,LOCATION_MZONE,0,nil))
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), targetp, LOCATION_MZONE, 0, NO_EX);
    let rc = getrace(d, &g);
    // if rc==0 then return false end
    if rc == 0 {
        return false;
    }
    // return c:GetRace()~=rc
    card::get_race(d, c) != rc
}
/// `s.getrace(g)`
fn getrace(d: &mut Duel, g: &CardSet) -> u64 {
    let mut arc = 0u64;
    for tc in group::iter(d, g) {
        arc |= card::get_race(d, tc);
    }
    arc
}
/// `s.rmfilter(c,rc)`
fn rmfilter(d: &mut Duel, c: CardId, rc: u64) -> bool {
    card::get_race(d, c) == rc
}
/// `s.adjustop(e,tp,eg,ep,ev,re,r,rp)`
fn adjustop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let phase = duel::get_current_phase(d);
    if (phase == PHASE_DAMAGE && !duel::is_damage_calculated(d)) || phase == PHASE_DAMAGE_CAL {
        return;
    }
    // local g1=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let mut g1 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let mut g2 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    if g1.is_empty() {
        set_s_var(d, ID, tp as u64, Value::Int(0));
    } else {
        let mut rac = getrace(d, &g1);
        if (rac & rac.wrapping_sub(1)) != 0 {
            let st = s_var(d, ID, tp as u64).int();
            if st == 0 || (st & rac as i64) == 0 {
                duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
                rac = duel::announce_race(d, tp, 1, rac);
            } else {
                rac = st as u64;
            }
        }
        group::remove(d, &mut g1, &|d, c| rmfilter(d, c, rac), NO_EX);
        set_s_var(d, ID, tp as u64, Value::Int(rac as i64));
    }
    if g2.is_empty() {
        set_s_var(d, ID, (1 - tp) as u64, Value::Int(0));
    } else {
        let mut rac = getrace(d, &g2);
        if (rac & rac.wrapping_sub(1)) != 0 {
            let st = s_var(d, ID, (1 - tp) as u64).int();
            if st == 0 || (st & rac as i64) == 0 {
                duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, aux::stringid(ID, 0));
                rac = duel::announce_race(d, 1 - tp, 1, rac);
            } else {
                rac = st as u64;
            }
        }
        group::remove(d, &mut g2, &|d, c| rmfilter(d, c, rac), NO_EX);
        set_s_var(d, ID, (1 - tp) as u64, Value::Int(rac as i64));
    }
    let mut readjust = false;
    if !g1.is_empty() {
        duel::send_to_grave(d, g1, REASON_RULE, Some(PLAYER_NONE as u8), Some(tp));
        readjust = true;
    }
    if !g2.is_empty() {
        duel::send_to_grave(d, g2, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - tp));
        readjust = true;
    }
    if readjust {
        duel::readjust(d);
    }
}
/// `function(e,re,tp) return re:HasSelfChangePositionCost() and s[tp]>0 and re:GetHandler():IsRaceExcept(s[tp]) end`
fn e7_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    let tp = a.p(2);
    if !effect::has_self_change_position_cost(d, re) {
        return false;
    }
    let st = s_var(d, ID, tp as u64).int();
    if st <= 0 {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_race_except(d, h, st as u64, None, 0, None)
}
