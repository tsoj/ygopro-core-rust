//! Dream Mirror of Chaos
use super::prelude::*;

pub const ID: u32 = 98570539;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DREAM_MIRROR], listed_names: &[CARD_DREAM_MIRROR_JOY, CARD_DREAM_MIRROR_TERROR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion summon 1 "Dream Mirror" fusion monster
    // Using monsters from your field as material
    // If "Dream Mirror of Joy" is on the field, include from hand as well
    // If "Dream Mirror of Terror" is on the field, banish from GY as well
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_DREAM_MIRROR),
    //     matfilter=Fusion.OnFieldMat,extrafil=s.fextra,extraop=s.extraop,extratg=s.extrtarget})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_DREAM_MIRROR as u16]))),
            matfilter: Some(fusion::on_field_mat()),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::extra_op(extraop)),
            extratg: Some(cost_target(extrtarget)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // local joy=Duel.IsEnvironment(CARD_DREAM_MIRROR_JOY,PLAYER_ALL,LOCATION_FZONE)
    let joy = duel::is_environment(d, CARD_DREAM_MIRROR_JOY, Some(PLAYER_ALL as u8), Some(LOCATION_FZONE));
    // local terror=Duel.IsEnvironment(CARD_DREAM_MIRROR_TERROR,PLAYER_ALL,LOCATION_FZONE) and not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION)
    let terror = duel::is_environment(d, CARD_DREAM_MIRROR_TERROR, Some(PLAYER_ALL as u8), Some(LOCATION_FZONE))
        && duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none();
    // local joyg=Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_HAND,0,nil)
    let mut joyg = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)), tp, LOCATION_HAND, 0, NO_EX);
    // local terrorg=Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
    let terrorg = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if joy and terror then
    if joy && terror {
        // joyg:Merge(terrorg)
        group::add_card(&mut joyg, &terrorg);
        // return joyg
        return (Some(joyg), None);
    }
    // if joy then return joyg end
    if joy {
        return (Some(joyg), None);
    }
    // if terror then return terrorg end
    if terror {
        return (Some(terrorg), None);
    }
    (None, None)
}
/// `s.extraop(e,tc,tp,sg)`
fn extraop(d: &mut Duel, _e: EffectId, _tc: Option<CardId>, _tp: u8, sg: &mut CardSet) -> bool {
    // local g1=sg:Filter(Card.IsLocation,nil,LOCATION_GRAVE)
    let g1 = group::filter(d, sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // Duel.Remove(g1,POS_FACEUP,REASON_EFFECT+REASON_MATERIAL+REASON_FUSION)
    duel::remove(d, g1.clone(), POS_FACEUP as u8, REASON_EFFECT + REASON_MATERIAL + REASON_FUSION, None, None);
    // sg:Sub(g1)
    group::remove_card(sg, &g1);
    true
}
/// `s.extrtarget(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extrtarget(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,0,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, a.tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 0, a.tp, LOCATION_GRAVE as i32);
    true
}
