//! Chaos Dragon Levianeer
use super::prelude::*;

pub const ID: u32 = 55878038;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    card::enable_revive_limit(d, c);
    // Special summon procedure (from hand)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_value(d, e1, 1);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    card::register_effect(d, c, e1, false);
    // Apply 1 of 3 effects, depending on monsters banished
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_cost(d, e2, Some(cost_target(cost)));
    effect::set_target(d, e2, Some(cost_target(target)));
    card::register_effect(d, c, e2, false);
    effect::set_label_object(d, e1, Value::Effect(e2));
}
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    if a.card(1).is_none() {
        return true;
    }
    let tp = effect::get_handler_player(d, e);
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    ft > -3
        && rg.len() > 2
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true)
        .unwrap_or_default();
    if !g.is_empty() {
        let mut tab: Vec<i64> = Vec::new();
        for tc in group::iter(d, &g) {
            tab.push(card::get_card_id(d, tc) as i64);
        }
        // e:SetLabel(table.unpack(tab))
        effect::set_label(d, e, &tab);
        return true;
    }
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let mut g = CardSet::new();
    for id in effect::get_labels(d, e) {
        let Some(tc) = duel::get_card_from_card_id(d, id as u32) else { return };
        g.insert(tc);
    }
    let lt = group::filter_count(d, &g, &|d, c| card::is_attribute(d, c, ATTRIBUTE_LIGHT), NO_EX);
    let dt = group::filter_count(d, &g, &|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK), NO_EX);
    let mut lbl: i64 = 1;
    if dt == 0 {
        lbl = ATTRIBUTE_LIGHT as i64;
    } else if lt == 0 {
        lbl = ATTRIBUTE_DARK as i64;
    }
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, lo, &[lbl]);
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
}
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 1])
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::get_attack_announced_count(d, h) == 0;
    }
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let op = effect::get_label(d, e);
    if a.chk == Some(0) {
        if op == 0 {
            return false;
        }
        let res = if op == ATTRIBUTE_LIGHT as i64 {
            duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(d, Some(&|d, c| spfilter2(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        } else if op == ATTRIBUTE_DARK as i64 {
            duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), tp, 0, LOCATION_HAND, 1, NO_EX)
        } else {
            duel::get_field_group_count(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD) > 0
        };
        if !res {
            effect::set_label(d, e, &[0]);
        }
        return res;
    }
    if op == 0 {
        effect::set_category(d, e, 0);
        effect::set_operation(d, e, None);
    } else if op == ATTRIBUTE_LIGHT as i64 {
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        effect::set_operation(d, e, Some(operation(spop2)));
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    } else if op == ATTRIBUTE_DARK as i64 {
        effect::set_category(d, e, CATEGORY_TODECK);
        effect::set_operation(d, e, Some(operation(tdop)));
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, 1 - tp, LOCATION_HAND as i32);
    } else {
        effect::set_category(d, e, CATEGORY_DESTROY);
        effect::set_operation(d, e, Some(operation(desop)));
        let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    }
    effect::set_label(d, e, &[0]);
    true
}
/// "Cannot attack this turn" (the operations of this card add it first)
fn cannot_attack(d: &mut Duel, e: EffectId) {
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, 3206);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_OATH | EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, h, e1, false);
}
fn spop2(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Cannot attack this turn
    cannot_attack(d, e);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| spfilter2(d, c, e, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    // Cannot attack this turn
    cannot_attack(d, a.e);
    let hg = duel::get_matching_group(d, Some(&card::is_able_to_deck), a.tp, 0, LOCATION_HAND, NO_EX);
    if !hg.is_empty() {
        let sg = group::random_select(d, &hg, a.tp, 1);
        duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // Cannot attack this turn
    cannot_attack(d, a.e);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
