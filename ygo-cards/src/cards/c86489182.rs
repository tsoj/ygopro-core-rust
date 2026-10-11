//! Simorgh, Bird of Ancestry
use super::prelude::*;

pub const ID: u32 = 86489182;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // normal monster
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_code(d, e1, EFFECT_ADD_TYPE);
    effect::set_value(d, e1, TYPE_NORMAL as i64);
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_REMOVE_TYPE);
    effect::set_value(d, e2, TYPE_EFFECT as i64);
    card::register_effect(d, c, e2, false);
    // decrease tribute
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_DECREASE_TRIBUTE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_HAND, LOCATION_HAND);
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_WIND))));
    effect::set_value(d, e3, 0x10001);
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_DECREASE_TRIBUTE_SET);
    card::register_effect(d, c, e4, false);
    // summon success
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 0));
    effect::set_category(d, e5, CATEGORY_TOHAND);
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e5, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e5, Some(condition(cond)));
    effect::set_target(d, e5, Some(cost_target(target)));
    effect::set_operation(d, e5, Some(operation(op)));
    card::register_effect(d, c, e5, false);
    // tribute check
    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e6, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e6, lua_fn(valcheck));
    effect::set_label_object(d, e6, Value::Effect(e5));
    card::register_effect(d, c, e6, false);
}
/// `s.valcheck(e,c)`: sets the label of e5 (the label object of this effect)
fn valcheck(d: &mut Duel, a: Args) -> Value {
    let (e, c) = (a.e(0), a.c(1));
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_WIND)==#g then
    let n = group::filter_count(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_WIND), NO_EX);
    let label = if n == g.len() { 1 } else { 0 };
    // e:GetLabelObject():SetLabel(1) / SetLabel(0)
    let e5 = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, e5, &[label]);
    Value::Nil
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned() and e:GetLabel()==1
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h) && effect::get_label(d, a.e) == 1
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_hand(d, chkc, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,2,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local g=tg:Filter(Card.IsRelateToEffect,nil,e)
    let g = group::filter(d, &tg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #g>0 then Duel.SendtoHand(g,nil,REASON_EFFECT) end
    if group::get_count(&g) > 0 {
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
