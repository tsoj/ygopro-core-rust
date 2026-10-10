//! Resh Shaddoll Incarnation
use super::prelude::*;

pub const ID: u32 = 23912837;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHADDOLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Shaddoll" monster from your GY in face-up or face-down Defense Position
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_POSITION);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e2, Some(cost_target(effcost)));
    effect::set_target(d, e2, Some(cost_target(efftg)));
    effect::set_operation(d, e2, Some(operation(effop)));
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_DEFENSE)
    card::is_set_card(d, c, &[SET_SHADDOLL as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_DEFENSE)>0 and tc:IsFacedown() then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_DEFENSE as u8, None) > 0
        && card::is_facedown(d, tc)
    {
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, tc);
    }
}
/// `s.effcostfilter(c)`
fn effcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() and Duel.IsExistingMatchingCard(s.effcostfilter,tp,LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::is_existing_matching_card(d, Some(&effcostfilter), a.tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.effcostfilter,tp,LOCATION_GRAVE,0,1,1,c)
    let mut g = duel::select_matching_card(d, a.tp, Some(&effcostfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(g+c,POS_FACEUP,REASON_COST)
    group::add_card(&mut g, &CardSet::from_card(c));
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=Duel.IsExistingMatchingCard(aux.AND(Card.IsFacedown,Card.IsCanChangePosition),tp,LOCATION_MZONE,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c) && card::is_can_change_position(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(Card.IsCanTurnSet,tp,LOCATION_MZONE,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&card::is_can_turn_set), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    let op = op.map_or(0, |v| v as i64);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_POSITION)
        effect::set_category(d, a.e, CATEGORY_POSITION);
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,tp,POS_FACEUP_DEFENSE)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, a.tp, POS_FACEUP_DEFENSE as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_POSITION + CATEGORY_SET);
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,tp,POS_FACEDOWN_DEFENSE)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Change 1 face-down monster you control to face-up Defense Position
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.AND(Card.IsFacedown,Card.IsCanChangePosition),tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_facedown(d, c) && card::is_can_change_position(d, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.ChangePosition(g,POS_FACEUP_DEFENSE)
            duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, None, None, None, false);
        }
    // elseif op==2 then
    } else if op == 2 {
        // Change 1 face-up monster you control to face-down Defense Position
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsCanTurnSet,tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&card::is_can_turn_set), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
