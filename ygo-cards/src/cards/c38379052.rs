//! Argostars - Slayer Eteo
use super::prelude::*;

pub const ID: u32 = 38379052;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARGOSTARS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // While this card is face-up in the Spell & Trap Zone, Warrior monsters you control cannot be destroyed by card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsRace(RACE_WARRIOR) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_WARRIOR))));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn, when your opponent activates a card or effect: You can Special Summon this card as an Effect Monster ...
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp end)
    effect::set_cost(d, e2, Some(cost_target(e2_cost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// e2 cost: ep==1-tp
fn e2_cost(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp
}
// The monster data of the card itself (TYPE_MONSTER+TYPE_EFFECT, 800/800, Level 4, Warrior, LIGHT, SET_ARGOSTARS)
fn mdata() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_ARGOSTARS as u16]),
        type_: Some(TYPE_MONSTER | TYPE_EFFECT),
        attack: Some(800),
        defense: Some(800),
        level: Some(4),
        race: Some(RACE_WARRIOR),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:HasFlagEffect(id) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_ARGOSTARS,TYPE_MONSTER|TYPE_EFFECT,800,800,4,RACE_WARRIOR,ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        return !card::has_flag_effect(d, c, ID, None)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID, mdata(), None, None, 0);
    }
    // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END&~(RESET_TOFIELD|RESET_LEAVE|RESET_TURN_SET),0,1)
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END & !(RESET_TOFIELD | RESET_LEAVE | RESET_TURN_SET), 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
// function s.argosfilter(c)
fn argosfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ARGOSTARS) and c:IsMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_ARGOSTARS as u16]) && card::is_monster(d, c) && card::is_faceup(d, c)
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or not (Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_ARGOSTARS,TYPE_MONSTER|TYPE_EFFECT,800,800,4,RACE_WARRIOR,ATTRIBUTE_LIGHT)) then return end
    if !card::is_relate_to_effect(d, c, a.e)
        || !(duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::is_player_can_special_summon_monster(d, tp, ID, mdata(), None, None, 0))
    {
        return;
    }
    // c:AddMonsterAttribute(TYPE_EFFECT|TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_EFFECT | TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    // Once per turn (Quick Effect): You can place this card face-up in your Spell & Trap Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end end)
    effect::set_target(d, e1, Some(cost_target(e1_target)));
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... Duel.MoveToField(c,tp,tp,LOCATION_SZONE,POS_FACEUP,true) ... end)
    effect::set_operation(d, e1, Some(operation(e1_op)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // if Duel.SpecialSummonComplete()>0 and Duel.IsExistingMatchingCard(s.argosfilter,tp,LOCATION_REMOVED,0,1,nil) then
    if duel::special_summon_complete(d) > 0 && duel::is_existing_matching_card(d, Some(&argosfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX) {
        // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
        if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.HintSelection(sg)
            duel::hint_selection(d, sg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
        }
    }
}
// e1 target: if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end
fn e1_target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    true
}
// e1 operation: if c:IsRelateToEffect(e) then Duel.MoveToField(c,tp,tp,LOCATION_SZONE,POS_FACEUP,true) end
fn e1_op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
