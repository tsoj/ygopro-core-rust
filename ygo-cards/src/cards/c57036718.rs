//! Zubaba Buster
use super::prelude::*;

pub const ID: u32 = 57036718;

pub const INFO: ScriptInfo = ScriptInfo { ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
    let c = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) != 0
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup()
    card::is_faceup(d, c)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if group::get_count(&g) == 0 {
        return true;
    }
    // local tg=g:GetMinGroup(Card.GetAttack)
    let tg = group::get_min_group(d, &g, &|d, c| card::get_attack(d, c) as i64).nil().0;
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg), 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // local tg=g:GetMinGroup(Card.GetAttack)
        let tg = group::get_min_group(d, &g, &|d, c| card::get_attack(d, c) as i64).nil().0;
        // if #tg>1 then
        if group::get_count(&tg) > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local sg=tg:Select(tp,1,1,nil)
            let sg = group::select(d, &tg, tp, 1, 1, false, NO_EX).nil();
            // Duel.Destroy(sg,REASON_EFFECT)
            duel::destroy(d, sg, REASON_EFFECT, None, None);
        } else {
            // else Duel.Destroy(tg,REASON_EFFECT) end
            duel::destroy(d, tg, REASON_EFFECT, None, None);
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-800)
        effect::set_value(d, e1, -800);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
