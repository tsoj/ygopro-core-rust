//! Order to Smash
use super::prelude::*;

pub const ID: u32 = 39019325;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}

/// `s.rfilter(c,e)`
fn rfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // local tpe=c:GetType()
    let tpe = card::get_type(d, c);
    // return (tpe&TYPE_NORMAL)~=0 and (tpe&TYPE_TOKEN)==0
    //     and c:IsFaceup() and c:IsLevelBelow(2) and c:IsReleasable() and c:IsReleasableByEffect() and not c:IsImmuneToEffect(e)
    (tpe & TYPE_NORMAL) != 0
        && (tpe & TYPE_TOKEN) == 0
        && card::is_faceup(d, c)
        && card::is_level_below(d, c, 2)
        && card::is_releasable(d, c, Some(REASON_COST), None)
        && card::is_releasable_by_effect(d, c, None, None)
        && !card::is_immune_to_effect(d, c, e)
}

/// `s.dfilter(c)`: `c:IsSpellTrap()`
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c)
}

/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.rfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && rfilter(d, chkc, a.e);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rfilter,tp,LOCATION_MZONE,0,1,nil,e)
    //     and Duel.IsExistingMatchingCard(s.dfilter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rfilter(d, c, a.e)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&dfilter), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local rg=Duel.SelectTarget(tp,s.rfilter,tp,LOCATION_MZONE,0,1,1,nil,e)
    let _rg = duel::select_target(d, tp, Some(&|d, c| rfilter(d, c, a.e)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // local g=Duel.GetMatchingGroup(s.dfilter,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&dfilter), tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}

/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and Duel.Release(tc,REASON_EFFECT)>0 then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && duel::release(d, tc, REASON_EFFECT, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local dg=Duel.SelectMatchingCard(tp,s.dfilter,tp,0,LOCATION_ONFIELD,1,2,nil)
        let dg = duel::select_matching_card(d, tp, Some(&dfilter), tp, 0, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
        // if #dg>0 then
        if !dg.is_empty() {
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
