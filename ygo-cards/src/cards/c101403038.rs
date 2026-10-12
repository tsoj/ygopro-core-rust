//! Aitvaras the White Crow Knight
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 101403038;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WHITE_CROW], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Illusion Tuner + 1+ non-Tuners
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_ILLUSION),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_ILLUSION, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // If this card is Special Summoned: You can target 1 card you control and 1 card your opponent controls; return them to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rthtg)
    effect::set_target(d, e1, Some(cost_target(rthtg)));
    // e1:SetOperation(s.rthop)
    effect::set_operation(d, e1, Some(operation(rthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is sent to the GY: You can activate this effect based on where it was sent from;
    // ● Field: Target 1 card on the field; shuffle it into the Deck
    // ● Extra Deck: Special Summon 1 "White Crow" monster from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD|LOCATION_EXTRA) end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_previous_location(d, h, LOCATION_ONFIELD | LOCATION_EXTRA)
        })),
    );
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetTargetGroup(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_target_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(
                d,
                &g,
                Some(a.e),
                a.tp,
                Some(2),
                Some(2),
                Some(&aux::dpcheck(|d: &mut Duel, c: CardId| card::get_controler(d, c) as i64)),
                0,
                a.tp,
                None,
                None,
                None,
                false,
            ),
            Err(true)
        );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_RTOHAND)
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(2),
        Some(2),
        Some(&aux::dpcheck(|d: &mut Duel, c: CardId| card::get_controler(d, c) as i64)),
        1,
        a.tp,
        Some(HINTMSG_RTOHAND as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), 2, a.tp, 0);
    true
}
// function s.rthop(e,tp,eg,ep,ev,re,r,rp)
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_WHITE_CROW) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_WHITE_CROW as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local prev_loc=c:GetPreviousLocation()
    let prev_loc = card::get_previous_location(d, c);
    // if chkc then return (e:GetChainData().previous_location&LOCATION_ONFIELD)>0 and chkc:IsOnField() and chkc:IsAbleToDeck() end
    if let Some(chkc) = a.chkc() {
        let pl = chain::effect_data(d, a.e, None, chain::field::CARD).int() as u32;
        return (pl & LOCATION_ONFIELD) > 0 && card::is_on_field(d, chkc) && card::is_able_to_deck(d, chkc);
    }
    // if chk==0 then return ((prev_loc&LOCATION_ONFIELD)>0
    //     and Duel.IsExistingTarget(Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil))
    //     or ((prev_loc&LOCATION_EXTRA)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)) end
    if a.chk == Some(0) {
        return (prev_loc & LOCATION_ONFIELD) > 0
            && duel::is_existing_target(d, Some(&card::is_able_to_deck), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
            || (prev_loc & LOCATION_EXTRA) > 0
                && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // e:GetChainData().previous_location=prev_loc
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(prev_loc as i64));
    // if (prev_loc&LOCATION_ONFIELD)>0 then
    if (prev_loc & LOCATION_ONFIELD) > 0 {
        // ● Field: Target 1 card on the field; shuffle it into the Deck
        // e:SetCategory(CATEGORY_TODECK)
        effect::set_category(d, a.e, CATEGORY_TODECK);
        // e:SetProperty(e:GetProperty()|EFFECT_FLAG_CARD_TARGET)
        let (prop, _) = effect::get_property(d, a.e);
        effect::set_property(d, a.e, prop | EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&card::is_able_to_deck), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, a.tp, 0);
    // elseif (prev_loc&LOCATION_EXTRA)>0 then
    } else if (prev_loc & LOCATION_EXTRA) > 0 {
        // ● Extra Deck: Special Summon 1 "White Crow" monster from your GY
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local prev_loc=e:GetChainData().previous_location
    let prev_loc = chain::effect_data(d, a.e, None, chain::field::CARD).int() as u32;
    // if (prev_loc&LOCATION_ONFIELD)>0 then
    if (prev_loc & LOCATION_ONFIELD) > 0 {
        // ● Field: Target 1 card on the field; shuffle it into the Deck
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    // elseif (prev_loc&LOCATION_EXTRA)>0 then
    } else if (prev_loc & LOCATION_EXTRA) > 0 {
        // ● Extra Deck: Special Summon 1 "White Crow" monster from your GY
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
