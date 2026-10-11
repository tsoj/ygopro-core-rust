//! Vaalmonica Scelta
//! ヴァルモニカ・シェルタ
use super::prelude::*;

pub const ID: u32 = 5605529;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_VAALMONICA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RECOVER+CATEGORY_TODECK+CATEGORY_DRAW+CATEGORY_DAMAGE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(
        d,
        e1,
        CATEGORY_RECOVER + CATEGORY_TODECK + CATEGORY_DRAW + CATEGORY_DAMAGE + CATEGORY_TOHAND + CATEGORY_SEARCH,
    );
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, 500);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, tp, 500);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAALMONICA) and c:IsSpellTrap() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_VAALMONICA as u16])
        && card::is_spell_trap(d, c)
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`: the operation of the activation
fn activate(d: &mut Duel, a: &EffArgs) {
    activate_op(d, a, None);
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp,angello_or_dimonno)`: the 9th parameter is used by
/// "Angello Vaalmonica" and "Dimonno Vaalmonica"
pub fn activate_op(d: &mut Duel, a: &EffArgs, angello_or_dimonno: Option<i64>) {
    let tp = a.tp;
    // local op=nil
    // if angello_or_dimonno then
    let op: Option<i64> = if let Some(v) = angello_or_dimonno {
        // op=angello_or_dimonno
        Some(v)
    } else {
        // local sel_player=Duel.IsExistingMatchingCard(Card.IsSetCard,tp,LOCATION_PZONE,0,1,nil,SET_VAALMONICA) and tp or 1-tp
        let sel_player = if duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_VAALMONICA as u16])), tp, LOCATION_PZONE, 0, 1, NO_EX) {
            tp
        } else {
            1 - tp
        };
        // local offset=sel_player==1-tp and 2 or 0
        let offset = if sel_player == 1 - tp { 2 } else { 0 };
        // op=Duel.SelectEffect(sel_player,{true,aux.Stringid(id,1+offset)},{true,aux.Stringid(id,2+offset)})
        duel::select_effect(
            d,
            sel_player,
            &[(true, aux::stringid(ID, 1 + offset)), (true, aux::stringid(ID, 2 + offset))],
        )
        .map(|v| v as i64)
    };
    // if op==1 then
    if op == Some(1) {
        // Gain 500 LP and draw 2 cards
        // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_HAND,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_HAND, 0, NO_EX);
        // if Duel.Recover(tp,500,REASON_EFFECT)>0 and #g>0 and Duel.IsPlayerCanDraw(tp,2)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,5)) then
        if duel::recover(d, tp, 500, REASON_EFFECT, false, None) > 0
            && !g.is_empty()
            && duel::is_player_can_draw(d, tp, 2)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 5))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
            // local sc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if not (Duel.SendtoDeck(sc,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_DECK)) then return end
            if !(duel::send_to_deck(d, sc, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_DECK)) {
                return;
            }
            // Duel.Draw(tp,2,REASON_EFFECT)
            duel::draw(d, tp, 2, REASON_EFFECT);
        }
    // elseif op==2 then
    } else if op == Some(2) {
        // Take 500 damage and search 1 "Valmonica" Spell/Trap
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        // if Duel.Damage(tp,500,REASON_EFFECT)>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,6)) then
        if duel::damage(d, tp, 500, REASON_EFFECT, false, None) > 0 && !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 6)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, sg);
        }
    }
}
