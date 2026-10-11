//! Gonogo
use super::prelude::*;

pub const ID: u32 = 59797187;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pos
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tg=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tg = card::get_battle_target(d, h);
    // if chk==0 then return tg and tg:IsRelateToBattle() end
    if a.chk == Some(0) {
        return match tg {
            Some(tg) => card::is_relate_to_battle(d, tg),
            None => false,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,tg,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, tg.map(CardSet::from_card), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h).nil();
    // if bc:IsRelateToBattle() and bc:IsFaceup() then
    if card::is_relate_to_battle(d, bc) && card::is_faceup(d, bc) {
        // Duel.ChangePosition(bc,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, bc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
