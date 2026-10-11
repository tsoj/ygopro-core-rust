//! Heraldic Beast Eale
use super::prelude::*;

pub const ID: u32 = 82315772;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERALDIC_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| spcon(d, a.card(1)))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, c: Option<CardId>) -> bool {
    // if c==nil then return true end
    let Some(c) = c else { return true };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0 and
    //   Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_HERALDIC_BEAST),c:GetControler(),LOCATION_MZONE,0,2,nil)
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HERALDIC_BEAST as u16])),
            tp,
            LOCATION_MZONE,
            0,
            2,
            NO_EX,
        )
}
