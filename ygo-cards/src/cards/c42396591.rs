//! Number C104: Umbral Horror Masquerade Victory
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 42396591;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(104), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 4 Level 5 monsters
    // Xyz.AddProcedure(c,nil,5,4)
    xyz::add_procedure(d, c, None, Some(5), 4, None, None, None, None, false, None);
    // Once per turn (Quick Effect): You can detach 1 material from this card, then target 1 card your opponent controls; destroy it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk,chkc) ... end)
    effect::set_target(d, e1, Some(cost_target(e1_target)));
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e1, Some(operation(e1_op)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card battles an opponent's monster, during damage calculation: This card's ATK/DEF become that opponent's monster's ATK or DEF (whichever is higher) +100 during damage calculation only
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2, Some(operation(e2_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card destroys a monster by battle: Send 1 random card from your opponent's hand to the GY, and if you do, halve your opponent's LP
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetCondition(aux.bdcon)
    effect::set_condition(d, e3, Some(aux::bdcon()));
    // e3:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e3, Some(cost_target(e3_target)));
    // e3:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e3, Some(operation(e3_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk,chkc)` of e1
fn e1_target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
    let tc = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc, 1, tp, 0);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` operation of e1
fn e1_op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` condition of e2
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // local bc=c:GetBattleTarget()
    // return bc and bc:IsControler(1-tp)
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_controler(d, bc, 1 - tp),
        None => false,
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` operation of e2
fn e2_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    // local bc=c:GetBattleTarget()
    // if bc and bc:IsControler(1-tp) and bc:IsFaceup() then
    if let Some(bc) = card::get_battle_target(d, c) {
        if card::is_controler(d, bc, 1 - a.tp) && card::is_faceup(d, bc) {
            // local val=math.max(bc:GetAttack(),bc:GetDefense())
            let val = card::get_attack(d, bc).max(card::get_defense(d, bc)) as i64;
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(val+100)
            effect::set_value(d, e1, val + 100);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_PHASE|PHASE_DAMAGE_CAL)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
        }
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` target of e3
fn e3_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, LOCATION_HAND as i32);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` operation of e3
fn e3_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local hc=Duel.GetFieldGroup(tp,0,LOCATION_HAND):RandomSelect(tp,1):GetFirst()
    let hand = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    let sel = group::random_select(d, &hand, tp, 1);
    let hc = group::get_first(d, &sel);
    // if hc and Duel.SendtoGrave(hc,REASON_EFFECT)>0 and hc:IsLocation(LOCATION_GRAVE) then
    if let Some(hc) = hc {
        if duel::send_to_grave(d, hc, REASON_EFFECT, None, None) > 0 && card::is_location(d, hc, LOCATION_GRAVE) {
            // local opp=1-tp
            let opp = 1 - tp;
            // Duel.SetLP(opp,Duel.GetLP(opp)/2)
            let half = lua_int(duel::get_lp(d, opp) as f64 / 2.0) as i32;
            duel::set_lp(d, opp, half);
        }
    }
}
