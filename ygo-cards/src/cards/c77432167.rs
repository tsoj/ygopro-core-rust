//! Sun God Unification
use super::prelude::*;

pub const ID: u32 = 77432167;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate the turn it is set
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_SET_TURN);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetCondition(s.actcon)
    effect::set_condition(d, e2, Some(condition(actcon)));
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Increase ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e3, TIMING_DAMAGE_STEP, None);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetCost(s.atkcost)
    effect::set_cost(d, e3, Some(cost_target(atkcost)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Recover LP
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e4, CATEGORY_RECOVER);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCost(s.lpcost)
    effect::set_cost(d, e4, Some(cost_target(lpcost)));
    // e4:SetTarget(s.lptg)
    effect::set_target(d, e4, Some(cost_target(lptg)));
    // e4:SetOperation(s.lpop)
    effect::set_operation(d, e4, Some(operation(lpop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsOriginalCodeRule,CARD_RA),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_original_code_rule(d, c, &[CARD_RA])), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local cost=Duel.GetLP(tp)-100
    let cost = duel::get_lp(d, a.tp) as i64 - 100;
    // if chk==0 then return cost>0 end
    if a.chk == Some(0) {
        return cost > 0;
    }
    // e:SetLabel(cost)
    effect::set_label(d, a.e, &[cost]);
    // Duel.PayLPCost(tp,cost)
    duel::pay_lp_cost(d, a.tp, cost as u32);
    true
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_RA) and c:IsSpecialSummoned()
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_RA]) && card::is_special_summoned(d, c)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkfilter,tp,LOCATION_MZONE,0,1,nil) and Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX) && duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_CHAIN, 0, 1, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACK as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // local atk=e:GetLabel()
    let atk = effect::get_label(d, a.e);
    // if atk>0 and tc then
    if let (true, Some(tc)) = (atk > 0, tc) {
        // tc:UpdateAttack(atk,nil,c)
        card::update_attack(d, tc, atk, None, Some(c), None);
        // tc:UpdateDefense(atk,nil,c)
        card::update_defense(d, tc, atk, None, Some(c), None);
    }
}
/// `s.lpcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lpcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return true end
    true
}
/// `s.lpfilter(c)`
fn lpfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_RA) and c:GetAttack()>0
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_RA]) && card::get_attack(d, c) > 0
}
/// `s.lptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 then return false end
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.CheckReleaseGroupCost(tp,s.lpfilter,1,false,nil,nil) and Duel.GetFlagEffect(tp,id)==0
        return duel::check_release_group_cost(d, a.tp, Some(&lpfilter), 1, 1, false, None, NO_EX) && duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,s.lpfilter,1,1,false,nil,nil)
    let sg = duel::select_release_group_cost(d, a.tp, Some(&lpfilter), 1, 1, false, None, NO_EX);
    // local tc=sg:GetFirst()
    let tc = group::get_first(d, &sg).nil();
    // local rec=tc:GetAttack()
    let rec = card::get_attack(d, tc) as i64;
    // Duel.Release(tc,REASON_COST)
    duel::release(d, tc, REASON_COST, None);
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(rec)
    duel::set_target_param(d, rec as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,rec)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, rec as i32);
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_CHAIN, 0, 1, 0);
    true
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
    let _ = a;
}
