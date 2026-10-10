//! Fusion Destiny
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 52947044;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_HERO, SET_DESTINY_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.ListsArchetypeAsMaterial,SET_DESTINY_HERO),Fusion.InHandMat,s.fextra,nil,nil,s.stage2,nil,nil,nil,nil,nil,nil,nil,s.extratg)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::lists_archetype_as_material(d, c, &[SET_DESTINY_HERO]))),
            matfilter: Some(fusion::in_hand_mat()),
            extrafil: Some(Arc::new(fextra)),
            stage2: Some(Arc::new(stage2)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(aux.FilterBoolFunction(Card.IsSetCard,SET_DESTINY_HERO,fc,SUMMON_TYPE_FUSION,tp),1,nil)
    let tp = a.p(0);
    let sg = a.g(1);
    let fc = a.c(2);
    group::is_exists(
        d,
        &sg,
        &|d, c| card::is_set_card_ctx(d, c, &[SET_DESTINY_HERO as u16], Some(fc), SUMMON_TYPE_FUSION as u64, Some(tp)),
        1,
        NO_EX,
    )
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_DECK,0,nil),s.fcheck
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, NO_EX);
    (Some(g), Some(lua_fn(fcheck)))
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,0,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.stage2(e,tc,tp,mg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, _mg: Option<&CardSet>, chk: i64) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then
    if chk == 0 {
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,2)
        card::register_flag_effect(d, tc.nil(), ID, RESETS_STANDARD_PHASE_END, 0, 2, 0, 0);
    }
    // if chk==1 then
    if chk == 1 {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e2:SetCondition(s.descon)
        effect::set_condition(d, e2, Some(condition(descon)));
        // e2:SetOperation(s.desop)
        effect::set_operation(d, e2, Some(operation(desop)));
        // e2:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None) as i64;
        effect::set_label(d, e2, &[turn]);
        // e2:SetLabelObject(tc)
        effect::set_label_object(d, e2, Value::card(tc));
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
    }
    // if chk==2 then
    if chk == 2 {
        // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
        if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(aux.Stringid(id,0))
            effect::set_description(d, e1, aux::stringid(ID, 0));
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetTarget(s.splimit)
            effect::set_target(d, e1, Some(effect_card_fn(splimit)));
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
        }
    }
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not (c:IsSetCard(SET_HERO) and c:IsAttribute(ATTRIBUTE_DARK))
    !(card::is_set_card(d, c, &[SET_HERO as u16]) && card::is_attribute(d, c, ATTRIBUTE_DARK))
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // return Duel.GetTurnCount()~=e:GetLabel() and tc:GetFlagEffect(id)~=0
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e) && card::get_flag_effect(d, tc.nil(), ID) != 0
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Destroy(tc,REASON_EFFECT)
    duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
}
