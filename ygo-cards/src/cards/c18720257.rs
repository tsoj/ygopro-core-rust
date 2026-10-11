//! The Weather Forecast
use super::prelude::*;

pub const ID: u32 = 18720257;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THE_WEATHER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Can treat "The Weather" cards in your Spell/Trap Zones as Link Material
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_EXTRA_MATERIAL);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_operation(d, e2, Some(lua_fn(|_d, _a| true)));
    effect::set_value_fn(d, e2, lua_fn(extraval));
    card::register_effect(d, c, e2, false);
    let e2a = effect::create_effect(d, c);
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2a, EFFECT_ADD_TYPE);
    effect::set_range(d, e2a, LOCATION_FZONE);
    effect::set_target_range(d, e2a, LOCATION_SZONE, 0);
    effect::set_condition(d, e2a, Some(condition(addtypecon)));
    effect::set_target(d, e2a, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_THE_WEATHER as u16]))));
    effect::set_value(d, e2a, TYPE_MONSTER as i64);
    card::register_effect(d, c, e2a, false);
    // Normal Summon 1 "The Weather" monster from your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_target(d, e3, Some(cost_target(nstg)));
    effect::set_operation(d, e3, Some(operation(nsop)));
    card::register_effect(d, c, e3, false);
}
/// `s.plfilter(c,tp)`
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_spell_trap(d, c)
        && card::is_set_card(d, c, &[SET_THE_WEATHER as u16])
        && !card::is_forbidden(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
        && !card::is_type(d, c, TYPE_FIELD)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        return;
    }
    let g = duel::get_matching_group(d, Some(&|d, x| plfilter(d, x, tp)), tp, LOCATION_DECK, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        let tc = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let Some(tc) = group::get_first(d, &tc) else {
            return;
        };
        duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_THE_WEATHER as u16]) && card::get_sequence(d, c) < 5
}
/// `s.extraval(chk,summon_type,e,...)`
fn extraval(d: &mut Duel, a: Args) -> Value {
    let chk = a.int(0);
    if chk == 0 {
        // local tp,sc=...
        let tp = a.p(3);
        let sc = a.card(4);
        let summon_type = a.int(1) as u32;
        // if summon_type~=SUMMON_TYPE_LINK or not (sc and sc:IsSetCard(SET_THE_WEATHER)) then
        let weather_sc = match sc {
            Some(sc) => card::is_set_card(d, sc, &[SET_THE_WEATHER as u16]),
            None => false,
        };
        if summon_type != SUMMON_TYPE_LINK || !weather_sc {
            // return Group.CreateGroup()
            return Value::Group(group::create_group(&[]));
        }
        // Duel.RegisterFlagEffect(tp,id,0,0,1)
        duel::register_flag_effect(d, tp, ID, 0, 0, 1, 0);
        // return Duel.GetMatchingGroup(s.matfilter,tp,LOCATION_SZONE,0,nil)
        Value::Group(duel::get_matching_group(d, Some(&matfilter), tp, LOCATION_SZONE, 0, NO_EX))
    } else if chk == 2 {
        // Duel.ResetFlagEffect(e:GetHandlerPlayer(),id)
        let p = effect::get_handler_player(d, a.e(2));
        duel::reset_flag_effect(d, p, ID);
        Value::Nil
    } else {
        Value::Nil
    }
}
/// `s.addtypecon(e,tp,eg,ep,ev,re,r,rp)`
fn addtypecon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::get_flag_effect(d, p, ID) > 0
}
/// `s.nsfilter(c)`
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_THE_WEATHER as u16]) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
/// `s.nsop(e,tp,eg,ep,ev,re,r,rp)`
fn nsop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        duel::summon(d, tp, tc, true, None, 0, None);
    }
}
