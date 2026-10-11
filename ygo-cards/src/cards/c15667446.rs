//! Latinum, Exarch of Dark World
use super::prelude::*;

pub const ID: u32 = 15667446;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let prev = card::get_previous_controler(d, c) as i64;
    effect::set_label(d, a.e, &[prev]);
    card::is_previous_location(d, c, LOCATION_HAND) && (a.r & (REASON_DISCARD | REASON_EFFECT)) == (REASON_DISCARD | REASON_EFFECT)
}
// function s.atfilter(c) return c:IsFaceup() and c:IsRace(RACE_FIEND) end
fn atfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && atfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    if a.rp != a.tp && a.tp as i64 == effect::get_label(d, a.e) {
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON | CATEGORY_ATKCHANGE);
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
        duel::select_target(d, a.tp, Some(&atfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    } else {
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) && duel::special_summon(d, h, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        let tc = duel::first_target(d);
        if let Some(tc) = tc {
            if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
                duel::break_effect(d);
                let e1 = effect::create_effect(d, h);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                effect::set_value(d, e1, 500);
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, tc, e1, false);
            }
        }
    }
}
