//! Storm Caller
use super::prelude::*;

pub const ID: u32 = 15935204;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.dmcon)
    effect::set_condition(d, e2, Some(condition(dmcon)));
    // e2:SetTarget(s.dmtg)
    effect::set_target(d, e2, Some(cost_target(dmtg)));
    // e2:SetOperation(s.dmop)
    effect::set_operation(d, e2, Some(operation(dmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local a=Duel.GetAttacker()
    let attacker = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // local b1=not a:IsType(TYPE_TOKEN) and a:IsStatus(STATUS_BATTLE_DESTROYED) and a:IsControler(1-tp)
    //     and d and d~=c and d:IsRace(RACE_PSYCHIC)
    let b1 = !card::is_type(d, attacker, TYPE_TOKEN)
        && card::is_status(d, attacker, STATUS_BATTLE_DESTROYED)
        && card::is_controler(d, attacker, 1 - tp)
        && match dt {
            Some(x) => x != c && card::is_race(d, x, RACE_PSYCHIC as u64),
            None => false,
        };
    // local b2=d and not d:IsType(TYPE_TOKEN) and d:IsStatus(STATUS_BATTLE_DESTROYED) and d:IsControler(1-tp)
    //     and a~=c and a:IsRace(RACE_PSYCHIC)
    let b2 = match dt {
        Some(x) => {
            !card::is_type(d, x, TYPE_TOKEN)
                && card::is_status(d, x, STATUS_BATTLE_DESTROYED)
                && card::is_controler(d, x, 1 - tp)
                && attacker != c
                && card::is_race(d, attacker, RACE_PSYCHIC as u64)
        }
        None => false,
    };
    // if (not b1 and not b2) or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
    if (!b1 && !b2) || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    if b1 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetCode(EFFECT_SEND_REPLACE)
        effect::set_code(d, e1, EFFECT_SEND_REPLACE);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e1:SetTarget(s.reptg)
        effect::set_target(d, e1, Some(cost_target(reptg)));
        // e1:SetOperation(s.repop)
        effect::set_operation(d, e1, Some(operation(repop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // a:RegisterEffect(e1)
        card::register_effect(d, attacker, e1, false);
    }
    if b2 {
        let target_card = dt.nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetCode(EFFECT_SEND_REPLACE)
        effect::set_code(d, e1, EFFECT_SEND_REPLACE);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e1:SetTarget(s.reptg)
        effect::set_target(d, e1, Some(cost_target(reptg)));
        // e1:SetOperation(s.repop)
        effect::set_operation(d, e1, Some(operation(repop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // d:RegisterEffect(e1)
        card::register_effect(d, target_card, e1, false);
    }
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetDestination()==LOCATION_GRAVE and c:IsReason(REASON_BATTLE) end
    if a.chk == Some(0) {
        return card::get_destination(d, c) == LOCATION_GRAVE && card::is_reason(d, c, REASON_BATTLE);
    }
    // return true
    true
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoDeck(e:GetHandler(),nil,SEQ_DECKTOP,REASON_EFFECT)
    let c = effect::get_handler(d, a.e).nil();
    duel::send_to_deck(d, c, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
}
/// `s.dmcon(e,tp,eg,ep,ev,re,r,rp)`
fn dmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsReason(REASON_BATTLE)
    let c = effect::get_handler(d, a.e).nil();
    !card::is_reason(d, c, REASON_BATTLE)
}
/// `s.dmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.SetTargetPlayer(c:GetPreviousControler())
    let prev = card::get_previous_controler(d, c);
    duel::set_target_player(d, prev);
    // Duel.SetTargetParam(c:GetAttack())
    let atk = card::get_attack(d, c);
    duel::set_target_param(d, atk as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,c:GetPreviousControler(),c:GetAttack())
    let prev = card::get_previous_controler(d, c);
    let atk = card::get_attack(d, c);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, prev, atk);
    true
}
/// `s.dmop(e,tp,eg,ep,ev,re,r,rp)`
fn dmop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
