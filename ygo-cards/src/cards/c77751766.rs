//! Summer Schoolwork Successful!
use super::prelude::*;

pub const ID: u32 = 77751766;

const COUNTER_SCHOOLWORK: u16 = 0x213;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SCHOOLWORK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SCHOOLWORK)
    card::enable_counter_permit(d, c, COUNTER_SCHOOLWORK, None, None);
    // Activate this card by placing 5 Success Counters on it
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_target(d, e1, Some(cost_target(actg)));
    card::register_effect(d, c, e1, false);
    // Remove 1 Success Counter from this card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_RECOVER + CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(cond2)));
    effect::set_target(d, e2, Some(cost_target(rcttg)));
    effect::set_operation(d, e2, Some(operation(rctop)));
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_condition(d, e3, Some(condition(cond3)));
    card::register_effect(d, c, e3, false);
}
fn actg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsCanAddCounter(tp,COUNTER_SCHOOLWORK,5,c) end
    if a.chk == Some(0) {
        return duel::is_can_add_counter(d, a.tp, Some((COUNTER_SCHOOLWORK, 5, c)));
    }
    // c:AddCounter(COUNTER_SCHOOLWORK,5)
    card::add_counter(d, c, COUNTER_SCHOOLWORK, 5, false);
    true
}
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:IsSpellTrapEffect() and eg:IsExists(Card.IsSummonLocation,1,nil,LOCATION_EXTRA)
    let Some(re) = a.re else { return false };
    effect::is_spell_trap_effect(d, re) && {
        let eg = a.eg.clone().nil();
        group::is_exists(d, &eg, &|d, c| card::is_summon_location(d, c, LOCATION_EXTRA), 1, NO_EX)
    }
}
fn cond3(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:IsSpellTrapEffect() and eg:IsExists(Card.IsPreviousLocation,1,nil,LOCATION_DECK)
    let Some(re) = a.re else { return false };
    effect::is_spell_trap_effect(d, re) && {
        let eg = a.eg.clone().nil();
        group::is_exists(d, &eg, &|d, c| card::is_previous_location(d, c, LOCATION_DECK), 1, NO_EX)
    }
}
fn rcttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,4000)
    duel::set_possible_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 4000);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SCHOOLWORK) and c:IsTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_SCHOOLWORK as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
fn rctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:RemoveCounter(tp,COUNTER_SCHOOLWORK,1,REASON_EFFECT) and not c:HasCounter(COUNTER_SCHOOLWORK)) then return end
    if !(card::is_relate_to_effect(d, c, a.e)
        && card::remove_counter(d, c, tp, COUNTER_SCHOOLWORK, 1, REASON_EFFECT)
        && !card::has_counter(d, c, COUNTER_SCHOOLWORK))
    {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.Destroy(c,REASON_EFFECT)==0 or Duel.Recover(tp,4000,REASON_EFFECT)==0 then return end
    if duel::destroy(d, c, REASON_EFFECT, None, None) == 0 || duel::recover(d, tp, 4000, REASON_EFFECT, false, None) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.setfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| setfilter(d, c))),
        tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else { return };
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.SSet(tp,sc)>0 and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<=1 then
    if duel::s_set(d, tp, sc, None, true) > 0 && duel::get_field_group_count(d, tp, LOCATION_DECK, 0) <= 1 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Win(tp,WIN_REASON_SUMMER_SCHOOLWORK)
        duel::win(d, tp, WIN_REASON_SUMMER_SCHOOLWORK);
    }
}
