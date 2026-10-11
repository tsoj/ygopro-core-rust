//! Secret Guards of the Ice Barrier
//! 氷結界の御庭番
use super::prelude::*;

pub const ID: u32 = 27527047;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ICE_BARRIER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // untargetable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_ICE_BARRIER))
    effect::set_target(
        d,
        e1,
        Some(effect_card_fn(|d, _e, c| {
            card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
        })),
    );
    // e1:SetValue(s.tgval)
    effect::set_value_fn(d, e1, lua_fn(tgval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tgval(e,re,rp)
fn tgval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    let rp = a.p(2);
    // return rp~=e:GetHandlerPlayer() and re:IsMonsterEffect()
    rp != effect::get_handler_player(d, e) && effect::is_monster_effect(d, re)
}
