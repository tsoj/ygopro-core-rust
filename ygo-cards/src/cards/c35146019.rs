//! Altergeist Manifestation
use super::prelude::*;

pub const ID: u32 = 35146019;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALTERGEIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // Destroy
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    // to hand
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_TOHAND);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}

fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_ALTERGEIST as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_ATTACK as u8), None, None)
}

fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}

fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label_object(d, e).as_card() == Some(c)
}

fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::get_first_target(d).first().copied();
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None) > 0 {
            if !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
                return;
            }
            duel::equip(d, a.tp, c, tc, true, false);
            // Add Equip limit
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
            effect::set_label_object(d, e1, Value::Card(tc));
            card::register_effect(d, c, e1, false);
            return;
        }
    }
    if card::is_relate_to_effect(d, c, a.e) && !card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        card::cancel_to_grave(d, c, false);
    }
}

fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = card::get_first_card_target(d, c) {
        if card::is_location(d, tc, LOCATION_MZONE) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}

fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_trap(d, c) && card::is_set_card(d, c, &[SET_ALTERGEIST as u16]) && card::is_able_to_hand(d, c, None)
}

fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}

fn thop(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::get_first_target(d).first().copied() {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
