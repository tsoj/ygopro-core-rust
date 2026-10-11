//! Nettles
//! ウィード
use super::prelude::*;

pub const ID: u32 = 19505896;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy replace
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.desreptg)
    effect::set_target(d, e1, Some(cost_target(desreptg)));
    // e1:SetOperation(s.desrepop)
    effect::set_operation(d, e1, Some(operation(desrepop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.repfilter(c,e)`
fn repfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_PLANT)
    //     and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT) && card::is_destructable(d, c, Some(e)) && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE)
    //     and Duel.IsExistingMatchingCard(s.repfilter,tp,LOCATION_MZONE,0,1,c,e) end
    if a.chk == Some(0) {
        let e = a.e;
        return !card::is_reason(d, c, REASON_REPLACE)
            && duel::is_existing_matching_card(d, Some(&|d, x| repfilter(d, x, e)), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESREPLACE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.repfilter,tp,LOCATION_MZONE,0,1,1,c,e)
        let e = a.e;
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| repfilter(d, x, e)), a.tp, LOCATION_MZONE, 0, 1, 1, false, c.into()).nil();
        // e:SetLabelObject(g:GetFirst())
        let tc = group::get_first(d, &g).nil();
        effect::set_label_object(d, a.e, Value::Card(tc));
        // g:GetFirst():SetStatus(STATUS_DESTROY_CONFIRMED,true)
        card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, true);
        // return true
        return true;
    }
    // else return false end
    false
}
/// `s.desrepop(e,tp,eg,ep,ev,re,r,rp)`
fn desrepop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // tc:SetStatus(STATUS_DESTROY_CONFIRMED,false)
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(tc,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, tc, REASON_EFFECT | REASON_REPLACE, None, None);
}
