//! Shaddoll Schism
use super::prelude::*;

pub const ID: u32 = 21011044;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHADDOLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Fusion Summon 1 "Shaddoll" Fusion Monster from your Extra Deck, by banishing Fusion Materials mentioned on it from your field or GY, but it cannot attack directly
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_SHADDOLL),matfilter=Fusion.OnFieldMat(Card.IsAbleToRemove),
    //     extrafil=s.fextra,extraop=Fusion.BanishMaterial,stage2=s.stage2,desc=aux.Stringid(id,0),extratg=s.extratg})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_SHADDOLL as u16]))),
            matfilter: Some(fusion::on_field_mat_with(fusion::mat_filter_fn(|d, c, _e, tp, chk| card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32))))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::banish_material()),
            stage2: Some(std::sync::Arc::new(stage2)),
            desc: Some(aux::stringid(ID, 0)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_GRAVE, 0, NO_EX);
        return (Some(g), None);
    }
    // end
    (None, None)
}
/// `s.tgfilter(c,attr)`
fn tgfilter(d: &mut Duel, c: CardId, attr: u32) -> bool {
    // return c:IsAttribute(attr) and c:IsFaceup() and c:IsAbleToGrave()
    card::is_attribute(d, c, attr) && card::is_faceup(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk == 1 {
        let tc = tc.nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3207)
        effect::set_description(d, e1, 3207);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local attr=tc:GetAttribute()
        let attr = card::get_attribute(d, tc);
        // if Duel.IsExistingMatchingCard(s.tgfilter,tp,0,LOCATION_MZONE,1,nil,attr)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, attr)), tp, 0, LOCATION_MZONE, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
            // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,0,LOCATION_MZONE,1,1,nil,attr)
            let g = duel::select_matching_card(d, tp, Some(&|d, c| tgfilter(d, c, attr)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.HintSelection(g)
                duel::hint_selection(d, g.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoGrave(g,REASON_EFFECT)
                duel::send_to_grave(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
