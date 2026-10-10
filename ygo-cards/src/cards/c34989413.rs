//! Reprodocus
use super::prelude::*;

pub const ID: u32 = 34989413;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,2,2)
    link::add_procedure(d, c, None, 2, Some(2), None, None, None);
    // change property
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():GetLinkedGroup():FilterCount(aux.FaceupFilter(Card.IsLocation,LOCATION_MZONE),nil)>0 end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        let lg = card::get_linked_group(d, c);
        return group::filter_count(d, &lg, &|d, x| card::is_faceup(d, x) && card::is_location(d, x, LOCATION_MZONE), NO_EX) > 0;
    }
    // e:SetLabel(Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2)))
    let v = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    effect::set_label(d, a.e, &[v as i64]);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local lg=c:GetLinkedGroup():Filter(aux.FaceupFilter(Card.IsLocation,LOCATION_MZONE),nil)
    let lg0 = card::get_linked_group(d, c);
    let lg = group::filter(d, &lg0, &|d, x| card::is_faceup(d, x) && card::is_location(d, x, LOCATION_MZONE), NO_EX);
    // if #lg==0 then return end
    if lg.is_empty() {
        return;
    }
    // local eff=0
    // local val=0
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // eff=EFFECT_CHANGE_RACE
        // val=Duel.AnnounceAnotherRace(lg,tp)
        let eff = EFFECT_CHANGE_RACE;
        let val = duel::announce_another_race(d, &lg, tp);
        change_all(d, c, &lg, eff, val as i64);
    } else {
        // eff=EFFECT_CHANGE_ATTRIBUTE
        // val=Duel.AnnounceAnotherAttribute(lg,tp)
        let eff = EFFECT_CHANGE_ATTRIBUTE;
        let val = duel::announce_another_attribute(d, &lg, tp);
        change_all(d, c, &lg, eff, val as i64);
    }
}
/// `for tc in lg:Iter() do ... end` with `eff` and `val`
fn change_all(d: &mut Duel, c: CardId, lg: &CardSet, eff: u32, val: i64) {
    // for tc in lg:Iter() do
    for tc in group::iter(d, lg) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(eff)
        effect::set_code(d, e1, eff);
        // e1:SetValue(val)
        effect::set_value(d, e1, val);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
