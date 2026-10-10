//! Sky Striker Special Maneuver - Lemnisgate!
use super::prelude::*;

pub const ID: u32 = 34433770;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SKY_STRIKER_ACE, SET_SKY_STRIKER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Immediately after this effect resolves, Link Summon 1 "Sky Striker Ace" Link Monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.tdfilter(c,e)
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return ((c:IsSetCard(SET_SKY_STRIKER_ACE) and c:IsMonster()) or (c:IsSetCard(SET_SKY_STRIKER) and c:IsSpell()))
    //     and c:IsAbleToDeck() and c:IsCanBeEffectTarget(e)
    ((card::is_set_card(d, c, &[SET_SKY_STRIKER_ACE as u16]) && card::is_monster(d, c))
        || (card::is_set_card(d, c, &[SET_SKY_STRIKER as u16]) && card::is_spell(d, c)))
        && card::is_able_to_deck(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.rescon(full_mct,full_sct)
//     return function(sg,e,tp,mg)
fn rescon(full_mct: i64, full_sct: i64) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| {
        // local mct=sg:FilterCount(Card.IsMonster,nil)
        let mct = group::filter_count(d, sg, &card::is_monster, NO_EX) as i64;
        // local sct=#sg-mct
        let sct = sg.len() as i64 - mct;
        // if mct==sct then return true end
        if mct == sct {
            return (true, false);
        }
        // local rem_mct=full_mct-mct
        let rem_mct = full_mct - mct;
        // local rem_sct=full_sct-sct
        let rem_sct = full_sct - sct;
        // return false,mct>sct and rem_sct<(mct-sct) or rem_mct<(sct-mct)
        (false, (mct > sct && rem_sct < (mct - sct)) || rem_mct < (sct - mct))
    }
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| tdfilter(d, c, a.e)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local full_mct=g:FilterCount(Card.IsMonster,nil)
    let full_mct = group::filter_count(d, &g, &card::is_monster, NO_EX) as i64;
    // local full_sct=#g-full_mct
    let full_sct = g.len() as i64 - full_mct;
    // local rescon=s.rescon(full_mct,full_sct)
    let rescon = rescon(full_mct, full_sct);
    // if chk==0 then return full_mct>0 and full_sct>0 and aux.SelectUnselectGroup(g,e,tp,2,2,rescon,0) end
    if a.chk == Some(0) {
        return full_mct > 0
            && full_sct > 0
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,math.min(full_mct,full_sct)*2,rescon,1,tp,HINTMSG_TODECK,rescon)
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(2),
        Some((full_mct.min(full_sct) * 2) as usize),
        Some(&rescon),
        1,
        a.tp,
        Some(HINTMSG_TODECK as u64),
        Some(&rescon),
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,#tg,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg.clone()), tg.len() as u8, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 and Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) then
    if !tg.is_empty()
        && duel::send_to_deck(d, tg.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
    {
        // local ct=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)//3
        let ct = group::filter_count(d, &duel::get_operated_group(d), &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX) / 3;
        // if ct>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if ct > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,ct,nil)
            let g = duel::select_matching_card(
                d,
                a.tp,
                Some(&|d, c| card::is_able_to_hand(d, c, None)),
                a.tp,
                LOCATION_ONFIELD,
                LOCATION_ONFIELD,
                1,
                ct as u16,
                false,
                NO_EX,
            )
            .nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.HintSelection(g)
                duel::hint_selection(d, g.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(g,nil,REASON_EFFECT)
                duel::send_to_hand(d, g, None, REASON_EFFECT, None);
            }
        }
    }
}
// function s.spconfilter(c,tp)
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SKY_STRIKER) and c:IsControler(tp) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_SKY_STRIKER as u16]) && card::is_controler(d, c, tp) && card::is_faceup(d, c)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spconfilter(d, c, a.tp), 1, NO_EX)
}
// function s.spfilter(c)
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SKY_STRIKER_ACE) and c:IsLinkSummonable()
    card::is_set_card(d, c, &[SET_SKY_STRIKER_ACE as u16]) && card::is_link_summonable(d, c, None, None, 0, 0)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&spfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let sg = duel::select_matching_card(d, a.tp, Some(&spfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    // if sc then
    if let Some(sc) = sc {
        // Duel.LinkSummon(tp,sc)
        duel::link_summon(d, a.tp, sc, None, None, 0, 0);
    }
}
