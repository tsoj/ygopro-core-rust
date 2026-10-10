//! Simul Archfiends
use super::prelude::*;

pub const ID: u32 = 94423983;

const TYPE_RITUAL_FUSION_SYNCHRO_XYZ_LINK: u32 = TYPE_RITUAL | TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(0,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, 0, ID) == 0;
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.RegisterFlagEffect(0,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, 0, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Players cannot Special Summon monsters with the same type as they control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetTarget(s.sumlimit)
    effect::set_target(d, e1, Some(lua_fn(sumlimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // Increase ATK of monsters
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsType,TYPE_RITUAL_FUSION_SYNCHRO_XYZ_LINK),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_RITUAL_FUSION_SYNCHRO_XYZ_LINK))), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local types={TYPE_RITUAL,TYPE_FUSION,TYPE_SYNCHRO,TYPE_XYZ,TYPE_LINK}
    let types = [TYPE_RITUAL, TYPE_FUSION, TYPE_SYNCHRO, TYPE_XYZ, TYPE_LINK];
    // for _,typ in ipairs(types) do
    for typ in types {
        // if g:IsExists(Card.IsType,2,nil,typ) then return end
        if group::is_exists(d, &g, &|d, c| card::is_type(d, c, typ), 2, NO_EX) {
            return;
        }
    }
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // Increase ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,sump,LOCATION_MZONE,0,nil)
    let sump = a.p(2);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), sump, LOCATION_MZONE, 0, NO_EX);
    // local mtypes=g:GetBitwiseOr(Card.GetType)&TYPE_RITUAL_FUSION_SYNCHRO_XYZ_LINK
    let mtypes = group::get_bitwise_or(d, &g, &|d, c| card::get_type(d, c) as i64) as u32 & TYPE_RITUAL_FUSION_SYNCHRO_XYZ_LINK;
    // return c:IsType(mtypes)
    let c = a.c(1);
    card::is_type(d, c, mtypes)
}
