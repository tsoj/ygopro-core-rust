//! Guardian of the Golden Land
use super::prelude::*;

pub const ID: u32 = 67007102;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_GOLDEN_LORD], listed_series: &[SET_ELDLIXIR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself as a monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 "Eldlixir" Spell/Trap from the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetHintTiming(TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, TIMING_END_PHASE, None);
    // e2:SetCondition(s.setcond)
    effect::set_condition(d, e2, Some(condition(setcond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// The monster `Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_ELDLIXIR,TYPE_MONSTER|TYPE_NORMAL,800,2500,8,RACE_ZOMBIE,ATTRIBUTE_LIGHT)` describes.
fn monster_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_ELDLIXIR as u16]),
        type_: Some(TYPE_MONSTER | TYPE_NORMAL),
        attack: Some(800),
        defense: Some(2500),
        level: Some(8),
        race: Some(RACE_ZOMBIE),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_ELDLIXIR,TYPE_MONSTER|TYPE_NORMAL,800,2500,8,RACE_ZOMBIE,ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID, monster_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
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
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_ELDLIXIR,TYPE_MONSTER|TYPE_NORMAL,800,2500,8,RACE_ZOMBIE,ATTRIBUTE_LIGHT) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_player_can_special_summon_monster(d, tp, ID, monster_data(), None, None, 0)
    {
        return;
    }
    // c:AddMonsterAttribute(TYPE_NORMAL+TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_NORMAL + TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,1,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 1, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_GOLDEN_LORD),tp,LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.HasNonZeroAttack),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_GOLDEN_LORD])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::has_non_zero_attack(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.HasNonZeroAttack),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_faceup(d, c) && card::has_non_zero_attack(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g).nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.setcond(e,tp,eg,ep,ev,re,r,rp)`
fn setcond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_END)
    duel::is_phase(d, PHASE_END)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ELDLIXIR) and c:IsSSetable() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_ELDLIXIR as u16]) && card::is_s_setable(d, c, false, None) && !card::is_forbidden(d, c)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g)
        duel::s_set(d, tp, g, None, true);
    }
}
