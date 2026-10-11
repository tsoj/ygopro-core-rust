//! Divine Serpent Geh
use super::prelude::*;

pub const ID: u32 = 82103466;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // connot special summon
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot be target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // disable and atk down
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e3, EFFECT_DISABLE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCondition(s.adcon)
    effect::set_condition(d, e3, Some(lua_fn(adcon)));
    // e3:SetTarget(s.adtg)
    effect::set_target(d, e3, Some(effect_card_fn(adtg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_DISABLE_EFFECT);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e5, EFFECT_SET_ATTACK_FINAL);
    // e5:SetValue(s.atkval)
    effect::set_value_fn(d, e5, effect_card_value(atkval));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // atk up
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e6, CATEGORY_ATKCHANGE);
    // e6:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e6, EFFECT_TYPE_QUICK_O);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e6, EVENT_PRE_DAMAGE_CALCULATE);
    // e6:SetCondition(s.atkcon)
    effect::set_condition(d, e6, Some(condition(atkcon)));
    // e6:SetTarget(s.atktg)
    effect::set_target(d, e6, Some(cost_target(atktg)));
    // e6:SetOperation(s.atkop)
    effect::set_operation(d, e6, Some(operation(atkop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
// s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and ((c:IsReason(REASON_BATTLE) and Duel.GetAttacker():IsControler(1-tp))
    //     or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    if !(card::is_previous_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_MZONE)) {
        return false;
    }
    if card::is_reason(d, c, REASON_BATTLE) {
        let atk = duel::get_attacker(d).nil();
        if card::is_controler(d, atk, 1 - tp) {
            return true;
        }
    }
    card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2))
    let lp = duel::get_lp(d, a.tp).div_euclid(2) as u32;
    duel::pay_lp_cost(d, a.tp, lp);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler() (the handler is only used below)
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,true,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, true, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.SpecialSummon(c,0,tp,tp,true,false,POS_FACEUP)~=0 then
    if duel::special_summon(d, c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) != 0 {
        // c:CompleteProcedure()
        card::complete_procedure(d, c);
    }
}
fn adcon(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // return Duel.GetAttacker()==c and c:GetBattleTarget()
    //     and (Duel.IsPhase(PHASE_DAMAGE) or Duel.IsPhase(PHASE_DAMAGE_CAL))
    duel::get_attacker(d) == Some(c)
        && card::get_battle_target(d, c).is_some()
        && (duel::is_phase(d, PHASE_DAMAGE) || duel::is_phase(d, PHASE_DAMAGE_CAL))
}
// s.adtg(e,c)
fn adtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, e).nil();
    Some(c) == card::get_battle_target(d, h)
}
// s.atkval(e,c)
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return math.ceil(c:GetBaseAttack()/2)
    (card::get_base_attack(d, c) as f64 / 2.0).ceil() as i64
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c==Duel.GetAttacker() or c==Duel.GetAttackTarget()
    Some(c) == duel::get_attacker(d) || Some(c) == duel::get_attack_target(d)
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,c)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
        // if #g==0 then return false end
        if group::get_count(&g) == 0 {
            return false;
        }
        // local g1,atk=g:GetMaxGroup(Card.GetBaseAttack)
        let (_g1, atk) = group::get_max_group(d, &g, &|d, x| card::get_base_attack(d, x) as i64).nil();
        // return c:GetAttack()~=atk and c:GetFlagEffect(id)==0
        return card::get_attack(d, c) as i64 != atk && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,c)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
    // if #g==0 then return end
    if group::get_count(&g) == 0 {
        return;
    }
    // local g1,atk=g:GetMaxGroup(Card.GetBaseAttack)
    let (_g1, atk) = group::get_max_group(d, &g, &|d, x| card::get_base_attack(d, x) as i64).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and atk>0 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && atk > 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
