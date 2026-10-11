//! Afterglow
use super::prelude::*;

pub const ID: u32 = 43575579;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH+EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e1, 1, ID, 0, (EFFECT_COUNT_CODE_OATH + EFFECT_COUNT_CODE_DUEL) as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(id) and c:IsAbleToRemove() and (not c:IsLocation(LOCATION_ONFIELD) or c:IsFaceup())
    card::is_code(d, c, &[ID]) && card::is_able_to_remove(d, c, None, None, None) && (!card::is_location(d, c, LOCATION_ONFIELD) || card::is_faceup(d, c))
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(id) and c:IsAbleToDeck()
    card::is_code(d, c, &[ID]) && card::is_able_to_deck(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToRemove() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_able_to_remove(d, h, None, None, None);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.Remove(c,POS_FACEUP,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_DECK|LOCATION_ONFIELD|LOCATION_HAND|LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_DECK | LOCATION_ONFIELD | LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, &g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        // if Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_REMOVED,0,1,nil) then
        if duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
            // local td=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_REMOVED,0,1,1,nil)
            let td = duel::select_matching_card(d, tp, Some(&tdfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
            // Duel.SendtoDeck(td,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, td, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetProperty(EFFECT_FLAG_DELAY)
            effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
            // e1:SetCode(EVENT_DRAW)
            effect::set_code(d, e1, EVENT_DRAW);
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetCondition(s.con)
            effect::set_condition(d, e1, Some(condition(con)));
            // e1:SetOperation(s.op)
            effect::set_operation(d, e1, Some(operation(op)));
            // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_DRAW) then
            if duel::is_turn_player(d, tp) && duel::is_phase(d, PHASE_DRAW) {
                // e1:SetLabel(Duel.GetTurnCount())
                let turn = duel::get_turn_count(d, None) as i64;
                effect::set_label(d, e1, &[turn]);
                // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN,2)
                effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, 2);
            } else {
                // e1:SetLabel(0)
                effect::set_label(d, e1, &[0]);
                // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN)
                effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, 1);
            }
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
        }
    }
}
// function s.con(e,tp,eg,ep,ev,re,r,rp)
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and (r&REASON_RULE)~=0 and Duel.IsTurnPlayer(tp) and Duel.GetTurnCount()~=e:GetLabel()
    //     and Duel.IsPhase(PHASE_DRAW) and #eg>0
    a.ep == a.tp
        && (a.r & REASON_RULE) != 0
        && duel::is_turn_player(d, a.tp)
        && duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e)
        && duel::is_phase(d, PHASE_DRAW)
        && !a.eg.clone().nil().is_empty()
}
// function s.op(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if #eg<1 then return end
    let eg = a.eg.clone().nil();
    if eg.len() < 1 {
        return;
    }
    // Duel.ConfirmCards(1-tp,eg)
    duel::confirm_cards(d, 1 - tp, &eg);
    // local g=eg:Filter(Card.IsCode,nil,id)
    let g = group::filter(d, &eg, &|d, c| card::is_code(d, c, &[ID]), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // Duel.Damage(1-tp,4000,REASON_EFFECT)
        duel::damage(d, 1 - tp, 4000, REASON_EFFECT, false, None);
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
}
