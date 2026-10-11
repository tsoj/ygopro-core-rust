//! Blossom Bombardment
//! ブロッサム・ボンバー
use super::prelude::*;

pub const ID: u32 = 13438207;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When a Plant-Type monster you control destroys an opponent's monster by battle and sends it to the Graveyard: Inflict damage to your opponent equal to the destroyed monster's ATK in the Graveyard
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    // local bc=tc:GetBattleTarget()
    let bc = card::get_battle_target(d, tc);
    // return tc:IsRelateToBattle() and tc:IsStatus(STATUS_OPPO_BATTLE) and tc:IsControler(tp) and tc:IsRace(RACE_PLANT)
    //     and bc:IsLocation(LOCATION_GRAVE) and bc:IsMonster() and bc:IsReason(REASON_BATTLE)
    if !card::is_relate_to_battle(d, tc)
        || !card::is_status(d, tc, STATUS_OPPO_BATTLE)
        || !card::is_controler(d, tc, a.tp)
        || !card::is_race(d, tc, RACE_PLANT)
    {
        return false;
    }
    let bc = bc.nil();
    card::is_location(d, bc, LOCATION_GRAVE) && card::is_monster(d, bc) && card::is_reason(d, bc, REASON_BATTLE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local atk=eg:GetFirst():GetBattleTarget():GetBaseAttack()
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    let bc = card::get_battle_target(d, tc).nil();
    let atk = card::get_base_attack(d, bc);
    // if chk==0 then return atk>0 end
    if a.chk == Some(0) {
        return atk > 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(atk)
    duel::set_target_param(d, atk as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,atk)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
