//! Amazement Administrator Arlekino
use super::prelude::*;

pub const ID: u32 = 94821366;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ATTRACTION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself from hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Equip 1 "Attraction" trap to 1 of opponent's monsters
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_EQUIP);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    effect::set_operation(d, e2, Some(operation(eqop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
    // Destroy opponent's cards
    let e4 = effect::create_effect(d, c);
    effect::set_category(d, e4, CATEGORY_DESTROY);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_cost(d, e4, Some(cost_target(descost)));
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    effect::is_trap_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and (not c:IsControler(tp)) and c:IsCanBeEffectTarget(e)
    //   and (not c:IsSummonPlayer(tp)) and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_DECK,0,1,nil)
    card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_MZONE)
        && !card::is_controler(d, c, tp)
        && card::is_can_be_effect_target(d, c, Some(e))
        && !card::is_summon_player(d, c, tp)
        && duel::is_existing_matching_card(d, Some(&eqfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ATTRACTION as u16]) && card::is_trap(d, c)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&a.eg.clone().nil(), chkc) && filter(d, chkc, e, tp);
    }
    let eg = a.eg.clone().nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && group::is_exists(d, &eg, &|d: &mut Duel, c: CardId| filter(d, c, e, tp), 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let g = group::filter_select(d, &eg, tp, &|d: &mut Duel, c: CardId| filter(d, c, e, tp), 1, 1, false, NO_EX).nil();
    duel::set_target_card(d, g);
    true
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let controler = card::get_controler(d, c);
    controler == effect::get_handler_player(d, e) || card::get_equip_target(d, effect::get_handler(d, e).nil()) == Some(c)
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        let sel = duel::select_matching_card(d, tp, Some(&eqfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if let Some(ec) = group::get_first(d, &sel) {
            duel::equip(d, tp, ec, tc, true, false);
            let e1 = effect::create_effect(d, ec);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, ec, e1, false);
        }
    }
}
/// `s.descfilter(c)`
fn descfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ATTRACTION as u16]) && card::is_trap(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&descfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    let maxtc = duel::get_target_count(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&descfilter), a.tp, LOCATION_GRAVE, 0, 1, maxtc as u16, false, NO_EX).nil();
    let cg = duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    effect::set_label(d, a.e, &[cg as i64]);
    true
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    let ct = effect::get_label(d, a.e) as u16;
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, ct, ct, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, ct as u8, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let rg = duel::get_target_cards(d, a.e).nil();
    if !rg.is_empty() {
        duel::destroy(d, rg, REASON_EFFECT, None, None);
    }
}
