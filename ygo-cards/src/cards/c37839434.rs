//! Ashtra Divine Domain
use super::prelude::*;

pub const ID: u32 = 37839434;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The "Ashtra" token of the Special Summon option (Aqua)
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_ASHTRA as u16]),
        type_: Some(TYPE_MONSTER | TYPE_NORMAL | TYPE_TUNER),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_AQUA),
        attribute: Some(ATTRIBUTE_WATER),
    }
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_ASHTRA,...)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE)
        && duel::is_player_can_special_summon_monster(d, tp, ID, token_data(), None, None, 0);
    // local b2_event,_,event_p,event_v,event_reff=Duel.CheckEvent(EVENT_CHAINING,true)
    // local b2=b2_event and event_p==1-tp and event_reff:IsSpellTrapEffect() and Chain.IsDisablable(event_v)
    //     and not Duel.IsExistingMatchingCard(Card.IsFacedown,tp,0,LOCATION_ONFIELD,1,nil)
    let evt = duel::check_event(d, EVENT_CHAINING);
    let b2 = match &evt {
        Some(evt) => {
            evt.event_player == 1 - tp
                && effect::is_spell_trap_effect(d, evt.reason_effect.nil())
                && duel::is_chain_disablable(d, evt.event_value as u8)
                && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
        }
        None => false,
    };
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let choice = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    chain::set_effect_data(d, e, None, chain::field::CARD, Value::Int(choice as i64));
    let h = effect::get_handler(d, e).nil();
    // if cd.choice==1 then
    if choice == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, tp, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // elseif cd.choice==2 then
    } else if choice == 2 {
        // cd.negate_chain_link=event_v
        let evt = evt.nil();
        chain::set_effect_data(d, e, None, chain::field::CARD + 1, Value::Int(evt.event_value as i64));
        // e:SetCategory(CATEGORY_DISABLE)
        effect::set_category(d, e, CATEGORY_DISABLE);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,event_reff:GetHandler(),1,tp,0)
        let reff = evt.reason_effect.nil();
        let reff_h = effect::get_handler(d, reff).nil();
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(reff_h)), 1, tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let choice = chain::effect_data(d, e, None, chain::field::CARD).int();
    // if cd.choice==1 then
    if choice == 1 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // if c:IsRelateToEffect(e) and Duel.IsPlayerCanSpecialSummonMonster(...) then
        if card::is_relate_to_effect(d, c, e) && duel::is_player_can_special_summon_monster(d, tp, ID, token_data(), None, None, 0) {
            // c:AddMonsterAttribute(TYPE_NORMAL|TYPE_TUNER)
            card::add_monster_attribute(d, c, TYPE_NORMAL | TYPE_TUNER, 0, 0, 0, 0, 0);
            // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
            // c:AddMonsterAttributeComplete()
            card::add_monster_attribute_complete(d, c);
        }
        // if Duel.SpecialSummonComplete()==0 then return end
        if duel::special_summon_complete(d) == 0 {
            return;
        }
        // local g=Duel.GetMatchingGroup(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,nil)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_synchro_summonable(d, x, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.SynchroSummon(tp,sg:GetFirst())
            let first = group::get_first(d, &sg).nil();
            duel::synchro_summon(d, tp, first, None, None, 0, 0);
        }
    // elseif cd.choice==2 then
    } else if choice == 2 {
        // Duel.NegateEffect(cd.negate_chain_link)
        let link = chain::effect_data(d, e, None, chain::field::CARD + 1).int() as u8;
        duel::negate_effect(d, link);
    }
}
