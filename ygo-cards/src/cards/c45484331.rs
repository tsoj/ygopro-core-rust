//! Springans Kitt
use super::prelude::*;

pub const ID: u32 = 45484331;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], listed_series: &[SET_BRANDED as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Add to hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_FUSION) and c:ListsCodeAsMaterial(CARD_ALBAZ)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION) && card::lists_code_as_material(d, c, &[CARD_ALBAZ])
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BRANDED) and c:IsSpellTrap() and c:IsAbleToHand()
    //     and (c:IsFaceup() or c:IsLocation(LOCATION_DECK))
    card::is_set_card(d, c, &[SET_BRANDED as u16])
        && card::is_spell_trap(d, c)
        && card::is_able_to_hand(d, c, None)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_DECK))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(thfilter)),
        a.tp,
        LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let Some(tc) = group::get_first(d, &g) else { return };
    // if not (tc and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND)) then return end
    if duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None) <= 0 || !card::is_location(d, tc, LOCATION_HAND) {
        return;
    }
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(tc));
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // if tc:IsPreviousLocation(LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if card::is_previous_location(d, tc, LOCATION_DECK) {
        duel::shuffle_deck(d, a.tp);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local td=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,1,nil)
    let td = duel::select_matching_card(d, a.tp, Some(&card::is_able_to_deck), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #td>0 then
    if !td.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoDeck(td,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, td, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
