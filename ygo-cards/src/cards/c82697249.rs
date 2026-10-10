//! No.59 Number 59: Crooked Cook
use super::prelude::*;

pub const ID: u32 = 82697249;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(59), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // immune
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(imcon)));
    effect::set_value_fn(d, e1, lua_fn(efilter));
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
/// `s.imcon(e)`
fn imcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_ONFIELD,0)==1
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, LOCATION_ONFIELD, 0) == 1
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwner()~=e:GetOwner()
    let (e, te) = (a.e(0), a.e(1));
    effect::get_owner(d, te) != effect::get_owner(d, e)
}
/// `s.destg(e,...,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.TRUE,tp,LOCATION_ONFIELD,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_ONFIELD, 0, 1, h.into());
    }
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,0,e:GetHandler())
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, 0, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// `s.ctfilter(c)`
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsMonster()
    card::is_location(d, c, LOCATION_GRAVE) && card::is_monster(d, c)
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,0,c)
    let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, 0, c.into());
    // if Duel.Destroy(g,REASON_EFFECT)~=0 then
    if duel::destroy(d, g, REASON_EFFECT, None, None) != 0 {
        // local ct=Duel.GetOperatedGroup():FilterCount(s.ctfilter,nil)
        let og = duel::get_operated_group(d);
        let ct = group::filter_count(d, &og, &|d, c| ctfilter(d, c), NO_EX);
        // if ct>0 and c:IsFaceup() and c:IsRelateToEffect(e) then
        if ct > 0 && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
            effect::set_value(d, e1, (ct * 300) as i64);
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
