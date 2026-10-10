//! Cyber Shield
use super::prelude::*;

pub const ID: u32 = 63224564;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_HARPIE_LADY, CARD_HARPIE_LADY_SISTERS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsCode,CARD_HARPIE_LADY,CARD_HARPIE_LADY_SISTERS))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_code(d, c, &[CARD_HARPIE_LADY, CARD_HARPIE_LADY_SISTERS]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // Atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
