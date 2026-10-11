//! Aqua Armor Ninja
use super::prelude::*;

pub const ID: u32 = 96427353;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NINJA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NINJA) and c:IsMonster()
    card::is_set_card(d, c, &[SET_NINJA as u16]) && card::is_monster(d, c)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.GetAttackTarget()==nil
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, 1 - a.tp)
        && duel::get_attack_target(d).is_none()
        && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, 1, h.into())
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tg=Duel.GetAttacker()
    let tg = duel::get_attacker(d);
    // if chkc then return chkc==tg end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tg;
    }
    // if chk==0 then return tg:IsOnField() and tg:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        let tg = tg.nil();
        return card::is_on_field(d, tg) && card::is_can_be_effect_target(d, tg, Some(a.e));
    }
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, CardSet::from_card(tg.nil()));
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // Duel.NegateAttack()
    duel::negate_attack(d);
}
