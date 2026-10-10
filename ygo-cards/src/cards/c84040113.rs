//! Muddy Mudragon
//! ドロドロゴン
use super::prelude::*;

pub const ID: u32 = 84040113;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
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
    // fusion summon summon
    // local params = {nil,Fusion.OnFieldMat,nil,nil,Fusion.ForcedHandler}
    let params = fusion::SummonEff { matfilter: Some(fusion::on_field_mat()), gc: Some(fusion::forced_handler()), ..Default::default() };
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(Fusion.SummonEffTG(table.unpack(params)))
    let tg = fusion::summon_eff_tg(d, params.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(table.unpack(params)))
    let op = fusion::summon_eff_op(d, params);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.subcon(e)`
fn subcon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_ONFIELD|LOCATION_GRAVE)
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_location(d, h, LOCATION_ONFIELD | LOCATION_GRAVE)
}
/// `s.spcon(e)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsSynchroSummoned()
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_synchro_summoned(d, h)
}
