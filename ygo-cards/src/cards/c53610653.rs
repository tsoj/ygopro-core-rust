//! Bound Wand
use super::prelude::*;

pub const ID: u32 = 53610653;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsRace,RACE_SPELLCASTER))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_SPELLCASTER as u64))), None, None, None, None, None, None);
    // --Atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --spsummon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCondition(s.spcon)
    effect::set_condition(d, e4, Some(condition(spcon)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetLevel()*100
    card::get_level(d, c) as i64 * 100
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetPreviousEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_previous_equip_target(d, h);
    // return e:GetHandler():IsReason(REASON_LOST_TARGET) and ec and ec:IsReason(REASON_DESTROY)
    //   and ec:IsLocation(LOCATION_GRAVE) and ec:GetReasonPlayer()==1-tp
    if !card::is_reason(d, h, REASON_LOST_TARGET) {
        return false;
    }
    let Some(ec) = ec else { return false };
    card::is_reason(d, ec, REASON_DESTROY) && card::is_location(d, ec, LOCATION_GRAVE) && card::get_reason_player(d, ec) == 1 - a.tp
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetPreviousEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_previous_equip_target(d, h);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and ec:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, ec.nil(), a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetTargetCard(ec)
    if let Some(ec) = ec {
        duel::set_target_card(d, ec);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,ec,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, ec.map(CardSet::from_card), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
