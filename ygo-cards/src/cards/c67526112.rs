//! Rapid Trigger
use super::prelude::*;

pub const ID: u32 = 67526112;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff{handler=c,extraop=s.extraop,stage2=s.stage2,matfilter=s.matfil,extratg=s.extratg}
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            matfilter: Some(fusion::mat_filter_fn(|d, c, e, tp, chk| matfil(d, c, e, tp, chk))),
            extraop: Some(fusion::extra_op(extraop)),
            stage2: Some(std::sync::Arc::new(stage2)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON + CATEGORY_DESTROY);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.matfil(c,e,tp,chk)`
fn matfil(d: &mut Duel, c: CardId, e: EffectId, _tp: u8, _chk: i64) -> bool {
    // return c:IsOnField() and c:IsDestructable(e) and not c:IsImmuneToEffect(e)
    card::is_on_field(d, c) && card::is_destructable(d, c, Some(e)) && !card::is_immune_to_effect(d, c, e)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=Duel.GetFusionMaterial(tp):Filter(Card.IsOnField,nil,e,tp,0)
    let fm = duel::get_fusion_material(d, a.tp);
    let dg = group::filter(d, &fm, &|d, c| card::is_on_field(d, c), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,2,tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 2, a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.extraop(e,tc,tp,sg)`
fn extraop(d: &mut Duel, _e: EffectId, _tc: Option<CardId>, _tp: u8, sg: &mut CardSet) -> bool {
    // local res=Duel.Destroy(sg,REASON_EFFECT+REASON_MATERIAL+REASON_FUSION)==#sg
    let n = sg.len() as i32;
    let res = duel::destroy(d, sg.clone(), REASON_EFFECT + REASON_MATERIAL + REASON_FUSION, None, None) == n;
    // sg:Clear()
    *sg = CardSet::new();
    // return res
    res
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk == 1 {
        let tc = tc.nil();
        let h = effect::get_handler(d, e).nil();
        // Limits the battle targets
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
        effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(s.atlimit)
        effect::set_value_fn(d, e1, effect_card_fn(|d, _e, c| atlimit(d, c)));
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
        // Prevent direct attacks
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1b = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1b, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
        effect::set_code(d, e1b, EFFECT_CANNOT_DIRECT_ATTACK);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1b, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1b, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1b, true);
        // Immune
        // local e3=Effect.CreateEffect(e:GetHandler())
        let e3 = effect::create_effect(d, h);
        // e3:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e3, aux::stringid(ID, 0));
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_IMMUNE_EFFECT)
        effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
        // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e3:SetRange(LOCATION_MZONE)
        effect::set_range(d, e3, LOCATION_MZONE);
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // e3:SetValue(s.efilter)
        effect::set_value_fn(d, e3, lua_fn(efilter));
        // tc:RegisterEffect(e3,true)
        card::register_effect(d, tc, e3, true);
    }
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, c: CardId) -> bool {
    // return not (c:IsSpecialSummoned() and c:IsSummonLocation(LOCATION_EXTRA))
    !(card::is_special_summoned(d, c) && card::is_summon_location(d, c, LOCATION_EXTRA))
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    // local tc=te:GetOwner()
    let tc = effect::get_owner(d, te).nil();
    // return tc:IsSpecialSummoned() and tc:IsSummonLocation(LOCATION_EXTRA) and tc~=e:GetHandler()
    //     and te:IsMonsterEffect() and te:IsActivated() and te:GetActivateLocation()==LOCATION_MZONE
    card::is_special_summoned(d, tc)
        && card::is_summon_location(d, tc, LOCATION_EXTRA)
        && Some(tc) != effect::get_handler(d, e)
        && effect::is_monster_effect(d, te)
        && effect::is_activated(d, te)
        && effect::get_activate_location(d, te) == LOCATION_MZONE
}
