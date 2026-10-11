//! Sorcerer of the Swamp
use super::prelude::*;

pub const ID: u32 = 45223540;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is in your hand: You can reveal 1 Fusion Monster in your Extra Deck and banish 1 material whose name is mentioned on it from your Deck, then Special Summon this card, and its name can be treated as the banished monster's if used as Fusion Material this turn. You can only use this effect of "Magical King of the Swamp" once per turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
/// `s.revealfilter(c, tp)`: a Fusion Monster with `c.material` (a non-empty material list here:
/// `class::material` is empty when the script set none, the Lua nil)
fn revealfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_fusion_monster(d, c) && !crate::class::material(d, c).is_empty() && {
        let mat = crate::class::material(d, c);
        duel::is_existing_matching_card(d, Some(&|d, cc| banfilter(d, cc, &mat)), tp, LOCATION_DECK, 0, 1, NO_EX)
    }
}
/// `s.banfilter(c, codes)`
fn banfilter(d: &mut Duel, c: CardId, codes: &[u32]) -> bool {
    card::is_code(d, c, codes) && card::is_able_to_remove(d, c, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, cc| revealfilter(d, cc, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| revealfilter(d, cc, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let Some(reveal_card) = group::get_first(d, &g) else { return };
    duel::confirm_cards(d, 1 - tp, reveal_card);
    duel::shuffle_extra(d, tp);
    let c = effect::get_handler(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let codes = crate::class::material(d, reveal_card);
    let g2 = duel::select_matching_card(d, tp, Some(&|d, cc| banfilter(d, cc, &codes)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let banish_card = group::get_first(d, &g2);
    if let Some(banish_card) = banish_card {
        if duel::remove(d, banish_card, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 && card::is_relate_to_effect(d, c, a.e) {
            duel::break_effect(d);
            if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // Its name can be treated as the banished monster's if used as Fusion Material this turn
                let e1 = effect::create_effect(d, c);
                effect::set_description(d, e1, aux::stringid(ID, 1));
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
                effect::set_code(d, e1, EFFECT_ADD_CODE);
                let banished_code = card::get_original_code_rule(d, banish_card)[0] as i64;
                effect::set_value(d, e1, banished_code);
                effect::set_operation(d, e1, Some(lua_fn(|_d, args: Args| {
                    let sumtype = args.int(1) as u64;
                    (sumtype & MATERIAL_FUSION as u64) > 0 || (sumtype & SUMMON_TYPE_FUSION as u64) > 0
                })));
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                card::register_effect(d, c, e1, false);
            }
        }
    }
}
