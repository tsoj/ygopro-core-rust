//! Depresspard
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 10474647;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==1 end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) == 1)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make this card's ATK become 2500
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(function(e) return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+1) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 1])
    })));
    // e2:SetOperation(s.atk2500op)
    effect::set_operation(d, e2, Some(operation(atk2500op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card battles an opponent's monster, you take any battle damage your opponent would have taken from that battle
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_REFLECT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_REFLECT_BATTLE_DAMAGE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetCondition(s.dmgcon)
    effect::set_condition(d, e3, Some(condition(dmgcon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Change this card's ATK to 0, then, if your LP are 2000 or less, its ATK becomes 5000
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e4, EVENT_BATTLE_DAMAGE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==tp and e:GetHandler():IsRelateToBattle() end)
    effect::set_condition(d, e4, Some(condition(e4_condition)));
    // e4:SetOperation(s.atk0op)
    effect::set_operation(d, e4, Some(operation(atk0op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,1,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 1, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,1,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 1, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.atk2500op(e,tp,eg,ep,ev,re,r,rp)`
fn atk2500op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and not c:IsAttack(2500) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && !card::is_attack(d, c, &[2500]) {
        // local ct=Duel.IsTurnPlayer(tp) and 2 or 1
        let ct: u16 = if duel::is_turn_player(d, tp) { 2 } else { 1 };
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(2500)
        effect::set_value(d, e1, 2500);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END,ct)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, ct);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.dmgcon(e)`
fn dmgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    // return c:IsRelateToBattle() and bc and bc:IsControler(1-e:GetHandlerPlayer())
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    match card::get_battle_target(d, c) {
        Some(bc) => {
            let p = effect::get_handler_player(d, a.e);
            card::is_controler(d, bc, 1 - p)
        }
        None => false,
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return ep==tp and e:GetHandler():IsRelateToBattle() end`
fn e4_condition(d: &mut Duel, a: &EffArgs) -> bool {
    if a.ep != a.tp {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::is_relate_to_battle(d, h)
}
/// `s.atk0op(e,tp,eg,ep,ev,re,r,rp)`
fn atk0op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and not c:IsAttack(0) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && !card::is_attack(d, c, &[0]) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // if Duel.GetLP(tp)<=2000 then
        if duel::get_lp(d, tp) <= 2000 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e2, EFFECT_SET_ATTACK_FINAL);
            // e2:SetValue(5000)
            effect::set_value(d, e2, 5000);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e2, RESET_EVENT + RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
        }
    }
}
