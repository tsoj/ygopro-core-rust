//! 切り番
//! You're Finished
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 88346805;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card as an Effect Monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,aux.FALSE)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, aux_false());
    // Destroy as many cards as possible
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(_,tp,_,_,_,_,_,rp) return rp==1-tp end)
    effect::set_condition(d, e2, Some(condition(|_d, a| a.rp == 1 - a.tp)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn spcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetCustomActivityCount(id,0,ACTIVITY_CHAIN)+Duel.GetCustomActivityCount(id,1,ACTIVITY_CHAIN)>=10
    duel::get_custom_activity_count(d, ID, 0, ACTIVITY_CHAIN) + duel::get_custom_activity_count(d, ID, 1, ACTIVITY_CHAIN) >= 10
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,3000,3000,10,RACE_CYBERSE,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        let md = duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPE_MONSTER | TYPE_EFFECT),
            attack: Some(3000),
            defense: Some(3000),
            level: Some(10),
            race: Some(RACE_CYBERSE),
            attribute: Some(ATTRIBUTE_DARK),
        };
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID, md, None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and s.sptg(e,tp,eg,ep,ev,re,r,rp,0) then
    if card::is_relate_to_effect(d, c, a.e) && sptg(d, &EffArgs { chk: Some(0), ..a.clone() }) {
        // c:AddMonsterAttribute(TYPE_EFFECT|TYPE_TRAP)
        card::add_monster_attribute(d, c, TYPE_EFFECT | TYPE_TRAP, 0, 0, 0, 0, 0);
        // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon_step(d, c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        // c:AddMonsterAttributeComplete()
        card::add_monster_attribute_complete(d, c);
        // aux.DelayedOperation(c,PHASE_END,id,e,tp,s.setop,function() return Duel.IsTurnPlayer(tp) end,0)
        let tp = a.tp;
        aux::delayed_operation(
            d,
            c,
            PHASE_END,
            ID,
            a.e,
            a.tp,
            Some(std::sync::Arc::new(setop)),
            Some(std::sync::Arc::new(move |d: &mut Duel, _ag: &CardSet, _a: &EffArgs| duel::is_turn_player(d, tp))),
            Some(0),
            None,
            None,
            None,
        );
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.setop(ag,e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, ag: &CardSet, a: &EffArgs) -> bool {
    // local c=ag:GetFirst()
    let c = group::get_first(d, ag).nil();
    // if not (c:IsLocation(LOCATION_MZONE) and Duel.CanPlayerSetSpellTrap(tp,c)) then return end
    if !(card::is_location(d, c, LOCATION_MZONE) && duel::can_player_set_spell_trap(d, a.tp, Some(c))) {
        return false;
    }
    // if c:IsSSetable() then
    if card::is_s_setable(d, c, false, None) {
        // Duel.SSet(tp,c,tp,false)
        duel::s_set(d, a.tp, c, Some(a.tp), false);
    } else {
        // Duel.SendtoGrave(c,REASON_RULE)
        duel::send_to_grave(d, c, REASON_RULE, None, None);
    }
    false
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if Duel.Destroy(g,REASON_EFFECT)>0 and c:IsRelateToEffect(e) and c:IsSSetable(true) then
    if duel::destroy(d, g, REASON_EFFECT, None, None) > 0 && card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, true, None) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SSet(tp,c,tp,false)
        duel::s_set(d, a.tp, c, Some(a.tp), false);
    }
}
