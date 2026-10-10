//! Mystic Swordsman LV6
use super::prelude::*;

pub const ID: u32 = 60482781;

pub const INFO: ScriptInfo = ScriptInfo {
    lv: Some((6, SET_MYSTIC_SWORDSMAN)),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // summon limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_LIMIT_SUMMON_PROC)
    effect::set_code(d, e2, EFFECT_LIMIT_SUMMON_PROC);
    // e2:SetCondition(s.sumcon)
    effect::set_condition(d, e2, Some(lua_fn(sumcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return e:GetHandler()==Duel.GetAttacker() and d and d:IsFacedown() and d:IsDefensePos()
    let c = effect::get_handler(d, a.e).nil();
    if duel::get_attacker(d) != Some(c) {
        return false;
    }
    match at {
        Some(dc) => card::is_facedown(d, dc) && card::is_defense_pos(d, dc),
        None => false,
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttackTarget(),1,0,0)
    let at = duel::get_attack_target(d).map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, at, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local d=Duel.GetAttackTarget()
    let dc = duel::get_attack_target(d).nil();
    // if d:IsRelateToBattle() then
    if card::is_relate_to_battle(d, dc) {
        // if d:GetOwner()~=tp and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if card::get_owner(d, dc) != a.tp && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            // Duel.Destroy(d,REASON_EFFECT,LOCATION_DECK)
            duel::destroy(d, CardSet::from_card(dc), REASON_EFFECT, Some(LOCATION_DECK), None);
        } else {
            // Duel.Destroy(d,REASON_EFFECT)
            duel::destroy(d, CardSet::from_card(dc), REASON_EFFECT, None, None);
        }
    }
}
/// `s.sumcon(e,c,minc)`
fn sumcon(_d: &mut Duel, a: Args) -> bool {
    // if not c then return true end
    // return false
    a.card(1).is_none()
}
