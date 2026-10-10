//! Pyramid of Light
use super::prelude::*;

pub const ID: u32 = 53569894;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15013468, 51402177], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Register when this card leaves the field
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD_P);
    effect::set_operation(d, e2, Some(operation(checkop)));
    card::register_effect(d, c, e2, false);
    // Destroy and banish "Andro Sphinx" and "Sphinx Teleia" you control
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_REMOVE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e3, Some(operation(desop)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and not c:IsDisabled() then e:SetLabel(1) else e:SetLabel(0) end
    if card::is_faceup(d, c) && !card::is_disabled(d, c) {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    // if e:GetLabelObject():GetLabel()==1 and c:IsPreviousControler(tp) then
    if effect::get_label(d, lo) == 1 && card::is_previous_controler(d, c, a.tp) {
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,15013468,51402177),tp,LOCATION_ONFIELD,0,nil)
        let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[15013468, 51402177]))), a.tp, LOCATION_ONFIELD, 0, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Hint(HINT_CARD,1-tp,id)
            duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
            // Duel.Destroy(g,REASON_EFFECT,LOCATION_REMOVED)
            duel::destroy(d, g, REASON_EFFECT, Some(LOCATION_REMOVED), None);
        }
    }
}
