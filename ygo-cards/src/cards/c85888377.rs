//! Apophis the Swamp Deity
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 85888377;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The monster data of the Lua call `Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_NORMAL,2000,2200,6,RACE_REPTILE,ATTRIBUTE_EARTH)`
fn monster_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPE_MONSTER | TYPE_NORMAL),
        attack: Some(2000),
        defense: Some(2200),
        level: Some(6),
        race: Some(RACE_REPTILE),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and
    //     Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_NORMAL,2000,2200,6,RACE_REPTILE,ATTRIBUTE_EARTH) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID, monster_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,nil,0,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, None, 0, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsTrap() and c:IsType(TYPE_CONTINUOUS)
    card::is_faceup(d, c) && card::is_trap(d, c) && card::is_type(d, c, TYPE_CONTINUOUS)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_NORMAL,2000,2200,6,RACE_REPTILE,ATTRIBUTE_EARTH) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_player_can_special_summon_monster(d, tp, ID, monster_data(), None, None, 0)
    {
        return;
    }
    // c:AddMonsterAttribute(TYPE_NORMAL+TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_NORMAL + TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // if Duel.SpecialSummonComplete()==0 then return end
    if duel::special_summon_complete(d) == 0 {
        return;
    }
    // local ct=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_ONFIELD,0,c)
    let ct = duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_ONFIELD, 0, c.into());
    // local cg=Duel.GetMatchingGroup(Card.IsNegatable,tp,0,LOCATION_ONFIELD,nil)
    let cg = duel::get_matching_group(d, Some(&|d, x| card::is_negatable(d, x)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if ct>0 and #cg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if ct > 0 && !cg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        // local sg=cg:Select(tp,1,ct,nil)
        let sg = group::select(d, &cg, tp, 1, ct as u16, false, NO_EX).nil();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // Duel.HintSelection(sg,true)
        duel::hint_selection(d, &sg, true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // for tc in aux.Next(sg) do
        for tc in group::iter(d, &sg) {
            // --Negate its effects
            // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
            duel::negate_related_chain(d, tc, RESET_TURN_SET);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // if tc:IsType(TYPE_TRAPMONSTER) then
            if card::is_type(d, tc, TYPE_TRAPMONSTER) {
                // local e3=Effect.CreateEffect(c)
                let e3 = effect::create_effect(d, c);
                // e3:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
                effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
                // e3:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
                // tc:RegisterEffect(e3)
                card::register_effect(d, tc, e3, false);
            }
        }
    }
}
