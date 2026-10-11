//! Galaxy Brave
use super::prelude::*;

pub const ID: u32 = 55168550;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PHOTON, SET_GALAXY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card and change its Level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make this card's ATK/DEF become equal to a "Galaxy" monster's in your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.atkdeftg)
    effect::set_target(d, e2, Some(cost_target(atkdeftg)));
    // e2:SetOperation(s.atkdefop)
    effect::set_operation(d, e2, Some(operation(atkdefop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PHOTON) and c:HasLevel() and not c:IsPublic()
    card::is_set_card(d, c, &[SET_PHOTON as u16]) && card::has_level(d, c) && !card::is_public(d, c)
}
// s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &g).nil();
    // Duel.ConfirmCards(1-tp,rc)
    duel::confirm_cards(d, 1 - a.tp, rc);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // e:SetLabel(rc:GetLevel())
    let lv = card::get_level(d, rc);
    effect::set_label(d, a.e, &[lv as i64]);
    true
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lv=e:GetLabel()
    let lv = effect::get_label(d, a.e);
    // if c:IsRelateToEffect(e) and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP) and not c:IsLevel(lv) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon_step(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None)
        && !card::is_level(d, c, &[lv as u32])
    {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(lv)
        effect::set_value(d, e1, lv);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// s.atkdeffilter(c,atk,def)
fn atkdeffilter(d: &mut Duel, c: CardId, atk: i32, def: i32) -> bool {
    // return c:IsSetCard(SET_GALAXY) and c:HasDefense() and not (c:IsAttack(atk) and c:IsDefense(def))
    card::is_set_card(d, c, &[SET_GALAXY as u16]) && card::has_defense(d, c)
        && !(card::is_attack(d, c, &[atk as u32]) && card::is_defense(d, c, &[def as u32]))
}
// s.atkdeftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atkdeftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk,def=c:GetAttack(),c:GetDefense()
    let atk = card::get_attack(d, c);
    let def = card::get_defense(d, c);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.atkdeffilter(chkc,atk,def) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && atkdeffilter(d, chkc, atk, def);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkdeffilter,tp,LOCATION_GRAVE,0,1,nil,atk,def) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| atkdeffilter(d, c, atk, def)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.atkdeffilter,tp,LOCATION_GRAVE,0,1,1,nil,atk,def)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| atkdeffilter(d, c, atk, def)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    true
}
// s.atkdefop(e,tp,eg,ep,ev,re,r,rp)
fn atkdefop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local atk,def=tc:GetAttack(),tc:GetDefense()
    let atk = card::get_attack(d, tc);
    let def = card::get_defense(d, tc);
    // if not (c:IsAttack(atk) and c:IsDefense(def)) then
    if !(card::is_attack(d, c, &[atk as u32]) && card::is_defense(d, c, &[def as u32])) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // e2:SetValue(def)
        effect::set_value(d, e2, def as i64);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
