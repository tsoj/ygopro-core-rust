//! Cybernetic Horizon
//! サイバネティック・ホライゾン
use super::prelude::*;

pub const ID: u32 = 63031396;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CYBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
/// `s.counterfilter(c)`
fn counterfilter(d: &mut Duel, a: Args<'_>) -> bool {
    // return c:IsRace(RACE_MACHINE) or c:GetSummonLocation()~=LOCATION_EXTRA
    let c = a.c(0);
    card::is_race(d, c, RACE_MACHINE) || card::get_summon_location(d, c) != LOCATION_EXTRA
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON|RACE_MACHINE) and c:IsSetCard(SET_CYBER) and c:IsAbleToGraveAsCost()
    card::is_race(d, c, RACE_DRAGON | RACE_MACHINE) && card::is_set_card(d, c, &[SET_CYBER as u16]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON|RACE_MACHINE) and c:IsSetCard(SET_CYBER) and c:IsAbleToHand()
    card::is_race(d, c, RACE_DRAGON | RACE_MACHINE) && card::is_set_card(d, c, &[SET_CYBER as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.check(sg,e,tp,mg)`
fn check(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetAttribute)==2 and sg:GetClassCount(Card.GetLocation)==2
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,sg)
    let res = group::get_class_count(d, sg, &|d, c| card::get_attribute(d, c) as i64) == 2
        && group::get_class_count(d, sg, &|d, c| card::get_location(d, c) as i64) == 2
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, sg.into());
    (res, false)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local rg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
    let rg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0
    //     and aux.SelectUnselectGroup(rg,e,tp,2,2,s.check,0) end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0
            && matches!(
                aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(2), Some(2), Some(&check), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.check,1,tp,HINTMSG_TOGRAVE)
    let g = aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(2), Some(2), Some(&check), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // Cannot Special Summon from the Extra Deck, except Machines
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // lizard check
    // aux.addTempLizardCheck(e:GetHandler(),tp,s.lizfilter)
    aux::add_temp_lizard_check(d, h, tp, Some(effect_card_fn(|d, _e, c| lizfilter(d, c))), None, None, None, None);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args<'_>) -> bool {
    // return not c:IsRace(RACE_MACHINE) and c:IsLocation(LOCATION_EXTRA)
    let c = a.c(1);
    !card::is_race(d, c, RACE_MACHINE) && card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsOriginalRace(RACE_MACHINE)
    !card::is_original_race(d, c, RACE_MACHINE)
}
/// `s.gyfilter(c)`
fn gyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsSetCard(SET_CYBER) and c:IsType(TYPE_FUSION) and c:IsAbleToGrave()
    card::is_race(d, c, RACE_MACHINE)
        && card::is_set_card(d, c, &[SET_CYBER as u16])
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_able_to_grave(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.gyfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&gyfilter), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local hg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let hg = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #hg>0 and Duel.SendtoHand(hg,nil,REASON_EFFECT)>0 then
    if !hg.is_empty() && duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,hg)
        duel::confirm_cards(d, 1 - tp, hg);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.gyfilter,tp,LOCATION_EXTRA,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&gyfilter), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then Duel.SendtoGrave(g,REASON_EFFECT) end
        if !g.is_empty() {
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    }
}
