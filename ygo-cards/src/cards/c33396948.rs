//! Exodia the Forbidden One
use super::prelude::*;

pub const ID: u32 = 33396948;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[8124921, 44519536, 70903634, 7902349], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // win
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_TO_HAND);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn check(d: &mut Duel, g: &CardSet) -> bool {
    // local a1=false ... local a5=false
    let mut a1 = false;
    let mut a2 = false;
    let mut a3 = false;
    let mut a4 = false;
    let mut a5 = false;
    // local tc=g:GetFirst()
    let _tc = group::get_first(d, g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, g) {
        // local code=tc:GetCode()
        let code = card::get_code(d, tc)[0];
        if code == 8124921 {
            a1 = true;
        } else if code == 44519536 {
            a2 = true;
        } else if code == 70903634 {
            a3 = true;
        } else if code == 7902349 {
            a4 = true;
        } else if code == ID {
            a5 = true;
        }
    }
    // return a1 and a2 and a3 and a4 and a5
    a1 && a2 && a3 && a4 && a5
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g1=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g1 = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    // local g2=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g2 = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // local wtp=s.check(g1)
    let wtp = check(d, &g1);
    // local wntp=s.check(g2)
    let wntp = check(d, &g2);
    if wtp && !wntp {
        duel::confirm_cards(d, 1 - tp, g1);
        duel::win(d, tp, WIN_REASON_EXODIA);
    } else if !wtp && wntp {
        duel::confirm_cards(d, tp, g2);
        duel::win(d, 1 - tp, WIN_REASON_EXODIA);
    } else if wtp && wntp {
        duel::confirm_cards(d, 1 - tp, g1);
        duel::confirm_cards(d, tp, g2);
        duel::win(d, PLAYER_NONE as u8, WIN_REASON_EXODIA);
    }
}
