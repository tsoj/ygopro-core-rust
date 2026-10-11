//! Kozmo Dark Planet
use super::prelude::*;

pub const ID: u32 = 85991529;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KOZMO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // special summon condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // special summon rule
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // cannot be targeted
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_value_fn(d, e3, aux::tgoval());
    card::register_effect(d, c, e3, false);
    // negate
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e4, EVENT_CHAINING);
    effect::set_condition(d, e4, Some(condition(discon)));
    effect::set_cost(d, e4, Some(cost_target(discost)));
    effect::set_target(d, e4, Some(cost_target(distg)));
    effect::set_operation(d, e4, Some(operation(disop)));
    card::register_effect(d, c, e4, false);
    // to hand
    let e5 = effect::create_effect(d, c);
    // (the Lua script sets the description of e4 here, not of e5: kept as written)
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e5, EVENT_TO_GRAVE);
    effect::set_property(d, e5, EFFECT_FLAG_DELAY, 0);
    effect::set_condition(d, e5, Some(condition(thcon)));
    effect::set_cost(d, e5, Some(cost_target(thcost)));
    effect::set_target(d, e5, Some(cost_target(thtg)));
    effect::set_operation(d, e5, Some(operation(thop)));
    card::register_effect(d, c, e5, false);
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    duel::set_selected_card(d, sg.clone());
    (group::check_with_sum_greater(d, sg, &|d, c| card::get_level(d, c) as i64, 10).0, false)
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_KOZMO as u16]) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    if a.card(1).is_none() {
        return true;
    }
    let tp = effect::get_handler_player(d, e);
    let ex = match effect::get_handler(d, e) {
        Some(h) => h.into(),
        None => NO_EX,
    };
    let g = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_HAND, 0, ex);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(g.len()), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let ex = match effect::get_handler(d, e) {
        Some(h) => h.into(),
        None => NO_EX,
    };
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_HAND, 0, ex);
    let g = aux::select_unselect_group(
        d,
        &rg,
        Some(e),
        tp,
        Some(1),
        Some(rg.len()),
        Some(&rescon),
        1,
        tp,
        Some(HINTMSG_REMOVE as u64),
        Some(&rescon),
        None,
        true,
    )
    .unwrap_or_default();
    if !g.is_empty() {
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: &EffArgs) {
    if let Some(g) = effect::get_label_object(d, a.e).as_group().cloned() {
        duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    }
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
        && effect::is_spell_effect(d, re)
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&spfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&spfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
        }
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_DESTROY) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::is_able_to_remove_as_cost(d, c, None) && card::is_location(d, c, LOCATION_GRAVE);
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::remove(d, CardSet::from_card(c), POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_KOZMO as u16]) && card::is_level_below(d, c, 9) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
