//! Ojama Pajama
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 75884822;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_OJAMA, SET_ARMED_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // search
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // destroy replace
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_target(d, e3, Some(cost_target(reptg)));
    effect::set_value_fn(d, e3, effect_card_fn(repval));
    effect::set_operation(d, e3, Some(operation(repop)));
    card::register_effect(d, c, e3, false);
    // spsummon
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, _a: &EffArgs) -> bool {
    let _ph = duel::get_current_phase(d);
    duel::is_main_phase(d, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_OJAMA as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        duel::confirm_cards(d, 1 - tp, g);
        duel::break_effect(d);
        duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_ONFIELD)
        && ((card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_MACHINE) && card::is_type(d, c, TYPE_FUSION))
            || (card::is_set_card(d, c, &[SET_ARMED_DRAGON as u16]) && card::is_monster(d, c)))
        && (card::is_reason(d, c, REASON_BATTLE) || card::is_reason(d, c, REASON_EFFECT))
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.desfilter(c,e,tp)`
fn desfilter(d: &mut Duel, c: CardId, _e: EffectId, tp: u8) -> bool {
    ((card::is_faceup(d, c) && card::is_location(d, c, LOCATION_ONFIELD)) || card::is_location(d, c, LOCATION_HAND | LOCATION_GRAVE))
        && card::is_controler(d, c, tp)
        && card::is_set_card(d, c, &[SET_OJAMA as u16])
        && card::is_able_to_remove(d, c, None, None, None)
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED | STATUS_BATTLE_DESTROYED)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| repfilter(d, c, tp), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, a.e, tp)), tp, LOCATION_ONFIELD | LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    let c = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, cc| desfilter(d, cc, a.e, tp)), tp, LOCATION_ONFIELD | LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &g);
        effect::set_label_object(d, a.e, tc.map_or(Value::Nil, Value::from));
        card::set_status(d, tc.nil(), STATUS_DESTROY_CONFIRMED, true);
        return true;
    }
    false
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_OJAMA as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > -1
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, ft as u8, a.tp, LOCATION_REMOVED as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft <= 0 {
        return;
    }
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_REMOVED, 0, ft as u16, ft as u16, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
