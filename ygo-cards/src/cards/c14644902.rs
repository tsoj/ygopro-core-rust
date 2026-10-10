//! Summoner of Illusions
use super::prelude::*;

pub const ID: u32 = 14644902;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn rfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsReleasableByEffect() and not c:IsImmuneToEffect(e)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_releasable_by_effect(d, c, None, None)
        && !card::is_immune_to_effect(d, c, e)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // return c:IsType(TYPE_FUSION) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_FUSION)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local rg=Duel.SelectReleaseGroup(tp,s.rfilter,1,1,c,e,tp)
    let rg = duel::select_release_group(d, tp, Some(&|d, x| rfilter(d, x, e, tp)), 1, 1, duel::ReleaseArgs::default(), c.into(), false);
    // if rg and Duel.Release(rg,REASON_EFFECT)>0 then
    if let Some(rg) = rg {
        if duel::release(d, rg, REASON_EFFECT, None) > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
            let sg = duel::select_matching_card(d, tp, Some(&|d, x| filter(d, x, e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                // e1:SetRange(LOCATION_MZONE)
                effect::set_range(d, e1, LOCATION_MZONE);
                // e1:SetCode(EVENT_PHASE+PHASE_END)
                effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
                // e1:SetCondition(s.descon)
                effect::set_condition(d, e1, Some(condition(descon)));
                // e1:SetOperation(s.desop)
                effect::set_operation(d, e1, Some(operation(desop)));
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // e1:SetCountLimit(1)
                effect::set_count_limit(d, e1, 1, 0, 0, 0);
                // sg:GetFirst():RegisterEffect(e1)
                let first = group::get_first(d, &sg).nil();
                card::register_effect(d, first, e1, false);
            }
        }
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsHasEffect(66235877)
    let h = effect::get_handler(d, a.e).nil();
    card::is_has_effect(d, h, 66235877, None).is_empty()
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
