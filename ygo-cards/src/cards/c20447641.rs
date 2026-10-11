//! Qlife's End
use super::prelude::*;

pub const ID: u32 = 20447641;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_QLI, SET_APOQLIPHORT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_QLI,TYPE_MONSTER|TYPE_EFFECT,1800,1000,4,RACE_MACHINE,ATTRIBUTE_EARTH)
fn can_summon(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID,
        duel::MonsterData {
            setcodes: Some(vec![SET_QLI as u16]),
            type_: Some(TYPE_MONSTER + TYPE_EFFECT),
            attack: Some(1800),
            defense: Some(1000),
            level: Some(4),
            race: Some(RACE_MACHINE),
            attribute: Some(ATTRIBUTE_EARTH),
        },
        None,
        None,
        0,
    )
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && can_summon(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if card::is_relate_to_effect(d, c, a.e) && can_summon(d, a.tp) {
        // c:AddMonsterAttribute(TYPE_EFFECT)
        card::add_monster_attribute(d, c, TYPE_EFFECT, 0, 0, 0, 0, 0);
        // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon_step(d, c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        // c:AddMonsterAttributeComplete()
        card::add_monster_attribute_complete(d, c);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_TRIPLE_TRIBUTE)
        effect::set_code(d, e1, EFFECT_TRIPLE_TRIBUTE);
        // e1:SetValue(aux.TargetBoolFunction(Card.IsSetCard,SET_APOQLIPHORT))
        effect::set_value_fn(d, e1, effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_APOQLIPHORT as u16])));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        // e2:SetTargetRange(LOCATION_ONFIELD,0)
        effect::set_target_range(d, e2, LOCATION_ONFIELD, 0);
        // e2:SetTarget(s.indtg)
        effect::set_target(d, e2, Some(effect_card_fn(indtg)));
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.indtg(e,c)
fn indtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsSetCard(SET_QLI)
    card::is_spell_trap(d, c) && card::is_set_card(d, c, &[SET_QLI as u16])
}
