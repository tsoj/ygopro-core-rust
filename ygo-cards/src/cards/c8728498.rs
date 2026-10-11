//! Donner, Dagger Fur Hire
use super::prelude::*;

pub const ID: u32 = 8728498;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_FUR_HIRE as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2 monsters with different Types
    // Link.AddProcedure(c,nil,2,2,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(2), Some(group_check(lcheck)), None, None);
    // Destroy 1 monster on each field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 monster "Fur Hire"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:CheckDifferentPropertyBinary(Card.GetRace,lc,sumtype,tp)
    group::check_different_property_binary(d, g, &|d, c| card::get_race_ctx(d, c, Some(lc), sumtype, Some(tp)))
}
/// `s.desfilter(c,e,tp)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeEffectTarget(e) and (c:IsControler(1-tp) or (c:IsFaceup() and c:IsSetCard(SET_FUR_HIRE)))
    card::is_can_be_effect_target(d, c, Some(e))
        && (card::is_controler(d, c, 1 - tp) || (card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_FUR_HIRE as u16])))
}
/// `s.desrescon(sg,e,tp,mg)`
fn desrescon(d: &mut Duel, sg: &CardSet) -> bool {
    // return sg:GetClassCount(Card.GetControler)==2
    group::get_class_count(d, sg, &|d, c| card::get_controler(d, c) as i64) == 2
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.desrescon,0) end
    if a.chk == Some(0) {
        let rescon = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| (desrescon(d, sg), false);
        return aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false).err().unwrap_or(false);
    }
    // local dg=aux.SelectUnselectGroup(g,e,tp,2,2,s.desrescon,1,tp,HINTMSG_DESTROY)
    let rescon = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| (desrescon(d, sg), false);
    let dg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_DESTROY as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(dg)
    duel::set_target_card(d, dg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,2,PLAYER_ALL,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 2, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.spcostfilter(c,e,tp)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp,c:GetOriginalCode())
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 0 {
        return false;
    }
    let code = card::get_original_code(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard(SET_FUR_HIRE) and not c:IsOriginalCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_FUR_HIRE as u16])
        && !card::is_original_code(d, c, &[code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.spcostfilter,1,false,nil,nil,e,tp) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, a.tp, Some(&|d, c| spcostfilter(d, c, a.e, a.tp)), 1, 1, false, None, NO_EX);
    }
    // local rc=Duel.SelectReleaseGroupCost(tp,s.spcostfilter,1,1,false,nil,nil,e,tp):GetFirst()
    let g = aux::select_release_group_cost(d, a.tp, Some(&|d, c| spcostfilter(d, c, a.e, a.tp)), 1, 1, false, None, NO_EX);
    let rc = group::get_first(d, &g).nil();
    // e:SetLabel(rc:GetOriginalCode(),rc:GetType())
    let code = card::get_original_code(d, rc);
    let typ = card::get_type(d, rc);
    effect::set_label(d, a.e, &[code as i64, typ as i64]);
    // Duel.Release(rc,REASON_COST)
    duel::release(d, rc, REASON_COST, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local code,type=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let code = labels.first().copied().unwrap_or(0) as u32;
    let typ = labels.get(1).copied().unwrap_or(0) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp,code)
    let g1 = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, a.tp, code))),
        a.tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g1>0 and Duel.SpecialSummon(g1,0,tp,tp,false,false,POS_FACEUP)>0
    if !g1.is_empty() && duel::special_summon(d, g1, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // and type&TYPE_LINK==TYPE_LINK and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        if typ & TYPE_LINK == TYPE_LINK
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            // and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp,code)
            && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, a.tp, code))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
            // and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local g2=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp,code)
            let g2 = duel::select_matching_card(
                d,
                a.tp,
                Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, a.tp, code))),
                a.tp,
                LOCATION_HAND | LOCATION_GRAVE,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil();
            // if #g2==0 then return end
            if g2.is_empty() {
                return;
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g2,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g2, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
