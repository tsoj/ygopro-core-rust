//! Two Toads with One Sting
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 6203182;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Destroy
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD_P);
    effect::set_operation(d, e2, Some(operation(checkop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e3, Some(operation(desop)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
    // Limit attack
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e4, EFFECT_CANNOT_ATTACK);
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EFFECT_CANNOT_BE_BATTLE_TARGET);
    effect::set_value_fn(d, e5, aux::imval1());
    card::register_effect(d, c, e5, false);
    // Change control
    let e6 = effect::create_effect(d, c);
    effect::set_category(d, e6, CATEGORY_CONTROL);
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e6, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e6, EVENT_TO_GRAVE);
    effect::set_condition(d, e6, Some(condition(ctcon)));
    effect::set_target(d, e6, Some(cost_target(cttg)));
    effect::set_operation(d, e6, Some(operation(ctop)));
    card::register_effect(d, c, e6, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP as u8, None) {
                duel::equip(d, a.tp, c, tc, true, false);
                // Add Equip limit
                let e0 = effect::create_effect(d, tc);
                effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
                effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
                effect::set_value_fn(d, e0, effect_card_fn(eqlimit));
                card::register_effect(d, c, e0, false);
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_DISABLE);
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, tc, e1, false);
                let e2 = effect::clone(d, e1);
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                card::register_effect(d, tc, e2, false);
                duel::special_summon_complete(d);
            }
        }
    }
}
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetOwner()==c
    effect::get_owner(d, e) == Some(c)
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_disabled(d, h) {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject():GetLabel()~=0 then return end
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, lo) != 0 {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, h);
    if let Some(tc) = tc {
        if card::is_location(d, tc, LOCATION_MZONE) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_previous_equip_target(d, c);
    if card::is_reason(d, c, REASON_LOST_TARGET) {
        if let Some(ec) = ec {
            if card::is_reason(d, ec, REASON_SYNCHRO + REASON_FUSION + REASON_XYZ + REASON_LINK) {
                let rc = card::get_reason_card(d, ec).map_or(Value::Nil, Value::Card);
                effect::set_label_object(d, a.e, rc);
                return true;
            }
        }
    }
    false
}
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tc = effect::get_label_object(d, a.e).as_card();
    if a.chk == Some(0) {
        return tc.is_some_and(|tc| card::is_controler(d, tc, 1 - a.tp) && card::is_controler_can_be_changed(d, tc, false, None));
    }
    let tc = tc.nil();
    card::create_effect_relation(d, tc, a.e);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(tc)), 1, 0, 0);
    true
}
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card();
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::get_control(d, tc, a.tp, 0, 0, Some(0xff), None);
        }
    }
}
