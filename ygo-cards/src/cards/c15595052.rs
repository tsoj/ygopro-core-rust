//! Disciple of the Forbidden Spell
use super::prelude::*;

pub const ID: u32 = 15595052;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.ancop)
    effect::set_operation(d, e1, Some(operation(ancop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ancop(e,tp,eg,ep,ev,re,r,rp)`
fn ancop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=Duel.GetMatchingGroupCount(Card.IsCode,tp,LOCATION_GRAVE,0,nil,id)
    let ct = duel::get_matching_group_count(d, Some(&|d, x| card::is_code(d, x, &[ID])), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if ct>0 and c:IsFaceup() and c:IsRelateToEffect(e) then
    if ct > 0 && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTRIBUTE as u64);
        // local att=Duel.AnnounceAttribute(tp,ct,ATTRIBUTE_ALL)
        let att = duel::announce_attribute(d, a.tp, ct as u8, ATTRIBUTE_ALL);
        // e:GetHandler():SetHint(CHINT_ATTRIBUTE,att)
        card::set_hint(d, c, CHINT_ATTRIBUTE as u8, att as u64);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e1, CATEGORY_DESTROY);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCode(EVENT_BATTLE_START)
        effect::set_code(d, e1, EVENT_BATTLE_START);
        // e1:SetTarget(s.destg)
        effect::set_target(d, e1, Some(cost_target(destg)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // e1:SetLabel(att)
        effect::set_label(d, e1, &[att as i64]);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d);
    // if chk==0 then return c==Duel.GetAttacker() and bc and bc:IsFaceup() and bc:IsAttribute(e:GetLabel()) end
    if a.chk == Some(0) {
        let label = effect::get_label(d, a.e) as u32;
        let attacker = duel::get_attacker(d);
        return attacker == Some(c) && bc.is_some_and(|bc| card::is_faceup(d, bc) && card::is_attribute(d, bc, label));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, _a: &EffArgs) {
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d);
    // if bc and bc:IsRelateToBattle() and bc:IsFaceup() then
    if let Some(bc) = bc {
        if card::is_relate_to_battle(d, bc) && card::is_faceup(d, bc) {
            // Duel.Destroy(bc,REASON_EFFECT)
            duel::destroy(d, CardSet::from_card(bc), REASON_EFFECT, None, None);
        }
    }
}
