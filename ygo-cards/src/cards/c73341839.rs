//! Miniborrel Dragon
use super::prelude::*;

pub const ID: u32 = 73341839;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BORREL, SET_ROKKET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ROKKET),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, _sc, _st, _tp| card::is_set_card(d, c, &[SET_ROKKET as u16]))), 2, Some(2), None, None, None);
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_BORREL) and c:IsLinkMonster()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BORREL as u16]) && card::is_link_monster(d, c)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.rfilter(c)`
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLinkBelow(3)
    card::is_link_below(d, c, 3)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK)
    //     and Duel.CheckReleaseGroupCost(tp,s.rfilter,1,false,aux.ReleaseCheckMMZ,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return card::is_can_be_special_summoned(d, h, e, 0, tp, false, false, Some(POS_FACEUP_ATTACK as u8), None, None)
            && duel::check_release_group_cost(d, tp, Some(&rfilter), 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,s.rfilter,1,1,false,aux.ReleaseCheckMMZ,nil)
    let rg = duel::select_release_group_cost(d, tp, Some(&rfilter), 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
    // local r=rg:GetFirst()
    let r = group::get_first(d, &rg).nil();
    // local lk=r:GetLink()
    let lk = card::get_link(d, r);
    // e:SetLabel(lk)
    effect::set_label(d, e, &[lk as i64]);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP_ATTACK) then
    if card::is_relate_to_effect(d, c, e) && duel::special_summon_step(d, c, 0, tp, tp, false, false, POS_FACEUP_ATTACK as u8, None) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
        effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
        // e1:SetValue(s.lnklimit)
        effect::set_value_fn(d, e1, lua_fn(lnklimit));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetLabel(e:GetLabel())
        let lbl = effect::get_labels(d, e);
        effect::set_label(d, e1, &lbl);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.lnklimit(e,c)`
fn lnklimit(d: &mut Duel, a: Args) -> bool {
    // if not c then return false end
    let Some(c) = a.card(1) else {
        return false;
    };
    // return c:IsLink(e:GetLabel())
    let lbl = effect::get_label(d, a.e(0));
    card::is_link(d, c, &[lbl as u32])
}
