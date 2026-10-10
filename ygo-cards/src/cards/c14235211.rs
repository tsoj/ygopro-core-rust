//! Rider of the Storm Winds
use super::prelude::*;

pub const ID: u32 = 14235211;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    // (no comment in the script)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_PIERCE);
    card::register_effect(d, c, e2, false);
    // destroy sub
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e3, EFFECT_DESTROY_SUBSTITUTE);
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_NORMAL)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_NORMAL)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let _g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if c:IsLocation(LOCATION_MZONE) and c:IsFacedown() then return end
    if card::is_location(d, c, LOCATION_MZONE) && card::is_facedown(d, c) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or tc:GetControler()~=tp or tc:IsFacedown() or not tc:IsRelateToEffect(e) then
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0
        || card::get_controler(d, tc.nil()) != tp
        || card::is_facedown(d, tc.nil())
        || !card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        return;
    }
    // Duel.Equip(tp,c,tc,true)
    duel::equip(d, tp, c, tc.nil(), true, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(s.eqlimit)
    effect::set_value_fn(d, e1, lua_fn(eqlimit));
    // e1:SetLabelObject(tc)
    effect::set_label_object(d, e1, Value::Card(tc.nil()));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, a.e(0)).as_card() == a.card(1)
}
