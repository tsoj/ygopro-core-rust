//! Icejade Creation Kingfisher
use super::prelude::*;

pub const ID: u32 = 66749546;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ICEJADE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    // Attack while in defense position
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_DEFENSE_ATTACK);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    // Return to hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON | CATEGORY_TOHAND);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, a.tp)
            && c != chkc
            && card::is_faceup(d, chkc)
            && card::is_attribute(d, chkc, ATTRIBUTE_WATER);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&eqfilter), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, a.tp, Some(&eqfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if card::is_location(d, c, LOCATION_MZONE) && card::is_facedown(d, c) {
        return;
    }
    let tc = duel::first_target(d);
    let ok = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && match tc {
            Some(t) => card::is_faceup(d, t) && card::is_relate_to_effect(d, t, a.e),
            None => false,
        };
    if ok {
        let tc = tc.nil();
        duel::equip(d, a.tp, c, tc, true, false);
        // Equip limit
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_value_fn(d, e1, lua_fn(equip_limit));
        effect::set_label_object(d, e1, Value::Card(tc));
        card::register_effect(d, c, e1, false);
    } else {
        duel::send_to_grave(d, c, REASON_RULE, None, None);
    }
}
fn equip_limit(d: &mut Duel, a: Args) -> bool {
    a.card(1) == effect::get_label_object(d, a.e(0)).as_card()
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let t = card::get_equip_target(d, c).nil();
    card::is_set_card(d, t, &[SET_ICEJADE as u16])
}
fn thfilter(d: &mut Duel, c: CardId, def: i32) -> bool {
    card::is_faceup(d, c) && card::is_attack_below(d, c, def) && card::is_able_to_hand(d, c, None)
}
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, c).is_some()
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let eqt = card::get_equip_target(d, c).nil();
    let def = card::get_defense(d, eqt);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && thfilter(d, chkc, def);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&|d, x| thfilter(d, x, def)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, x| thfilter(d, x, def)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let eqt = card::get_equip_target(d, c).nil();
    let def = card::get_defense(d, eqt);
    let tc = duel::first_target(d);
    if duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        if let Some(tc) = tc {
            if card::is_faceup(d, tc)
                && card::is_controler(d, tc, 1 - a.tp)
                && card::is_relate_to_effect(d, tc, a.e)
                && card::is_attack_below(d, tc, def)
            {
                duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
            }
        }
    }
}
