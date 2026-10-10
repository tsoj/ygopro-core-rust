//! Draco-Utopian Aura
//! 龍皇の波動
use super::prelude::*;

pub const ID: u32 = 9070454;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let re = a.re.nil();
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // return Duel.IsChainNegatable(ev) and re:IsMonsterEffect() and (loc&LOCATION_MZONE)~=0 and ep==1-tp
    duel::is_chain_negatable(d, a.ev as u8)
        && effect::is_monster_effect(d, re)
        && (loc & LOCATION_MZONE as i64) != 0
        && a.ep == 1 - tp
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove()
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re);
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc.nil(), None) && card::is_relate_to_effect(d, rc.nil(), re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,rc,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, rc.map(CardSet::from_card), 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let re = a.re.nil();
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)>0 then
    if duel::negate_activation(d, a.ev as u8) && {
        let rh = effect::get_handler(d, re).nil();
        card::is_relate_to_effect(d, rh, re)
    } && duel::destroy(d, a.eg(), REASON_EFFECT, None, None) > 0
    {
        // local oc=Duel.GetOperatedGroup():GetFirst()
        let og = duel::get_operated_group(d);
        let oc = group::get_first(d, &og);
        // local cg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND,0,nil)
        let cg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND, 0, NO_EX);
        // if oc:IsLocation(LOCATION_GRAVE) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and #cg>0 and oc:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if card::is_location(d, oc.nil(), LOCATION_GRAVE)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && !cg.is_empty()
            && card::is_can_be_special_summoned(d, oc.nil(), e, 0, tp, false, false, None, None, None)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
            // local rg=cg:Select(tp,1,1,nil)
            let rg = group::select(d, &cg, tp, 1, 1, false, NO_EX).nil();
            // if #rg==0 then return end
            if rg.is_empty() {
                return;
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)>0 and Duel.SpecialSummonStep(oc,0,tp,tp,false,false,POS_FACEUP) then
            if duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
                && duel::special_summon_step(d, oc.nil(), 0, tp, tp, false, false, POS_FACEUP as u8, None)
            {
                // local c=e:GetHandler()
                let c = effect::get_handler(d, e).nil();
                // --Negate its effects
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_DISABLE)
                effect::set_code(d, e1, EFFECT_DISABLE);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // oc:RegisterEffect(e1)
                card::register_effect(d, oc.nil(), e1, false);
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, c);
                // e2:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                // e2:SetCode(EFFECT_DISABLE_EFFECT)
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                // e2:SetValue(RESET_TURN_SET)
                effect::set_value(d, e2, RESET_TURN_SET as i64);
                // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                // oc:RegisterEffect(e2)
                card::register_effect(d, oc.nil(), e2, false);
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
