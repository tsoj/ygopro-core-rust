//! Cocoon Rebirth
use super::prelude::*;

pub const ID: u32 = 43644025;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.activate)
    effect::set_operation(d, e2, Some(operation(activate)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.list={[42682609]=17955766,[43751755]=43237273,[17363041]=54959865, [29246354]=17732278,[16241441]=89621922,[42239546]=80344569}`
/// (`s.list[code]`: the code of the monster the Ritual-style material is sent for; `nil` for other codes)
fn list(code: u32) -> Option<u32> {
    match code {
        42682609 => Some(17955766),
        43751755 => Some(43237273),
        17363041 => Some(54959865),
        29246354 => Some(17732278),
        16241441 => Some(89621922),
        42239546 => Some(80344569),
        _ => None,
    }
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:IsFacedown() then return false end
    if card::is_facedown(d, c) {
        return false;
    }
    // local code=c:GetCode()
    let code = card::get_code(d, c)[0];
    // local tcode=s.list[code]
    // return tcode and Duel.IsExistingTarget(s.filter2,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,tcode,e,tp)
    match list(code) {
        Some(tcode) => duel::is_existing_target(d, Some(&|d, x| filter2(d, x, tcode, e, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX),
        None => false,
    }
}
/// `s.filter2(c,tcode,e,tp)`
fn filter2(d: &mut Duel, c: CardId, tcode: u32, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(tcode) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[tcode]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=e:GetLabel()==1
        let res = effect::get_label(d, e) == 1;
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return res and Duel.GetLocationCount(tp,LOCATION_MZONE)>-1
        //     and Duel.CheckReleaseGroupCost(tp,s.filter1,1,false,aux.ReleaseCheckMMZ,nil,e,tp) end
        return res
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -1
            && duel::check_release_group_cost(
                d,
                tp,
                Some(&|d, c| filter1(d, c, e, tp)),
                1,
                1,
                false,
                Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_mmz(d, sg, tp, exg)),
                NO_EX,
            );
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    // local rg=Duel.SelectReleaseGroupCost(tp,s.filter1,1,1,false,aux.ReleaseCheckMMZ,nil,e,tp)
    let rg = duel::select_release_group_cost(
        d,
        tp,
        Some(&|d, c| filter1(d, c, e, tp)),
        1,
        1,
        false,
        Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_mmz(d, sg, tp, exg)),
        NO_EX,
    );
    // local code=rg:GetFirst():GetCode()
    let first = group::get_first(d, &rg).nil();
    let code = card::get_code(d, first)[0];
    // local tcode=s.list[code]
    let tcode = list(code).nil();
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,tcode,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter2(d, c, tcode, e, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
