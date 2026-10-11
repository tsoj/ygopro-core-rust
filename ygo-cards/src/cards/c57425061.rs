//! 幻影融合 (Vision Fusion)
use super::prelude::*;

pub const ID: u32 = 57425061;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 "HERO" Fusion Monster from your Extra Deck, using monsters from your hand or field as Fusion Material
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=function(c) return c:IsSetCard(SET_HERO) end,extrafil=s.fextra,extraop=s.extraop,extratg=s.extratg})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_HERO as u16]))),
            extrafil: Some(std::sync::Arc::new(|d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet| fextra(d, tp))),
            extraop: Some(fusion::extra_op(|d, _e, _tc, _tp, sg| {
                extraop(d, sg);
                true
            })),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.fextrafilter(c)`
fn fextrafilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonsterCard() and c:IsContinuousTrap() and c:IsAbleToRemove() and c:IsLocation(LOCATION_STZONE)
    card::is_monster_card(d, c) && card::is_continuous_trap(d, c) && card::is_able_to_remove(d, c, None, None, None) && card::is_location(d, c, LOCATION_STZONE)
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // return sg:FilterCount(s.fextrafilter,nil)<=2
    let sg = a.g(1);
    group::filter_count(d, &sg, &fextrafilter, NO_EX) <= 2
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, tp: u8) -> (Option<CardSet>, Option<LuaFn>) {
    // local sg=Duel.GetMatchingGroup(s.fextrafilter,tp,LOCATION_STZONE,0,nil)
    let sg = duel::get_matching_group(d, Some(&fextrafilter), tp, LOCATION_STZONE, 0, NO_EX);
    // if #sg>0 then return sg,s.fcheck end
    if !sg.is_empty() {
        return (Some(sg), Some(lua_fn(fcheck)));
    }
    // return nil
    (None, None)
}
/// `s.extraop(e,tc,tp,sg)`
fn extraop(d: &mut Duel, sg: &mut CardSet) {
    // local rg=sg:Filter(s.fextrafilter,nil)
    let rg = group::filter(d, sg, &fextrafilter, NO_EX);
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT|REASON_MATERIAL|REASON_FUSION)
        duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT | REASON_MATERIAL | REASON_FUSION, None, None);
        // sg:Sub(rg)
        group::remove_card(sg, &rg);
    }
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_STZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_STZONE as i32);
    true
}
