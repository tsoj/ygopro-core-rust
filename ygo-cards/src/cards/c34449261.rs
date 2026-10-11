//! Fusion Fright Waltz
use super::prelude::*;

pub const ID: u32 = 34449261;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FRIGHTFUR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter1(c,tp)
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_FUSION) and c:IsSetCard(SET_FRIGHTFUR)
    //     and Duel.IsExistingTarget(s.filter2,tp,0,LOCATION_MZONE,1,nil,tp,c)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_set_card(d, c, &[SET_FRIGHTFUR as u16])
        && duel::is_existing_target(d, Some(&|d, x| filter2(d, x, tp, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
// function s.filter2(c,tp,tc)
fn filter2(d: &mut Duel, c: CardId, tp: u8, tc: CardId) -> bool {
    // local tg=Group.FromCards(c,tc)
    let mut tg = CardSet::from_card(c);
    group::add_card(&mut tg, &CardSet::from_card(tc));
    // return c:IsFaceup() and c:IsType(TYPE_FUSION)
    //     and Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,tg)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_FUSION)
        && duel::is_existing_matching_card(d, Some(&|d, x| desfilter(d, x, &tg)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
// function s.desfilter(c,tg)
fn desfilter(d: &mut Duel, c: CardId, tg: &CardSet) -> bool {
    // return not tg:IsContains(c) and c:IsSpecialSummoned()
    !tg.contains(c) && card::is_special_summoned(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let mut g1 = duel::select_target(d, tp, Some(&|d, c| filter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,0,LOCATION_MZONE,1,1,nil,tp,g1:GetFirst())
    let tc1 = group::get_first(d, &g1).nil();
    let g2 = duel::select_target(d, tp, Some(&|d, c| filter2(d, c, tp, tc1)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,g1)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, &g1)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    // local dam=g1:GetSum(Card.GetAttack)
    let dam = group::get_sum(d, &g1, &|d, c| card::get_attack(d, c) as i64);
    // if g:FilterCount(Card.IsControler,nil,1-tp)==0 then
    if group::filter_count(d, &g, &|d, c| card::is_controler(d, c, 1 - tp), NO_EX) == 0 {
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,dam)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, tp, dam as i32);
    // elseif g:FilterCount(Card.IsControler,nil,tp)==0 then
    } else if group::filter_count(d, &g, &|d, c| card::is_controler(d, c, tp), NO_EX) == 0 {
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, dam as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,dam)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, dam as i32);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // local dam=tg:Filter(Card.IsFaceup,nil):GetSum(Card.GetAttack)
    let faceup = group::filter(d, &tg, &|d, c| card::is_faceup(d, c), NO_EX);
    let dam = group::get_sum(d, &faceup, &|d, c| card::get_attack(d, c) as i64);
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tg)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, &tg)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)~=0 and dam>0 then
    if !g.is_empty() && duel::destroy(d, &g, REASON_EFFECT, None, None) != 0 && dam > 0 {
        // local dg=Duel.GetOperatedGroup()
        let dg = duel::get_operated_group(d);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if dg:IsExists(aux.FilterEqualFunction(Card.GetPreviousControler,tp),1,nil) then Duel.Damage(tp,dam,REASON_EFFECT,true) end
        if group::is_exists(d, &dg, &|d, c| card::get_previous_controler(d, c) == tp, 1, NO_EX) {
            duel::damage(d, tp, dam, REASON_EFFECT, true, None);
        }
        // if dg:IsExists(aux.FilterEqualFunction(Card.GetPreviousControler,1-tp),1,nil) then Duel.Damage(1-tp,dam,REASON_EFFECT,true) end
        if group::is_exists(d, &dg, &|d, c| card::get_previous_controler(d, c) == 1 - tp, 1, NO_EX) {
            duel::damage(d, 1 - tp, dam, REASON_EFFECT, true, None);
        }
        // Duel.RDComplete()
        duel::rd_complete(d);
    }
}
