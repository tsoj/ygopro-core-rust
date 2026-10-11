//! U.A. Powered Jersey
use super::prelude::*;

pub const ID: u32 = 35884610;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_UA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip only to a "U.A." monster
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_UA))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_set_card(d, x, &[SET_UA as u16]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // It gains 1000 ATK and DEF
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1a, EFFECT_TYPE_EQUIP);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetValue(1000)
    effect::set_value(d, e1a, 1000);
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Also if it battles an opponent's monster, any battle damage it inflicts to your opponent is doubled
    // local e1c=Effect.CreateEffect(c)
    let e1c = effect::create_effect(d, c);
    // e1c:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1c, EFFECT_TYPE_EQUIP);
    // e1c:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1c, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1c:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e1c, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e1c:SetCondition(function(e) local bc=e:GetHandler():GetEquipTarget():GetBattleTarget() return bc and bc:IsControler(1-e:GetHandlerPlayer()) end)
    effect::set_condition(d, e1c, Some(condition(dmg_cond)));
    // e1c:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e1c, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e1c)
    card::register_effect(d, c, e1c, false);
    // Make the equipped monster able to make a second attack during this Battle Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.secondatkcon)
    effect::set_condition(d, e2, Some(condition(secondatkcon)));
    // e2:SetOperation(s.secondatkop)
    effect::set_operation(d, e2, Some(operation(secondatkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish the equipped monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_STANDBY);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) and e:GetHandler():GetEquipTarget() end)
    effect::set_condition(d, e3, Some(condition(rm_cond)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Return this card to the hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCondition(s.thcon)
    effect::set_condition(d, e4, Some(condition(thcon)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e) local bc=e:GetHandler():GetEquipTarget():GetBattleTarget() return bc and bc:IsControler(1-e:GetHandlerPlayer()) end`
fn dmg_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetEquipTarget():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let et = card::get_equip_target(d, h).nil();
    let bc = card::get_battle_target(d, et);
    // return bc and bc:IsControler(1-e:GetHandlerPlayer())
    match bc {
        Some(bc) => card::is_controler(d, bc, 1 - effect::get_handler_player(d, a.e)),
        None => false,
    }
}
/// `s.secondatkcon(e,tp,eg,ep,ev,re,r,rp)`
fn secondatkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let Some(ec) = card::get_equip_target(d, h) else {
        // return ec and ...
        return false;
    };
    // return ec and ec:IsRelateToBattle() and ec:CanChainAttack() and ec:GetBattleTarget()
    card::is_relate_to_battle(d, ec) && card::can_chain_attack(d, ec, None, false) && card::get_battle_target(d, ec).is_some()
}
/// `s.secondatkop(e,tp,eg,ep,ev,re,r,rp)`
fn secondatkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=e:GetHandler():GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // if ec then
    if let Some(ec) = ec {
        // It can make a second attack during this Battle Phase
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
        // ec:RegisterEffect(e1)
        card::register_effect(d, ec, e1, false);
    }
}
/// `function(e,tp) return Duel.IsTurnPlayer(tp) and e:GetHandler():GetEquipTarget() end`
fn rm_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and e:GetHandler():GetEquipTarget()
    if !duel::is_turn_player(d, a.tp) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, h).is_some()
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,e:GetHandler():GetEquipTarget(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, ec.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    // if ec then
    if let Some(ec) = card::get_equip_target(d, h) {
        // Duel.Remove(ec,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, ec, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetPreviousEquipTarget()
    let ec = card::get_previous_equip_target(d, c);
    // return c:IsReason(REASON_LOST_TARGET) and ec and ec:IsLocation(LOCATION_HAND)
    if !card::is_reason(d, c, REASON_LOST_TARGET) {
        return false;
    }
    match ec {
        Some(ec) => card::is_location(d, ec, LOCATION_HAND),
        None => false,
    }
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
