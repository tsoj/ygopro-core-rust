//! Eldlixir of the Glorious Golden Land
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 22669793;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_GOLDEN_LORD], listed_series: &[SET_GOLDEN_LAND, SET_ELDLIXIR], ..ScriptInfo::DEFAULT };

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
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.PayLP(800))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(800))));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_GOLDEN_LAND,SET_ELDLIXIR}) and c:IsSSetable() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_GOLDEN_LAND as u16, SET_ELDLIXIR as u16]) && card::is_s_setable(d, c, false, None) && card::is_faceup(d, c)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_GOLDEN_LAND,TYPE_MONSTER|TYPE_TOKEN,1500,2800,10,RACE_ZOMBIE,ATTRIBUTE_LIGHT)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_ELDLIXIR,TYPE_MONSTER|TYPE_TOKEN,1500,2800,10,RACE_ZOMBIE,ATTRIBUTE_LIGHT)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, ID, duel::MonsterData { setcodes: Some(vec![SET_GOLDEN_LAND as u16]), type_: Some(TYPE_MONSTER | TYPE_TOKEN), attack: Some(1500), defense: Some(2800), level: Some(10), race: Some(RACE_ZOMBIE), attribute: Some(ATTRIBUTE_LIGHT) }, None, None, 0)
        && duel::is_player_can_special_summon_monster(d, tp, ID, duel::MonsterData { setcodes: Some(vec![SET_ELDLIXIR as u16]), type_: Some(TYPE_MONSTER | TYPE_TOKEN), attack: Some(1500), defense: Some(2800), level: Some(10), race: Some(RACE_ZOMBIE), attribute: Some(ATTRIBUTE_LIGHT) }, None, None, 0);
    // local b2=Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_REMOVED,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
        let h = effect::get_handler(d, e).nil();
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, tp, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_EITHER,LOCATION_MZONE)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SET)
        effect::set_category(d, e, CATEGORY_SET);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // if op==1 then
    if op == 1 {
        // --Special Summon this card as a Normal Monster
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // if not (c:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_GOLDEN_LAND,TYPE_MONSTER|TYPE_TOKEN,1500,2800,10,RACE_ZOMBIE,ATTRIBUTE_LIGHT)
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_ELDLIXIR,TYPE_MONSTER|TYPE_TOKEN,1500,2800,10,RACE_ZOMBIE,ATTRIBUTE_LIGHT)) then return end
        if !(card::is_relate_to_effect(d, c, e)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID, duel::MonsterData { setcodes: Some(vec![SET_GOLDEN_LAND as u16]), type_: Some(TYPE_MONSTER | TYPE_TOKEN), attack: Some(1500), defense: Some(2800), level: Some(10), race: Some(RACE_ZOMBIE), attribute: Some(ATTRIBUTE_LIGHT) }, None, None, 0)
            && duel::is_player_can_special_summon_monster(d, tp, ID, duel::MonsterData { setcodes: Some(vec![SET_ELDLIXIR as u16]), type_: Some(TYPE_MONSTER | TYPE_TOKEN), attack: Some(1500), defense: Some(2800), level: Some(10), race: Some(RACE_ZOMBIE), attribute: Some(ATTRIBUTE_LIGHT) }, None, None, 0))
        {
            return;
        }
        // c:AddMonsterAttribute(TYPE_NORMAL|TYPE_TRAP)
        card::add_monster_attribute(d, c, TYPE_NORMAL | TYPE_TRAP, 0, 0, 0, 0, 0);
        // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
        // c:AddMonsterAttributeComplete()
        card::add_monster_attribute_complete(d, c);
        // if Duel.SpecialSummonComplete()==0 then return end
        if duel::special_summon_complete(d) == 0 {
            return;
        }
        // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_GOLDEN_LORD),tp,LOCATION_ONFIELD,0,1,nil)
        //     and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[CARD_GOLDEN_LORD])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_hand(d, x, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
            let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_able_to_hand(d, x, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.HintSelection(g)
                duel::hint_selection(d, &g, true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(g,nil,REASON_EFFECT)
                duel::send_to_hand(d, g, None, REASON_EFFECT, None);
            }
        }
    // elseif op==2 then
    } else if op == 2 {
        // --Set 1 of your banished "Golden Land" or "Eldlixir" Spells/Traps
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_REMOVED,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SSet(tp,g)
            duel::s_set(d, tp, g, None, true);
        }
    }
}
