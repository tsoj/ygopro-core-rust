//! Mikanko Kagura
//! Scripted by Hatter
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 16310544;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MIKANKO as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual Summon
    let e1 = ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_MIKANKO as u16]))),
            stage2: Some(Arc::new(|d: &mut Duel, mat: &CardSet, a: &EffArgs, tc: CardId| stage2(d, mat, a, tc))),
            ..Default::default()
        },
    );
    // e1:SetCategory(e1:GetCategory()|CATEGORY_DESTROY|CATEGORY_DAMAGE)
    let cat = effect::get_category(d, e1);
    effect::set_category(d, e1, cat | CATEGORY_DESTROY | CATEGORY_DAMAGE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
}
/// `s.stage2(mat,e,tp,eg,ep,ev,re,r,rp,tc)`
fn stage2(d: &mut Duel, _mat: &CardSet, a: &EffArgs, _tc: CardId) {
    let tp = a.tp;
    let g = duel::get_field_group(d, 0, 0, LOCATION_ONFIELD);
    if g.is_empty() {
        return;
    }
    // local eg=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_GRAVE,0,nil,TYPE_EQUIP)
    let eg = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_EQUIP)), tp, LOCATION_GRAVE, 0, NO_EX);
    // local ct=eg:GetClassCount(Card.GetCode)
    let ct = group::get_class_count(d, &eg, &|d, c| card::get_code(d, c).first().copied().unwrap_or(0) as i64);
    if ct == 0 || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let dg = group::select(d, &g, tp, 1, ct as u16, false, NO_EX).nil();
    if dg.is_empty() {
        return;
    }
    duel::hint_selection(d, &dg, true);
    duel::break_effect(d);
    let des = duel::destroy(d, &dg, REASON_EFFECT, None, None);
    if des > 0 {
        duel::damage(d, 1 - tp, des as i64 * 1000, REASON_EFFECT, false, None);
    }
}
