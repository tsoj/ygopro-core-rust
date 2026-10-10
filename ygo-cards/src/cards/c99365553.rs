//! Lightpulsar Dragon
use super::prelude::*;

pub const ID: u32 = 99365553;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon procedure from hand
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon1)));
    effect::set_target(d, e1, Some(lua_fn(sptg1)));
    effect::set_operation(d, e1, Some(lua_fn(spop1)));
    card::register_effect(d, c, e1, false);
    // Special summon procedure from graveyard
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_condition(d, e2, Some(lua_fn(spcon2)));
    effect::set_target(d, e2, Some(lua_fn(sptg2)));
    effect::set_operation(d, e2, Some(lua_fn(spop2)));
    card::register_effect(d, c, e2, false);
    // Special summon a monster
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET | EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_condition(d, e3, Some(condition(spcon3)));
    effect::set_target(d, e3, Some(cost_target(sptg3)));
    effect::set_operation(d, e3, Some(operation(spop3)));
    card::register_effect(d, c, e3, false);
}
/// `s.atchk1(c,sg)`
fn atchk1(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && group::filter_count(d, sg, &|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK), c.into()) == 1
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let ok = aux::chkf_mmz(1)(d, sg, e, tp, mg, None).0 && group::is_exists(d, sg, &|d, c| atchk1(d, c, sg), 1, NO_EX);
    (ok, false)
}
fn spfilter1(d: &mut Duel, c: CardId, att: u32) -> bool {
    card::is_attribute(d, c, att) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon1(e,c)`
fn spcon1(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let Some(c) = a.card(1) else { return true };
    let tp = card::get_controler(d, c);
    let rg1 = duel::get_matching_group(d, Some(&|d, x| spfilter1(d, x, ATTRIBUTE_LIGHT)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter1(d, x, ATTRIBUTE_DARK)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let mut rg = rg1.clone();
    group::add_card(&mut rg, &rg2);
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    ft > -2
        && !rg1.is_empty()
        && !rg2.is_empty()
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg1(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg1(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let _c = effect::get_handler(d, e);
    let rg = duel::get_matching_group(d, Some(&|d, x| spfilter1(d, x, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true)
        .unwrap_or_default();
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
/// `s.spop1(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop1(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
fn spfilter2(d: &mut Duel, c: CardId, att: u32) -> bool {
    card::is_attribute(d, c, att) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.spcon2(e,c)`
fn spcon2(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let Some(c) = a.card(1) else { return true };
    for te in card::get_card_effect(d, c, EFFECT_NECRO_VALLEY) {
        match effect::get_operation(d, te) {
            None => return false,
            Some(op) => {
                if op.call(d, &[Value::Effect(e), Value::Card(c)]).truthy() {
                    return false;
                }
            }
        }
    }
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, ATTRIBUTE_LIGHT)), tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, ATTRIBUTE_DARK)), tp, LOCATION_HAND, 0, 1, NO_EX)
}
fn rescon2(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (group::is_exists(d, sg, &|d, c| atchk1(d, c, sg), 1, NO_EX), false)
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg2(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let _c = effect::get_handler(d, e);
    let rg = duel::get_matching_group(d, Some(&|d, x| spfilter2(d, x, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)), tp, LOCATION_HAND, 0, NO_EX);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon2), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true)
        .unwrap_or_default();
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop2(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // g:DeleteGroup()
}
fn spcon3(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
fn spfilter3(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_above(d, c, 5)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_race(d, c, RACE_DRAGON)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg3(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter3(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter3(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter3(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop3(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
