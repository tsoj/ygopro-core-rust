//! Therion "Reaper" Fum
use super::prelude::*;

pub const ID: u32 = 21727231;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THERION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon self
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,Card.EquipByEffectAndLimitRegister,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| eqval(d, a.c(0), a.p(2))),
        lua_fn(|d, a| card::equip_by_effect_and_limit_register(d, a.c(0), a.e(1), a.p(2), a.c(3), None, None)),
        e1,
        None,
        None,
        None,
    );
    // Send to hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // Equipped monster gains ATK
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_condition(d, e3, Some(lua_fn(e3_cond)));
    effect::set_value(d, e3, 700);
    card::register_effect(d, c, e3, false);
    // Equipped monster gains effect
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    effect::set_target(d, e4, Some(effect_card_fn(e4_target)));
    effect::set_label_object(d, e4, Value::Effect(e2));
    card::register_effect(d, c, e4, false);
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && (card::is_set_card(d, c, &[SET_THERION as u16]) || card::is_race(d, c, RACE_AQUA))
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, ec: CardId, tp: u8) -> bool {
    card::is_controler(d, ec, tp) && eqfilter(d, ec)
}
/// `function(e) return e:GetHandler():GetEquipTarget():IsSetCard(SET_THERION) end`
fn e3_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    let et = card::get_equip_target(d, h).nil();
    card::is_set_card(d, et, &[SET_THERION as u16])
}
/// `function(e,c) return c==e:GetHandler():GetEquipTarget() and c:IsSetCard(SET_THERION) end`
fn e4_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    card::get_equip_target(d, h) == Some(c) && card::is_set_card(d, c, &[SET_THERION as u16])
}
/// `s.sptg(e,tp,...,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && eqfilter(d, chkc) && !card::is_forbidden(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&|d, x| eqfilter(d, x) && !card::is_forbidden(d, x)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let g = duel::select_target(d, tp, Some(&|d, x| eqfilter(d, x) && !card::is_forbidden(d, x)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_HAND as i32);
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g.clone(), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
    {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::is_monster(d, tc) && !card::is_forbidden(d, tc) {
            card::equip_by_effect_and_limit_register(d, c, a.e, tp, tc, None, None);
        }
    }
}
/// `s.thfilter(c,e,tp)`
fn thfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_effect_target(d, c, Some(e))
        && card::is_able_to_hand(d, c, None)
        && (card::is_controler(d, c, 1 - tp)
            || (card::get_sequence(d, c) < 5 && card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_THERION as u16])))
}
/// `s.threscon(sg,e,tp,mg)`
fn threscon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let n = group::filter_count(d, sg, &|d, x| card::is_controler(d, x, tp), NO_EX);
    (n == 1, false)
}
/// `s.thtg(e,tp,...,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    if a.chkc().is_some() {
        return false;
    }
    let rg = duel::get_matching_group(d, Some(&|d, x| thfilter(d, x, e, tp)), tp, LOCATION_SZONE, LOCATION_ONFIELD, NO_EX);
    let rs: aux::Rescon = &threscon;
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, e));
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_RTOHAND as u64), None, None, false).unwrap_or_default();
    duel::set_target_card(d, g.clone());
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 2, 0, 0);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
    }
}
