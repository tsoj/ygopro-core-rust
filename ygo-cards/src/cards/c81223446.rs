//! Necro Fusion
use super::prelude::*;

pub const ID: u32 = 81223446;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 Fusion Monster from your Extra Deck, by banishing Fusion Materials mentioned on it from your GY face-down, but it cannot attack this turn
    // local e1=Fusion.CreateSummonEff({
    //     handler=c,
    //     matfilter=aux.FALSE,
    //     extrafil=s.fextra,
    //     extratg=s.extratg,
    //     extraop=s.extraop,
    //     stage2=s.stage2}
    // )
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extratg: Some(cost_target(extratg)),
            extraop: Some(fusion::extra_op(extraop)),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.matfilter(c,tp)`
fn matfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToRemove(tp,POS_FACEDOWN) and aux.SpElimFilter(c)
    card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(s.matfilter),tp,LOCATION_GRAVE|LOCATION_MZONE,0,nil,tp)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_monster(d, c) && matfilter(d, c, tp)),
        tp,
        LOCATION_GRAVE | LOCATION_MZONE,
        0,
        NO_EX,
    );
    (Some(g), None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.extraop(e,fc,tp,sg)`
fn extraop(d: &mut Duel, _e: EffectId, _fc: Option<CardId>, tp: u8, sg: &mut CardSet) -> bool {
    // local faceup,facedown=sg:Split(Card.IsFaceup,nil)
    let (faceup, facedown) = group::split(d, sg, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #faceup>0 then Duel.HintSelection(faceup) end
    if !faceup.is_empty() {
        duel::hint_selection(d, faceup, true);
    }
    // if #facedown>0 then Duel.ConfirmCards(1-tp,facedown) end
    if !facedown.is_empty() {
        duel::confirm_cards(d, 1 - tp, facedown);
    }
    // Duel.Remove(sg,POS_FACEDOWN,REASON_EFFECT|REASON_MATERIAL|REASON_FUSION)
    duel::remove(d, sg.clone(), POS_FACEDOWN as u8, REASON_EFFECT | REASON_MATERIAL | REASON_FUSION, None, None);
    // sg:Clear()
    sg.clear();
    true
}
/// `s.stage2(e,fc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, fc: Option<CardId>, _tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk == 1 {
        // It cannot attack this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3206)
        effect::set_description(d, e1, 3206);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // fc:RegisterEffect(e1,true)
        card::register_effect(d, fc.nil(), e1, true);
    }
}
