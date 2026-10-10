//! Dark Coffin
use super::prelude::*;

pub const ID: u32 = 1804528;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_DESTROY)~=0
    //     and e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    //     and e:GetHandler():IsPreviousPosition(POS_FACEDOWN)
    let h = effect::get_handler(d, a.e).nil();
    (a.r & REASON_DESTROY) != 0 && card::is_previous_location(d, h, LOCATION_ONFIELD) && card::is_previous_position(d, h, POS_FACEDOWN)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g1=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g1 = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // local g2=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g2 = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    // local opt=0 (every branch below sets it, or returns)
    // if #g1>0 and #g2>0 then
    let opt = if g1.len() > 0 && g2.len() > 0 {
        // opt=Duel.SelectOption(1-tp,aux.Stringid(id,1),aux.Stringid(id,2))
        duel::select_option(d, 1 - tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
    // elseif #g1>0 then
    } else if g1.len() > 0 {
        // opt=Duel.SelectOption(1-tp,aux.Stringid(id,1))
        duel::select_option(d, 1 - tp, true, &[aux::stringid(ID, 1)])
    // elseif #g2>0 then
    } else if g2.len() > 0 {
        // opt=Duel.SelectOption(1-tp,aux.Stringid(id,2))+1
        duel::select_option(d, 1 - tp, true, &[aux::stringid(ID, 2)]) + 1
    // else return end
    } else {
        return;
    };
    // if opt==0 then
    if opt == 0 {
        // local dg=g1:RandomSelect(1-tp,1)
        let dg = group::random_select(d, &g1, 1 - tp, 1);
        // Duel.SendtoGrave(dg,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, dg, REASON_EFFECT | REASON_DISCARD, None, None);
    } else {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_DESTROY as u64);
        // local dg=g2:Select(1-tp,1,1,nil)
        let dg = group::select(d, &g2, 1 - tp, 1, 1, false, NO_EX).nil();
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
