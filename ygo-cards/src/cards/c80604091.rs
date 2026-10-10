//! Ultimate Offering
use super::prelude::*;

pub const ID: u32 = 80604091;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // instant
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SUMMON + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_cost(d, e2, Some(cost_target(cost)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(activate)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tn=Duel.GetTurnPlayer()
    let tn = duel::get_turn_player(d);
    // return (tn==tp and Duel.IsMainPhase()) or (tn~=tp and Duel.IsBattlePhase())
    (tn == a.tp && duel::is_main_phase(d, None)) || (tn != a.tp && duel::is_battle_phase(d, None))
}
fn summon_filter(d: &mut Duel, c: CardId) -> bool {
    // Card.CanSummonOrSet with the arguments (true, nil)
    card::can_summon_or_set(d, c, true, None, 0, None)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckLPCost(tp,500) end
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, a.tp, 500);
    }
    // if e:GetHandler():GetFlagEffect(id)==0 then
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID) == 0 {
        // e:GetHandler():RegisterFlagEffect(id,RESET_CHAIN,0,1,Duel.GetMatchingGroupCount(Card.CanSummonOrSet,tp,LOCATION_HAND|LOCATION_MZONE,0,nil,true,nil))
        let label = duel::get_matching_group_count(d, Some(&summon_filter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX) as i64;
        card::register_flag_effect(d, h, ID, RESET_CHAIN, 0, 1, label, 0);
    }
    // Duel.PayLPCost(tp,500)
    duel::pay_lp_cost(d, a.tp, 500);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // if e:GetHandler():GetFlagEffect(id)==0 then
        let h = effect::get_handler(d, a.e).nil();
        if card::get_flag_effect(d, h, ID) == 0 {
            // return Duel.GetMatchingGroupCount(Card.CanSummonOrSet,tp,LOCATION_HAND|LOCATION_MZONE,0,nil,true,nil)>0
            return duel::get_matching_group_count(d, Some(&summon_filter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX) > 0;
        } else {
            // else return e:GetHandler():GetFlagEffectLabel(id)>0 end
            return card::get_flag_effect_label(d, h, ID).first().copied().nil() > 0;
        }
    }
    // e:GetHandler():SetFlagEffectLabel(id,e:GetHandler():GetFlagEffectLabel(id)-1)
    let h = effect::get_handler(d, a.e).nil();
    let label = card::get_flag_effect_label(d, h, ID).first().copied().nil();
    card::set_flag_effect_label(d, h, ID, (label - 1) as u32);
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,Card.CanSummonOrSet,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,true,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&summon_filter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SummonOrSet(tp,tc,true,nil)
        duel::summon_or_set(d, a.tp, tc, true, None, 0, None);
    }
}
