//! Proxy Horse
use super::prelude::*;

pub const ID: u32 = 16684346;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    aux::global_check(d, ID, |_d| {
        // s.flagmap={}: the per-card flag effects live in the duel (see `flagmap_key`)
    });
    // Extra Link Material
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_code(d, e1, EFFECT_EXTRA_MATERIAL);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_operation(d, e1, Some(lua_fn(extracon)));
    effect::set_value_fn(d, e1, lua_fn(extraval));
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    effect::set_target(d, e2, Some(effect_card_fn(eftg)));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
    // Return 2 Link monsters to Extra Deck
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_TOEXTRA);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| duel::is_turn_player(d, a.p(1)))));
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_target(d, e3, Some(cost_target(tetg)));
    effect::set_operation(d, e3, Some(operation(teop)));
    card::register_effect(d, c, e3, false);
}
/// `s.flagmap[c]`: the flag effect that `extraval` registered on card `c`
fn flagmap_key(c: CardId) -> u64 {
    0x1000_0000 + c.0 as u64
}
/// `s.eftg(e,c)`
fn eftg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_race(d, c, RACE_CYBERSE) && card::is_can_be_link_material(d, c, None, None)
}
/// `s.extrafilter(c,tp)`
fn extrafilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
/// `s.extracon(c,e,tp,sg,mg,lc,og,chk)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let sg = a.g(3);
    let mg = a.g(4);
    let og = a.g(6);
    let ct = group::filter_count(d, &sg, &|d, c| card::has_flag_effect(d, c, ID, None), NO_EX);
    if ct == 0 {
        return true;
    }
    let hp = effect::get_handler_player(d, e);
    let mut sm = sg.clone();
    group::add_card(&mut sm, &mg);
    let filtered = group::filter(d, &sm, &|d, c| extrafilter(d, c, hp), NO_EX);
    group::is_exists(d, &filtered, &|d, c| card::is_code(d, c, &[ID]), 1, (&og).into()) && ct < 2
}
/// `s.extraval(chk,summon_type,e,...)`
fn extraval(d: &mut Duel, a: Args) -> Value {
    let chk = a.int(0);
    let summon_type = a.int(1);
    let e = a.e(2);
    let c = effect::get_handler(d, e).nil();
    if chk == 0 {
        // local tp,sc=...
        let tp = a.p(3);
        let sc = a.c(4);
        if summon_type != SUMMON_TYPE_LINK as i64 || !card::is_race(d, sc, RACE_CYBERSE) || duel::get_flag_effect(d, tp, ID) > 0 {
            Value::Group(CardSet::new())
        } else {
            let fe = card::register_flag_effect(d, c, ID, 0, 0, 1, 0, 0);
            set_s_var(d, ID, flagmap_key(c), Value::Effect(fe));
            Value::Group(CardSet::from_card(c))
        }
    } else if chk == 1 {
        // local sg,sc,tp=...
        let sg = a.g(3);
        let tp = a.p(5);
        if summon_type & SUMMON_TYPE_LINK as i64 == SUMMON_TYPE_LINK as i64 && !sg.is_empty() && duel::get_flag_effect(d, tp, ID) == 0 {
            duel::hint(d, HINT_CARD as u8, tp, ID as u64);
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        Value::Nil
    } else if chk == 2 {
        if let Value::Effect(fe) = s_var(d, ID, flagmap_key(c)) {
            effect::reset(d, fe);
            set_s_var(d, ID, flagmap_key(c), Value::Nil);
        }
        Value::Nil
    } else {
        Value::Nil
    }
}
/// `s.tedfilter(c,e)`
fn tedfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_link_monster(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (group::is_exists(d, sg, &|d, c| card::is_race(d, c, RACE_CYBERSE), 1, NO_EX), false)
}
/// `s.tetg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tetg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    let rg = duel::get_matching_group(d, Some(&|d, c| tedfilter(d, c, a.e)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    if a.chk == Some(0) {
        return rg.len() >= 2 && matches!(aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    let g = aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_TODECK as u64), None, None, false).unwrap_or_default();
    let n = g.len() as u8;
    duel::set_target_card(d, g.clone());
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(g), n, 0, 0);
    true
}
/// `s.teop(e,tp,eg,ep,ev,re,r,rp)`
fn teop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    if !g.is_empty() {
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
