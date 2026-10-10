//! Bingo Machine, Go!!!
use super::prelude::*;

pub const ID: u32 = 93437091;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLUE_EYES], listed_names: &[CARD_BLUEEYES_W_DRAGON, 23995346], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return ( ( (c:IsSetCard(SET_BLUE_EYES) and c:IsMonster()) ) or
    //     ( (c:ListsCode(CARD_BLUEEYES_W_DRAGON) or c:ListsCode(23995346)) and c:IsSpellTrap() and not c:IsCode(id) ) )
    //     and c:IsAbleToHand()
    let found = (card::is_set_card(d, c, &[SET_BLUE_EYES as u16]) && card::is_monster(d, c))
        || ((card::lists_code(d, c, &[CARD_BLUEEYES_W_DRAGON]) || card::lists_code(d, c, &[23995346]))
            && card::is_spell_trap(d, c)
            && !card::is_code(d, c, &[ID]));
    found && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,3,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 3, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>=3 then
    if g.len() >= 3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local sg=g:Select(tp,3,3,nil)
        let sg = group::select(d, &g, tp, 3, 3, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg.clone());
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_ATOHAND as u64);
        // local tc=sg:Select(1-tp,1,1,nil):GetFirst()
        let sel = group::select(d, &sg, 1 - tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel);
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc.nil(), None, REASON_EFFECT, None);
    }
}
