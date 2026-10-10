//! The Light - Hex-Sealed Fusion
use super::prelude::*;

pub const ID: u32 = 15717011;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // fusion substitute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_FUSION_SUBSTITUTE)
    effect::set_code(d, e2, EFFECT_FUSION_SUBSTITUTE);
    // e2:SetCondition(s.subcon)
    effect::set_condition(d, e2, Some(condition(subcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.subcon(e)`: `return e:GetHandler():IsLocation(LOCATION_GRAVE|LOCATION_SZONE|LOCATION_MZONE|LOCATION_HAND)`
fn subcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE | LOCATION_SZONE | LOCATION_MZONE | LOCATION_HAND)
}
/// `s.filter(c,e,tp,m,gc,chkf)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, m: &CardSet, gc: CardId, chkf: u64) -> bool {
    // return c:IsType(TYPE_FUSION) and c:IsAttribute(ATTRIBUTE_LIGHT)
    card::is_type(d, c, TYPE_FUSION)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        // and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        // and c:CheckFusionMaterial(m,gc,chkf)
        && card::check_fusion_material(d, c, Some(m), Some(&CardSet::from_card(gc)), Some(chkf))
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.fil(c,tp)`
fn fil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCanBeFusionMaterial(nil,MATERIAL_FUSION) and c:IsHasEffect(EFFECT_EXTRA_RELEASE_NONSUM)
    //     and c:IsReleasable()
    card::is_can_be_fusion_material(d, c, None, Some(MATERIAL_FUSION), None)
        && !card::is_has_effect(d, c, EFFECT_EXTRA_RELEASE_NONSUM, None).is_empty()
        && card::is_releasable(d, c, None, None)
}
/// `s.fil2(c)`
fn fil2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCanBeFusionMaterial(nil,MATERIAL_FUSION) and c:IsReleasable()
    card::is_can_be_fusion_material(d, c, None, Some(MATERIAL_FUSION), None) && card::is_releasable(d, c, None, None)
}
/// `s.fcheck(mg)`: `return function(tp,sg,fc) return #(sg&mg)<2 end`
fn fcheck(mg: CardSet) -> LuaFn {
    lua_fn(move |_d, a: Args| {
        let sg = a.g(1);
        group::intersection(&sg, &mg).len() < 2
    })
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local chkf=tp|FUSPROC_NOTFUSION
    let chkf = (tp as u64) | (FUSPROC_NOTFUSION as u64);
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 or not c:IsReleasable() then return false end
        if effect::get_label(d, e) != 1 || !card::is_releasable(d, c, None, None) {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // local mg=Duel.GetMatchingGroup(s.fil2,tp,LOCATION_MZONE,0,nil)
        let mg = duel::get_matching_group(d, Some(&fil2), tp, LOCATION_MZONE, 0, NO_EX);
        // local mg2=Duel.GetMatchingGroup(s.fil,tp,0,LOCATION_MZONE,nil,tp)
        let mg2 = duel::get_matching_group(d, Some(&fil), tp, 0, LOCATION_MZONE, NO_EX);
        // Fusion.CheckAdditional=s.fcheck(mg2)
        fusion::set_check_additional(d, Some(fcheck(mg2.clone())));
        // local res=Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil,e,tp,mg+mg2,c,chkf)
        let mut mg_all = mg.clone();
        group::add_card(&mut mg_all, &mg2);
        let res = duel::is_existing_matching_card(d, Some(&|d, cc| filter(d, cc, e, tp, &mg_all, c, chkf)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // Fusion.CheckAdditional=nil
        fusion::set_check_additional(d, None);
        // return res
        return res;
    }
    // local mg=Duel.GetMatchingGroup(s.fil2,tp,LOCATION_MZONE,0,nil)
    let mg = duel::get_matching_group(d, Some(&fil2), tp, LOCATION_MZONE, 0, NO_EX);
    // local mg2=Duel.GetMatchingGroup(s.fil,tp,0,LOCATION_MZONE,nil,tp)
    let mg2 = duel::get_matching_group(d, Some(&fil), tp, 0, LOCATION_MZONE, NO_EX);
    // Fusion.CheckAdditional=s.fcheck(mg2)
    fusion::set_check_additional(d, Some(fcheck(mg2.clone())));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let mut mg_all = mg.clone();
    group::add_card(&mut mg_all, &mg2);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,mg+mg2,c,chkf)
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| filter(d, cc, e, tp, &mg_all, c, chkf)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local mat=Duel.SelectFusionMaterial(tp,g:GetFirst(),mg+mg2,c,chkf)
    let g_first = group::get_first(d, &g).nil();
    let mat = duel::select_fusion_material(d, tp, g_first, &mg_all, Some(CardSet::from_card(c)), Some(chkf));
    // Fusion.CheckAdditional=nil
    fusion::set_check_additional(d, None);
    // if #(mat&mg2)>0 then
    let mat_mg2 = group::intersection(&mat, &mg2);
    if !mat_mg2.is_empty() {
        // local eff=(mat&mg2):GetFirst():IsHasEffect(EFFECT_EXTRA_RELEASE_NONSUM)
        let first = group::get_first(d, &mat_mg2).nil();
        let eff = card::is_has_effect(d, first, EFFECT_EXTRA_RELEASE_NONSUM, None).first().copied();
        // if eff then
        if let Some(eff) = eff {
            // eff:UseCountLimit(tp,1)
            effect::use_count_limit(d, eff, tp, 1, false);
            // Duel.Hint(HINT_CARD,0,eff:GetHandler():GetCode())
            let h = effect::get_handler(d, eff).nil();
            let code = card::get_code(d, h)[0];
            duel::hint(d, HINT_CARD as u8, 0, code as u64);
        }
    }
    // Duel.Release(mat,REASON_COST)
    duel::release(d, mat, REASON_COST, None);
    // e:SetLabel(g:GetFirst():GetCode())
    let g_first = group::get_first(d, &g).nil();
    let code = card::get_code(d, g_first)[0];
    effect::set_label(d, e, &[code as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.filter2(c,e,tp,code,rp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32, rp: u8) -> bool {
    // return c:IsCode(code) and Duel.GetLocationCountFromEx(tp,rp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[code])
        && duel::get_location_count_from_ex(d, tp, Some(rp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let rp = a.rp;
    // local code=e:GetLabel()
    let code = effect::get_label(d, a.e) as u32;
    // local tc=Duel.GetFirstMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,nil,e,tp,code,rp)
    let tc = duel::get_first_matching_card(d, Some(&|d, c| filter2(d, c, a.e, tp, code, rp)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
