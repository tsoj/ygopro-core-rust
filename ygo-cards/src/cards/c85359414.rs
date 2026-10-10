//! Freezing Beast
use super::prelude::*;

pub const ID: u32 = 85359414;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[59364406],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddUnionProcedure(c,aux.FilterBoolFunction(Card.IsCode,59364406),true)
    aux::add_union_procedure(d, c, Some(std::sync::Arc::new(|d: &mut Duel, cc: CardId| card::is_code(d, cc, &[59364406]))), true, None);
    // destroy
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e3, Some(condition(descon)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
/// `s.descon(e,tp,eg,ep,...)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // aux.IsUnionState(e)
    if !aux::is_union_state().call(d, &[Value::Effect(a.e)]).truthy() {
        return false;
    }
    // and ep~=tp
    if a.ep == a.tp {
        return false;
    }
    // and eg:GetFirst()==e:GetHandler():GetEquipTarget()
    let first = group::get_first(d, &a.eg.clone().nil());
    let h = effect::get_handler(d, a.e).nil();
    first == card::get_equip_target(d, h)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown()
    card::is_facedown(d, c)
}
/// `s.destg(e,tp,...,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and s.desfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && desfilter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&desfilter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// `s.desop(e,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFacedown() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_facedown(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
