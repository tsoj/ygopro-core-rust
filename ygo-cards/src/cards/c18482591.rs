//! Warlock of the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 18482591;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ICE_BARRIER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot activate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetValue(s.aclimit)
    effect::set_value_fn(d, e2, lua_fn(aclimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_SSET)
    effect::set_code(d, e3, EVENT_SSET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.con)
    effect::set_condition(d, e3, Some(condition(con)));
    // e3:SetOperation(s.aclimset)
    effect::set_operation(d, e3, Some(operation(aclimset)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.con(e)
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_ICE_BARRIER),e:GetHandler():GetControler(),LOCATION_MZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, h);
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| {
            card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
        })),
        p,
        LOCATION_MZONE,
        0,
        1,
        h.into(),
    )
}
// function s.aclimit(e,re,tp)
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // if not re:IsHasType(EFFECT_TYPE_ACTIVATE) or not re:IsSpellEffect() then return false end
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) || !effect::is_spell_effect(d, re) {
        return false;
    }
    // local c=re:GetHandler()
    let c = effect::get_handler(d, re).nil();
    // return not c:IsLocation(LOCATION_SZONE) or c:GetFlagEffect(id)>0
    !card::is_location(d, c, LOCATION_SZONE) || card::get_flag_effect(d, c, ID) > 0
}
// function s.aclimset(e,tp,eg,ep,ev,re,r,rp)
fn aclimset(d: &mut Duel, a: &EffArgs) {
    // for tc in eg:Iter() do
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN,0,1)
        card::register_flag_effect(
            d,
            tc,
            ID,
            RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN,
            0,
            1,
            0,
            0,
        );
    }
}
