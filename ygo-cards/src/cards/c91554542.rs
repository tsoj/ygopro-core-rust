//! Neo Flamvell Sabre
use super::prelude::*;

pub const ID: u32 = 91554542;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.val(e)`
fn val(d: &mut Duel, e: EffectId) -> i64 {
    // local gct=Duel.GetFieldGroupCount(e:GetHandler():GetControler(),0,LOCATION_GRAVE)
    let h = effect::get_handler(d, e).nil();
    let p = card::get_controler(d, h);
    let gct = duel::get_field_group_count(d, p, 0, LOCATION_GRAVE);
    // if gct<=4 then return 600
    if gct <= 4 {
        600
    // elseif gct>=8 then return -300
    } else if gct >= 8 {
        -300
    // else return 0 end
    } else {
        0
    }
}
