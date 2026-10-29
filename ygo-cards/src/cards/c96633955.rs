//! Swordsoul Supreme Sovereign - Chengying
use super::prelude::*;

pub const ID: u32 = 96633955;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro Summon
    card::enable_revive_limit(d, c);
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Gains 100 ATK/DEF for each banished card
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e1, effect_value(atkval));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e2, false);
    // Monsters your opponent controls lose 100 ATK/DEF for each banished card
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    effect::set_value_fn(d, e3, effect_value(val));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e4, false);
    // If this card would be destroyed by card effect, you can banish 1 card from your GY instead
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e5, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_target(d, e5, Some(cost_target(desreptg)));
    card::register_effect(d, c, e5, false);
    // Banish 1 card each from both your opponent's field and GY
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 0));
    effect::set_category(d, e6, CATEGORY_REMOVE);
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e6, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e6, EVENT_REMOVE);
    effect::set_range(d, e6, LOCATION_MZONE);
    effect::set_count_limit(d, e6, 1, ID, 0, 0);
    effect::set_condition(d, e6, Some(condition(remcon)));
    effect::set_target(d, e6, Some(cost_target(remtg)));
    effect::set_operation(d, e6, Some(operation(remop)));
    card::register_effect(d, c, e6, false);
}
fn atkval(d: &mut Duel, _e: EffectId) -> i64 {
    // Duel.GetFieldGroupCount(0,LOCATION_REMOVED,LOCATION_REMOVED)*100
    (duel::get_field_group_count(d, 0, LOCATION_REMOVED, LOCATION_REMOVED) * 100) as i64
}
fn val(d: &mut Duel, _e: EffectId) -> i64 {
    // Duel.GetFieldGroupCount(0,LOCATION_REMOVED,LOCATION_REMOVED)*(-100)
    (duel::get_field_group_count(d, 0, LOCATION_REMOVED, LOCATION_REMOVED) * -100) as i64
}
fn repfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && card::is_reason(d, c, REASON_EFFECT)
            && duel::is_existing_matching_card(d, Some(&repfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESREPLACE as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&repfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        return true;
    }
    false
}
fn remcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(function(c) return not c:IsType(TYPE_TOKEN) end,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| !card::is_type(d, c, TYPE_TOKEN), 1, NO_EX)
}
fn remtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let hp = effect::get_handler_player(d, a.e);
        return duel::is_player_affected_by_effect(d, hp, CARD_SPIRIT_ELIMINATION).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 2, 0, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
fn remop(d: &mut Duel, a: &EffArgs) {
    // local g1=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,nil)
    let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_GRAVE, NO_EX);
    // if #g1>0 and #g2>0 then
    if !g1.is_empty() && !g2.is_empty() {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local sg1=g1:Select(tp,1,1,nil)
        let mut sg1 = group::select(d, &g1, a.tp, 1, 1, false, NO_EX).nil();
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local sg2=g2:Select(tp,1,1,nil)
        let sg2 = group::select(d, &g2, a.tp, 1, 1, false, NO_EX).nil();
        // sg1:Merge(sg2)
        group::add_card(&mut sg1, &sg2);
        // Duel.HintSelection(sg1,true)
        duel::hint_selection(d, sg1.clone(), true);
        // Duel.Remove(sg1,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, sg1, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
