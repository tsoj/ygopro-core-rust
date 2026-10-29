//! Crusadia Spatha
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 39528955;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,99,s.matcheck)
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_type_ctx(d, x, TYPE_EFFECT, sc, st, Some(tp)))), 2, Some(99), Some(group_check(matcheck)), None, None);
    // Increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.atklimit)
    effect::set_target(d, e2, Some(effect_card_fn(atklimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Move
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(aux.zptcon(aux.FilterBoolFunction(Card.IsType,TYPE_EFFECT)))
    effect::set_condition(d, e3, Some(aux::zptcon(Some(zpt_filter))));
    // e3:SetTarget(s.seqtg)
    effect::set_target(d, e3, Some(cost_target(seqtg)));
    // e3:SetOperation(s.seqop)
    effect::set_operation(d, e3, Some(operation(seqop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `aux.FilterBoolFunction(Card.IsType,TYPE_EFFECT)` of the Move effect
fn zpt_filter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    card::is_type(d, c, TYPE_EFFECT)
}
/// `s.matcheck(g,lc,sumtype,tp)`
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsSetCard,1,nil,SET_CRUSADIA,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_set_card_ctx(d, c, &[SET_CRUSADIA as u16], Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=e:GetHandler():GetLinkedGroup():Filter(Card.IsFaceup,nil)
    let h = effect::get_handler(d, e).nil();
    let linked = card::get_linked_group(d, h);
    let g = group::filter(d, &linked, &|d, c| card::is_faceup(d, c), NO_EX);
    // return g:GetSum(Card.GetBaseAttack)
    group::get_sum(d, &g, &|d, c| card::get_base_attack(d, c) as i64)
}
/// `s.atklimit(e,c)`
fn atklimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetLinkedGroup():IsContains(c)
    let h = effect::get_handler(d, e).nil();
    let linked = card::get_linked_group(d, h);
    group::is_contains(&linked, c)
}
/// `s.seqfilter(c)`
fn seqfilter(d: &mut Duel, c: CardId) -> bool {
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return c:IsFaceup() and c:GetSequence()<5 and Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL)>0
    card::is_faceup(d, c)
        && card::get_sequence(d, c) < 5
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
}
/// `s.seqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn seqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.seqfilter(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && seqfilter(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.seqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| seqfilter(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // Duel.SelectTarget(tp,s.seqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c)
    duel::select_target(d, a.tp, Some(&|d, x| seqfilter(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    true
}
/// `s.seqop(e,tp,eg,ep,ev,re,r,rp)`
fn seqop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local ttp=tc:GetControler()
    let ttp = card::get_controler(d, tc);
    // if not tc or not tc:IsRelateToEffect(e) or tc:IsImmuneToEffect(e) or Duel.GetLocationCount(ttp,LOCATION_MZONE,ttp,LOCATION_REASON_CONTROL)<=0 then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_immune_to_effect(d, tc, a.e) || duel::get_location_count(d, ttp, LOCATION_MZONE as u8, Some(ttp), Some(LOCATION_REASON_CONTROL), None).0 <= 0 {
        return;
    }
    // local p1,p2,i
    // if tc:IsControler(tp) then
    let (p1, p2, i): (u32, u32, f64);
    if card::is_controler(d, tc, a.tp) {
        // i=0
        i = 0.0;
        // p1=LOCATION_MZONE
        p1 = LOCATION_MZONE;
        // p2=0
        p2 = 0;
    } else {
        // i=16
        i = 16.0;
        // p2=LOCATION_MZONE
        p2 = LOCATION_MZONE;
        // p1=0
        p1 = 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOZONE as u64);
    // Duel.MoveSequence(tc,math.log(Duel.SelectDisableField(tp,1,p1,p2,0),2)-i)
    let zone = duel::select_disable_field(d, a.tp, 1, p1, p2, Some(0), false);
    let seq = (zone as f64).log2() - i;
    duel::move_sequence(d, tc, seq as u8, None);
}
