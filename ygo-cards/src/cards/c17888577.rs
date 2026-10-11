//! Sonic Tracker
use super::prelude::*;

pub const ID: u32 = 17888577;

pub const INFO: ScriptInfo = ScriptInfo { fit_monster: &[34072799], listed_names: &[34072799], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreater({handler=c,filter=aux.FilterBoolFunction(Card.IsCode,34072799),stage2=s.stage2})
    ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            filter: Some(std::sync::Arc::new(|d: &mut Duel, x: CardId| card::is_code(d, x, &[34072799]))),
            stage2: Some(std::sync::Arc::new(|d: &mut Duel, mat: &CardSet, a: &EffArgs, tc: CardId| stage2(d, mat, a, tc))),
            ..Default::default()
        },
    );
    // to gy
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_banish()));
    effect::set_target(d, e1, Some(cost_target(gytg)));
    effect::set_operation(d, e1, Some(operation(gyop)));
    card::register_effect(d, c, e1, false);
}
/// `s.stage2(mat,e,tp,eg,ep,ev,re,r,rp,tc)`
fn stage2(d: &mut Duel, mat: &CardSet, a: &EffArgs, tc: CardId) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(mat:GetSum(Card.GetLevel))
    let sum = group::get_sum(d, mat, &|d, x| card::get_level(d, x) as i64);
    effect::set_value(d, e1, sum);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
/// `s.gyfilter1(c,tp)`
fn gyfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_ritual_monster(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| gyfilter2(d, x, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.gyfilter2(c,tc)`
fn gyfilter2(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    let att = card::get_attribute(d, tc);
    let race = card::get_race(d, tc);
    card::is_ritual_monster(d, c) && card::is_able_to_grave(d, c, None) && (card::is_attribute(d, c, att) || card::is_race(d, c, race))
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && gyfilter1(d, chkc, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| gyfilter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, tp, Some(&|d, c| gyfilter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| gyfilter2(d, c, tc)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::send_to_grave(d, &g, REASON_EFFECT, None, None);
        }
    }
}
