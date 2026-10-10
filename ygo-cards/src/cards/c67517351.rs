//! HRUM - Utopia Force
use super::prelude::*;

pub const ID: u32 = 67517351;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_UTOPIC, SET_UTOPIA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Attach
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(mtg)));
    effect::set_operation(d, e2, Some(operation(mop)));
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e2:SetLabelObject(g)
    effect::set_label_object(d, e2, Value::Group(CardSet::new()));
    card::register_effect(d, c, e2, false);
    // Mass register
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_label_object(d, e3, Value::Effect(e2));
    effect::set_operation(d, e3, Some(operation(regop)));
    card::register_effect(d, c, e3, false);
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    let pg_len = pg.len();
    ((pg_len <= 0) || (pg_len == 1 && group::is_contains(&pg, c)))
        && card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_UTOPIA as u16])
        && card::is_rank_below(d, c, 9)
        && duel::is_existing_matching_card(d, Some(&|d, mc| filter2(d, mc, e, tp, c, &pg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.filter2(c,e,tp,mc,pg)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, _pg: &CardSet) -> bool {
    // if c.rum_limit and not c.rum_limit(mc,e) then return false end
    if let Some(rum_limit) = crate::card_info(d, c).rum_limit {
        if !rum_limit(d, mc, e) {
            return false;
        }
    }
    card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_set_card(d, c, &[SET_UTOPIC as u16])
        && card::is_rank_above(d, c, 10)
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter1(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let _ = duel::select_target(d, tp, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    let tc = duel::first_target(d);
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&tc.map(CardSet::from_card).unwrap_or_default()), tp, None, None, REASON_XYZ);
    let tc = match tc {
        Some(tc) => tc,
        None => return,
    };
    if card::is_facedown(d, tc)
        || !card::is_relate_to_effect(d, tc, e)
        || card::is_controler(d, tc, 1 - tp)
        || card::is_immune_to_effect(d, tc, e)
        || pg.len() > 1
        || (pg.len() == 1 && !group::is_contains(&pg, tc))
    {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, e, tp, tc, &pg)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    if let Some(sc) = sc {
        card::set_material(d, sc, CardSet::from_card(tc));
        duel::overlay(d, sc, tc, false);
        duel::special_summon(d, sc, SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None);
        card::complete_procedure(d, sc);
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    let re = a.re.nil();
    let rc = effect::get_handler(d, re);
    let rc = match rc {
        Some(rc) => rc,
        None => return,
    };
    if !(effect::is_activated(d, re) && card::is_type(d, rc, TYPE_XYZ) && card::is_set_card(d, rc, &[SET_UTOPIC as u16]) && card::is_rank_above(d, rc, 10)) {
        return;
    }
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ), NO_EX);
    if !tg.is_empty() {
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e2 = effect::get_label_object(d, e).as_effect().nil();
        let mut g = effect::get_label_object(d, e2).as_group().cloned().nil();
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        group::add_card(&mut g, &tg);
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        effect::set_label_object(d, e2, Value::Group(g));
        let c = effect::get_handler(d, e).nil();
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(e), 0, tp, tp, 0);
    }
}
/// `s.mcfilter(c,e)`
fn mcfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.mtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn mtg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    let lo = effect::get_label_object(d, e).as_group().cloned().nil();
    let g = group::filter(d, &lo, &|d, c| mcfilter(d, c, e), NO_EX);
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && card::is_type(d, chkc, TYPE_XYZ) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return !g.is_empty() && duel::get_flag_effect(d, tp, ID) == 0;
    }
    duel::register_flag_effect(d, tp, ID, RESET_CHAIN, 0, 1, 0);
    if g.len() == 1 {
        duel::set_target_card(d, group::get_first(d, &g).nil());
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        let tc = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        duel::set_target_card(d, tc);
    }
    let c = effect::get_handler(d, e).nil();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.mop(e,tp,eg,ep,ev,re,r,rp)`
fn mop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, c, e) && card::is_relate_to_effect(d, tc, e) && !card::is_immune_to_effect(d, tc, e) {
        duel::overlay(d, tc, c, false);
    }
}
