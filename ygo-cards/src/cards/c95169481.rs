//! Diamond Dire Wolf
//! 恐牙狼 ダイヤウルフ
use super::prelude::*;

pub const ID: u32 = 95169481;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.desfilter(c)`: `c:IsFaceup() and c:IsRace(RACES_BEAST_BWARRIOR_WINGB)
///     and Duel.IsExistingTarget(aux.TRUE,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c)
        && card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64)
        && duel::is_existing_target(d, None, 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into())
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&desfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g1=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let mut g1 = duel::select_target(d, tp, Some(&desfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g2=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,g1:GetFirst())
    let first = group::get_first(d, &g1).nil();
    let g2 = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, first.into()).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g1,#g1,0,0)
    let n = g1.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).unwrap_or_default();
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
