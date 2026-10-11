//! Flint Lock
use super::prelude::*;

pub const ID: u32 = 83812099;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[75560629], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.eqcon1)
    effect::set_condition(d, e1, Some(condition(eqcon1)));
    // e1:SetTarget(s.eqtg1)
    effect::set_target(d, e1, Some(cost_target(eqtg1)));
    // e1:SetOperation(s.eqop1)
    effect::set_operation(d, e1, Some(operation(eqop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.eqcon2)
    effect::set_condition(d, e2, Some(condition(eqcon2)));
    // e2:SetTarget(s.eqtg2)
    effect::set_target(d, e2, Some(cost_target(eqtg2)));
    // e2:SetOperation(s.eqop2)
    effect::set_operation(d, e2, Some(operation(eqop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetCondition(s.eqcon2)
    effect::set_condition(d, e3, Some(condition(eqcon2)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.eqcon1(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():GetEquipGroup():IsExists(Card.IsCode,1,nil,75560629)
    let h = effect::get_handler(d, a.e).nil();
    let eq = card::get_equip_group(d, h);
    !group::is_exists(d, &eq, &|d, c| card::is_code(d, c, &[75560629]), 1, NO_EX)
}
/// `s.filter1(c,ec)`
fn filter1(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(75560629) and c:CheckEquipTarget(ec)
    card::is_faceup(d, c) && card::is_code(d, c, &[75560629]) && card::check_equip_target(d, c, ec)
}
/// `s.eqtg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_SZONE,LOCATION_SZONE,1,nil,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| filter1(d, c, h)), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,0,LOCATION_SZONE)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, 0, LOCATION_SZONE as i32);
    true
}
/// `s.eqop1(e,tp,eg,ep,ev,re,r,rp)`
fn eqop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil,c)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter1(d, x, c)), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
    // local eqc=g:GetFirst()
    let eqc = group::get_first(d, &g);
    // if eqc then Duel.Equip(tp,eqc,c) end
    if let Some(eqc) = eqc {
        duel::equip(d, a.tp, eqc, c, true, false);
    }
}
/// `s.eqcon2(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipGroup():IsExists(Card.IsCode,1,nil,75560629)
    let h = effect::get_handler(d, a.e).nil();
    let eq = card::get_equip_group(d, h);
    group::is_exists(d, &eq, &|d, c| card::is_code(d, c, &[75560629]), 1, NO_EX)
}
/// `s.filter2(c,eqc)`
fn filter2(d: &mut Duel, c: CardId, eqc: CardId) -> bool {
    // return c:IsFaceup() and eqc:CheckEquipTarget(c)
    card::is_faceup(d, c) && card::check_equip_target(d, eqc, c)
}
/// The "Flint" monster equipped to the card of the effect (`e:GetHandler():GetEquipGroup():Filter(Card.IsCode,nil,75560629):GetFirst()`).
fn equipped_flint(d: &mut Duel, e: EffectId) -> Option<CardId> {
    let h = effect::get_handler(d, e).nil();
    let eq = card::get_equip_group(d, h);
    let f = group::filter(d, &eq, &|d, c| card::is_code(d, c, &[75560629]), NO_EX);
    group::get_first(d, &f)
}
/// `s.eqtg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // local eqc=e:GetHandler():GetEquipGroup():Filter(Card.IsCode,nil,75560629):GetFirst()
    let eqc = equipped_flint(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter2(chkc,eqc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter2(d, chkc, eqc);
    }
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler(),eqc) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter2(d, c, eqc)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, Exclude::Card(h));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,e:GetHandler(),eqc)
    duel::select_target(d, a.tp, Some(&|d, c| filter2(d, c, eqc)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, Exclude::Card(h));
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,eqc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(eqc)), 1, 0, 0);
    true
}
/// `s.eqop2(e,tp,eg,ep,ev,re,r,rp)`
fn eqop2(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local eqc=e:GetHandler():GetEquipGroup():Filter(Card.IsCode,nil,75560629):GetFirst()
    let eqc = equipped_flint(d, a.e);
    // if eqc then Duel.Equip(tp,eqc,tc) end
    if let Some(eqc) = eqc {
        duel::equip(d, a.tp, eqc, tc.nil(), true, false);
    }
}
