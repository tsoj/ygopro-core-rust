//! Fusion Parasite
use super::prelude::*;

pub const ID: u32 = 6205579;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion substitute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_FUSION_SUBSTITUTE)
    effect::set_code(d, e1, EFFECT_FUSION_SUBSTITUTE);
    // e1:SetCondition(s.subcon)
    effect::set_condition(d, e1, Some(lua_fn(subcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(id)
    effect::set_code(d, e2, ID);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon
    // local params = {nil,Fusion.OnFieldMat,nil,nil,Fusion.ForcedHandler}
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    let p = fusion::SummonEff { matfilter: Some(fusion::on_field_mat()), gc: Some(fusion::forced_handler()), ..Default::default() };
    // e3:SetTarget(Fusion.SummonEffTG(table.unpack(params)))
    let tg = fusion::summon_eff_tg(d, p.clone());
    effect::set_target(d, e3, Some(tg));
    // e3:SetOperation(Fusion.SummonEffOP(table.unpack(params)))
    let op = fusion::summon_eff_op(d, p);
    effect::set_operation(d, e3, Some(op));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.subcon(e)`
fn subcon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_location(d, h, LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // return ph~=PHASE_DAMAGE and ph~=PHASE_DAMAGE_CAL
    ph != PHASE_DAMAGE && ph != PHASE_DAMAGE_CAL
}
