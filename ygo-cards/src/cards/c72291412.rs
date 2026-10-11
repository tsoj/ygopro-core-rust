//! D/D Necro Slime
use super::prelude::*;

pub const ID: u32 = 72291412;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DDD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion summon using materials from the GY
    // local params = {fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_DDD),matfilter=aux.FALSE,extrafil=s.fextra,
    //                 extraop=Fusion.BanishMaterial,gc=Fusion.ForcedHandler,extratg=s.extratarget}
    let params = || fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_DDD as u16]))),
        matfilter: Some(std::sync::Arc::new(|_d: &mut Duel, _c: CardId, _e: EffectId, _tp: u8, _chk: i64| false)),
        extrafil: Some(std::sync::Arc::new(|d: &mut Duel, _e: EffectId, tp: u8, mg: &CardSet| fextra(d, tp, mg))),
        extraop: Some(fusion::banish_material()),
        gc: Some(fusion::forced_handler()),
        extratg: Some(cost_target(extratarget)),
        ..Default::default()
    };
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(Fusion.SummonEffTG(params))
    let tg = fusion::summon_eff_tg(d, params());
    effect::set_target(d, e1, Some(tg));
    // e1:SetOperation(Fusion.SummonEffOP(params))
    let op = fusion::summon_eff_op(d, params());
    effect::set_operation(d, e1, Some(op));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)),
            tp,
            LOCATION_GRAVE,
            0,
            NO_EX,
        );
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
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,e:GetHandler(),0,tp,LOCATION_GRAVE)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(h.into()), 0, a.tp, LOCATION_GRAVE as i32);
    true
}
