//! Trias Hierarchia
use super::prelude::*;

pub const ID: u32 = 26866984;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from hand or GY, then you can apply extra effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,...)`: `return Duel.IsMainPhase()`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.spcheck(sg,tp)`
fn spcheck(d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0
    (duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0, false)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.CheckReleaseGroupCost(tp,Card.IsRace,1,false,s.spcheck,nil,RACE_FAIRY)
        return aux::check_release_group_cost(d, a.tp, Some(&|d, c| card::is_race(d, c, RACE_FAIRY)), 1, 1, false, Some(&spcheck), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsRace,1,3,false,s.spcheck,nil,RACE_FAIRY)
    let g = aux::select_release_group_cost(d, a.tp, Some(&|d, c| card::is_race(d, c, RACE_FAIRY)), 1, 3, false, Some(&spcheck), NO_EX);
    // local ct=Duel.Release(g,REASON_COST)
    let ct = duel::release(d, g, REASON_COST, None);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // if ct>=2 then Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_ONFIELD) end
    if ct >= 2 {
        duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    }
    // if ct==3 then Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,1,tp,2) end
    if ct == 3 {
        duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, a.tp, 2);
    }
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Banish it if it leaves the field
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3300);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        card::register_effect(d, c, e1, true);
        // local ct=e:GetLabel()
        let ct = effect::get_label(d, a.e);
        // if ct==1 then return end
        if ct == 1 {
            return;
        }
        // If you Tributed 2+ monsters, you can destroy 1 card the opponent controls
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
        let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
        // if #g>0 and ct>=2 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !g.is_empty() && ct >= 2 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(sg,true)
            duel::hint_selection(d, sg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(sg,REASON_EFFECT)
            duel::destroy(d, sg, REASON_EFFECT, None, None);
        }
        // If you Tributed 3 monsters, you can draw 2 cards
        // if ct==3 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if ct == 3 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,2,REASON_EFFECT)
            duel::draw(d, a.tp, 2, REASON_EFFECT);
        }
    }
}
