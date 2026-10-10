//! Bystial Magnamhut
use super::prelude::*;

pub const ID: u32 = 33854624;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand (Ignition)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(aux.NOT(s.spquickcon))
    effect::set_condition(d, e1, Some(condition(not_spquickcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon itself from the hand (Quick if the opponent controls monsters)
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCondition(s.spquickcon)
    effect::set_condition(d, e2, Some(condition(spquickcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Search 1 Dragon monster in the End Phase after it is Special Summoned
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.regtg)
    effect::set_target(d, e3, Some(cost_target(regtg)));
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c,tp)`
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT+ATTRIBUTE_DARK) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    //   and c:IsFaceup() and Duel.GetMZoneCount(tp,c)>0
    card::is_attribute(d, c, ATTRIBUTE_LIGHT + ATTRIBUTE_DARK)
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
        && card::is_faceup(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.spfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && spfilter(d, chkc, a.tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_GRAVE,1,nil,tp)
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, x| spfilter(d, x, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_REMOVED) and c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, tc, LOCATION_REMOVED)
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.spquickcon(e,tp,eg,ep,ev,re,r,rp)`
fn spquickcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0
}
/// `aux.NOT(s.spquickcon)`
fn not_spquickcon(d: &mut Duel, a: &EffArgs) -> bool {
    !spquickcon(d, a)
}
/// `s.regtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn regtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_race(d, c, RACE_DRAGON) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(thfilter)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(thfilter)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
