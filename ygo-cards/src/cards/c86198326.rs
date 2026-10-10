//! 7 Completed
use super::prelude::*;

pub const ID: u32 = 86198326;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsRace,RACE_MACHINE),nil,nil,nil,s.operation)
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_race(d, x, RACE_MACHINE))),
        None,
        None,
        None,
        Some(operation(op)),
        None,
        None,
    );
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
                // local opt=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
                let opt = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_EQUIP)
                effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
                // if opt==0 then
                if opt == 0 {
                    // e1:SetCode(EFFECT_UPDATE_ATTACK)
                    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                } else {
                    // e1:SetCode(EFFECT_UPDATE_DEFENSE)
                    effect::set_code(d, e1, EFFECT_UPDATE_DEFENSE);
                }
                // e1:SetValue(700)
                effect::set_value(d, e1, 700);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // c:RegisterEffect(e1)
                card::register_effect(d, c, e1, false);
            }
        }
    }
}
