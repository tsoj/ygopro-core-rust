//! Baby Spider
use super::prelude::*;

pub const ID: u32 = 60023855;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Increase the level of all "Baby Spiders"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(lvcost)));
    effect::set_operation(d, e1, Some(operation(lvop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 DARK monster from the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::has_level(d, c)
        && duel::is_existing_matching_card(d, Some(&lvfilter), tp, LOCATION_MZONE, 0, 1, c.into())
}
// function s.lvfilter(c)
fn lvfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[ID]) && card::is_faceup(d, c) && card::has_level(d, c)
}
// function s.lvcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn lvcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    let rg = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    let lvl = card::get_level(d, group::get_first(d, &rg).nil());
    effect::set_label(d, a.e, &[lvl as i64]);
    duel::release(d, rg, REASON_COST, None);
    true
}
// function s.lvop(e,tp,eg,ep,ev,re,r,rp)
fn lvop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&lvfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    if g.is_empty() {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    let lvl = effect::get_label(d, a.e);
    for tc in group::iter(d, &g) {
        // Increase Level
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_value(d, e1, lvl);
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
}
// function s.xyzfilter(c)
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_XYZ) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_faceup(d, c)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let xyzg = duel::get_matching_group(d, Some(&xyzfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::check_remove_overlay_card(d, a.tp, 1, 0, 1, REASON_COST, Some(&xyzg));
    }
    // Duel.PayLPCost(tp,Duel.GetLP(tp)//2)
    let half = duel::get_lp(d, a.tp).div_euclid(2) as u32;
    duel::pay_lp_cost(d, a.tp, half);
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    duel::remove_overlay_card(d, a.tp, 1, 0, 1, 1, REASON_COST, Some(&xyzg));
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let tg = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
