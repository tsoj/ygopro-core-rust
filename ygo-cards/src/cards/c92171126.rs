//! Deftero Enneacraft - "alazoneIA"
use super::prelude::*;

pub const ID: u32 = 92171126;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_EC], listed_series: &[SET_ENNEACRAFT as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_EC,LOCATION_PZONE)
    card::enable_counter_permit(d, c, COUNTER_EC as u16, Some(LOCATION_PZONE), None);
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Each time a monster(s) is flipped face-up, place 1 E.C. Counter on this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_FLIP)
    effect::set_code(d, e1, EVENT_FLIP);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetOperation(s.counterop)
    effect::set_operation(d, e1, Some(operation(counterop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Reveal 3 "Enneacraft" cards from your Deck, your opponent randomly picks 1 for you to add to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.PayLP(900))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(900))));
    // e2:SetTarget(s.rvtg)
    effect::set_target(d, e2, Some(cost_target(rvtg)));
    // e2:SetOperation(s.rvop)
    effect::set_operation(d, e2, Some(operation(rvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 monster from your hand in face-down Defense Position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCost(Cost.AND(Cost.SelfReveal,s.spcost))
    effect::set_cost(d, e3, Some(cost::and(vec![cost::self_reveal(), cost_target(spcost)])));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return c:IsPosition(POS_FACEDOWN_DEFENSE) end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counter_filter));
    // Add 1 "Enneacraft" card from your Deck to your hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e4, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.thcon)
    effect::set_condition(d, e4, Some(condition(thcon)));
    // e4:SetCost(Cost.SelfChangePosition(POS_FACEUP_DEFENSE))
    effect::set_cost(d, e4, Some(cost::self_change_position(POS_FACEUP_DEFENSE)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Banish 1 random card from your opponent's hand face-up until the End Phase
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e5, aux::stringid(ID, 3));
    // e5:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e5, CATEGORY_REMOVE);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_FLIP)
    effect::set_code(d, e5, EVENT_FLIP);
    // e5:SetTarget(s.rmtg)
    effect::set_target(d, e5, Some(cost_target(rmtg)));
    // e5:SetOperation(s.rmop)
    effect::set_operation(d, e5, Some(operation(rmop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.counterop(e,tp,eg,ep,ev,re,r,rp)
fn counterop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // c:AddCounter(COUNTER_EC,1)
        card::add_counter(d, c, COUNTER_EC as u16, 1, false);
    } else {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
        // e1:SetRange(LOCATION_PZONE)
        effect::set_range(d, e1, LOCATION_PZONE);
        // e1:SetOperation(function() c:AddCounter(COUNTER_EC,1) end)
        effect::set_operation(d, e1, Some(operation(move |d, _a| {
            card::add_counter(d, c, COUNTER_EC as u16, 1, false);
        })));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
        // e2:SetOperation(function() e1:Reset() end)
        effect::set_operation(d, e2, Some(operation(move |d, _a| {
            effect::reset(d, e1);
        })));
        // e2:SetReset(RESET_CHAIN)
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
    }
}
// function s.rvfilter(c)
fn rvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ENNEACRAFT) and c:IsAbleToHand() and not c:IsPublic()
    card::is_set_card(d, c, &[SET_ENNEACRAFT as u16]) && card::is_able_to_hand(d, c, None) && !card::is_public(d, c)
}
// function s.rvtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rvfilter,tp,LOCATION_DECK,0,3,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rvfilter), tp, LOCATION_DECK, 0, 3, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.rvop(e,tp,eg,ep,ev,re,r,rp)
fn rvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rvfilter,tp,LOCATION_DECK,0,3,3,nil)
    let g = duel::select_matching_card(d, tp, Some(&rvfilter), tp, LOCATION_DECK, 0, 3, 3, false, NO_EX).nil();
    // if #g~=3 then return end
    if g.len() != 3 {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // local sg=g:RandomSelect(1-tp,1)
    let sg = group::random_select(d, &g, 1 - tp, 1);
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
    }
}
// function(c) return c:IsPosition(POS_FACEDOWN_DEFENSE) end  (the SPSUMMON activity counter)
fn counter_filter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    card::is_position(d, c, POS_FACEDOWN_DEFENSE)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
    // e1:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e1, aux::stringid(ID, 4));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_FORCE_SPSUMMON_POSITION)
    effect::set_code(d, e1, EFFECT_FORCE_SPSUMMON_POSITION);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(POS_FACEDOWN_DEFENSE)
    effect::set_value(d, e1, POS_FACEDOWN_DEFENSE as i64);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
// function s.spfilter(c,e,tp) (Card.IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE))
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(Card.IsCanBeSpecialSummoned,tp,LOCATION_HAND,0,1,nil,e,0,tp,false,false,POS_FACEDOWN_DEFENSE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsCanBeSpecialSummoned,tp,LOCATION_HAND,0,1,1,nil,e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
    }
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if ep==tp or e:GetHandler():IsFaceup() then return false end
    if a.ep == tp || card::is_faceup(d, effect::get_handler(d, e).nil()) {
        return false;
    }
    let re = a.re.nil();
    // if re:IsHasCategory(CATEGORY_SEARCH) or re:IsHasCategory(CATEGORY_DRAW) then return true end
    if effect::is_has_category(d, re, CATEGORY_SEARCH) || effect::is_has_category(d, re, CATEGORY_DRAW) {
        return true;
    }
    // local ex1,g1,gc1,dp1,loc1=Duel.GetOperationInfo(ev,CATEGORY_TOHAND)
    let info1 = duel::get_operation_info(d, a.ev as u32, CATEGORY_TOHAND);
    let g1 = info1.as_ref().and_then(|i| i.op_cards.clone());
    let loc1 = info1.as_ref().map_or(0, |i| i.op_param);
    // local ex2,g2,gc2,dp2,loc2=Duel.GetPossibleOperationInfo(ev,CATEGORY_TOHAND)
    let info2 = duel::get_possible_operation_info(d, a.ev as u32, CATEGORY_TOHAND);
    let g2 = info2.as_ref().and_then(|i| i.op_cards.clone());
    let loc2 = info2.as_ref().map_or(0, |i| i.op_param);
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // if g1 then g:Merge(g1) end
    if let Some(g1) = g1 {
        group::add_card(&mut g, &g1);
    }
    // if g2 then g:Merge(g2) end
    if let Some(g2) = g2 {
        group::add_card(&mut g, &g2);
    }
    // return (((loc1 or 0)|(loc2 or 0))&LOCATION_DECK)>0 or (#g>0 and g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK))
    ((loc1 | loc2) & LOCATION_DECK as i32) > 0 || (!g.is_empty() && group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX))
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ENNEACRAFT) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ENNEACRAFT as u16]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_HAND as i32);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_HAND, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local rg=g:RandomSelect(tp,1)
    let rg = group::random_select(d, &g, tp, 1);
    // if #rg>0 then
    if !rg.is_empty() {
        // --Banish 1 random card from your opponent's hand face-up until the End Phase
        // aux.RemoveUntil(rg,POS_FACEUP,REASON_EFFECT|REASON_TEMPORARY,PHASE_END,id,e,tp,function(ag,e,tp) Duel.SendtoHand(ag,nil,REASON_EFFECT) end)
        let oper: aux::DelayedFn = std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
            duel::send_to_hand(d, ag.clone(), None, REASON_EFFECT, None);
            true
        });
        aux::remove_until(
            d,
            rg,
            Some(POS_FACEUP as u8),
            REASON_EFFECT | REASON_TEMPORARY,
            PHASE_END,
            ID,
            a.e,
            tp,
            Some(oper),
            None,
            None,
            None,
            None,
            None,
        );
    }
}
