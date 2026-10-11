//! Pegasus Wing
use super::prelude::*;

pub const ID: u32 = 72083436;
pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_UNION], listed_series: &[SET_VALKYRIE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsAbleToEnterBP() and Duel.IsExistingMatchingCard(Card.IsType,tp,LOCATION_GRAVE,0,1,nil,TYPE_UNION)
    duel::is_able_to_enter_bp(d)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_type(d, x, TYPE_UNION)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VALKYRIE) and c:IsFaceup() and c:CanAttack() and not c:IsHasEffect(EFFECT_DIRECT_ATTACK)
    card::is_set_card(d, c, &[SET_VALKYRIE as u16])
        && card::is_faceup(d, c)
        && card::can_attack(d, c)
        && card::is_has_effect(d, c, EFFECT_DIRECT_ATTACK, None).is_empty()
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local ct=Duel.GetMatchingGroupCount(s.tgfilter,tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,ct,nil)
    let _ = duel::select_target(d, a.tp, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, ct as u16, false, NO_EX);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        let h = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
        effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
        // e2:SetCondition(function(e) ... end)
        effect::set_condition(d, e2, Some(lua_fn(direct_damage_cond)));
        // e2:SetValue(aux.ChangeBattleDamage(1,HALF_DAMAGE))
        effect::set_value_fn(d, e2, aux::change_battle_damage(1, HALF_DAMAGE as i64));
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
// function(e) return Duel.GetAttackTarget()==nil and e:GetHandler():GetEffectCount(EFFECT_DIRECT_ATTACK)<2
//   and Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_MZONE)>0 end
fn direct_damage_cond(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    if duel::get_attack_target(d).is_some() {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    if card::get_effect_count(d, h, EFFECT_DIRECT_ATTACK) >= 2 {
        return false;
    }
    let p = effect::get_handler_player(d, e);
    duel::get_field_group_count(d, p, 0, LOCATION_MZONE) > 0
}
