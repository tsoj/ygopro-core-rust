//! S-Force Retroactive
use super::prelude::*;

pub const ID: u32 = 53782828;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_S_FORCE], ..ScriptInfo::DEFAULT };

/// `s.flagmap[c]`: the list of flag effects that `extraval` registered on card `c` (a Lua table of
/// effects in the duel, see Proxy Horse for the same pattern)
fn flagmap_key(c: CardId) -> u64 {
    0x1000_0000 + c.0 as u64
}
fn flagmap_get(d: &Duel, c: CardId) -> Vec<Value> {
    match s_var(d, ID, flagmap_key(c)) {
        Value::Multi(v) => v,
        _ => Vec::new(),
    }
}
pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Use as Link Material from hand
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_code(d, e1, EFFECT_EXTRA_MATERIAL);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_operation(d, e1, Some(lua_fn(extracon)));
    effect::set_value_fn(d, e1, lua_fn(extraval));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Level 2 or higher "S-Force" monster from the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // Replace cost
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e3, EFFECT_COST_REPLACE);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_target_range(d, e3, 1, 0);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_able_to_remove_as_cost(d, h, None)
    })));
    effect::set_value_fn(d, e3, lua_fn(repval));
    effect::set_operation(d, e3, Some(lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        duel::remove(d, h, POS_FACEUP as u8, REASON_COST, None, None);
        Value::Nil
    })));
    card::register_effect(d, c, e3, false);
}
/// `s.extrafilter(c,tp)`
fn extrafilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
/// `s.extracon(c,e,tp,sg,mg,lc,og,chk)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let sg = a.g(3);
    let mg = a.g(4);
    let og = a.g(6);
    let ct = group::filter_count(d, &sg, &|d, c| card::has_flag_effect(d, c, ID, None), NO_EX);
    if ct >= 2 {
        return false;
    }
    let hp = effect::get_handler_player(d, e);
    let mut sm = sg.clone();
    group::add_card(&mut sm, &mg);
    group::is_exists(d, &sm, &|d, c| extrafilter(d, c, hp), 1, (&og).into())
}
/// `s.extraval(chk,summon_type,e,...)`
fn extraval(d: &mut Duel, a: Args) -> Value {
    let chk = a.int(0);
    let summon_type = a.int(1);
    let e = a.e(2);
    let c = effect::get_handler(d, e).nil();
    if chk == 0 {
        // local tp,sc=...
        let tp = a.p(3);
        let sc = a.c(4);
        if summon_type != SUMMON_TYPE_LINK as i64 || !card::is_set_card(d, sc, &[SET_S_FORCE as u16]) || duel::get_flag_effect(d, tp, ID) > 0 {
            Value::Group(CardSet::new())
        } else {
            let fe = card::register_flag_effect(d, c, ID, 0, 0, 1, 0, 0);
            let mut list = flagmap_get(d, c);
            list.push(Value::Effect(fe));
            set_s_var(d, ID, flagmap_key(c), Value::Multi(list));
            Value::Group(CardSet::from_card(c))
        }
    } else if chk == 1 {
        // local sg,sc,tp=...
        let sg = a.g(3);
        let tp = a.p(5);
        if summon_type & SUMMON_TYPE_LINK as i64 == SUMMON_TYPE_LINK as i64 && !sg.is_empty() {
            duel::hint(d, HINT_CARD as u8, tp, ID as u64);
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        Value::Nil
    } else if chk == 2 {
        for eff in flagmap_get(d, c) {
            if let Value::Effect(fe) = eff {
                effect::reset(d, fe);
            }
        }
        set_s_var(d, ID, flagmap_key(c), Value::Multi(Vec::new()));
        Value::Nil
    } else {
        Value::Nil
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_S_FORCE as u16]) && card::is_level_above(d, c, 2) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
            && card::is_able_to_hand(d, c, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_hand(d, c, None, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_HAND)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// The extra check of the replace cost: `extracon(base,e,tp,e,tp,eg,ep,ev,re,r,rp,chk)` with the
/// arguments of `s.repval` (`raw`: base, extracon, e, tp, eg, ep, ev, re, r, rp, chk).
fn call_extracon(d: &mut Duel, raw: &[Value]) -> bool {
    let f = match raw.get(1) {
        Some(Value::Fn(f)) => f.clone(),
        _ => crate::lua_error("attempt to call a nil value"),
    };
    let args = [
        raw[0].clone(),
        raw[2].clone(),
        raw[3].clone(),
        raw[2].clone(),
        raw[3].clone(),
        raw[4].clone(),
        raw[5].clone(),
        raw[6].clone(),
        raw[7].clone(),
        raw[8].clone(),
        raw[9].clone(),
        raw[10].clone(),
    ];
    f.call(d, &args).truthy()
}
/// `s.repval(base,extracon,e,tp,eg,ep,ev,re,r,rp,chk)`
fn repval(d: &mut Duel, a: Args) -> bool {
    let raw: Vec<Value> = a.0.to_vec();
    let e = a.e(2);
    let tp = a.p(3);
    if a.int(10) == 0 {
        let c = effect::get_handler(d, e).nil();
        return card::is_faceup(d, c)
            && card::is_controler(d, c, tp)
            && card::is_location(d, c, LOCATION_MZONE)
            && card::is_set_card(d, c, &[SET_S_FORCE as u16])
            && call_extracon(d, &raw);
    }
    chain::is_triggering(d, 0, chain::Prop::Controler, &[tp as i64]).truthy()
        && chain::is_triggering(d, 0, chain::Prop::Location, &[LOCATION_MZONE as i64]).truthy()
        && chain::is_triggering(d, 0, chain::Prop::Position, &[POS_FACEUP as i64]).truthy()
        && chain::is_triggering(d, 0, chain::Prop::SetCard, &[SET_S_FORCE as i64]).truthy()
        && call_extracon(d, &raw)
}
