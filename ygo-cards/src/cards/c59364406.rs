//! Burning Beast
use super::prelude::*;

pub const ID: u32 = 59364406;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[85359414],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddUnionProcedure(c,aux.FilterBoolFunction(Card.IsCode,85359414),true)
    aux::add_union_procedure(d, c, Some(std::sync::Arc::new(|d: &mut Duel, cc: CardId| card::is_code(d, cc, &[85359414]))), true, None);
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
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
    // return c:IsFaceup() and c:IsSpellTrap()
    card::is_faceup(d, c) && card::is_spell_trap(d, c)
}
/// `s.destg(e,tp,...,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and s.desfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && desfilter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&desfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// `s.desop(e,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
