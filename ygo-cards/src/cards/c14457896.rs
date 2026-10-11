//! Parasite Paranoid
use super::prelude::*;

pub const ID: u32 = 14457896;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip this card to 1 face-up monster on the field
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Level 7 or higher Insect monster from your hand, ignoring its Summoning conditions
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if card::is_location(d, c, LOCATION_MZONE) && card::is_facedown(d, c) {
        return;
    }
    let tc_opt = duel::first_target(d);
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0
        || card::is_facedown(d, tc_opt.nil())
        || !card::is_relate_to_effect(d, tc_opt.nil(), a.e)
    {
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        return;
    }
    let tc = tc_opt.nil();
    duel::equip(d, tp, c, tc, true, false);
    // e1: EFFECT_EQUIP_LIMIT
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
    effect::set_label_object(d, e1, Value::Card(tc));
    card::register_effect(d, c, e1, false);
    // The equipped monster becomes an Insect monster
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_CHANGE_RACE);
    effect::set_value(d, e2, RACE_INSECT as i64);
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e2, false);
    // The equipped monster cannot attack Insect monsters
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_value_fn(d, e3, effect_card_fn(atlimit));
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e3, false);
    // The equipped monster's effects that activate by targeting an Insect monster(s) are negated
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_label_object(d, e4, Value::Card(tc));
    effect::set_condition(d, e4, Some(condition(discon)));
    effect::set_operation(d, e4, Some(operation(disop)));
    effect::set_reset(d, e4, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e4, false);
    let att = duel::get_attacker(d);
    if att.is_some() && att == Some(tc) {
        let tg = card::get_battle_target(d, tc);
        if let Some(tg) = tg {
            if atlimit_card(d, tg) {
                // cannot attack workaround
                let e5 = effect::create_effect(d, c);
                effect::set_type(d, e5, EFFECT_TYPE_EQUIP);
                effect::set_code(d, e5, EFFECT_CANNOT_ATTACK);
                effect::set_reset(d, e5, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, c, e5, false);
                // reset after unsuccessful attack
                let e5r = effect::create_effect(d, c);
                effect::set_type(d, e5r, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                effect::set_code(d, e5r, EVENT_ADJUST);
                effect::set_range(d, e5r, 0xff);
                effect::set_property(d, e5r, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_label_object(d, e5r, Value::Effect(e5));
                effect::set_operation(d, e5r, Some(operation(resop)));
                effect::set_reset(d, e5r, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, c, e5r, false);
                duel::adjust_instantly(d, None);
            }
        }
    }
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label_object(d, e).as_card() == Some(c)
}
/// `s.atlimit(e,c)`
fn atlimit_card(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_INSECT) && card::is_faceup(d, c)
}
/// `s.atlimit(e,c)` as a value function
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    atlimit_card(d, c)
}
/// `s.disfilter(c)`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_race(d, c, RACE_INSECT)
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let Some(tc) = effect::get_label_object(d, a.e).as_card() else { return false };
    let re = a.re.nil();
    let rc = effect::get_handler(d, re);
    if rc != Some(tc) {
        return false;
    }
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    let Value::Group(g) = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).into_iter().next().unwrap_or(Value::Nil) else {
        return false;
    };
    group::is_exists(d, &g, &disfilter, 1, NO_EX) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
/// `s.resop(e,tp,eg,ep,ev,re,r,rp)`
fn resop(d: &mut Duel, a: &EffArgs) {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::reset(d, lo);
    effect::reset(d, a.e);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_SZONE) && (card::get_previous_type_on_field(d, c) & TYPE_EQUIP) > 0
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_INSECT) && card::is_level_above(d, c, 7) && card::is_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, &g, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
