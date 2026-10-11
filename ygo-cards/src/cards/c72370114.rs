//! Geargiattacker
use super::prelude::*;

pub const ID: u32 = 72370114;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GEARGIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // turn set
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FLIP)
    effect::set_code(d, e2, EVENT_FLIP);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanTurnSet() and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::is_can_turn_set(d, c) && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET),0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,c,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Duel.ChangePosition(c,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, c, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GEARGIA)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GEARGIA as u16])
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler() (only used by the check below)
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,e:GetHandler())
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, c.into())
            && duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,0,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let ct = duel::get_matching_group_count(d, Some(&cfilter), tp, LOCATION_MZONE, 0, c.into());
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if ct>0 and #g>0 then
    if ct > 0 && !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local dg=g:Select(tp,1,ct,nil)
        let dg = group::select(d, &g, tp, 1, ct as u16, false, NO_EX).nil();
        // Duel.HintSelection(dg)
        duel::hint_selection(d, dg.clone(), true);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
