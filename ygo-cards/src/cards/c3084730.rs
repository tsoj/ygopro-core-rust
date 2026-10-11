//! Bio-Insect Armor
use super::prelude::*;

pub const ID: u32 = 3084730;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip itself to an insect monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.eqcond)
    effect::set_condition(d, e1, Some(condition(eqcond)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equipped monster gains 1500 ATK/DEF during Battle Phase and Main Phase 2
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetCondition(function() return Duel.IsBattlePhase() or Duel.IsPhase(PHASE_MAIN2) end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_battle_phase(d, None) || duel::is_phase(d, PHASE_MAIN2))));
    // e2:SetValue(1500)
    effect::set_value(d, e2, 1500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // e3:SetValue(2000)
    effect::set_value(d, e3, 2000);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.eqcond(e,tp,eg,ep,ev,re,r,rp)
fn eqcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,id),tp,LOCATION_ONFIELD,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[ID]))), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
// function s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup() and chkc:IsRace(RACE_INSECT) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && card::is_faceup(d, chkc) && card::is_race(d, chkc, RACE_INSECT);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsRace,RACE_INSECT),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_INSECT))), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsRace,RACE_INSECT),tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_INSECT))), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // if c:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_GRAVE) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,0,0,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 0, 0, 0);
    }
    true
}
// function s.eqop(e,tp,eg,ep,ev,re,r,rp)
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)==0 or tc:IsFacedown() or not tc:IsRelateToEffect(e) then
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 == 0
        || card::is_facedown(d, tc.nil())
        || !card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // return Duel.SendtoGrave(c,REASON_RULE)
        duel::send_to_grave(d, CardSet::from_card(c), REASON_RULE, None, None);
        return;
    }
    // if Duel.Equip(tp,c,tc) then
    if duel::equip(d, a.tp, c, tc.nil(), true, false) {
        // Equip limit registration
        // local e0=Effect.CreateEffect(c)
        let e0 = effect::create_effect(d, c);
        // e0:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
        // e0:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
        // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
        // e0:SetValue(function(e,c) return c==e:GetLabelObject() end)
        effect::set_value_fn(d, e0, effect_card_fn(|d, e, cc| Some(cc) == effect::get_label_object(d, e).as_card()));
        // e0:SetLabelObject(tc)
        effect::set_label_object(d, e0, Value::Card(tc.nil()));
        // c:RegisterEffect(e0)
        card::register_effect(d, c, e0, false);
        // Banish itself and the equipped monster when they leave the field
        // local rg=Group.FromCards(c,tc)
        let mut rg = CardSet::from_card(c);
        group::add_card(&mut rg, &CardSet::from_card(tc.nil()));
        // for tc in rg:Iter() do
        for tc in group::iter(d, &rg) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
            effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
            // e1:SetValue(LOCATION_REMOVED)
            effect::set_value(d, e1, LOCATION_REMOVED as i64);
            // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
