//! No.87 雪月花美神クイーン・オブ・ナイツ
//! Number 87: Queen of the Night
use super::prelude::*;

pub const ID: u32 = 89516305;

pub const INFO: ScriptInfo = ScriptInfo {
    xyz_number: Some(87),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 8 monsters
    // Xyz.AddProcedure(c,nil,8,3)
    xyz::add_procedure(d, c, None, Some(8), 3, None, None, None, None, false, None);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP|TIMING_SSET,TIMING_DAMAGE_STEP|TIMING_SSET|TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(
        d,
        e1,
        TIMING_DAMAGE_STEP | TIMING_SSET,
        Some(TIMING_DAMAGE_STEP | TIMING_SSET | TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E),
    );
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PLANT) and c:IsCanTurnSet()
    card::is_race(d, c, RACE_PLANT) && card::is_can_turn_set(d, c)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local op=e:GetLabel()
        let op = effect::get_label(d, a.e);
        // if op==1 then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_SZONE) and chkc:IsFacedown()
        if op == 1 {
            return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_SZONE) && card::is_facedown(d, chkc);
        }
        // elseif op==2 then return chkc:IsLocation(LOCATION_MZONE) and s.setfilter(chkc)
        if op == 2 {
            return card::is_location(d, chkc, LOCATION_MZONE) && setfilter(d, chkc);
        }
        // elseif op==3 then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
        if op == 3 {
            return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
        }
    }
    // local not_dmg_step=not Duel.IsPhase(PHASE_DAMAGE)
    let not_dmg_step = !duel::is_phase(d, PHASE_DAMAGE);
    // local b1=not_dmg_step and Duel.IsExistingTarget(Card.IsFacedown,tp,0,LOCATION_SZONE,1,nil)
    let b1 = not_dmg_step && duel::is_existing_target(d, Some(&|d, x| card::is_facedown(d, x)), tp, 0, LOCATION_SZONE, 1, NO_EX);
    // local b2=not_dmg_step and Duel.IsExistingTarget(s.setfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let b2 = not_dmg_step && duel::is_existing_target(d, Some(&setfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    // local b3=aux.StatChangeDamageStepCondition()
    //     and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let b3 = aux::stat_change_damage_step_condition(d)
        && duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)},{b3,aux.Stringid(id,3)})
    let op = duel::select_effect(
        d,
        tp,
        &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))],
    )
    .nil();
    let op = op as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // e:SetCategory(0)
    effect::set_category(d, a.e, 0);
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEDOWN)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEDOWN as u64);
        // Duel.SelectTarget(tp,Card.IsFacedown,tp,0,LOCATION_SZONE,1,1,nil)
        let _g = duel::select_target(d, tp, Some(&|d, x| card::is_facedown(d, x)), tp, 0, LOCATION_SZONE, 1, 1, false, NO_EX);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_POSITION + CATEGORY_SET);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
        // local g=Duel.SelectTarget(tp,s.setfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_target(d, tp, Some(&setfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEDOWN_DEFENSE)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, tp, POS_FACEDOWN_DEFENSE as i32);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
        // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let _g = duel::select_target(d, tp, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied().nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if op == 1 {
        // Target 1 Set Spell/Trap your opponent controls; while this card is face-up on the field, that Set card cannot be activated
        // if c:IsRelateToEffect(e) and tc:IsFacedown() then
        if card::is_relate_to_effect(d, c, a.e) && card::is_facedown(d, tc) {
            // c:SetCardTarget(tc)
            card::set_card_target(d, c, tc);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_OWNER_RELATE)
            effect::set_property(d, e1, EFFECT_FLAG_OWNER_RELATE, 0);
            // e1:SetCode(EFFECT_CANNOT_TRIGGER)
            effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
            // e1:SetCondition(function() return c:IsHasCardTarget(tc) and tc:IsFacedown() end)
            effect::set_condition(
                d,
                e1,
                Some(condition(move |d, _a| card::is_has_card_target(d, c, tc) && card::is_facedown(d, tc))),
            );
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    } else if op == 2 {
        // Target 1 Plant monster on the field; change that target to face-down Defense Position
        // if tc:IsRace(RACE_PLANT) then
        if card::is_race(d, tc, RACE_PLANT) {
            // Duel.ChangePosition(tc,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    } else if op == 3 {
        // Target 1 face-up monster on the field; that target gains 300 ATK
        // if tc:IsFaceup() then
        if card::is_faceup(d, tc) {
            // tc:UpdateAttack(300,nil,c)
            card::update_attack(d, tc, 300, None, Some(c), None);
        }
    }
}
