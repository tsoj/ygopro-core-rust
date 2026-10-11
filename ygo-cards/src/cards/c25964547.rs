//! Dream Mirror Hypnagogia
use super::prelude::*;

pub const ID: u32 = 25964547;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DREAM_MIRROR_JOY, CARD_DREAM_MIRROR_TERROR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place 1 "Dream Mirror of Joy" and 1 "Dream Mirror of Terror" from hand/deck to the field zones
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFieldSpell() and c:IsCode(CARD_DREAM_MIRROR_JOY,CARD_DREAM_MIRROR_TERROR)
    card::is_field_spell(d, c) && card::is_code(d, c, &[CARD_DREAM_MIRROR_JOY, CARD_DREAM_MIRROR_TERROR])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_DECK | LOCATION_HAND, 0, NO_EX);
    // if chk==0 then return g:GetClassCount(Card.GetCode)>1 end
    if a.chk == Some(0) {
        return group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64) > 1;
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,nil)
    let mut g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_DECK | LOCATION_HAND, 0, NO_EX);
    // if g:GetClassCount(Card.GetCode)<2 then return end
    if group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64) < 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local tg1=g:Select(tp,1,1,nil)
    let tg1 = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    // g:Remove(Card.IsCode,nil,tg1:GetFirst():GetCode())
    let code = card::get_code(d, group::get_first(d, &tg1).nil())[0];
    group::remove(d, &mut g, &|d, x| card::is_code(d, x, &[code]), NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local tg2=g:Select(tp,1,1,nil)
    let tg2 = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    // local cs={tg1:GetFirst(),tg2:GetFirst()}
    let f1 = group::get_first(d, &tg1);
    let f2 = group::get_first(d, &tg2);
    let mut cs = [f1, f2];
    // if tp==1 then
    if a.tp == 1 {
        // cs={tg2:GetFirst(),tg1:GetFirst()}
        cs = [f2, f1];
    }
    // for p=0,1 do
    for p in 0..2u8 {
        // local tc=cs[p+1]
        let tc = cs[p as usize].nil();
        // local fc=Duel.GetFieldCard(p,LOCATION_FZONE,0)
        let fc = duel::get_field_card(d, p, LOCATION_FZONE, 0);
        // if fc then
        if let Some(fc) = fc {
            // Duel.SendtoGrave(fc,REASON_RULE)
            duel::send_to_grave(d, fc, REASON_RULE, None, None);
            // Duel.BreakEffect()
            duel::break_effect(d);
        }
        // Duel.MoveToField(tc,tp,p,LOCATION_FZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, a.tp, p, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
    }
}
