//! Magistery Alchemist
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 58270977;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_ATKCHANGE+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c,e,tp)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_HERO) and c:IsFaceup() and c:IsAbleToRemoveAsCost()
    //     and aux.SpElimFilter(c,true,true)
    card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_HERO as u16])
        && card::is_faceup(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, true)
}
/// `s.spcheck(sg,e,tp)`
fn spcheck(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0 and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,sg,e,tp)
    let e = e.nil();
    let res = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, sg.into());
    (res, false)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_HERO) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_HERO as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(-1)
    effect::set_label(d, a.e, &[-1]);
    // local rg=Duel.GetMatchingGroup(s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,e,tp)
    let rg = duel::get_matching_group(d, Some(&costfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #rg>=4 and aux.SelectUnselectGroup(rg,e,tp,4,4,s.spcheck,0) end
    if a.chk == Some(0) {
        return rg.len() >= 4
            && matches!(
                aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(4), Some(4), Some(&spcheck), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,4,4,s.spcheck,1,tp,HINTMSG_REMOVE)
    let g = aux::select_unselect_group(d, &rg, Some(a.e), tp, Some(4), Some(4), Some(&spcheck), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // if g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_EARTH)
    //     and g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_WATER)
    //     and g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_FIRE)
    //     and g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_WIND) then
    if group::is_exists(d, &g, &|d, c| card::is_attribute(d, c, ATTRIBUTE_EARTH), 1, NO_EX)
        && group::is_exists(d, &g, &|d, c| card::is_attribute(d, c, ATTRIBUTE_WATER), 1, NO_EX)
        && group::is_exists(d, &g, &|d, c| card::is_attribute(d, c, ATTRIBUTE_FIRE), 1, NO_EX)
        && group::is_exists(d, &g, &|d, c| card::is_attribute(d, c, ATTRIBUTE_WIND), 1, NO_EX)
    {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    }
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=(e:GetLabel()==-1 or Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
        //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
        let label = effect::get_label(d, a.e);
        let res = (label == -1 || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0)
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return res
        return res;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // if Duel.SpecialSummonStep(tc,0,tp,tp,true,false,POS_FACEUP) and e:GetLabel()==1 then
    if duel::special_summon_step(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None) && effect::get_label(d, a.e) == 1 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // --Double its original ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_BASE_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
        // e1:SetValue(tc:GetBaseAttack()*2)
        let base = card::get_base_attack(d, tc) as i64 * 2;
        effect::set_value(d, e1, base);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local ng=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_ONFIELD,nil)
        let ng = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
        // for tcn in ng:Iter() do
        for tcn in group::iter(d, &ng) {
            // --Negate its effects
            // tcn:NegateEffects(c,nil,true)
            card::negate_effects(d, tcn, c, None, true, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
