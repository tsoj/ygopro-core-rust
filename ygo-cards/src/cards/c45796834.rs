//! Maha Vailo, Light of the Heavens
//! Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 45796834;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gains effects based on number of equips cards equipped to it
    // 1+: Gains 1000 ATK for each equip card equipped to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_card_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // 2+: Opponent cannot activate monster effects during battle phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCondition(s.moncond)
    effect::set_condition(d, e2, Some(condition(moncond)));
    // e2:SetValue(s.monlimit)
    effect::set_value_fn(d, e2, lua_fn(monlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // 3+: Negate opponent's card/effect that targets this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e3, CATEGORY_DISABLE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.ngcon)
    effect::set_condition(d, e3, Some(condition(ngcon)));
    // e3:SetCost(s.ngcost)
    effect::set_cost(d, e3, Some(cost_target(ngcost)));
    // e3:SetTarget(s.ngtg)
    effect::set_target(d, e3, Some(cost_target(ngtg)));
    // e3:SetOperation(s.ngop)
    effect::set_operation(d, e3, Some(operation(ngop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // 4+: Opponent cannot activate cards/effects during battle phase
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
    // e4:SetTargetRange(0,1)
    effect::set_target_range(d, e4, 0, 1);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.actcon)
    effect::set_condition(d, e4, Some(condition(actcon)));
    // e4:SetValue(1)
    effect::set_value(d, e4, 1);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // 5+: Double battle damage inflicted by this card
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e5, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e5:SetCondition(s.damcon)
    effect::set_condition(d, e5, Some(condition(damcon)));
    // e5:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e5, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.val(e,c)`: gains 1000 ATK for each equip card equipped to it
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetEquipCount()*1000
    card::get_equip_count(d, c) as i64 * 1000
}
/// `s.moncond(e)`: if 2+ equips, opponent cannot activate monster effects during battle phase
fn moncond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.IsBattlePhase() and c:GetEquipCount()>=2
    duel::is_battle_phase(d, None) && card::get_equip_count(d, c) >= 2
}
/// `s.monlimit(e,re,tp)`
fn monlimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsMonsterEffect()
    let re = a.e(1);
    effect::is_monster_effect(d, re)
}
/// `s.ngcon(e,tp,eg,ep,ev,re,r,rp)`: if targeted by opponent's card/effect and this card is equipped with 3+ equips
fn ngcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if rp==tp or c:IsStatus(STATUS_BATTLE_DESTROYED) or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then
    if a.rp == a.tp || card::is_status(d, c, STATUS_BATTLE_DESTROYED) || !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        // return false
        return false;
    }
    // local gp=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let gp = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // return c:GetEquipCount()>=3 and gp and gp:IsContains(c) and Duel.IsChainDisablable(ev)
    card::get_equip_count(d, c) >= 3 && gp.is_some_and(|gp| group::is_contains(&gp, c)) && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.ngfilter(c)`: check for an equip card
fn ngfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EQUIP) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EQUIP) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.ngcost(e,tp,eg,ep,ev,re,r,rp,chk)`: send 1 equip card to GY as cost
fn ngcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ngfilter,tp,LOCATION_SZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&ngfilter), tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.ngfilter,tp,LOCATION_SZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&ngfilter), tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.ngtg(e,tp,eg,ep,ev,re,r,rp,chk)`: activation legality
fn ngtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.ngop(e,tp,eg,ep,ev,re,r,rp)`: if 3+ equips, negate opponent's card/effect that targets this card
fn ngop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
/// `s.actcon(e)`: if 4+ equips, opponent cannot activate cards/effects during battle phase
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ph=Duel.GetCurrentPhase()
    let _ph = duel::get_current_phase(d);
    // return c:GetEquipCount()>=4 and Duel.IsBattlePhase()
    card::get_equip_count(d, c) >= 4 && duel::is_battle_phase(d, None)
}
/// `s.damcon(e)`: if 5+ equips, this card inflicts double battle damage
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetEquipCount()>=5
    card::get_equip_count(d, c) >= 5
}
