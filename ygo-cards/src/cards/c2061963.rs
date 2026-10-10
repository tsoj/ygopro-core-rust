//! Number 104: Masquerade
use super::prelude::*;

pub const ID: u32 = 2061963;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(104), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz.AddProcedure(c,nil,4,3)
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // negate activate
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e2, CATEGORY_DECKDES);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.decktg)
    effect::set_target(d, e2, Some(cost_target(decktg)));
    // e2:SetOperation(s.deckop)
    effect::set_operation(d, e2, Some(operation(deckop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let _ph = duel::get_current_phase(d);
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and ep~=tp
    //     and Duel.IsBattlePhase() and re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
        && a.ep != a.tp
        && duel::is_battle_phase(d, None)
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,800)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 800);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) then
    if duel::negate_activation(d, a.ev as u8) {
        // Duel.Damage(1-tp,800,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, 800, REASON_EFFECT, false, None);
    }
}
/// `s.decktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn decktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, 1 - a.tp, 1);
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.deckop(e,tp,eg,ep,ev,re,r,rp)`
fn deckop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.DiscardDeck(p,d,REASON_EFFECT)
    duel::discard_deck(d, info[0].int() as u8, info[1].int() as u16, REASON_EFFECT);
}
