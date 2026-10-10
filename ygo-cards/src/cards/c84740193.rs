//! Buster Rancher
use super::prelude::*;

pub const ID: u32 = 84740193;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsAttackBelow,1000),s.eqlimit)
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_attack_below(d, x, 1000))),
        Some(effect_card_fn(|d, _e, x| eqlimit(d, x))),
        None,
        None,
        None,
        None,
        None,
    );
    // Atk,def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetValue(2500)
    effect::set_value(d, e2, 2500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, c: CardId) -> bool {
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) or c:IsAttackBelow(1000)
    duel::is_phase(d, PHASE_DAMAGE_CAL) || card::is_attack_below(d, c, 1000)
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h).nil();
    // if not ec:IsRelateToBattle() then return end
    if !card::is_relate_to_battle(d, ec) {
        return false;
    }
    // local bc=ec:GetBattleTarget()
    let bc = card::get_battle_target(d, ec);
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and bc
    //     and ((bc:IsAttackPos() and bc:IsAttackAbove(2500)) or (bc:IsDefensePos() and bc:IsDefenseAbove(2500)))
    if !duel::is_phase(d, PHASE_DAMAGE_CAL) {
        return false;
    }
    let Some(bc) = bc else {
        return false;
    };
    (card::is_attack_pos(d, bc) && card::is_attack_above(d, bc, 2500)) || (card::is_defense_pos(d, bc) && card::is_defense_above(d, bc, 2500))
}
