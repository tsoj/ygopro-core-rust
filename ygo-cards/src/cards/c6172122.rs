//! Red-Eyes Fusion
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 6172122;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_REDEYES_B_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=aux.FilterBoolFunction(Card.ListsArchetypeAsMaterial,SET_RED_EYES),
    //   extrafil=s.fextra,stage2=s.stage2,extratg=s.extratg})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::lists_archetype_as_material(d, c, &[SET_RED_EYES]))),
            extrafil: Some(Arc::new(fextra)),
            stage2: Some(Arc::new(stage2)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetActivityCount(tp,ACTIVITY_SUMMON)==0 and Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_activity_count(d, tp, &[ACTIVITY_SUMMON]).first() == Some(&0)
            && duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON]).first() == Some(&0);
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetLabelObject(e)
    effect::set_label_object(d, e2, Value::Effect(a.e));
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return se~=e:GetLabelObject()
    let lo = effect::get_label_object(d, a.e(0)).as_effect();
    a.v(6).as_effect() != lo
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,0,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.fextra(e,tp,mg,sumtype)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_DECK,0,nil),s.fcheck
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, NO_EX);
    (Some(g), Some(lua_fn(fcheck)))
}
/// `s.fcheck(tp,sg,fc,sumtype,tp)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(s.ffilter,1,nil,fc,sumtype,tp)
    let sg = a.g(1);
    let fc = a.c(2);
    let sumtype = a.int(3) as u64;
    let tp = a.p(4);
    group::is_exists(d, &sg, &|d, c| ffilter(d, c, fc, sumtype, tp), 1, NO_EX)
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_RED_EYES,fc,sumtype,tp) and (not fc.material or c:IsSummonCode(fc,sumtype,tp,fc.material))
    if !card::is_set_card_ctx(d, c, &[SET_RED_EYES as u16], Some(fc), sumtype, Some(tp)) {
        return false;
    }
    let material = crate::class::material(d, fc);
    material.is_empty() || card::is_summon_code(d, c, Some(fc), sumtype, tp, &material)
}
/// `s.stage2(e,tc,tp,mg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _mg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk == 1 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_CODE)
        effect::set_code(d, e1, EFFECT_CHANGE_CODE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(CARD_REDEYES_B_DRAGON)
        effect::set_value(d, e1, CARD_REDEYES_B_DRAGON as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
    }
}
