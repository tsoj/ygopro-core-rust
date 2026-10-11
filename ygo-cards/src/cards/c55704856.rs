//! Cyberload Fusion
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 55704856;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CYBER_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.ListsArchetypeAsMaterial,SET_CYBER_DRAGON),Fusion.OnFieldMat(Card.IsAbleToDeck),s.fextra,Fusion.ShuffleMaterial,nil,s.stage2)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::lists_archetype_as_material(d, c, &[SET_CYBER_DRAGON as u32]))),
            matfilter: Some(fusion::on_field_mat_with(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_able_to_deck(d, c)))),
            extrafil: Some(Arc::new(|d: &mut Duel, e: EffectId, tp: u8, mg: &CardSet| fextra(d, e, tp, mg))),
            extraop: Some(fusion::shuffle_material()),
            gc: None,
            stage2: Some(Arc::new(|d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64| stage2(d, e, tc, tp, sg, chk))),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.fcheck(tp,sg,fc)
fn fcheck(tp: u8) -> LuaFn {
    lua_fn(move |d, a| {
        // return sg:IsExists(aux.FilterBoolFunction(Card.IsSetCard,SET_CYBER_DRAGON,fc,SUMMON_TYPE_FUSION,tp),1,nil)
        let sg = a.g(1);
        let fc = a.card(2);
        group::is_exists(d, &sg, &|d, c| card::is_set_card_ctx(d, c, &[SET_CYBER_DRAGON as u16], fc, SUMMON_TYPE_FUSION as u64, Some(tp)), 1, NO_EX)
    })
}
// function s.fextra(e,tp,mg)
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(aux.AND(Card.IsAbleToDeck,Card.IsFaceup)),tp,LOCATION_REMOVED,0,nil),s.fcheck
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_deck(d, c) && card::is_faceup(d, c)), tp, LOCATION_REMOVED, 0, NO_EX);
    (Some(g), Some(fcheck(tp)))
}
// function s.stage2(e,tc,tp,sg,chk)
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk == 1 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        // e1:SetLabel(tc:GetFieldID())
        let tc = tc.nil();
        effect::set_label(d, e1, &[card::get_field_id(d, tc) as i64]);
        // e1:SetTarget(s.atktg)
        effect::set_target(d, e1, Some(effect_card_fn(atktg)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
// function s.atktg(e,c)
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetLabel()~=c:GetFieldID()
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
