//! Expendable Dai
//! 刺し違ＧＵＹ
use super::prelude::*;

pub const ID: u32 = 84125619;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
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
/// `s.equipcheck(sg,tp,exg,dg)`
fn equipcheck(d: &mut Duel, sg: &CardSet, dg: &CardSet) -> bool {
    // local a=0
    let mut a: i64 = 0;
    // for c in aux.Next(sg) do
    for c in group::iter(d, sg) {
        // if dg:IsContains(c) then a=a+1 end
        if group::is_contains(dg, c) {
            a += 1;
        }
        // for tc in aux.Next(c:GetEquipGroup()) do
        let eq = card::get_equip_group(d, c);
        for tc in group::iter(d, &eq) {
            // if dg:IsContains(tc) then a=a+1 end
            if group::is_contains(dg, tc) {
                a += 1;
            }
        }
    }
    // return #dg-a>=1
    dg.len() as i64 - a >= 1
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WARRIOR)
    card::is_race(d, c, RACE_WARRIOR)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsOnField() and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, e).nil();
        return card::is_on_field(d, chkc) && chkc != h;
    }
    // local dg=Duel.GetMatchingGroup(Card.IsCanBeEffectTarget,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler(),e)
    let h = effect::get_handler(d, e).nil();
    let dg = duel::get_matching_group(d, Some(&|d, c| card::is_can_be_effect_target(d, c, Some(e))), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // if chk==0 then
    if a.chk == Some(0) {
        // if not Duel.IsPlayerCanDraw(tp,1) then return false end
        if !duel::is_player_can_draw(d, tp, 1) {
            return false;
        }
        // if e:GetLabel()==1 then
        if effect::get_label(d, e) == 1 {
            // e:SetLabel(0)
            effect::set_label(d, e, &[0]);
            // return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,s.equipcheck,nil,dg)
            return duel::check_release_group_cost(
                d,
                tp,
                Some(&cfilter),
                1,
                1,
                false,
                Some(&|d, sg, _tp, _exg| (equipcheck(d, sg, &dg), false)),
                NO_EX,
            );
        } else {
            // return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler())
            return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
        }
    }
    // if e:GetLabel()==1 then
    if effect::get_label(d, e) == 1 {
        // local sg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,s.equipcheck,nil,dg)
        let sg = duel::select_release_group_cost(
            d,
            tp,
            Some(&cfilter),
            1,
            1,
            false,
            Some(&|d, sg, _tp, _exg| (equipcheck(d, sg, &dg), false)),
            NO_EX,
        );
        // Duel.Release(sg,REASON_COST)
        duel::release(d, sg, REASON_COST, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,e:GetHandler())
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    }
}
