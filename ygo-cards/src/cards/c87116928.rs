//! Chimeratech Megafleet Dragon
use super::prelude::*;

pub const ID: u32 = 87116928;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CYBER_DRAGON], material_setcode: &[SET_CYBER, SET_CYBER_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixRep(c,true,true,s.matfilter,1,99,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_CYBER_DRAGON))
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))),
        1,
        99,
        vec![fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
            card::is_set_card_ctx(d, c, &[SET_CYBER_DRAGON as u16], Some(fc), st, Some(tp))
        }))],
    );
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            ..Default::default()
        },
    );
    // cannot be fusion material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_FUSION_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.matfilter(c,fc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, _fc: CardId, _st: u64, tp: u8) -> bool {
    // return c:GetSequence()>4 and c:IsLocation(LOCATION_MZONE) and (c:IsControler(tp) or c:IsFaceup())
    card::get_sequence(d, c) > 4 && card::is_location(d, c, LOCATION_MZONE) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetLocation()~=LOCATION_EXTRA
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_location(d, h) != LOCATION_EXTRA
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToGraveAsCost() and (c:IsControler(tp) or c:IsFaceup())
    card::is_able_to_grave_as_cost(d, c) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp)
    duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX)
}
/// `s.contactop(g,tp,c)`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, c: CardId) {
    // Duel.SendtoGrave(g,REASON_COST|REASON_MATERIAL)
    duel::send_to_grave(d, g.clone(), REASON_COST | REASON_MATERIAL, None, None);
    // spsummon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // e1:SetValue(#g*1200)
    effect::set_value(d, e1, g.len() as i64 * 1200);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
