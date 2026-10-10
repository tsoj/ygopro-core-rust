//! Ally of Justice Catastor
//! Ａ・Ｏ・Ｊ カタストル
use super::prelude::*;

pub const ID: u32 = 26593852;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Destroy a non-DARK monster that battles with this card
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
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // if chk==0 then return bc and bc:IsFaceup() and bc:IsAttributeExcept(ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return match bc {
            Some(bc) => card::is_faceup(d, bc) && card::is_attribute_except(d, bc, ATTRIBUTE_DARK, None, 0, None),
            None => false,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h).nil();
    // if bc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, bc) {
        // Duel.Destroy(bc,REASON_EFFECT)
        duel::destroy(d, bc, REASON_EFFECT, None, None);
    }
}
