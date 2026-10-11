//! Vaalmonica Versare
//! ヴァルモニカ・ヴェルサーレ
use super::prelude::*;

pub const ID: u32 = 42193638;

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
    // e1:SetCategory(CATEGORY_RECOVER+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DAMAGE+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_RECOVER + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DAMAGE + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(lua_fn(activate_lua)));
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
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 500);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAALMONICA) and not c:IsCode(id) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_VAALMONICA as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_grave(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`: the operation of the activation
/// (registered as a raw function: "Angello Vaalmonica" / "Dimonno Vaalmonica" call it with a 9th argument)
fn activate_lua(d: &mut Duel, a: Args) {
    let extra = match a.v(8) {
        Value::Int(n) => Some(*n),
        _ => None,
    };
    activate_op(d, &EffArgs::new(a.0), extra);
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
        // Gain 500 LP and excavate cards from the top of your Deck
        // local g=Duel.GetMatchingGroup(Card.IsSetCard,tp,LOCATION_DECK,0,nil,SET_VAALMONICA)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_set_card(d, c, &[SET_VAALMONICA as u16])), tp, LOCATION_DECK, 0, NO_EX);
        // if Duel.Recover(tp,500,REASON_EFFECT)>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,5)) then
        if duel::recover(d, tp, 500, REASON_EFFECT, false, None) > 0 && !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 5)) {
            // local spcard,seq=g:GetMaxGroup(Card.GetSequence)
            let max = group::get_max_group(d, &g, &|d, c| card::get_sequence(d, c) as i64);
            let (max_group, seq) = match max {
                Some((mg, seq)) => (mg, Some(seq)),
                None => (CardSet::new(), None),
            };
            // spcard=spcard:GetFirst()
            let spcard = group::get_first(d, &max_group);
            // if not spcard then return end
            let Some(spcard) = spcard else {
                return;
            };
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ConfirmDecktop(tp,Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)-seq)
            let deck_count = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
            duel::confirm_decktop(d, tp, (deck_count - seq.unwrap_or(0) as i32) as u32);
            // if spcard:IsAbleToHand() then
            if card::is_able_to_hand(d, spcard, None) {
                // Duel.SendtoHand(spcard,nil,REASON_EFFECT)
                duel::send_to_hand(d, spcard, None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,spcard)
                duel::confirm_cards(d, 1 - tp, spcard);
            } else {
                // Duel.SendtoGrave(spcard,REASON_RULE)
                duel::send_to_grave(d, spcard, REASON_RULE, None, None);
            }
        }
    // elseif op==2 then
    } else if op == Some(2) {
        // Take 500 damage and send 1 "Vaalmonica" card from your Deck to the GY
        // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_DECK, 0, NO_EX);
        // if Duel.Damage(tp,500,REASON_EFFECT)>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,6)) then
        if duel::damage(d, tp, 500, REASON_EFFECT, false, None) > 0 && !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 6)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoGrave(sg,REASON_EFFECT)
            duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
        }
    }
}
