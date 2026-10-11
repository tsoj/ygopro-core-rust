//! Lucky Punch
//! ラッキーパンチ
use super::prelude::*;

pub const ID: u32 = 36378044;
pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Toss a coin and either draw 3 cards or destroy this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW+CATEGORY_DESTROY+CATEGORY_COIN)
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_DESTROY + CATEGORY_COIN);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Lose 6000 LP when it leaves the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // return at and at:IsControler(1-tp)
    match at {
        Some(at) => card::is_controler(d, at, 1 - a.tp),
        None => false,
    }
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 3);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local r1,r2,r3=Duel.TossCoin(tp,3)
    let r = duel::toss_coin(d, a.tp, 3);
    // if Duel.CountHeads(r1,r2,r3)==3 then
    if duel::count_heads(&r) == 3 {
        // Duel.Draw(tp,3,REASON_EFFECT)
        duel::draw(d, a.tp, 3, REASON_EFFECT);
    // elseif Duel.CountTails(r1,r2,r3)==3 then
    } else if duel::count_tails(&r) == 3 {
        // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_EFFECT, None, None);
    }
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEUP)
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_previous_position(d, c, POS_FACEUP as u32)
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local lp=Duel.GetLP(tp)
    let lp = duel::get_lp(d, a.tp);
    // Duel.SetLP(tp,lp-6000)
    duel::set_lp(d, a.tp, lp - 6000);
}
