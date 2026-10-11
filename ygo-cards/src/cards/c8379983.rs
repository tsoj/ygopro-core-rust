//! Lunalight Gold Leo
use super::prelude::*;

pub const ID: u32 = 8379983;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LUNALIGHT], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Lunalight" monster from your Deck to your hand, except "Lunalight Gold Leo", then discard 1 card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(deckthtg)));
    effect::set_operation(d, e1, Some(operation(deckthop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // Add 1 "Lunalight" monster sent to the GY to the hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOHAND);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e3, EVENT_CUSTOM + ID);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(gythtg)));
    effect::set_operation(d, e3, Some(operation(gythop)));
    card::register_effect(d, c, e3, false);
    // local g=Group.CreateGroup() g:KeepAlive() e3:SetLabelObject(g)
    effect::set_label_object(d, e3, Value::Group(CardSet::new()));
    // Keep track of "Lunalight" monsters sent to the GY
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_label_object(d, e4, Value::Effect(e3));
    effect::set_operation(d, e4, Some(operation(regop)));
    card::register_effect(d, c, e4, false);
}
/// `s.deckthfilter(c)`
fn deckthfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_LUNALIGHT as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
fn deckthtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&deckthfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
fn deckthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&deckthfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty()
        && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0
        && card::is_location(d, group::get_first(d, &g).nil(), LOCATION_HAND)
    {
        duel::confirm_cards(d, 1 - tp, g);
        duel::shuffle_hand(d, tp);
        duel::break_effect(d);
        duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
/// `s.gythfilter(c,e,tp)`
fn gythfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_LUNALIGHT as u16])
        && card::is_monster(d, c)
        && card::is_able_to_hand(d, c, None)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_location(d, c, LOCATION_GRAVE)
        && card::is_controler(d, c, tp)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| gythfilter(d, c, e, tp), NO_EX);
    if !tg.is_empty() {
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e3 = effect::get_label_object(d, e).as_effect().nil();
        let mut g = effect::get_label_object(d, e3).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e3, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let h = effect::get_handler(d, e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(e), 0, tp, tp, 0);
    }
}
/// `s.gythtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gythtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let set = effect::get_label_object(d, e).as_group().cloned().nil();
    let g = group::filter(d, &set, &|d, c| gythfilter(d, c, e, tp), NO_EX);
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && gythfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let tc = if g.len() > 1 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel)
    } else {
        group::get_first(d, &g)
    };
    duel::set_target_card(d, tc.nil());
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, tc.map(CardSet::from_card), 1, tp, 0);
    true
}
fn gythop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
