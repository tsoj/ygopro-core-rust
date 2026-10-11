//! Xyz Revive Splash
use super::prelude::*;

pub const ID: u32 = 32764863;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // rank up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.rutg)
    effect::set_target(d, e2, Some(cost_target(rutg)));
    // e2:SetOperation(s.ruop)
    effect::set_operation(d, e2, Some(operation(ruop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsRankBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_XYZ) && card::is_rank_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, tc, e) {
        // SpecialSummonStep returns a boolean, and `~= 0` is true for either value: the branch always runs
        let _ = duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(ATTRIBUTE_WATER)
        effect::set_value(d, e1, ATTRIBUTE_WATER as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WATER)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,pg)
    (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
        && card::is_faceup(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_WATER)
        && duel::is_existing_matching_card(d, Some(&|d, mc| filter2(d, mc, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.filter2(c,e,tp,mc,pg)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return mc:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:IsType(TYPE_XYZ) and c:IsAttribute(ATTRIBUTE_WATER)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsRank(mc:GetRank()+1)
    //     and mc:IsCanBeXyzMaterial(c,tp) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    let rk = card::get_rank(d, mc) + 1;
    card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_attribute(d, c, ATTRIBUTE_WATER)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_rank(d, c, &[rk])
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
}
/// `s.rutg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rutg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter1(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter1(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let _ = duel::select_target(d, tp, Some(&|d, c| filter1(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.ruop(e,tp,eg,ep,ev,re,r,rp)`
fn ruop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&tc.map_or(CardSet::new(), CardSet::from_card)), tp, None, None, REASON_XYZ);
    // if not tc or tc:IsFacedown() or not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e)
    //     or #pg>1 or (#pg==1 and not pg:IsContains(tc)) then return end
    let Some(tc) = tc else { return };
    if card::is_facedown(d, tc)
        || !card::is_relate_to_effect(d, tc, e)
        || card::is_controler(d, tc, 1 - tp)
        || card::is_immune_to_effect(d, tc, e)
        || pg.len() > 1
        || (pg.len() == 1 && !group::is_contains(&pg, tc))
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,pg)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, e, tp, tc)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g:GetFirst()
    // if sc then
    if let Some(sc) = group::get_first(d, &g) {
        // sc:SetMaterial(tc)
        card::set_material(d, sc, CardSet::from_card(tc));
        // Duel.Overlay(sc,tc)
        duel::overlay(d, sc, tc, false);
        // Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
