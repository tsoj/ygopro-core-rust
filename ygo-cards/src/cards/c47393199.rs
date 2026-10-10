//! Guardragon Cataclysm
use super::prelude::*;

pub const ID: u32 = 47393199;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.spcheck(sg,tp,exg,dg)`
fn spcheck(d: &mut Duel, sg: &CardSet, exg: &CardSet, dg: &CardSet) -> bool {
    let _ = exg;
    // local a=0
    let mut a: usize = 0;
    // for c in aux.Next(sg) do
    for c in group::iter(d, sg) {
        // if dg:IsContains(c) then a=a+1 end
        if dg.contains(c) {
            a += 1;
        }
        // for tc in aux.Next(c:GetEquipGroup()) do
        let eq = card::get_equip_group(d, c);
        for tc in group::iter(d, &eq) {
            // if dg:IsContains(tc) then a=a+1 end
            if dg.contains(tc) {
                a += 1;
            }
        }
    }
    // return #dg-a>=2
    dg.len() as i64 - a as i64 >= 2
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON)
    card::is_race(d, c, RACE_DRAGON)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_on_field(d, chkc) && chkc != h;
    }
    // local dg=Duel.GetMatchingGroup(Card.IsCanBeEffectTarget,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler(),e)
    let h = effect::get_handler(d, a.e).nil();
    let dg = duel::get_matching_group(d, Some(&|d, c| card::is_can_be_effect_target(d, c, Some(a.e))), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==1 then
        if effect::get_label(d, a.e) == 1 {
            // e:SetLabel(0)
            effect::set_label(d, a.e, &[0]);
            // return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,s.spcheck,nil,dg)
            let check = |d: &mut Duel, sg: &CardSet, _tp: u8, exg: &CardSet| (spcheck(d, sg, exg, &dg), false);
            return duel::check_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, Some(&check), NO_EX);
        } else {
            // return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler())
            return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
        }
    }
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // local sg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,s.spcheck,nil,dg)
        let check = |d: &mut Duel, sg: &CardSet, _tp: u8, exg: &CardSet| (spcheck(d, sg, exg, &dg), false);
        let sg = duel::select_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, Some(&check), NO_EX);
        // Duel.Release(sg,REASON_COST)
        duel::release(d, sg, REASON_COST, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,2,2,e:GetHandler())
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 2, 2, false, h.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().nil().clone();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
