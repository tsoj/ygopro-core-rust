//! Artifact Durendal
use super::prelude::*;

pub const ID: u32 = 69840739;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 5 monsters
    // Xyz.AddProcedure(c,nil,5,2)
    xyz::add_procedure(d, c, None, Some(5), 2, None, None, None, None, false, None);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local event_chaining,_,_,event_value,reason_effect,_,reason_player=Duel.CheckEvent(EVENT_CHAINING,true)
    let pe = duel::check_event(d, EVENT_CHAINING);
    let event_chaining = pe.is_some();
    let event_value = pe.as_ref().map_or(0, |p| p.event_value);
    let reason_effect = pe.as_ref().and_then(|p| p.reason_effect);
    let reason_player = pe.as_ref().map_or(0, |p| p.reason_player);
    // local b1=event_chaining and Duel.IsExistingMatchingCard(Card.IsSpellTrap,reason_player,0,LOCATION_ONFIELD,1,nil)
    //     and ((reason_effect:IsMonsterEffect() and Duel.GetChainInfo(event_value,CHAININFO_TRIGGERING_LOCATION)==LOCATION_MZONE)
    //     or (reason_effect:GetHandler():IsNormalSpellTrap() and reason_effect:IsHasType(EFFECT_TYPE_ACTIVATE)))
    let b1 = event_chaining
        && duel::is_existing_matching_card(d, Some(&card::is_spell_trap), reason_player, 0, LOCATION_ONFIELD, 1, NO_EX)
        && {
            let re = reason_effect.nil();
            let monster_ok = effect::is_monster_effect(d, re)
                && duel::get_chain_info(d, event_value as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int() == LOCATION_MZONE as i64;
            monster_ok || {
                let h = effect::get_handler(d, re).nil();
                card::is_normal_spell_trap(d, h) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
            }
        };
    // local b2=(Duel.IsPlayerCanDraw(tp) or Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0)
    //     and (Duel.IsPlayerCanDraw(1-tp) or Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)==0)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_HAND,LOCATION_HAND,1,nil)
    let b2 = (duel::is_player_can_draw(d, tp, 0) || duel::get_field_group_count(d, tp, LOCATION_HAND, 0) == 0)
        && (duel::is_player_can_draw(d, 1 - tp, 0) || duel::get_field_group_count(d, tp, 0, LOCATION_HAND) == 0)
        && duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), tp, LOCATION_HAND, LOCATION_HAND, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=not b1 and 2 or Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op: i64 = if !b1 {
        2
    } else {
        duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil() as i64
    };
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // Duel.SetTargetParam(event_value)
        duel::set_target_param(d, event_value);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_TODECK + CATEGORY_DRAW);
        // if not b1 then Duel.Hint(HINT_OPSELECTED,1-tp,aux.Stringid(id,2)) end
        if !b1 {
            duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, aux::stringid(ID, 2));
        }
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_ALL,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_ALL as u8, LOCATION_HAND as i32);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // The activated effect becomes "Destroy 1 Spell/Trap Card your opponent controls"
        // local g=Group.CreateGroup()
        let g = CardSet::new();
        // local event_value=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
        let event_value = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u8;
        // Duel.ChangeTargetCard(event_value,g)
        duel::change_target_card(d, event_value, &g);
        // Duel.ChangeChainOperation(event_value,s.repop)
        duel::change_chain_operation(d, event_value, operation(repop));
    } else if op == 2 {
        // Each player with a hand shuffles their entire hand into the Deck, then draws the same number of cards they shuffled into the Deck
        // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,LOCATION_HAND)
        let g = duel::get_field_group(d, tp, LOCATION_HAND, LOCATION_HAND);
        // if Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT)>0 then
        if duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 0 {
            // local og=Duel.GetOperatedGroup():Match(Card.IsLocation,nil,LOCATION_DECK)
            let operated = duel::get_operated_group(d);
            let og = group::filter(d, &operated, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
            // local turn_player=Duel.GetTurnPlayer()
            let turn_player = duel::get_turn_player(d);
            // local non_turn_player=1-turn_player
            let non_turn_player = 1 - turn_player;
            // if og:IsExists(Card.IsControler,1,nil,turn_player) then Duel.ShuffleDeck(turn_player) end
            if group::is_exists(d, &og, &|d, c| card::is_controler(d, c, turn_player), 1, NO_EX) {
                duel::shuffle_deck(d, turn_player);
            }
            // if og:IsExists(Card.IsControler,1,nil,non_turn_player) then Duel.ShuffleDeck(non_turn_player) end
            if group::is_exists(d, &og, &|d, c| card::is_controler(d, c, non_turn_player), 1, NO_EX) {
                duel::shuffle_deck(d, non_turn_player);
            }
            // local ct1=og:FilterCount(Card.IsPreviousControler,nil,turn_player)
            let ct1 = group::filter_count(d, &og, &|d, c| card::is_previous_controler(d, c, turn_player), NO_EX);
            // local ct2=og:FilterCount(Card.IsPreviousControler,nil,non_turn_player)
            let ct2 = group::filter_count(d, &og, &|d, c| card::is_previous_controler(d, c, non_turn_player), NO_EX);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if ct1>0 then
            if ct1 > 0 {
                // Duel.Draw(turn_player,ct1,REASON_EFFECT)
                duel::draw(d, turn_player, ct1 as u32, REASON_EFFECT);
            }
            // if ct2>0 then
            if ct2 > 0 {
                // Duel.Draw(non_turn_player,ct2,REASON_EFFECT)
                duel::draw(d, non_turn_player, ct2 as u32, REASON_EFFECT);
            }
        }
    }
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsNormalSpellTrap() then
    if card::is_normal_spell_trap(d, c) {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
