//! Gluttonous Reptolphin Greethys
use super::prelude::*;

pub const ID: u32 = 8576764;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --You can target 1 Fish, Sea Serpent, or Aqua monster in your GY with an equal or lower Level than the number of cards in your opponent's hand; Special Summon it, but it cannot activate its effects this turn. You can only use this effect of "Gluttonous Reptolphin Greethys" once per turn.
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --If this card is sent to the GY as Synchro Material: You can make the Synchro Monster that used this card as material gain 200 ATK/DEF for each card currently in your opponent's hand.
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCondition(s.atkdefchcon)
    effect::set_condition(d, e2, Some(condition(atkdefchcon)));
    // e2:SetOperation(s.atkdefchop)
    effect::set_operation(d, e2, Some(operation(atkdefchop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp,lv)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i32) -> bool {
    // return c:IsMonster() and c:IsRace(RACE_AQUA|RACE_FISH|RACE_SEASERPENT)
    //     and c:HasLevel() and c:IsLevelBelow(lv) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP)
    card::is_monster(d, c)
        && card::is_race(d, c, RACE_AQUA | RACE_FISH | RACE_SEASERPENT)
        && card::has_level(d, c)
        && card::is_level_below(d, c, lv as u32)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local sum=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let sum = duel::get_field_group_count(d, tp, 0, LOCATION_HAND);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp,sum) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp, sum);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and sum>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,sum) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && sum > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp, sum)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,sum)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp, sum)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e)
            && duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None)
        {
            // --Cannot activate its effects this turn
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3302)
            effect::set_description(d, e1, 3302);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_TRIGGER)
            effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.atkdefchcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkdefchcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_GRAVE) and r==REASON_SYNCHRO and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0
    card::is_location(d, c, LOCATION_GRAVE) && a.r == REASON_SYNCHRO && duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0
}
/// `s.atkdefchop(e,tp,eg,ep,ev,re,r,rp)`
fn atkdefchop(d: &mut Duel, a: &EffArgs) {
    // local val=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)*200
    let val = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) as i64 * 200;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local sync=c:GetReasonCard()
    // if sync and sync:IsFaceup() then
    if let Some(sync) = card::get_reason_card(d, c) {
        if card::is_faceup(d, sync) {
            // sync:UpdateAttack(val,RESET_EVENT|RESETS_STANDARD,c)
            card::update_attack(d, sync, val, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None);
            // sync:UpdateDefense(val,RESET_EVENT|RESETS_STANDARD,c)
            card::update_defense(d, sync, val, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None);
        }
    }
}
