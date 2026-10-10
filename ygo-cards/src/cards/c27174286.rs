//! Return from the Different Dimension
use super::prelude::*;

pub const ID: u32 = 27174286;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(tg)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2))
    let lp = duel::get_lp(d, a.tp) as i64;
    duel::pay_lp_cost(d, a.tp, lp.div_euclid(2) as u32);
    true
}
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_REMOVED as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, ft as u16, ft as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // local fid=e:GetHandler():GetFieldID()
        let h = effect::get_handler(d, e).nil();
        let fid = card::get_field_id(d, h);
        // local tc=g:GetFirst()
        let _ = group::get_first(d, &g);
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,fid)
            card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid as i64, 0);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // g:KeepAlive() (groups are values in Rust)
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        effect::set_label(d, e1, &[fid as i64]);
        effect::set_label_object(d, e1, Value::Group(g.clone()));
        effect::set_condition(d, e1, Some(condition(rmcon)));
        effect::set_operation(d, e1, Some(operation(rmop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `function s.rmfilter(c,fid)`
fn rmfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    // if not g:IsExists(s.rmfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| rmfilter(d, c, fid), 1, NO_EX) {
        // g:DeleteGroup() (nothing in Rust)
        // e:Reset()
        effect::reset(d, a.e);
        return false;
    }
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    // local tg=g:Filter(s.rmfilter,nil,e:GetLabel())
    let tg = group::filter(d, &g, &|d, c| rmfilter(d, c, fid), NO_EX);
    // Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
