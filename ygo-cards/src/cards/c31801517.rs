//! Number 62: Galaxy-Eyes Prime Photon Dragon
use super::prelude::*;

pub const ID: u32 = 31801517;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_GALAXYEYES_P_DRAGON], xyz_number: Some(62), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 8 monsters
    // Xyz.AddProcedure(c,nil,8,2)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, None, None, false, None);
    // This card gains ATK equal to the combined Ranks of all Xyz Monsters currently on the field x 200
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():IsRelateToBattle() end)
    effect::set_condition(d, e1, Some(condition(e1con)));
    // e1:SetCost(Cost.AND(Cost.DetachFromSelf(1),Cost.SoftOncePerBattle(id)))
    effect::set_cost(d, e1, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(1), None, None), cost::soft_once_per_battle(ID, None)])));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card during your 2nd Standby Phase after activation and double its ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Any battle damage this card inflicts to your opponent is halved unless it has "Galaxy-Eyes Photon Dragon" as material
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e3:SetCondition(function(e) return not e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,CARD_GALAXYEYES_P_DRAGON) end)
    effect::set_condition(d, e3, Some(condition(e3con)));
    // e3:SetValue(aux.ChangeBattleDamage(1,HALF_DAMAGE))
    effect::set_value_fn(d, e3, aux::change_battle_damage(1, HALF_DAMAGE as i64));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():IsRelateToBattle() end`
fn e1con(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_relate_to_battle(d, c)
}
/// `function(e) return not e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,CARD_GALAXYEYES_P_DRAGON) end`
fn e3con(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, c);
    !group::is_exists(d, &og, &|d, x| card::is_code(d, x, &[CARD_GALAXYEYES_P_DRAGON]), 1, NO_EX)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local rks=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil):GetSum(Card.GetRank)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        let rks = group::get_sum(d, &g, &|d, x| card::get_rank(d, x) as i64);
        // if rks>0 then
        if rks > 0 {
            // c:UpdateAttack(rks*200,RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_PHASE|PHASE_DAMAGE_CAL)
            card::update_attack(d, c, rks * 200, Some(RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_PHASE | PHASE_DAMAGE_CAL), None, None);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousControler(tp) and rp==1-tp and c:IsReason(REASON_EFFECT) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:GetOverlayGroup():IsExists(Card.IsCode,1,nil,CARD_GALAXYEYES_P_DRAGON)
    if !(card::is_previous_controler(d, c, a.tp) && a.rp == 1 - a.tp && card::is_reason(d, c, REASON_EFFECT) && card::is_previous_location(d, c, LOCATION_MZONE)) {
        return false;
    }
    let og = card::get_overlay_group(d, c);
    group::is_exists(d, &og, &|d, x| card::is_code(d, x, &[CARD_GALAXYEYES_P_DRAGON]), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // c:SetTurnCounter(0)
        card::set_turn_counter(d, c, 0);
        // local turn_count=Duel.GetTurnCount()
        let turn_count = duel::get_turn_count(d, None);
        // local reset_count=(Duel.IsPhase(PHASE_STANDBY) and Duel.IsTurnPlayer(tp)) and 3 or 2
        let reset_count: u16 = if duel::is_phase(d, PHASE_STANDBY) && duel::is_turn_player(d, tp) { 3 } else { 2 };
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_STANDBY);
        // e1:SetRange(c:GetLocation())
        let loc = card::get_location(d, c);
        effect::set_range(d, e1, loc);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) and (reset_count==2 or Duel.GetTurnCount()~=turn_count) end)
        effect::set_condition(d, e1, Some(condition(move |d, a| duel::is_turn_player(d, a.tp) && (reset_count == 2 || duel::get_turn_count(d, None) != turn_count))));
        // e1:SetOperation(s.standbyspop)
        effect::set_operation(d, e1, Some(operation(standbyspop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,reset_count)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, reset_count);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.standbyspop(e,tp,eg,ep,ev,re,r,rp)`
fn standbyspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetTurnCounter()+1
    let ct = card::get_turn_counter(d, c) + 1;
    // c:SetTurnCounter(ct)
    card::set_turn_counter(d, c, ct as u16);
    // if ct==2 and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP) then
    if ct == 2 && duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_ATTACK);
        // e1:SetValue(c:GetAttack()*2)
        let atk = card::get_attack(d, c) * 2;
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
