//! Magical Dimension
use super::prelude::*;

pub const ID: u32 = 28553439;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.rfilter(c,tp)`
fn rfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReleasableByEffect() and Duel.GetMZoneCount(tp,c)>0
    card::is_releasable_by_effect(d, c, None, None) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.target`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.rfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && rfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rfilter,tp,LOCATION_MZONE,0,1,nil,tp)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local g=Duel.SelectTarget(tp,s.rfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| rfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,g)
    let dg = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, Exclude::Group(&g));
    // Duel.SetOperationInfo(0,CATEGORY_RELEASE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, Some(g.clone()), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,dg,1,0,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, 0, 0);
    true
}
/// `s.activate`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsControler(tp) and Duel.Release(tc,REASON_EFFECT)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, tc, e)
        && card::is_controler(d, tc, tp)
        && duel::release(d, tc, REASON_EFFECT, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 and Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)>0 then
        if !sg.is_empty() && duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // local dg=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
            let dg = duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE);
            // if #dg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
            if !dg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
                // local des=dg:Select(tp,1,1,nil)
                let des = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
                // Duel.HintSelection(des,true)
                duel::hint_selection(d, des.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Destroy(des,REASON_EFFECT)
                duel::destroy(d, des, REASON_EFFECT, None, None);
            }
        }
    }
}
