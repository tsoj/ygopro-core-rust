//! Salamangreat Charge
use super::prelude::*;

pub const ID: u32 = 83533296;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Shuffle 2 FIRE monsters into the Deck and Special Summon another one OR destroy 1 card on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsCanBeEffectTarget(e)
    //     and (c:IsAbleToDeck() or c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    card::is_faceup(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_can_be_effect_target(d, c, Some(e))
        && (card::is_able_to_deck(d, c) || card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsType(TYPE_RITUAL|TYPE_EXTRA) and c:GetAttack()~=c:GetBaseAttack()
    card::is_faceup(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_type(d, c, TYPE_RITUAL | TYPE_EXTRA)
        && card::get_attack(d, c) != card::get_base_attack(d, c)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8) -> bool {
    // return sg:IsExists(s.spcheck,1,nil,sg,e,tp)
    let e = e.nil();
    group::is_exists(d, sg, &|d, c| spcheck(d, c, sg, e, tp), 1, NO_EX)
}
/// `s.spcheck(c,sg,e,tp)`
fn spcheck(d: &mut Duel, c: CardId, sg: &CardSet, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and sg:FilterCount(Card.IsAbleToDeck,c)==2
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && group::filter_count(d, sg, &|d, x| card::is_able_to_deck(d, x), c.into()) == 2
}
/// `s.selectcond(resg)`: the `sg,e,tp,mg` function returned for `resg`
fn selectcond(d: &mut Duel, sg: &CardSet, resg: &CardSet, e: Option<EffectId>, tp: u8) -> bool {
    // return sg:IsExists(Card.IsAbleToDeck,2,nil) and resg:IsExists(Card.IsCanBeSpecialSummoned,1,sg,e,0,tp,false,false)
    let e = e.nil();
    group::is_exists(d, sg, &|d, c| card::is_able_to_deck(d, c), 2, NO_EX)
        && group::is_exists(d, resg, &|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None), 1, sg.into())
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| filter1(d, x, e, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // local b1=#g>=3 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,0)
    let rs0: aux::Rescon = &|d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>| (rescon(d, sg, e, tp), false);
    let b1 = g.len() >= 3
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(rs0), 0, tp, None, None, None, false), Err(true));
    // local b2=Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)
    let b2 = duel::is_existing_matching_card(d, Some(&filter2), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    // if chkc then return e:GetLabel()==2 and chkc:IsOnField() and chck~=c end
    // (`chck` is never assigned in the script, so it is nil and `chck~=c` is true)
    if let Some(chkc) = a.chkc() {
        return effect::get_label(d, e) == 2 && card::is_on_field(d, chkc);
    }
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    if op == 1 {
        // e:SetCategory(CATEGORY_TODECK|CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_TODECK | CATEGORY_SPECIAL_SUMMON);
        // local rg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_TARGET)
        let rs: aux::Rescon = &|d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>| (rescon(d, sg, e, tp), false);
        let rg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
        // Duel.SetTargetCard(rg)
        duel::set_target_card(d, rg.clone());
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,rg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(rg.clone()), 1, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,rg,2,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(rg), 2, tp, 0);
    } else {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e, CATEGORY_DESTROY);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local tc=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
        let tc = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc, 1, tp, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=e:GetLabel()
    let label = effect::get_label(d, e);
    if label == 1 {
        // Shuffle 2 FIRE monsters and Special Summon another
        // local g=Duel.GetTargetCards(e)
        let mut g = duel::get_target_cards(d, e).nil();
        // if #g<3 then return end
        if g.len() < 3 {
            return;
        }
        // local tdg=aux.SelectUnselectGroup(g,e,tp,2,2,s.selectcond(g),1,tp,HINTMSG_TODECK)
        let resg = g.clone();
        let rs: aux::Rescon = &|d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>| (selectcond(d, sg, &resg, e, tp), false);
        let tdg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_TODECK as u64), None, None, false).unwrap_or_default();
        // g:RemoveCard(tdg)
        group::remove_card(&mut g, &tdg);
        // if Duel.SendtoDeck(tdg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
        if duel::send_to_deck(d, tdg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
            return;
        }
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g).nil();
        // if tc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
        if card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None)
        {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, e).nil();
            // Negate its effects
            // tc:NegateEffects(c,RESET_PHASE|PHASE_END)
            card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), false, None);
            // Cannot attack
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3206)
            effect::set_description(d, e1, 3206);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    } else {
        // Destroy 1 card on the field
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
