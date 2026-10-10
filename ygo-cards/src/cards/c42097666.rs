//! Spirit with Eyes of Blue
use super::prelude::*;

pub const ID: u32 = 42097666;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[24382602], listed_series: &[SET_BLUE_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 1 Level 4 or lower Dragon or Spellcaster monster
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(matfilter)), 1, Some(1), None, None, None);

    // You cannot Special Summon, except Dragon monsters
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 1, 0);
    // function(e,c) return not c:IsRace(RACE_DRAGON) end
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_DRAGON))));
    card::register_effect(d, c, e1, false);

    // Add to your hand or send to the GY 1 "Mausoleum of White" from your Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // function(e) return e:GetHandler():IsLinkSummoned() end
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_link_summoned(d, h)
    })));
    effect::set_target(d, e2, Some(cost_target(thtgtg)));
    effect::set_operation(d, e2, Some(operation(thtgop)));
    card::register_effect(d, c, e2, false);

    // Special Summon 1 "Blue-Eyes" monster from your hand or GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_cost(d, e3, Some(cost::self_tribute()));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    card::is_level_below(d, c, 4) && card::is_race_ctx(d, c, RACE_DRAGON | RACE_SPELLCASTER, lc, sumtype, Some(tp))
}
/// `s.thtgfilter(c)`
fn thtgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[24382602]) && (card::is_able_to_hand(d, c, None) || card::is_able_to_grave(d, c, None))
}
/// `s.thtgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thtgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thtgop(e,tp,eg,ep,ev,re,r,rp)`
fn thtgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    let g = duel::select_matching_card(d, tp, Some(&thtgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        aux::to_hand_or_else(d, g, tp, None, None, None);
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_BLUE_EYES as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_m_zone_count(d, tp, h.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let nf = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp));
    let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP)
    //   and sc:IsType(TYPE_EFFECT) and sc:IsSummonLocation(LOCATION_GRAVE) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None)
            && card::is_type(d, sc, TYPE_EFFECT)
            && card::is_summon_location(d, sc, LOCATION_GRAVE)
        {
            let c = effect::get_handler(d, a.e).nil();
            // It cannot attack
            let e1 = effect::create_effect(d, c);
            effect::set_description(d, e1, 3206);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, sc, e1, false);
            // Its effects are negated
            card::negate_effects(d, sc, c, None, false, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
