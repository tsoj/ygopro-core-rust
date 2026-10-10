//! Wishes for Eyes of Blue
use super::prelude::*;

pub const ID: u32 = 80326401;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLUEEYES_W_DRAGON, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 Level 1 LIGHT Tuner and 1 Spell/Trap that mentions "Blue-Eyes White Dragon" from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);

    // Equip 1 "Blue-Eyes" monster from your Extra Deck to 1 "Blue-Eyes White Dragon" you control as an Equip Spell that gives it 400 ATK
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_EQUIP);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    effect::set_operation(d, e2, Some(operation(eqop)));
    card::register_effect(d, c, e2, false);
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, h.into());
    }
    duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    ((card::is_level(d, c, &[1]) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_type(d, c, TYPE_TUNER))
        || (card::is_spell_trap(d, c) && card::lists_code(d, c, &[CARD_BLUEEYES_W_DRAGON]) && !card::is_code(d, c, &[ID])))
        && card::is_able_to_hand(d, c, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsMonster,nil)==1
    (group::filter_count(d, sg, &|d, c| card::is_monster(d, c), NO_EX) == 1, false)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        // return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_ATOHAND)
    let rg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #rg>0 then
    if !rg.is_empty() {
        duel::send_to_hand(d, &rg, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &rg);
    }
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_BLUE_EYES as u16]) && card::check_unique_on_field(d, c, tp, None, None) && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_code(d, chkc, &[CARD_BLUEEYES_W_DRAGON]) && card::is_faceup(d, chkc);
    }
    // aux.FaceupFilter(Card.IsCode,CARD_BLUEEYES_W_DRAGON)
    let ff = aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON]));
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&ff), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    duel::select_target(d, tp, Some(&ff), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if tc:IsFacedown() or not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local ec=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_EXTRA,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let ec = group::get_first(d, &g);
    // if ec and Duel.Equip(tp,ec,tc) then
    if let Some(ec) = ec {
        if duel::equip(d, tp, ec, tc, true, false) {
            // The equipped monster gains 400 ATK
            let e1 = effect::create_effect(d, ec);
            effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, 400);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, ec, e1, false);
            // Equip limit
            let e2 = effect::create_effect(d, ec);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
            // function(e,c) return c==tc end
            effect::set_value_fn(d, e2, lua_fn(move |_d, a| a.card(1) == Some(tc)));
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, ec, e2, false);
        }
    }
}
