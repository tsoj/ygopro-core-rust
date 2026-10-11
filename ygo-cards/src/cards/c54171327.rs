//! Angelechy Disturbance
//! JP name
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 54171327;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_ANGELECHY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Give control of 1 "Angelechy" monster you control to your opponent (until the End Phase), then if your opponent controls a face-up card(s) in its adjacent Monster Zones and/or Spell & Trap Zones, their effects are negated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_CONTROL + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.ctrltg)
    effect::set_target(d, e1, Some(cost_target(ctrltg)));
    // e1:SetOperation(s.ctrlop)
    effect::set_operation(d, e1, Some(operation(ctrlop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY; add 1 "Angelechy" Spell/Trap from your Deck to your hand, except "Angelechy Disturbance"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.ctrlfilter(c)
fn ctrlfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and c:IsControlerCanBeChanged() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_ANGELECHY as u16]) && card::is_controler_can_be_changed(d, c, false, None) && card::is_faceup(d, c)
}
// function s.ctrltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn ctrltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ctrlfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&ctrlfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
// function s.optnegate(...) (a local function of s.ctrlop, with its upvalues passed in)
fn optnegate(d: &mut Duel, tp: u8, c: CardId, break_chk: &mut bool, loc: u32, nseq: u32) {
    // local nc=Duel.GetFieldCard(1-tp,loc,nseq)
    let nc = duel::get_field_card(d, 1 - tp, loc, nseq);
    // if nc and nc:IsNegatable() then
    if let Some(nc) = nc {
        if card::is_negatable(d, nc) {
            // if not break_chk then
            if !*break_chk {
                // break_chk=true
                *break_chk = true;
                // Duel.BreakEffect()
                duel::break_effect(d);
            }
            // nc:NegateEffects(c)
            card::negate_effects(d, nc, c, None, false, None);
        }
    }
}
// function s.ctrlop(e,tp,eg,ep,ev,re,r,rp)
fn ctrlop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.ctrlfilter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&ctrlfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if not (tc and Duel.GetControl(tc,1-tp,PHASE_END,1) and tc:IsControler(1-tp)) then return end
    let Some(tc) = tc else { return };
    // (a Lua number is always true: GetControl only needs to run)
    duel::get_control(d, tc, 1 - tp, PHASE_END as u16, 1, None, None);
    if !card::is_controler(d, tc, 1 - tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local break_chk=false
    let mut break_chk = false;
    // local seq=tc:GetSequence()
    let seq = card::get_sequence(d, tc);
    // optnegate(LOCATION_SZONE,seq) --down
    optnegate(d, tp, c, &mut break_chk, LOCATION_SZONE, seq);
    // if seq>0 then optnegate(LOCATION_MZONE,seq-1) end --left
    if seq > 0 {
        optnegate(d, tp, c, &mut break_chk, LOCATION_MZONE, seq - 1);
    }
    // if seq<4 then optnegate(LOCATION_MZONE,seq+1) end --right
    if seq < 4 {
        optnegate(d, tp, c, &mut break_chk, LOCATION_MZONE, seq + 1);
    }
    // if seq==1 then optnegate(LOCATION_MZONE,5) elseif seq==3 then optnegate(LOCATION_MZONE,6) end --up
    if seq == 1 {
        optnegate(d, tp, c, &mut break_chk, LOCATION_MZONE, 5);
    } else if seq == 3 {
        optnegate(d, tp, c, &mut break_chk, LOCATION_MZONE, 6);
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and c:IsSpellTrap() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ANGELECHY as u16]) && card::is_spell_trap(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
    // if #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
