//! Uni-Song Tuning
//! scripted Logical Nonsense
use super::prelude::*;

pub const ID: u32 = 12743620;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.lvtg)
    effect::set_target(d, e1, Some(cost_target(lvtg)));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TUNER) and c:IsAbleToRemove() and c:HasLevel()
    card::is_type(d, c, TYPE_TUNER) && card::is_able_to_remove(d, c, None, None, None) && card::has_level(d, c)
}
/// `s.lvfilter(c)`
fn lvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:HasLevel()
    card::is_faceup(d, c) && card::has_level(d, c)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.lvfilter(chkc,lv) end
    // (the global `lv` is nil in Lua; the extra argument is ignored by lvfilter)
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && lvfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&lvfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g1=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil)
    let mut g1 = duel::select_target(d, tp, Some(&filter), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1);
    let label = match first {
        Some(tc) => Value::Card(tc),
        None => Value::Nil,
    };
    effect::set_label_object(d, e, label);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g1,1,1,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g1.clone()), 1, 1, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g2=Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g2 = duel::select_target(d, tp, Some(&lvfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc1=e:GetLabelObject()
    let tc1 = effect::get_label_object(d, e).as_card();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local tc2=g:GetFirst()
    let mut tc2 = group::get_first(d, &g);
    // if tc1==tc2 then tc2=g:GetNext() end
    if tc1 == tc2 {
        tc2 = group::iter(d, &g).get(1).copied();
    }
    // if tc1:IsRelateToEffect(e) and Duel.Remove(tc1,POS_FACEUP,REASON_EFFECT)~=0 then
    let tc1 = tc1.nil();
    if card::is_relate_to_effect(d, tc1, e) && duel::remove(d, tc1, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        let h = effect::get_handler(d, e).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL_FINAL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL_FINAL);
        // e1:SetValue(tc1:GetLevel())
        let lv = card::get_level(d, tc1);
        effect::set_value(d, e1, lv as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc2:RegisterEffect(e1)
        let tc2 = tc2.nil();
        card::register_effect(d, tc2, e1, false);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e2, EFFECT_ADD_TYPE);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // e2:SetValue(TYPE_TUNER)
        effect::set_value(d, e2, TYPE_TUNER as i64);
        // tc2:RegisterEffect(e2)
        card::register_effect(d, tc2, e2, false);
    }
}
