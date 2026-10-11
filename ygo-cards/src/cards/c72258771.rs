//! Koa'ki Meiru Valafar
use super::prelude::*;

pub const ID: u32 = 72258771;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KOAKI_MEIRU], listed_names: &[36623431], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Once per turn, during your End Phases, send 1 "Iron Core of Koa'ki Meiru" from your hand to the GY or destroy this card
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e0, EVENT_PHASE + PHASE_END);
    // e0:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0, LOCATION_MZONE);
    // e0:SetCountLimit(1)
    effect::set_count_limit(d, e0, 1, 0, 0, 0);
    // e0:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e0, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e0:SetOperation(s.maintop)
    effect::set_operation(d, e0, Some(operation(maintop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // You can Tribute Summon this card by Tributing 1 "Koa'ki Meiru" monster
    // aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,1),
    //     function(c,tp) return c:IsSetCard(SET_KOAKI_MEIRU) and (c:IsControler(tp) or c:IsFaceup()) end)
    let _e0a = aux::add_normal_summon_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE as u32),
        Some(aux::stringid(ID, 1)),
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, tp: u8| card::is_set_card(d, c, &[SET_KOAKI_MEIRU as u16]) && (card::is_controler(d, c, tp) || card::is_faceup(d, c)))),
        None,
    );
    // Cannot be destroyed by Trap effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,re,rp) return re:IsTrapEffect() end)
    effect::set_value_fn(d, e1, lua_fn(|d, a| effect::is_trap_effect(d, a.e(1))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card attacks a Defense Position monster, inflict piercing battle damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e2, EFFECT_PIERCE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ironcorefilter(c)`
fn ironcorefilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(36623431) and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[36623431]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.maintop(e,tp,eg,ep,ev,re,r,rp)`
fn maintop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local b1=Duel.IsExistingMatchingCard(s.ironcorefilter,tp,LOCATION_HAND,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&ironcorefilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    // local b2=true
    let b2 = true;
    // Send 1 "Iron Core of Koa'ki Meiru" from your hand to the GY or destroy this card
    // local op=b1 and Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)}) or 2
    let op = if b1 { duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).map_or(2, |x| x as i32) } else { 2 };
    // if op==1 then
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.ironcorefilter,tp,LOCATION_HAND,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&ironcorefilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
    // elseif op==2 then
    } else if op == 2 {
        // Duel.Destroy(c,REASON_COST)
        duel::destroy(d, c, REASON_COST, None, None);
    }
}
