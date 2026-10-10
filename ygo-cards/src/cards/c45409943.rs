//! Luce the Dusk's Dark
use super::prelude::*;

pub const ID: u32 = 45409943;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Fusion Materials: 3 Fiend and/or Fairy monsters in your GY
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| fusmatfilter(d, c, fc, st, tp))), 3)]);
    // Monsters in your leftmost or rightmost Main Monster Zones cannot be destroyed by card effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(indes_target)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // Send 1 Fiend or Fairy monster from your Deck to the GY, and if you do, destroy 1 card your opponent controls
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(tgdestg)));
    effect::set_operation(d, e2, Some(operation(tgdesop)));
    card::register_effect(d, c, e2, false);
    // Destroy 1 card on the field
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_DESTROYED);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(descon)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
/// `s.fusionmatfilter(c,fc,sumtype,tp)`
fn fusmatfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    card::is_race_ctx(d, c, RACE_FIEND | RACE_FAIRY, Some(fc), sumtype, Some(tp)) && card::is_location(d, c, LOCATION_GRAVE) && card::is_controler(d, c, tp)
}
/// `function(e,c) return c:IsSequence(0,4) or (Duel.IsDuelType(DUEL_3_COLUMNS_FIELD) and c:IsSequence(1,3)) end`
fn indes_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_sequence(d, c, &[0, 4]) || (duel::is_duel_type(d, DUEL_3_COLUMNS_FIELD) && card::is_sequence(d, c, &[1, 3]))
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_FIEND | RACE_FAIRY) && card::is_able_to_grave(d, c, None)
}
fn tgdestg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    true
}
fn tgdesop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave returns a number, which is true in Lua
    if !g.is_empty() {
        duel::send_to_grave(d, &g, REASON_EFFECT, None, None);
        let first = group::get_first(d, &g).nil();
        if card::is_location(d, first, LOCATION_GRAVE) {
            let tc = duel::first_target(d).nil();
            if card::is_relate_to_effect(d, tc, a.e) {
                duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
            }
        }
    }
}
fn desconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_previous_controler(d, c, tp) && card::is_reason(d, c, REASON_EFFECT)
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| desconfilter(d, c, a.tp), 1, NO_EX)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
