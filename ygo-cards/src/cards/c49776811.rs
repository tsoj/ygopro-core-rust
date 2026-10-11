//! Shining Piecephilia
use super::prelude::*;

pub const ID: u32 = 49776811;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[0x20a],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x20a)
    card::enable_counter_permit(d, c, 0x20a, Some(LOCATION_MZONE), None);
    // c:SetCounterLimit(0x20a,4)
    card::set_counter_limit(d, c, 0x20a, 4);
    // Cannot be destroyed by battle
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // Place counter and apply effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_COUNTER + CATEGORY_DRAW + CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    effect::set_target(d, e2, Some(cost_target(cttg)));
    effect::set_operation(d, e2, Some(operation(ctop)));
    card::register_effect(d, c, e2, false);
}
// function s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return c:GetBattleTarget() and c:IsStatus(STATUS_OPPO_BATTLE) and c:IsLocation(LOCATION_MZONE) and c:IsRelateToBattle() and c:IsCanAddCounter(0x20a,1)
        return card::get_battle_target(d, c).is_some()
            && card::is_status(d, c, STATUS_OPPO_BATTLE)
            && card::is_location(d, c, LOCATION_MZONE)
            && card::is_relate_to_battle(d, c)
            && card::is_can_add_counter(d, c, 0x20a, 1, false, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,0x20a)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, 0x20a);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,1,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, a.tp, 1);
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand() and c:IsMonster()
    card::is_able_to_hand(d, c, None) && card::is_monster(d, c)
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup() and c:AddCounter(0x20a,1)) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::add_counter(d, c, 0x20a, 1, false)) {
        return;
    }
    // local ct=c:GetCounter(0x20a)
    let ct = card::get_counter(d, c, 0x20a);
    if ct == 1
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_monster(d, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local g=Duel.SelectMatchingCard(tp,Card.IsMonster,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_monster(d, c)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g);
        // if tc then
        if let Some(tc) = tc {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ShuffleDeck(tp)
            duel::shuffle_deck(d, tp);
            // Duel.MoveSequence(tc,0)
            duel::move_sequence(d, tc, 0, None);
            // Duel.ConfirmDecktop(tp,1)
            duel::confirm_decktop(d, tp, 1);
        }
    } else if ct == 2 && duel::is_player_can_draw(d, tp, 1) && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    } else if ct == 3
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    } else if ct == 4
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
    }
}
