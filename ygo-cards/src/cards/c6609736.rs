//! Noh-P.U.N.K. Deer Note
use super::prelude::*;

pub const ID: u32 = 6609736;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_PUNK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon either the revealed "P.U.N.K." monster or this card, and send the other to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::reveal(Some(std::sync::Arc::new(|d, c, e, tp| handspcostfilter(d, c, e, tp))), true, Some(cost::Count::N(1)), Some(cost::Count::N(1)), Some(std::sync::Arc::new(reveal_op)), None)));
    effect::set_target(d, e1, Some(cost_target(handsptg)));
    effect::set_operation(d, e1, Some(operation(handspop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "P.U.N.K." monster from your GY, except a Level 5 monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(gycond)));
    effect::set_target(d, e2, Some(cost_target(gysptg)));
    effect::set_operation(d, e2, Some(operation(gyspop)));
    card::register_effect(d, c, e2, false);
}
// function(e,tp,og) local rc=og:GetFirst() e:SetLabelObject(rc) rc:CreateEffectRelation(e) end
fn reveal_op(d: &mut Duel, e: EffectId, _tp: u8, og: &CardSet) {
    let rc = group::get_first(d, og).nil();
    effect::set_label_object(d, e, Value::Card(rc));
    card::create_effect_relation(d, rc, e);
}
fn handspcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !(card::is_set_card(d, c, &[SET_PUNK as u16]) && card::is_monster(d, c)) {
        return false;
    }
    let hc = effect::get_handler(d, e).nil();
    (card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) && card::is_able_to_grave(d, hc, None))
        || (card::is_can_be_special_summoned(d, hc, e, 0, tp, false, false, None, None, None) && card::is_able_to_grave(d, c, None))
}
// Group.FromCards(e:GetHandler(), e:GetLabelObject())
fn handspair(d: &mut Duel, e: EffectId) -> CardSet {
    let hc = effect::get_handler(d, e).nil();
    let mut g = CardSet::from_card(hc);
    if let Some(rc) = effect::get_label_object(d, e).as_card() {
        group::add_card(&mut g, &CardSet::from_card(rc));
    }
    g
}
fn handsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    let g = handspair(d, e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g.clone()), 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), 1, tp, 0);
    true
}
fn handspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let mut g = handspair(d, e);
    group::match_(d, &mut g, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    if g.len() != 2 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let sg = group::filter_select(d, &g, tp, &|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None), 1, 1, false, NO_EX).nil();
    if !sg.is_empty() && duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        let mut rest = g.clone();
        group::remove_card(&mut rest, &sg);
        duel::send_to_grave(d, rest, REASON_EFFECT, None, None);
    }
}
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_PUNK as u16]) && !card::is_level(d, c, &[5]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn gycond(d: &mut Duel, a: &EffArgs) -> bool {
    let handler = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, handler, LOCATION_ONFIELD)
}
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && gyspfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| gyspfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| gyspfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
fn gyspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, e) {
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // You cannot Special Summon "Noh-P.U.N.K. Deer Note" for the rest of this turn
    let handler = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, handler);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[ID]))));
    effect::set_reset(d, e1, RESET_PHASE + PHASE_END, 1);
    duel::register_effect(d, e1, tp);
}
