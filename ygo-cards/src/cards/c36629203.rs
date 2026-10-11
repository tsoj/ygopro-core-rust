//! Ally of Justice Core Destroyer
//! A・O・J コアデストロイ
use super::prelude::*;

pub const ID: u32 = 36629203;

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
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The monster that is not `c` in the battle (`local tc=Duel.GetAttacker(); if tc==c then tc=Duel.GetAttackTarget() end`).
fn battle_other(d: &mut Duel, c: CardId) -> Option<CardId> {
    let tc = duel::get_attacker(d);
    if tc == Some(c) {
        duel::get_attack_target(d)
    } else {
        tc
    }
}
/// `s.destg(e,tp,...,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetAttacker()
    // if tc==c then tc=Duel.GetAttackTarget() end
    let tc = battle_other(d, c);
    // if chk==0 then return tc and tc:IsFaceup() and tc:IsAttribute(ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        return match tc {
            Some(tc) => card::is_faceup(d, tc) && card::is_attribute(d, tc, ATTRIBUTE_LIGHT),
            None => false,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetAttacker()
    // if tc==c then tc=Duel.GetAttackTarget() end
    let tc = battle_other(d, c);
    // if tc:IsRelateToBattle() then Duel.Destroy(tc,REASON_EFFECT) end
    let tc = tc.nil();
    if card::is_relate_to_battle(d, tc) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
