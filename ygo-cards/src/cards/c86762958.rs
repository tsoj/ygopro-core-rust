//! Assist★Yummy!
//! アシスト★ヤミー！
use super::prelude::*;

pub const ID: u32 = 86762958;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_YUMMY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START|TIMING_BATTLE_PHASE|TIMING_BATTLE_STEP_END|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e0, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_START | TIMING_BATTLE_PHASE | TIMING_BATTLE_STEP_END | TIMING_BATTLE_END));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Gain 100 LP, and if you do, your opponent pays 100 LP
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_STANDBY);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.lptg)
    effect::set_target(d, e1, Some(cost_target(lptg)));
    // e1:SetOperation(s.lpop)
    effect::set_operation(d, e1, Some(operation(lpop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 card from the field or GY to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsMainPhase() or Duel.IsBattlePhase(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_main_phase(d, None) || duel::is_battle_phase(d, Some(1 - a.tp)))));
    // e2:SetCost(s.retthcost)
    effect::set_cost(d, e2, Some(cost_target(retthcost)));
    // e2:SetTarget(s.retthtg)
    effect::set_target(d, e2, Some(cost_target(retthtg)));
    // e2:SetOperation(s.retthop)
    effect::set_operation(d, e2, Some(operation(retthop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START|TIMING_BATTLE_PHASE|TIMING_BATTLE_STEP_END|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_START | TIMING_BATTLE_PHASE | TIMING_BATTLE_STEP_END | TIMING_BATTLE_END));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send 1 "Yummy" card from your Deck to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.tgtg)
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    // e3:SetOperation(s.tgop)
    effect::set_operation(d, e3, Some(operation(tgop)));
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.lptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckLPCost(1-tp,100) end
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, 1 - a.tp, 100);
    }
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,100)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 100);
    true
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    // if Duel.Recover(tp,100,REASON_EFFECT)>0 and Duel.CheckLPCost(1-tp,100) then
    if duel::recover(d, a.tp, 100, REASON_EFFECT, false, None) > 0 && duel::check_lp_cost(d, 1 - a.tp, 100) {
        // Duel.PayLPCost(1-tp,100)
        duel::pay_lp_cost(d, 1 - a.tp, 100);
    }
}
/// `s.costfilter(c,exc)`
fn costfilter(d: &mut Duel, c: CardId, exc: CardId) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsFaceup() and c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToHand,0,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,c+c:GetEquipGroup()+exc)
    if !(card::is_set_card(d, c, &[SET_YUMMY as u16]) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)) {
        return false;
    }
    let mut ex = CardSet::from_card(c);
    let eq = card::get_equip_group(d, c);
    group::add_card(&mut ex, &eq);
    group::add_card(&mut ex, &CardSet::from_card(exc));
    duel::is_existing_matching_card(
        d,
        Some(&|d, x| card::is_able_to_hand(d, x, None)),
        0,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        1,
        (&ex).into(),
    )
}
/// `s.retthcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn retthcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsStatus(STATUS_EFFECT_ENABLED) and c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_ONFIELD,0,1,c,c) end
    if a.chk == Some(0) {
        return card::is_status(d, c, STATUS_EFFECT_ENABLED)
            && card::is_able_to_grave_as_cost(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, x| costfilter(d, x, c)), tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_ONFIELD,0,1,1,c,c)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| costfilter(d, x, c)), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    // Duel.SendtoGrave(g+c,REASON_COST)
    let mut gc = g;
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::send_to_grave(d, gc, REASON_COST, None, None);
    true
}
/// `s.retthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn retthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::is_able_to_hand(d, x, None)),
            a.tp,
            LOCATION_ONFIELD | LOCATION_GRAVE,
            LOCATION_ONFIELD | LOCATION_GRAVE,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
/// `s.retthop(e,tp,eg,ep,ev,re,r,rp)`
fn retthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(Card.IsAbleToHand),tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,1,nil)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, x| card::is_able_to_hand(d, x, None))),
        tp,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsAbleToGrave() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_YUMMY as u16]) && card::is_able_to_grave(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
