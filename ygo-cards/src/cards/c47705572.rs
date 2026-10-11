//! Lunalight Wolf
//! 月光狼
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 47705572;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LUNALIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Prevent pendulum summon of non-"Lunalight" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Fusion summon
    // local params = {fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_LUNALIGHT),matfilter=Fusion.OnFieldMat(Card.IsAbleToRemove),
    //     extrafil=s.fextra,extraop=Fusion.BanishMaterial,extratg=s.extratarget}
    let params = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, x, _tp| card::is_set_card(d, x, &[SET_LUNALIGHT as u16]))),
        // Fusion.OnFieldMat(Card.IsAbleToRemove): the C function gets (c, e, tp, chk) by position
        matfilter: Some(fusion::on_field_mat_with(Arc::new(|d, x, _e, tp, chk| card::is_able_to_remove(d, x, None, Some(tp), Some(chk as u32))))),
        extrafil: Some(Arc::new(fextra)),
        extraop: Some(fusion::banish_material()),
        extratg: Some(cost_target(extratarget)),
        ..Default::default()
    };
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(Fusion.SummonEffTG(params))
    let tg = fusion::summon_eff_tg(d, params.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(params))
    let op = fusion::summon_eff_op(d, params);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Piercing damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e3, EFFECT_PIERCE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(s.ptg)
    effect::set_target(d, e3, Some(effect_card_fn(ptg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not (c:IsSetCard(SET_LUNALIGHT) and c:IsMonster()) and (sumtype&SUMMON_TYPE_PENDULUM)==SUMMON_TYPE_PENDULUM
    let c = a.c(1);
    let sumtype = a.int(3) as u64;
    !(card::is_set_card(d, c, &[SET_LUNALIGHT as u16]) && card::is_monster(d, c)) && (sumtype & SUMMON_TYPE_PENDULUM as u64) == SUMMON_TYPE_PENDULUM as u64
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_monster(d, x) && card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_GRAVE, 0, NO_EX);
        return (Some(g), None);
    }
    // return nil
    (None, None)
}
/// `s.extratarget(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratarget(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
/// `s.ptg(e,c)`
fn ptg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_LUNALIGHT) and c:IsMonster()
    card::is_set_card(d, c, &[SET_LUNALIGHT as u16]) && card::is_monster(d, c)
}
