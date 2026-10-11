//! Erebus the Underworld Monarch
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 23064604;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MONARCH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 1 tribute
    // local e1=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e1 = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(Arc::new(otfilter)), None);
    // local e2=aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e2 = aux::add_normal_set_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(Arc::new(otfilter)), None);
    // to grave
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOGRAVE+CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TOGRAVE + CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCondition(s.tdcon)
    effect::set_condition(d, e3, Some(condition(tdcon)));
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // tohand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_GRAVE);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.thcon)
    effect::set_condition(d, e4, Some(condition(thcon)));
    // e4:SetCost(s.thcost)
    effect::set_cost(d, e4, Some(cost_target(thcost)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.otfilter(c)`
fn otfilter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    // return c:IsTributeSummoned()
    card::is_tribute_summoned(d, c)
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MONARCH) and c:IsSpellTrap() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_MONARCH as u16]) && card::is_spell_trap(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
        // return g:GetClassCount(Card.GetCode)>1
        if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) <= 1 {
            return false;
        }
        //   and Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,1,nil)
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_able_to_deck(d, c)),
            tp,
            0,
            LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 2, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,1-tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, 1 - tp, (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
    let mut g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if g:GetClassCount(Card.GetCode)<2 then return end
    if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) < 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tg1=g:Select(tp,1,1,nil)
    let mut tg1 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // g:Remove(Card.IsCode,nil,tg1:GetFirst():GetCode())
    let first1 = group::get_first(d, &tg1).nil();
    let code1 = card::get_code(d, first1)[0];
    group::remove(d, &mut g, &|d, c| card::is_code(d, c, &[code1]), NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tg2=g:Select(tp,1,1,nil)
    let tg2 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // tg1:Merge(tg2)
    group::add_card(&mut tg1, &tg2);
    // if Duel.SendtoGrave(tg1,REASON_EFFECT)~=0 and tg1:IsExists(Card.IsLocation,2,nil,LOCATION_GRAVE) then
    if duel::send_to_grave(d, tg1.clone(), REASON_EFFECT, None, None) != 0 && group::is_exists(d, &tg1, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 2, NO_EX) {
        // local hg=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_HAND,nil)
        let hg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_HAND, NO_EX);
        // local b1=Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_HAND,1,nil)
        let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_HAND, 1, NO_EX);
        // local b2=Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,1,nil)
        let b2 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
        // local b3=Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_GRAVE,1,nil)
        let b3 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
        // if not b1 and not b2 and not b3 then return end
        if !b1 && !b2 && !b3 {
            return;
        }
        let mut op: i32;
        // if b1 then
        if b1 {
            // if b2 and b3 then
            if b2 && b3 {
                // op=Duel.SelectOption(tp,aux.Stringid(id,3),aux.Stringid(id,4),aux.Stringid(id,5))
                op = duel::select_option(d, tp, true, &[aux::stringid(ID, 3), aux::stringid(ID, 4), aux::stringid(ID, 5)]);
            // elseif b2 and not b3 then
            } else if b2 && !b3 {
                // op=Duel.SelectOption(tp,aux.Stringid(id,3),aux.Stringid(id,4))
                op = duel::select_option(d, tp, true, &[aux::stringid(ID, 3), aux::stringid(ID, 4)]);
            // elseif not b2 and b3 then
            } else if !b2 && b3 {
                // op=Duel.SelectOption(tp,aux.Stringid(id,3),aux.Stringid(id,5))
                op = duel::select_option(d, tp, true, &[aux::stringid(ID, 3), aux::stringid(ID, 5)]);
                // if op==1 then op=2 end
                if op == 1 {
                    op = 2;
                }
            } else {
                // op=0
                op = 0;
            }
        // else
        } else if b2 && b3 {
            // op=Duel.SelectOption(tp,aux.Stringid(id,4),aux.Stringid(id,5))+1
            op = duel::select_option(d, tp, true, &[aux::stringid(ID, 4), aux::stringid(ID, 5)]) + 1;
        } else if b2 && !b3 {
            // op=1
            op = 1;
        } else {
            // op=2
            op = 2;
        }
        // (sg is set by each branch of op)
        let sg: CardSet = if op == 0 {
            // sg=hg:RandomSelect(tp,1)
            group::random_select(d, &hg, tp, 1)
        } else if op == 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
            // sg=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,1,1,nil)
            let s = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(sg)
            duel::hint_selection(d, s.clone(), true);
            s
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
            // sg=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,0,LOCATION_GRAVE,1,1,nil)
            let s = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(sg)
            duel::hint_selection(d, s.clone(), true);
            s
        };
        // Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MONARCH) and c:IsSpellTrap() and c:IsDiscardable()
    card::is_set_card(d, c, &[SET_MONARCH as u16]) && card::is_spell_trap(d, c) && card::is_discardable(d, c, None, None)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttackAbove(2400) and c:GetDefense()==1000 and c:IsAbleToHand()
    card::is_attack_above(d, c, 2400) && card::get_defense(d, c) == 1000 && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
