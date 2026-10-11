//! Astrograph Sorcerer, the Star Magician
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 48171151;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    pendulum::add_procedure(d, c, None, None);
    // During your Main Phase: You can destroy this card, and if you do, place 1 Pendulum Monster whose Pendulum Scale is 1 from your hand or Deck in your Pendulum Zone, except "Astrograph Sorcerer, the Star Magician", but it cannot activate its Pendulum Effects this turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(pltg)));
    effect::set_operation(d, e1, Some(operation(plop)));
    card::register_effect(d, c, e1, false);
    // If a face-up Monster Card(s) you control is destroyed by battle or card effect: You can Special Summon this card from your hand, then you can Special Summon 1 of those destroyed monsters from your GY or face-up Extra Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY | EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Keep track of the destroyed monsters
    aux::global_check(d, ID, |d| {
        set_s_var(d, ID, 0, Value::Group(CardSet::new()));
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_DESTROYED);
        effect::set_operation(d, ge1, Some(operation(desgroupregop)));
        duel::register_effect(d, ge1, 0);
    });
    // During the Main Phase (Quick Effect): You can Tribute this card, then you can Fusion Summon 1 Dragon Fusion Monster from your Extra Deck, by shuffling its materials from your field and/or face-up Extra Deck into the Deck
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_RELEASE | CATEGORY_TODECK | CATEGORY_SPECIAL_SUMMON | CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_condition(d, e3, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    effect::set_target(d, e3, Some(cost_target(tribtg)));
    effect::set_operation(d, e3, Some(operation(tribop)));
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    card::register_effect(d, c, e3, false);
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_pendulum_monster(d, c) && card::is_scale(d, c, &[1]) && !card::is_code(d, c, &[ID]) && !card::is_forbidden(d, c)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, tp, 0);
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e) && duel::destroy(d, c, REASON_EFFECT, None, None) > 0 && duel::check_pendulum_zones(d, tp) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        let sel = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        if let Some(sc) = sc {
            if duel::move_to_field(d, sc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None) {
                // It cannot activate its Pendulum Effects this turn
                let e1 = effect::create_effect(d, c);
                effect::set_description(d, e1, 3302);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
                effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                card::register_effect(d, sc, e1, false);
            }
        }
    }
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster_card(d, c)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.desgroup`: the group of destroyed monsters (kept in the duel)
fn desgroup(d: &Duel) -> CardSet {
    s_var(d, ID, 0).as_group().cloned().nil()
}
/// `s.desgroupregop(e,tp,eg,ep,ev,re,r,rp)`
fn desgroupregop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg();
    let tg = group::filter(d, &eg, &spconfilter, NO_EX);
    if !tg.is_empty() {
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // if Duel.GetCurrentChain()==0 then s.desgroup:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            set_s_var(d, ID, 0, Value::Group(CardSet::new()));
        }
        let mut dg = desgroup(d);
        group::add_card(&mut dg, &tg);
        group::remove(d, &mut dg, &|d, c| !card::has_flag_effect(d, c, ID, None), NO_EX);
        set_s_var(d, ID, 0, Value::Group(dg.clone()));
        let tg_prev0 = group::is_exists(d, &tg, &|d, c| card::is_previous_controler(d, c, 0), 1, NO_EX);
        if tg_prev0 {
            let g = group::filter(d, &dg, &|d, c| card::is_previous_controler(d, c, 0), NO_EX);
            duel::raise_event(d, g, EVENT_CUSTOM + ID, a.re, a.r, a.rp, 0, a.ev as u32);
        }
        let tg_prev1 = group::is_exists(d, &tg, &|d, c| card::is_previous_controler(d, c, 1), 1, NO_EX);
        if tg_prev1 {
            let g = group::filter(d, &dg, &|d, c| card::is_previous_controler(d, c, 1), NO_EX);
            duel::raise_event(d, g, EVENT_CUSTOM + ID, a.re, a.r, a.rp, 1, a.ev as u32);
        }
    }
}
/// `s.spfilter(c,e,tp,mmz_chk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    if !(card::is_controler(d, c, tp) && card::is_relate_to_effect(d, c, e) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)) {
        return false;
    }
    if card::is_location(d, c, LOCATION_GRAVE) {
        return mmz_chk;
    } else if card::is_location(d, c, LOCATION_EXTRA) {
        return duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0;
    }
    false
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return a.ep == tp
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    let dg = desgroup(d);
    let g = group::filter(d, &dg, &spconfilter, NO_EX);
    effect::set_label_object(d, e, Value::Group(g.clone()));
    for ec in group::iter(d, &g) {
        card::create_effect_relation(d, ec, e);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        let lo = effect::get_label_object(d, e).as_group().cloned().nil();
        let nf = aux::necro_valley_filter(move |d, x| spfilter(d, x, e, tp, mmz_chk));
        let g = group::filter(d, &lo, &nf, NO_EX);
        if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
            return;
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        if !sg.is_empty() {
            duel::break_effect(d);
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.tribtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tribtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, c) = (a.tp, effect::get_handler(d, a.e).nil());
    if a.chk == Some(0) {
        return card::is_releasable_by_effect(d, c, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, Some(CardSet::from_card(c)), 1, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, (LOCATION_ONFIELD | LOCATION_EXTRA) as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `fusion_params` of `s.tribop`
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_DRAGON as u64))),
        extraop: Some(fusion::shuffle_material()),
        matfilter: Some(fusion::on_field_mat()),
        extrafil: Some(Arc::new(|d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet| {
            let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_able_to_deck(d, c)), tp, LOCATION_EXTRA, 0, NO_EX);
            (Some(g), None)
        })),
        ..Default::default()
    }
}
/// The arguments `(e,tp,eg,ep,ev,re,r,rp)` of this effect.
fn own_args(a: &EffArgs) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ]
}
/// `s.tribop(e,tp,eg,ep,ev,re,r,rp)`
fn tribop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e) && duel::release(d, c, REASON_EFFECT, None) > 0 {
        // if Fusion.SummonEffTG(params)(e,tp,eg,ep,ev,re,r,rp,0) and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
        let tg = fusion::summon_eff_tg(d, fusion_params());
        let mut args = own_args(a);
        args.push(Value::Int(0));
        if tg.call(d, &args).truthy() && duel::select_yes_no(d, tp, aux::stringid(ID, 4)) {
            duel::break_effect(d);
            let op = fusion::summon_eff_op(d, fusion_params());
            op.call(d, &own_args(a));
        }
    }
}
