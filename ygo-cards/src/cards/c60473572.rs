//! Metalfoes Steelen
use super::prelude::*;

pub const ID: u32 = 60473572;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_METALFOES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // destroy and set
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SET);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
/// `s.desfilter(c,tp)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // if c:IsFacedown() then return false end
    if card::is_facedown(d, c) {
        return false;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if ft==0 and c:IsLocation(LOCATION_SZONE) and c:GetSequence()<5 then
    if ft == 0 && card::is_location(d, c, LOCATION_SZONE) && card::get_sequence(d, c) < 5 {
        // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,true)
        duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, true)), tp, LOCATION_DECK, 0, 1, NO_EX)
    } else {
        // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,false)
        duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, false)), tp, LOCATION_DECK, 0, 1, NO_EX)
    }
}
/// `s.filter(c,ignore)`
fn filter(d: &mut Duel, c: CardId, ignore: bool) -> bool {
    // return c:IsSetCard(SET_METALFOES) and c:IsSpellTrap() and c:IsSSetable(ignore)
    card::is_set_card(d, c, &[SET_METALFOES as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, ignore, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=e:GetHandler()  (the Lua reads it only where it is used)
    if let Some(chkc) = a.chkc() {
        // return chkc:IsOnField() and chkc:IsControler(tp) and s.desfilter(chkc,tp) and chkc~=e:GetHandler()
        let h = effect::get_handler(d, a.e).nil();
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, a.tp) && desfilter(d, chkc, a.tp) && chkc != h;
    }
    if a.chk == Some(0) {
        // return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_ONFIELD,0,1,e:GetHandler(),tp)
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&|d, c| desfilter(d, c, a.tp)), a.tp, LOCATION_ONFIELD, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,1,e:GetHandler(),tp)
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_target(d, a.tp, Some(&|d, c| desfilter(d, c, a.tp)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, h.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.operation(e,tp,...)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
        && duel::destroy(d, tc.nil(), REASON_EFFECT, None, None) != 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,false)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x, false)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then Duel.SSet(tp,g:GetFirst()) end
        if !g.is_empty() {
            let f = group::get_first(d, &g).nil();
            duel::s_set(d, a.tp, f, None, true);
        }
    }
}
