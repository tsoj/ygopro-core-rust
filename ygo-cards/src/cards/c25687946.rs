//! Melffy Footsteps
use super::prelude::*;

pub const ID: u32 = 25687946;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_MELFFY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
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
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return c:IsSetCard(SET_MELFFY) and c:IsFaceup() end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(activity_counter));
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thdtg)
    effect::set_target(d, e2, Some(cost_target(thdtg)));
    // e2:SetOperation(s.thdop)
    effect::set_operation(d, e2, Some(operation(thdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(c) return c:IsSetCard(SET_MELFFY) and c:IsFaceup() end`
fn activity_counter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    card::is_set_card(d, c, &[SET_MELFFY as u16]) && card::is_faceup(d, c)
}
/// `s.cost(e,tp,...,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsSetCard(SET_MELFFY) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_MELFFY as u16]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MELFFY) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MELFFY as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local thg=aux.SelectUnselectGroup(g,e,tp,1,4,aux.dncheck,1,tp,HINTMSG_ATOHAND)
    let dn = aux::dncheck();
    let thg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(4), Some(&dn), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false)
        .unwrap_or_default();
    // if #thg>0 and Duel.SendtoHand(thg,nil,REASON_EFFECT)>0 then
    if !thg.is_empty() && duel::send_to_hand(d, thg.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,thg)
        duel::confirm_cards(d, 1 - tp, thg);
        // local turn_player=Duel.GetTurnPlayer()
        let turn_player = duel::get_turn_player(d);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.IsMainPhase1() then
        if duel::is_main_phase1(d, None) {
            // Duel.SkipPhase(turn_player,PHASE_MAIN1,RESET_PHASE|PHASE_END,1)
            duel::skip_phase(d, turn_player, PHASE_MAIN1, RESET_PHASE | PHASE_END, 1, 0);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e1:SetCode(EFFECT_CANNOT_BP)
            effect::set_code(d, e1, EFFECT_CANNOT_BP);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,turn_player)
            duel::register_effect(d, e1, turn_player);
        // elseif Duel.IsMainPhase2() then
        } else if duel::is_main_phase2(d, None) {
            // Duel.SkipPhase(turn_player,PHASE_MAIN2,RESET_PHASE|PHASE_END,1)
            duel::skip_phase(d, turn_player, PHASE_MAIN2, RESET_PHASE | PHASE_END, 1, 0);
        }
    }
}
/// `s.thdfilter(c)`
fn thdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MELFFY) and (c:IsAbleToHand() or c:IsAbleToDeck()) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_MELFFY as u16])
        && (card::is_able_to_hand(d, c, None) || card::is_able_to_deck(d, c))
        && !card::is_code(d, c, &[ID])
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsAbleToHand,nil)>=1 and sg:FilterCount(Card.IsAbleToDeck,nil)>=1
    let h = group::filter_count(d, sg, &|d, c| card::is_able_to_hand(d, c, None), NO_EX);
    let dk = group::filter_count(d, sg, &|d, c| card::is_able_to_deck(d, c), NO_EX);
    (h >= 1 && dk >= 1, false)
}
/// `s.thdtg(e,tp,...,chk,chkc)`
fn thdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetTargetGroup(s.thdfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_target_group(d, Some(&thdfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,aux.Stringid(id,3))
    let tg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(aux::stringid(ID, 3)), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg.clone()), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg), 1, tp, 0);
    true
}
/// `s.gythfilter(c,tg)`
fn gythfilter(d: &mut Duel, c: CardId, tg: &CardSet) -> bool {
    // return c:IsAbleToHand() and tg:IsExists(Card.IsAbleToDeck,1,c)
    card::is_able_to_hand(d, c, None) && group::is_exists(d, tg, &|d, cc| card::is_able_to_deck(d, cc), 1, c.into())
}
/// `s.thdop(e,tp,...)`
fn thdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let mut tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==2 then
    if tg.len() == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local hg=tg:FilterSelect(tp,s.gythfilter,1,1,nil,tg)
        let tg_snapshot = tg.clone();
        let hg = group::filter_select(d, &tg, tp, &|d, c| gythfilter(d, c, &tg_snapshot), 1, 1, false, NO_EX).nil();
        // if #hg==0 then return end
        if hg.is_empty() {
            return;
        }
        // Duel.HintSelection(hg)
        duel::hint_selection(d, hg.clone(), true);
        // if Duel.SendtoHand(hg,nil,REASON_EFFECT)==0 then return end
        if duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None) == 0 {
            return;
        }
        // local dg=tg-hg
        group::remove_card(&mut tg, &hg);
        let dg = tg;
        // if #dg==0 then return end
        if dg.is_empty() {
            return;
        }
        // Duel.HintSelection(dg)
        duel::hint_selection(d, dg.clone(), true);
        // Duel.SendtoDeck(dg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, dg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
