//! Familiar-Possessed - Dharc
use super::prelude::*;

pub const ID: u32 = 21390858;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[19327348], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon proc
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_DECK);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // search
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn spfilter1(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[19327348]) && card::is_able_to_grave_as_cost(d, c)
}
fn spfilter2(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_able_to_grave_as_cost(d, c)
}
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.chk,1,nil,sg)
    let mmz = aux::chkf_mmz(1)(d, sg, e, tp, mg, None).0;
    (mmz && group::is_exists(d, sg, &|d, c| chk(d, c, sg), 1, NO_EX), false)
}
fn chk(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsCode(19327348) and sg:IsExists(Card.IsAttribute,1,c,ATTRIBUTE_DARK)
    card::is_code(d, c, &[19327348]) && group::is_exists(d, sg, &|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK), 1, c.into())
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    // function(e,c) if c==nil then return true end ...
    let e = a.e(0);
    if a.card(1).is_none() {
        return true;
    }
    let tp = effect::get_handler_player(d, e);
    let g1 = duel::get_matching_group(d, Some(&spfilter1), tp, LOCATION_MZONE, 0, NO_EX);
    let g2 = duel::get_matching_group(d, Some(&spfilter2), tp, LOCATION_MZONE, 0, NO_EX);
    let mut g = g1.clone();
    group::add_card(&mut g, &g2);
    let loc = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2;
    loc && !g1.is_empty() && !g2.is_empty() && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.v(1).int() as u8;
    let g1 = duel::get_matching_group(d, Some(&spfilter1), tp, LOCATION_MZONE, 0, NO_EX);
    let g2 = duel::get_matching_group(d, Some(&spfilter2), tp, LOCATION_MZONE, 0, NO_EX);
    let mut g1 = g1;
    group::add_card(&mut g1, &g2);
    let g = aux::select_unselect_group(d, &g1, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    if !g.is_empty() {
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let tp = a.v(1).int() as u8;
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    duel::send_to_grave(d, g, REASON_COST, None, None);
    duel::shuffle_deck(d, tp);
    // pierce
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_PIERCE);
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    card::register_effect(d, h, e1, false);
}
fn tfilter(d: &mut Duel, c: CardId) -> bool {
    let lv = card::get_level(d, c);
    (lv == 3 || lv == 4) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_SPELLCASTER) && card::is_able_to_hand(d, c, None)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&tfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
