//! Fire Fortress atop Liang Peak
//! Scripted by ahtelel
use super::prelude::*;

pub const ID: u32 = 66750703;

pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[0x201], listed_series: &[SET_FIRE_FIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can hold Fire Fist Counters
    // c:EnableCounterPermit(0x201)
    card::enable_counter_permit(d, c, 0x201, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 1 counter on it each time a "Fire Fist" monster(s) is normal summoned
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Same as above, but for special summons
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Remove 2 counters: opponent cannot activate card/effects until end of damage step
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e4, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e4:SetCost(s.cost1)
    effect::set_cost(d, e4, Some(cost_target(cost1)));
    // e4:SetOperation(s.op1)
    effect::set_operation(d, e4, Some(operation(op1)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Remove 6 counters: add 1 beast-warrior monster from deck
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e5, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e5, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e5:SetCost(s.cost2)
    effect::set_cost(d, e5, Some(cost_target(cost2)));
    // e5:SetTarget(s.tg2)
    effect::set_target(d, e5, Some(cost_target(tg2)));
    // e5:SetOperation(s.op2)
    effect::set_operation(d, e5, Some(operation(op2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Remove 10 counters: special summon 1 beast-warrior monster from deck or extra deck
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e6, CATEGORY_SPECIAL_SUMMON);
    // e6:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e6, aux::stringid(ID, 2));
    // e6:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e6, EFFECT_TYPE_IGNITION);
    // e6:SetRange(LOCATION_FZONE)
    effect::set_range(d, e6, LOCATION_FZONE);
    // e6:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e6, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e6:SetCost(s.cost3)
    effect::set_cost(d, e6, Some(cost_target(cost3)));
    // e6:SetTarget(s.tg3)
    effect::set_target(d, e6, Some(cost_target(tg3)));
    // e6:SetOperation(s.op3)
    effect::set_operation(d, e6, Some(operation(op3)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
// function s.ctfilter(c)
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_FIRE_FIST)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_FIRE_FIST as u16])
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(s.ctfilter,1,nil) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &ctfilter, 1, NO_EX) {
        // e:GetHandler():AddCounter(0x201,1)
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, 0x201, 1, false);
    }
}
// function s.cost1(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,0x201,2,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, tp, 1, 0, 0x201, 2, REASON_COST);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // Duel.RemoveCounter(tp,1,0,0x201,2,REASON_COST)
    duel::remove_counter(d, tp, 1, 0, 0x201, 2, REASON_COST);
    true
}
// function s.op1(e,tp,eg,ep,ev,re,r,rp)
fn op1(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetCondition(s.actcon)
    effect::set_condition(d, e1, Some(condition(actcon)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE + PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function s.actcon(e)
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return tc and tc:IsControler(tp) and tc:IsRace(RACE_BEASTWARRIOR)
    match duel::get_attacker(d) {
        Some(tc) => card::is_controler(d, tc, tp) && card::is_race(d, tc, RACE_BEASTWARRIOR),
        None => false,
    }
}
// function s.cost2(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,0x201,6,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, tp, 1, 0, 0x201, 6, REASON_COST);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // Duel.RemoveCounter(tp,1,0,0x201,6,REASON_COST)
    duel::remove_counter(d, tp, 1, 0, 0x201, 6, REASON_COST);
    true
}
// function s.sfilter(c)
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsRace(RACE_BEASTWARRIOR) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_race(d, c, RACE_BEASTWARRIOR) && card::is_able_to_hand(d, c, None)
}
// function s.tg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.sfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&sfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.op2(e,tp,eg,ep,ev,re,r,rp)
fn op2(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.sfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&sfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
// function s.cost3(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost3(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,0x201,10,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, tp, 1, 0, 0x201, 10, REASON_COST);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // Duel.RemoveCounter(tp,1,0,0x201,10,REASON_COST)
    duel::remove_counter(d, tp, 1, 0, 0x201, 10, REASON_COST);
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,nil,c)==0 then return false end
    if card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 == 0 {
        return false;
    }
    // return c:IsRace(RACE_BEASTWARRIOR) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_race(d, c, RACE_BEASTWARRIOR) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
// function s.tg3(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tg3(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local loc=LOCATION_EXTRA
    let mut loc = LOCATION_EXTRA;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then loc=loc|LOCATION_DECK end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        loc |= LOCATION_DECK;
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,loc,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, loc, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,loc)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, loc as i32);
    true
}
// function s.op3(e,tp,eg,ep,ev,re,r,rp)
fn op3(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local loc=LOCATION_EXTRA
    let mut loc = LOCATION_EXTRA;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then loc=loc|LOCATION_DECK end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        loc |= LOCATION_DECK;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,loc,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, loc, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
