//! Morphtronic Magnen Bar
use super::prelude::*;

pub const ID: u32 = 45593005;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(cona)));
    effect::set_target(d, e1, Some(cost_target(tga)));
    effect::set_operation(d, e1, Some(operation(opa)));
    card::register_effect(d, c, e1, false);
    // def
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    effect::set_condition(d, e2, Some(condition(cond)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttackPos()
    card::is_faceup(d, c) && card::is_attack_pos(d, c)
}
fn cona(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsDisabled() and e:GetHandler():IsAttackPos()
    //     and Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,0,e:GetHandler())==2
    //     and not Duel.IsExistingMatchingCard(Card.IsDefensePos,tp,LOCATION_MZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    if card::is_disabled(d, h) || !card::is_attack_pos(d, h) {
        return false;
    }
    if duel::get_matching_group_count(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, h.into()) != 2 {
        return false;
    }
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_defense_pos(d, c)), a.tp, LOCATION_MZONE, 0, 1, h.into())
}
/// `s.tga(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tga(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,c)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_MZONE, 0, c.into());
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_OATH, 0);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(ftarget)));
    let fid = card::get_field_id(d, c) as i64;
    effect::set_label(d, e1, &[fid]);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    true
}
/// `s.ftarget(e,c)`
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetLabel()~=c:GetFieldID()
    let label = effect::get_label(d, e);
    label != card::get_field_id(d, c) as i64
}
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsRelateToEffect(e)
    card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e)
}
/// `s.opa(e,tp,eg,ep,ev,re,r,rp)`
fn opa(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local sg=g:Filter(s.filter,nil,e)
    let sg = group::filter(d, &g, &|d, c| filter(d, c, a.e), NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local atk=sg:GetSum(Card.GetAttack)
    let atk = group::get_sum(d, &sg, &|d, c| card::get_attack(d, c) as i64);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, atk);
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsDefensePos()
    let h = effect::get_handler(d, a.e).nil();
    card::is_defense_pos(d, h)
}
