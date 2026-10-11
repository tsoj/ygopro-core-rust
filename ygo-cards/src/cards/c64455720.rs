//! Ashtra Insect Poison
use super::prelude::*;

pub const ID: u32 = 64455720;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ASHTRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
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
/// The "Ashtra" token of the Special Summon option (Insect)
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_ASHTRA as u16]),
        type_: Some(TYPE_MONSTER | TYPE_NORMAL | TYPE_TUNER),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_INSECT),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ASHTRA) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ASHTRA as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, e, tp);
    }
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // local b1=mmz_chk and e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_ASHTRA,...)
    let b1 = mmz_chk && effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) && duel::is_player_can_special_summon_monster(d, tp, ID, token_data(), None, None, 0);
    // local b2=mmz_chk and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    let b2 = mmz_chk && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
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
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, tp, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // elseif cd.choice==2 then
    } else if choice == 2 {
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
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
        // local tc=Duel.GetFirstTarget()
        let tc = duel::get_first_target(d).first().copied().nil();
        // if tc:IsRelateToEffect(e) then Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end
        if card::is_relate_to_effect(d, tc, e) {
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
