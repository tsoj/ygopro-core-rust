//! Spellbook of the Grand Circle
//! scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 11911336;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CHARMER, SET_FAMILIAR_POSSESSED], ..ScriptInfo::DEFAULT };

/// `fusion_params` of the Fusion Summon effect: `matfilter` and `stage2`
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        // matfilter=function(c) return c:IsSetCard({SET_CHARMER,SET_FAMILIAR_POSSESSED}) end
        matfilter: Some(Arc::new(|d, c, _e, _tp, _chk| card::is_set_card(d, c, &[SET_CHARMER as u16, SET_FAMILIAR_POSSESSED as u16]))),
        // stage2=s.stage2
        stage2: Some(Arc::new(|d: &mut Duel, e: EffectId, fc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64| stage2(d, e, fc, tp, sg, chk))),
        ..Default::default()
    }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 4 "Charmer" monsters with different Attributes from your Deck, GY, and/or banishment to your hand, then shuffle 2 cards from your hand into the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Fusion Summon 1 Fusion Monster from your Extra Deck, using "Charmer" and/or "Familiar-Possessed" monsters from your hand or field,
    // also your opponent cannot activate cards or effects when a monster is Fusion Summoned this way
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(Fusion.SummonEffTG(fusion_params))
    let tg = fusion::summon_eff_tg(d, fusion_params());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(fusion_params))
    let op = fusion::summon_eff_op(d, fusion_params());
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CHARMER) and c:IsMonster() and (c:IsLocation(LOCATION_DECK) or c:IsFaceup())
    //     and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_CHARMER as u16])
        && card::is_monster(d, c)
        && (card::is_location(d, c, LOCATION_DECK) || card::is_faceup(d, c))
        && card::is_able_to_hand(d, c, None)
}
/// The attribute as a number for `aux.dpcheck(Card.GetAttribute)`
fn get_attribute_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_attribute(d, c) as i64
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,4,4,aux.dpcheck(Card.GetAttribute),0)
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(4), Some(4), Some(&aux::dpcheck(get_attribute_value)), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,2,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 2, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(thfilter)), a.tp, LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // if #g<4 then return end
    if g.len() < 4 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,4,4,aux.dpcheck(Card.GetAttribute),1,tp,HINTMSG_ATOHAND)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(4), Some(4), Some(&aux::dpcheck(get_attribute_value)), 1, a.tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #sg==4 and Duel.SendtoHand(sg,nil,REASON_EFFECT)==4 then
    if sg.len() == 4 && duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None) == 4 {
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        // local tdg=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,2,2,nil)
        let tdg = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_HAND, 0, 2, 2, false, NO_EX).nil();
        // if #tdg==2 then
        if tdg.len() == 2 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoDeck(tdg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, tdg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
/// `s.stage2(e,fc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, _fc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk == 1 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAIN_END)
        effect::set_code(d, e1, EVENT_CHAIN_END);
        // e1:SetOperation(function(e)
        //     Duel.SetChainLimitTillChainEnd(function(e,ep,tp) return tp==ep end)
        //     e:Reset()
        // end)
        effect::set_operation(d, e1, Some(operation(|d, a| {
            duel::set_chain_limit_till_chain_end(d, lua_fn(|_d, a2: Args| a2.p(2) == a2.p(1)));
            effect::reset(d, a.e);
        })));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
