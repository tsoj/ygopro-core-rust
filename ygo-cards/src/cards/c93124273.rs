//! Samurai Cavalry of Reptier
use super::prelude::*;

pub const ID: u32 = 93124273;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let tg = duel::get_attack_target(d);
    // if chk ==0 then return Duel.GetAttacker()==e:GetHandler()
    //     and d and d:IsFaceup() and not d:IsType(TYPE_PENDULUM) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        if duel::get_attacker(d) != Some(h) {
            return false;
        }
        return match tg {
            Some(t) => card::is_faceup(d, t) && !card::is_type(d, t, TYPE_PENDULUM),
            None => false,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,d,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tg.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local d=Duel.GetAttackTarget()
    let tg = duel::get_attack_target(d).nil();
    // if d:IsRelateToBattle() then
    if card::is_relate_to_battle(d, tg) {
        // Duel.Destroy(d,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
