//! Turbo-Tainted Hot Rod GT19
use super::prelude::*;

pub const ID: u32 = 16769305;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Register when it's flipped face-up
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e0, EVENT_FLIP);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_operation(d, e0, Some(operation(flipop)));
    card::register_effect(d, c, e0, false);
    // Change its own level when Flipped summoned
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(lvltg)));
    effect::set_operation(d, e1, Some(operation(lvlop)));
    card::register_effect(d, c, e1, false);
    // Synchro Summon using only itself and a target
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(cond2)));
    effect::set_target(d, e2, Some(cost_target(syncsumtg)));
    effect::set_operation(d, e2, Some(operation(syncsumop)));
    card::register_effect(d, c, e2, false);
}
// function s.flipop(e,tp,eg,ep,ev,re,r,rp)
fn flipop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
}
// function s.lvltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn lvltg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_LVRANK as u64);
    let level = card::get_level(d, c);
    let lv = duel::announce_number_range(d, a.tp, Some(1), Some(9), &[level]).nil().0 as i64;
    duel::set_target_param(d, lv as u32);
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(CardSet::from_card(c)), 1, a.tp, lv as i32);
    true
}
// function s.lvlop(e,tp,eg,ep,ev,re,r,rp)
fn lvlop(d: &mut Duel, a: &EffArgs) {
    let lv = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && lv != card::get_level(d, c) as i64 {
        let lv = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        effect::set_value(d, e1, lv);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
// function s.filter(c,this,tp)
fn filter(d: &mut Duel, c: CardId, this: CardId, tp: u8) -> bool {
    if !(card::is_faceup(d, c) && card::has_level(d, c) && card::is_can_be_synchro_material(d, c, None, None, None)) {
        return false;
    }
    if card::is_controler(d, c, tp) {
        let mg = mg_of(this, c);
        return duel::is_existing_matching_card(d, Some(&|d, x| synchro_summonable_with(d, x, &mg)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Temporarily register EFFECT_SYNCHRO_MATERIAL otherwise IsSynchroSummonable will fail with opponent's monsters
    let e1 = effect::create_effect(d, this);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SYNCHRO_MATERIAL);
    card::register_effect(d, c, e1, true);
    let mg = mg_of(this, c);
    let res = duel::is_existing_matching_card(d, Some(&|d, x| synchro_summonable_with(d, x, &mg)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    effect::reset(d, e1);
    res
}
// Card.IsSynchroSummonable(x,nil,mg)
fn synchro_summonable_with(d: &mut Duel, x: CardId, mg: &CardSet) -> bool {
    card::is_synchro_summonable(d, x, None, Some(mg.clone()), 0, 0)
}
// Group.FromCards(a,b)
fn mg_of(a: CardId, b: CardId) -> CardSet {
    let mut mg = CardSet::from_card(a);
    group::add_card(&mut mg, &CardSet::from_card(b));
    mg
}
// function s.syncsumtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn syncsumtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc, c, a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| filter(d, x, c, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, a.tp, Some(&|d, x| filter(d, x, c, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.syncsumop(e,tp,eg,ep,ev,re,r,rp)
fn syncsumop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_immune_to_effect(d, tc, a.e) {
        return;
    }
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e)) {
        return;
    }
    // Register EFFECT_SYNCHRO_MATERIAL, temporarily, only if it's an opponent's monster
    let e1 = effect::create_effect(d, c); // don't declare e1 localy inside the if, so it outlives that scope (to be used in e2)
    if card::is_controler(d, tc, 1 - a.tp) {
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SYNCHRO_MATERIAL);
        card::register_effect(d, tc, e1, true);
    }
    let mg = mg_of(c, tc);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| synchro_summonable_with(d, x, &mg)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    if let Some(sc) = sc {
        // Reset EFFECT_SYNCHRO_MATERIAL at the suitable time
        if card::is_controler(d, tc, 1 - a.tp) {
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
            effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
            effect::set_operation(d, e2, Some(operation(regop)));
            effect::set_label_object(d, e2, Value::Effect(e1));
            card::register_effect(d, sc, e2, true);
            let e3 = effect::clone(d, e2);
            effect::set_code(d, e3, EVENT_SPSUMMON_NEGATED);
            card::register_effect(d, sc, e3, true);
        }
        // Perform the Syncro Summon
        duel::synchro_summon(d, a.tp, sc, None, Some(mg), 0, 0);
    } else {
        effect::reset(d, e1);
    }
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::reset(d, lo);
    effect::reset(d, a.e);
}
// condition of e2: function(e) return Duel.IsMainPhase() and e:GetHandler():GetFlagEffect(id)>0 end
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    if !duel::is_main_phase(d, None) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
}
