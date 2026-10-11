//! Hushed Psychic Minister
//! 寡黙なるサイコミニスター
use super::prelude::*;

pub const ID: u32 = 36218106;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control a Psychic monster other than "Hushed Psychic Minister", you can Special Summon this card (from your hand) in Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SPSUM_PARAM)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SPSUM_PARAM, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetTargetRange(POS_FACEUP_DEFENSE,0)
    effect::set_target_range(d, e1, POS_FACEUP_DEFENSE as u32, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 face-up monster on the field until the End Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.rmvcost)
    effect::set_cost(d, e2, Some(cost_target(rmvcost)));
    // e2:SetTarget(s.rmvtg)
    effect::set_target(d, e2, Some(cost_target(rmvtg)));
    // e2:SetOperation(s.rmvop)
    effect::set_operation(d, e2, Some(operation(rmvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PSYCHIC) and not c:IsCode(id) and c:IsFaceup()
    card::is_race(d, c, RACE_PSYCHIC) && !card::is_code(d, c, &[ID]) && card::is_faceup(d, c)
}
/// `s.spcon(e,c)` (the Special Summon procedure's condition)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.rmvcostfilter(c)`
fn rmvcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PSYCHIC) and c:IsAbleToRemoveAsCost()
    card::is_race(d, c, RACE_PSYCHIC) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.rmvcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmvcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost()
    //     and Duel.IsExistingMatchingCard(s.rmvcostfilter,tp,LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None) && duel::is_existing_matching_card(d, Some(&rmvcostfilter), tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rmvcostfilter,tp,LOCATION_GRAVE,0,1,1,c)
    let mut g = duel::select_matching_card(d, tp, Some(&rmvcostfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `aux.FaceupFilter(Card.IsAbleToRemove)`
fn faceup_removable(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && faceup_removable(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToRemove),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&faceup_removable), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToRemove),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&faceup_removable), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    true
}
/// `s.rmvop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // aux.RemoveUntil(tc,nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp)
        aux::remove_until(d, tc, None, REASON_EFFECT, PHASE_END, ID, a.e, tp, Some(aux::default_field_return()), None, None, None, None, None);
    }
}
