//! Empress of Prophecy
use super::prelude::*;

pub const ID: u32 = 770365;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPELLBOOK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER),5,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_SPELLCASTER, sc, st, Some(tp)))), Some(5), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // atklimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // excavate and destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetOverlayCount(c:GetControler(),1,0)*300
    let p = card::get_controler(d, c);
    duel::get_overlay_count(d, p, 1, 0, None) as i64 * 300
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=5 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= 5;
    }
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPELLBOOK)
    card::is_set_card(d, c, &[SET_SPELLBOOK as u16])
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.ConfirmDecktop(tp,5)
    duel::confirm_decktop(d, tp, 5);
    // local g=Duel.GetDecktopGroup(tp,5)
    let g = duel::get_decktop_group(d, tp, 5);
    // local ct=g:FilterCount(s.filter,nil)
    let ct = group::filter_count(d, &g, &filter, NO_EX);
    // local sg=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if ct>0 and #sg>0 then
    if ct > 0 && !sg.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local dg=sg:Select(tp,1,ct,nil)
        let dg = group::select(d, &sg, tp, 1, ct as u16, false, NO_EX).nil();
        // Duel.HintSelection(dg)
        duel::hint_selection(d, dg.clone(), true);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
        // Duel.BreakEffect()
        duel::break_effect(d);
    }
    // Duel.SortDecktop(tp,tp,5)
    duel::sort_decktop(d, tp, tp, 5);
}
