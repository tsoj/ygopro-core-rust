//! Time Maiden
use super::prelude::*;

pub const ID: u32 = 27107590;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TIMELORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon rule
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(sprcon)));
    card::register_effect(d, c, e1, false);
    // double tribute
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_DOUBLE_TRIBUTE);
    effect::set_value_fn(d, e2, effect_card_fn(dtcon));
    card::register_effect(d, c, e2, false);
    // search
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_cost(d, e3, Some(cost::self_tribute()));
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
    // special summon
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_GRAVE);
    effect::set_cost(d, e4, Some(cost_target(spcost)));
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
}
/// `s.sprcon(e,c)`
fn sprcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.dtcon(e,c)`
fn dtcon(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_TIMELORD as u16])
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_TIMELORD as u16]) && card::is_monster(d, c) && card::get_attack(d, c) == 0 && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        let args = [
            Value::Effect(a.e),
            Value::Int(tp as i64),
            a.eg.clone().map_or(Value::Nil, Value::Group),
            Value::Int(a.ep as i64),
            Value::Int(a.ev as i64),
            Value::effect(a.re),
            Value::Int(a.r as i64),
            Value::Int(a.rp as i64),
            Value::Int(0),
        ];
        return cost::self_banish().call(d, &args).truthy() && duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0] == 0;
    }
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    effect::set_label_object(d, e1, Value::Effect(a.e));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let label = effect::get_label_object(d, a.e(0));
    match (a.v(6), label) {
        (Value::Effect(se), Value::Effect(lbl)) => *se != lbl,
        _ => true,
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_TIMELORD as u16])
        && card::is_monster(d, c)
        && card::get_attack(d, c) == 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
