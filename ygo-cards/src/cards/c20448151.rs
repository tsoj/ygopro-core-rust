//! Clown Crew Rehearsal
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 20448151;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute 1 monster from your hand or field; add 1 "Clown Crew Biancaviso" and 1 "Clown Crew" Spell/Trap from your Deck to your hand, except "Clown Crew "Rehearsal""
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.thcost)
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY; Ritual Summon 1 "Clown Crew" Ritual Monster from your hand, by Tributing monsters from your hand or field whose total Levels equal or exceed its Level
    // local e2=Ritual.CreateProc(c,RITPROC_GREATER,
    //     function(c) return c:IsSetCard(SET_CLOWN_CREW) end,
    //     nil,aux.Stringid(id,1))
    let e2 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(std::sync::Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_CLOWN_CREW as u16]))),
            desc: Some(aux::stringid(ID, 1)),
            ..Default::default()
        },
    );
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.listed_series={SET_CLOWN_CREW}
// s.listed_names={82159583,id} --"Clown Crew Biancaviso"
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[82159583, ID], listed_series: &[SET_CLOWN_CREW], ..ScriptInfo::DEFAULT };

/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,1,true,nil,nil) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, a.tp, None, 1, 1, true, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,1,1,true,nil,nil)
    let g = aux::select_release_group_cost(d, a.tp, None, 1, 1, true, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(82159583) or (c:IsSetCard(SET_CLOWN_CREW) and c:IsSpellTrap() and not c:IsCode(id))) and c:IsAbleToHand()
    (card::is_code(d, c, &[82159583])
        || (card::is_set_card(d, c, &[SET_CLOWN_CREW as u16]) && card::is_spell_trap(d, c) && !card::is_code(d, c, &[ID])))
        && card::is_able_to_hand(d, c, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsCode,1,nil,82159583) and sg:IsExists(Card.IsSpellTrap,1,nil)
    let res = group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[82159583]), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_spell_trap(d, c), 1, NO_EX);
    (res, false)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_ATOHAND)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_ATOHAND as u64), None, None, false)
        .unwrap_or_default();
    // if #sg==2 then
    if sg.len() == 2 {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
