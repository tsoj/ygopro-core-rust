//! Xyz Import
use super::prelude::*;

pub const ID: u32 = 14602126;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(TIMINGS_CHECK_MONSTER|TIMING_BATTLE_START)
    effect::set_hint_timing(d, e1, TIMINGS_CHECK_MONSTER | TIMING_BATTLE_START, None);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.acttg)
    effect::set_target(d, e1, Some(cost_target(acttg)));
    // e1:SetOperation(s.actop)
    effect::set_operation(d, e1, Some(operation(actop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.atchfilter(c,atk)
fn atchfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and not c:IsType(TYPE_TOKEN) and c:IsAttackBelow(atk)
    card::is_faceup(d, c) && !card::is_type(d, c, TYPE_TOKEN) && card::is_attack_below(d, c, atk)
}
// function s.xyzfilter(c,tp)
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsMonster() and c:IsType(TYPE_XYZ) and
    //     Duel.IsExistingTarget(s.atchfilter,tp,0,LOCATION_MZONE,1,nil,c:GetAttack())
    if !(card::is_faceup(d, c) && card::is_monster(d, c) && card::is_type(d, c, TYPE_XYZ)) {
        return false;
    }
    let atk = card::get_attack(d, c);
    duel::is_existing_target(d, Some(&|d, x| atchfilter(d, x, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
// function s.acttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| xyzfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local xyzc=Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_target(d, a.tp, Some(&|d, c| xyzfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let xyzc = group::get_first(d, &sel).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // Duel.SelectTarget(tp,s.atchfilter,tp,0,LOCATION_MZONE,1,1,nil,xyzc:GetAttack())
    let xyz_atk = card::get_attack(d, xyzc);
    duel::select_target(d, a.tp, Some(&|d, c| atchfilter(d, c, xyz_atk)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // e:SetLabelObject(xyzc)
    effect::set_label_object(d, a.e, Value::Card(xyzc));
    true
}
// function s.actop(e,tp,eg,ep,ev,re,r,rp)
fn actop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g<=1 then return end
    if g.len() <= 1 {
        return;
    }
    // local xyzc,tgc=(function()
    //     local c1=g:GetFirst()
    //     local c2=g:GetNext()
    //     if c1==e:GetLabelObject() then return c1,c2 else return c2,c1 end
    // end)()
    let ids = group::iter(d, &g);
    let (c1, c2) = (ids[0], ids[1]);
    let (xyzc, tgc) = if effect::get_label_object(d, a.e).as_card() == Some(c1) { (c1, c2) } else { (c2, c1) };
    // if xyzc:IsRelateToEffect(e) and xyzc:IsControler(tp) and tgc:IsRelateToEffect(e) and tgc:IsControler(1-tp)
    //     and not xyzc:IsImmuneToEffect(e) and not tgc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, xyzc, a.e)
        && card::is_controler(d, xyzc, a.tp)
        && card::is_relate_to_effect(d, tgc, a.e)
        && card::is_controler(d, tgc, 1 - a.tp)
        && !card::is_immune_to_effect(d, xyzc, a.e)
        && !card::is_immune_to_effect(d, tgc, a.e)
    {
        // Duel.Overlay(xyzc,tgc,true)
        duel::overlay(d, xyzc, CardSet::from_card(tgc), true);
    }
}
