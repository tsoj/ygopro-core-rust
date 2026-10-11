//! Fiendish Golem
use super::prelude::*;

pub const ID: u32 = 24662957;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RED_DRAGON_ARCHFIEND, 50078509], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 monster with 2000 or more ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttackAbove(2000) and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_attack_above(d, c, 2000) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rdafilter(c)`
fn rdafilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsCode(CARD_RED_DRAGON_ARCHFIEND)
    //     or (c:IsType(TYPE_SYNCHRO) and c:IsMonster() and c:ListsCode(CARD_RED_DRAGON_ARCHFIEND)))
    card::is_faceup(d, c)
        && (card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND])
            || (card::is_type(d, c, TYPE_SYNCHRO) && card::is_monster(d, c) && card::lists_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND])))
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(50078509) and c:IsSSetable()
    card::is_code(d, c, &[50078509]) && card::is_s_setable(d, c, false, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.rmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && rmfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&rmfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    // e:SetLabel(Duel.GetMatchingGroupCount(s.rdafilter,tp,LOCATION_ONFIELD,0,nil))
    let n = duel::get_matching_group_count(d, Some(&rdafilter), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    effect::set_label(d, a.e, &[n as i64]);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local turn_count=Duel.GetTurnCount()
    let turn_count = duel::get_turn_count(d, None);
    // Banish it until the End Phase of the next turn
    // local ret_eff=aux.RemoveUntil(tc,nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp,
    //     function() return Duel.GetTurnCount()==turn_count+1 end,nil,2)
    let ret_eff = aux::remove_until(
        d,
        tc,
        None,
        REASON_EFFECT,
        PHASE_END,
        ID,
        a.e,
        a.tp,
        Some(aux::default_field_return()),
        Some(std::sync::Arc::new(move |d: &mut Duel, _g: &CardSet, _a: &EffArgs| duel::get_turn_count(d, None) == turn_count + 1)),
        None,
        Some(2),
        None,
        None,
    );
    // if ret_eff and e:GetLabel()>0 and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.setfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if ret_eff.is_some()
        && effect::get_label(d, a.e) > 0
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(setfilter)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // Set 1 "Fiendish Chain" from the Deck or GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.setfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(setfilter)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SSet(tp,g)
            duel::s_set(d, a.tp, g, None, true);
        }
    }
}
