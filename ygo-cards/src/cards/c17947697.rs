//! Maiden of White
use super::prelude::*;

pub const ID: u32 = 17947697;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[62089826, CARD_BLUEEYES_W_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place 1 "True Light" from your hand, Deck, or GY face-up in your Spell & Trap Zone
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_target(d, e1, Some(cost_target(pltg)));
    effect::set_operation(d, e1, Some(operation(plop)));
    card::register_effect(d, c, e1, false);

    // Special Summon this card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(selfspcon)));
    effect::set_target(d, e2, Some(cost_target(selfsptg)));
    effect::set_operation(d, e2, Some(operation(selfspop)));
    card::register_effect(d, c, e2, false);

    // Special Summon 1 "Blue-Eyes White Dragon" or 1 Level 1 LIGHT Tuner from your GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_BECOME_TARGET);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_condition(d, e3, Some(condition(|d, a| {
        // function(e,tp,eg) return eg:IsContains(e:GetHandler()) end
        let h = effect::get_handler(d, a.e).nil();
        group::is_contains(&a.eg.clone().nil(), h)
    })));
    effect::set_target(d, e3, Some(cost_target(gysptg)));
    effect::set_operation(d, e3, Some(operation(gyspop)));
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EVENT_BE_BATTLE_TARGET);
    card::register_effect(d, c, e4, false);
}
/// `s.plfilter(c,tp)`
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_code(d, c, &[62089826]) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| plfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.plfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let nf = aux::necro_valley_filter(|d, c| plfilter(d, c, tp));
    let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.selfspconfilter(c,tp)`
fn selfspconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON]) && card::is_summon_player(d, c, tp) && card::is_faceup(d, c)
}
/// `s.selfspcon(e,tp,eg,ep,ev,re,r,rp)`
fn selfspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.selfspconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| selfspconfilter(d, c, tp), 1, NO_EX)
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.gyspfilter(c,e,tp)`
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    (card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON])
        || (card::is_level(d, c, &[1]) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_type(d, c, TYPE_TUNER)))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.gysptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| gyspfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.gyspop(e,tp,eg,ep,ev,re,r,rp)`
fn gyspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| gyspfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
