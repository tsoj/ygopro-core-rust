//! 幻影騎士団ロスト・ヴァンブレイズ
//! The Phantom Knights of Lost Vambrace
use super::prelude::*;

pub const ID: u32 = 36247316;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_THE_PHANTOM_KNIGHTS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasLevel() and c:IsFaceup()
    card::has_level(d, c) && card::is_faceup(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==0 then return false end
        if effect::get_label(d, a.e) == 0 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_THE_PHANTOM_KNIGHTS,TYPE_MONSTER|TYPE_NORMAL,600,0,2,RACE_WARRIOR,ATTRIBUTE_DARK)
        //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID, vambrace_data(), None, None, 0)
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// The token data of `Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_THE_PHANTOM_KNIGHTS,TYPE_MONSTER|TYPE_NORMAL,600,0,2,RACE_WARRIOR,ATTRIBUTE_DARK)`
fn vambrace_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_THE_PHANTOM_KNIGHTS as u16]),
        type_: Some(TYPE_MONSTER | TYPE_NORMAL),
        attack: Some(600),
        defense: Some(0),
        level: Some(2),
        race: Some(RACE_WARRIOR),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied().nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-600)
        effect::set_value(d, e1, -600);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e2, EFFECT_CHANGE_LEVEL);
        // e2:SetValue(2)
        effect::set_value(d, e2, 2);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD);
        // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
        // e3:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e3, LOCATION_MZONE, 0);
        // e3:SetTarget(s.indtarget)
        effect::set_target(d, e3, Some(effect_card_fn(indtarget)));
        // e3:SetValue(1)
        effect::set_value(d, e3, 1);
        // e3:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, tp);
        // if c:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_THE_PHANTOM_KNIGHTS,TYPE_MONSTER|TYPE_NORMAL,600,0,2,RACE_WARRIOR,ATTRIBUTE_DARK) then
        if card::is_relate_to_effect(d, c, a.e)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID, vambrace_data(), None, None, 0)
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // c:AddMonsterAttribute(TYPE_NORMAL)
            card::add_monster_attribute(d, c, TYPE_NORMAL, 0, 0, 0, 0, 0);
            // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP_DEFENSE)
            duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP_DEFENSE as u8, None);
            // c:AddMonsterAttributeComplete()
            card::add_monster_attribute_complete(d, c);
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
/// `s.indtarget(e,c)`
fn indtarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_THE_PHANTOM_KNIGHTS)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_THE_PHANTOM_KNIGHTS as u16])
}
