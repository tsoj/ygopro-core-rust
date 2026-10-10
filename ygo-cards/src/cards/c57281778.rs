//! Ryu Kokki
use super::prelude::*;

pub const ID: u32 = 57281778;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local t=Duel.GetAttackTarget()
    let mut t = duel::get_attack_target(d);
    // if ev==1 then t=Duel.GetAttacker() end
    if a.ev == 1 {
        t = duel::get_attacker(d);
    }
    // e:SetLabelObject(t)
    effect::set_label_object(d, a.e, t.map_or(Value::Nil, Value::Card));
    // return t and t:IsRace(RACE_SPELLCASTER|RACE_WARRIOR)
    match t {
        Some(t) => card::is_race(d, t, RACE_SPELLCASTER | RACE_WARRIOR),
        None => false,
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetLabelObject():IsRelateToBattle() end
    if a.chk == Some(0) {
        let tc = effect::get_label_object(d, a.e).as_card().nil();
        return card::is_relate_to_battle(d, tc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetLabelObject(),1,0,0)
    let tc = effect::get_label_object(d, a.e).as_card();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, tc) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
