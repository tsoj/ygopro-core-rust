//! Let's Go Together
use super::prelude::*;

pub const ID: u32 = 38625110;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Your opponent banishes 7 cards from their Extra Deck and/or the top of their Deck, face-down
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(s.rmcon)
    effect::set_condition(d, e1, Some(condition(rmcon)));
    // e1:SetCost(s.rmcost)
    effect::set_cost(d, e1, Some(cost_target(rmcost)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.rmcon(e,tp,eg,ep,ev,re,r,rp)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,id),tp,LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil)
    !duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[ID]))),
        a.tp,
        LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        NO_EX,
    )
}
// function s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetDecktopGroup(tp,7)
    let g = duel::get_decktop_group(d, tp, 7);
    // if chk==0 then return g:FilterCount(Card.IsAbleToRemoveAsCost,nil,POS_FACEDOWN)==7 end
    if a.chk == Some(0) {
        return group::filter_count(d, &g, &|d, c| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8)), NO_EX) == 7;
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.Remove(g,POS_FACEDOWN,REASON_COST)
    duel::remove(d, &g, POS_FACEDOWN as u8, REASON_COST, None, None);
    true
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local p=1-tp
        let p = 1 - tp;
        // local dg=Duel.GetDecktopGroup(p,7)
        let dg = duel::get_decktop_group(d, p, 7);
        // local exg=Duel.GetFieldGroup(p,LOCATION_EXTRA,0)
        let exg = duel::get_field_group(d, p, LOCATION_EXTRA, 0);
        // return (dg+exg):IsExists(Card.IsAbleToRemove,7,nil,p,POS_FACEDOWN,REASON_EFFECT)
        let sum = group::union(&dg, &exg);
        return group::is_exists(d, &sum, &|d, c| card::is_able_to_remove(d, c, Some(p), Some(POS_FACEDOWN as u8), Some(REASON_EFFECT)), 7, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,7,1-tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 7, 1 - tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local p=1-tp
    let p = 1 - a.tp;
    // local rg=nil
    let mut rg: CardSet;
    // local exg=Duel.GetMatchingGroup(Card.IsAbleToRemove,p,LOCATION_EXTRA,0,nil,p,POS_FACEDOWN,REASON_EFFECT)
    let exg = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_able_to_remove(d, c, Some(p), Some(POS_FACEDOWN as u8), Some(REASON_EFFECT))),
        p,
        LOCATION_EXTRA,
        0,
        NO_EX,
    );
    // if #exg>0 then
    if !exg.is_empty() {
        // local min=math.max(0,7-Duel.GetFieldGroupCount(p,LOCATION_DECK,0))
        let min = 0.max(7 - duel::get_field_group_count(d, p, LOCATION_DECK, 0));
        // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_REMOVE as u64);
        // rg=exg:Select(p,min,7,nil)
        rg = group::select(d, &exg, p, min as u16, 7, false, NO_EX).nil();
        // rg:Merge(Duel.GetDecktopGroup(p,7-#rg))
        let deck = duel::get_decktop_group(d, p, (7 - rg.len() as i32) as u32);
        group::add_card(&mut rg, &deck);
    } else {
        // rg=Duel.GetDecktopGroup(p,7)
        rg = duel::get_decktop_group(d, p, 7);
    }
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Remove(rg,POS_FACEDOWN,REASON_EFFECT,nil,p)
        duel::remove(d, &rg, POS_FACEDOWN as u8, REASON_EFFECT, None, Some(p));
    }
}
