//! Performapal Lebellman
use super::prelude::*;

pub const ID: u32 = 3752422;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum atributes
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Increase Level from the pendulum zone
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(lvtg)));
    effect::set_operation(d, e1, Some(operation(lvop)));
    card::register_effect(d, c, e1, false);
    // Increase Level from the monster zone
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
/// `s.lvfilter(c)`
fn lvfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_pendulum_summoned(d, c) && card::has_level(d, c)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&lvfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetMatchingGroup(s.lvfilter,tp,LOCATION_MZONE,0,nil)
    let tg = duel::get_matching_group(d, Some(&lvfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // for tc in aux.Next(tg) do
    for tc in group::iter(d, &tg) {
        // tc:UpdateLevel(1,RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD),e:GetHandler())
        let h = effect::get_handler(d, a.e);
        card::update_level(d, tc, 1, Some(RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD)), h);
    }
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PERFORMAPAL as u16]) && card::has_level(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return chkc != c && card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return card::get_level(d, c) > 1 && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // local p=math.min(c:GetLevel()-1,5)
    let p = (card::get_level(d, c) as i64 - 1).min(5);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_LVRANK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_LVRANK as u64);
    // local lv=Duel.AnnounceLevel(tp,1,p)
    let lv = duel::announce_number_range(d, a.tp, Some(1), Some(p as u32), &[]).nil().0 as u32;
    // Duel.SetTargetParam(lv)
    duel::set_target_param(d, lv);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,c)
    let _g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lv=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let lv = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if c:IsFaceup() and c:IsRelateToEffect(e) and c:UpdateLevel(-lv,RESETS_STANDARD_PHASE_END)~=0 then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && card::update_level(d, c, -lv, Some(RESETS_STANDARD_PHASE_END), None) != 0 {
        // local tc=Duel.GetFirstTarget()
        // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
        if let Some(tc) = duel::first_target(d) {
            if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
                // tc:UpdateLevel(lv,RESETS_STANDARD_PHASE_END,c)
                card::update_level(d, tc, lv, Some(RESETS_STANDARD_PHASE_END), Some(c));
            }
        }
    }
}
