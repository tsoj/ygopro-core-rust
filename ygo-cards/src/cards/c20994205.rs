//! Numbers Eveil
use super::prelude::*;

pub const ID: u32 = 20994205;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpecialSummoned() and c:IsSummonLocation(LOCATION_EXTRA)
    card::is_special_summoned(d, c) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,0,nil)==0
    duel::is_existing_matching_card(d, Some(&cfilter), tp, 0, LOCATION_MZONE, 1, NO_EX)
        && duel::get_matching_group_count(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX) == 0
}
/// `s.xyzfilter(c)`
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NUMBER) and c:IsType(TYPE_XYZ) and c.xyz_number
    card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_type(d, c, TYPE_XYZ) && crate::card_info(d, c).xyz_number.is_some()
}
/// `s.spfilter(c,e,p,g)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, p: u8, g: &CardSet) -> bool {
    // return Duel.GetLocationCountFromEx(p,p,g,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,p,false,false)
    //     and g:GetSum(function(mc)return mc.xyz_number end)==c.xyz_number
    if duel::get_location_count_from_ex(d, p, Some(p), g.into(), duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    if !card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, p, false, false, None, None, None) {
        return false;
    }
    let sum = group::get_sum(d, g, &|d, mc| crate::card_info(d, mc).xyz_number.unwrap_or(0) as i64);
    match crate::card_info(d, c).xyz_number {
        Some(n) => sum == n as i64,
        None => false,
    }
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // return sg:GetClassCount(Card.GetRank)==#sg and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,sg,e,tp,sg)
    let res = group::get_class_count(d, sg, &|d, c| card::get_rank(d, c) as i64) == sg.len()
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, sg)), tp, LOCATION_EXTRA, 0, 1, sg.into());
    (res, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local xg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil)
    let xg = duel::get_matching_group(d, Some(&xyzfilter), tp, LOCATION_EXTRA, 0, NO_EX);
    // if chk==0 then return #xg>4 and aux.SelectUnselectGroup(xg,e,tp,4,4,s.rescon,0) end
    if a.chk == Some(0) {
        return xg.len() > 4
            && matches!(
                aux::select_unselect_group(d, &xg, Some(a.e), tp, Some(4), Some(4), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, LOCATION_EXTRA as i32);
    true
}
/// `function(e) return e:GetHandler():GetControler()==e:GetLabel() end`
fn cannot_summon_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_controler(d, h) as i64 == effect::get_label(d, a.e)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local xg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil)
    let xg = duel::get_matching_group(d, Some(&xyzfilter), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #xg<5 then return end
    if xg.len() < 5 {
        return;
    }
    // local mg=aux.SelectUnselectGroup(xg,e,tp,4,4,s.rescon,1,tp,HINTMSG_XMATERIAL)
    let mg = aux::select_unselect_group(d, &xg, Some(a.e), tp, Some(4), Some(4), Some(&rescon), 1, tp, Some(HINTMSG_XMATERIAL as u64), None, None, false)
        .unwrap_or_default();
    // if #mg~=4 then return end
    if mg.len() != 4 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local spc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,mg,e,tp,mg):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, &mg)), tp, LOCATION_EXTRA, 0, 1, 1, false, (&mg).into()).nil();
    let spc = group::get_first(d, &sel);
    // if spc and Duel.SpecialSummonStep(spc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP) then
    if let Some(spc) = spc {
        if duel::special_summon_step(d, spc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) {
            // Cannot Special Summon, except "Number" Xyz Monsters
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(aux.Stringid(id,1))
            effect::set_description(d, e1, aux::stringid(ID, 1));
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetCondition(function(e) return e:GetHandler():GetControler()==e:GetLabel() end)
            effect::set_condition(d, e1, Some(condition(cannot_summon_cond)));
            // e1:SetTarget(function(_,c) return not (c:IsSetCard(SET_NUMBER) and c:IsType(TYPE_XYZ)) end)
            effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| {
                !(card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_type(d, c, TYPE_XYZ))
            })));
            // e1:SetLabel(tp)
            effect::set_label(d, e1, &[tp as i64]);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // spc:RegisterEffect(e1,true)
            card::register_effect(d, spc, e1, true);
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
            // spc:CompleteProcedure()
            card::complete_procedure(d, spc);
            // if not spc:IsImmuneToEffect(e) then Duel.Overlay(spc,mg) end
            if !card::is_immune_to_effect(d, spc, a.e) {
                duel::overlay(d, spc, mg, false);
            }
        }
    }
}
