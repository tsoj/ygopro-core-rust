//! Zero-Day Blaster
//! ゼロ・デイ・ブラスター
use super::prelude::*;

pub const ID: u32 = 93014827;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`: `c:IsAttribute(ATTRIBUTE_DARK) and c:IsLinkMonster()`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_link_monster(d, c)
}
/// `s.spcheck(sg,tp,exg,dg)`
fn spcheck(d: &mut Duel, sg: &CardSet, dg: &CardSet) -> bool {
    // local a=0
    let mut a: i64 = 0;
    // for c in aux.Next(sg) do
    for c in group::iter(d, sg) {
        // if dg:IsContains(c) then a=a+1 end
        if dg.contains(c) {
            a += 1;
        }
        // for tc in aux.Next(c:GetEquipGroup()) do
        let eg = card::get_equip_group(d, c);
        for tc in group::iter(d, &eg) {
            // if dg:IsContains(tc) then a=a+1 end
            if dg.contains(tc) {
                a += 1;
            }
        }
    }
    // return #dg-a>=sg:GetFirst():GetLink()
    let first = group::get_first(d, sg).nil();
    dg.len() as i64 - a >= card::get_link(d, first) as i64
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return true end
    // (the cost does nothing else: its return value is not used)
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    let e = a.e;
    // local dg=Duel.GetMatchingGroup(Card.IsCanBeEffectTarget,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler(),e)
    let dg = duel::get_matching_group(d, Some(&|d, c| card::is_can_be_effect_target(d, c, Some(e))), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel() ~= 100 then return false end
        if effect::get_label(d, e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,s.spcheck,nil,dg)
        let check = |d: &mut Duel, sg: &CardSet, _tp: u8, _exg: &CardSet| (spcheck(d, sg, &dg), false);
        return duel::check_release_group_cost(d, tp, Some(&cfilter), 1, 1, false, Some(&check), NO_EX);
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,s.spcheck,nil,dg)
    let check = |d: &mut Duel, sg: &CardSet, _tp: u8, _exg: &CardSet| (spcheck(d, sg, &dg), false);
    let sg = duel::select_release_group_cost(d, tp, Some(&cfilter), 1, 1, false, Some(&check), NO_EX);
    // local lnk=sg:GetFirst():GetLink()
    let first = group::get_first(d, &sg).nil();
    let lnk = card::get_link(d, first);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local dg=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,lnk,lnk,e:GetHandler())
    let dg = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, lnk as u16, lnk as u16, false, h.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,tp,0)
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, tp, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local dg=Duel.GetTargetCards(e)
    let dg = duel::get_target_cards(d, a.e);
    // if dg and #dg>0 then
    if let Some(dg) = dg {
        if !dg.is_empty() {
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
