//! Kashtira Big Bang
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 33925864;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KASHTIRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make both players banish monsters face-down so they control only 1
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_BATTLE_START|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.rmvcond)
    effect::set_condition(d, e1, Some(condition(rmvcond)));
    // e1:SetTarget(s.rmvtg)
    effect::set_target(d, e1, Some(cost_target(rmvtg)));
    // e1:SetOperation(s.rmvop)
    effect::set_operation(d, e1, Some(operation(rmvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 monster attached to the hand and Special Summon it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.xyzfilter(c)`
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsSetCard(SET_KASHTIRA)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_KASHTIRA as u16])
}
/// `s.rmvcond(e,tp,eg,ep,ev,re,r,rp)`
fn rmvcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&xyzfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g1=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g1 = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
    // local g2=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g2 = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    // if chk==0 then return (#g1>1 and g1:IsExists(Card.IsAbleToRemove,1,nil,tp,POS_FACEDOWN,REASON_RULE))
    //     or (#g2>1 and g2:IsExists(Card.IsAbleToRemove,1,nil,1-tp,POS_FACEDOWN,REASON_RULE)) end
    if a.chk == Some(0) {
        return (g1.len() > 1 && group::is_exists(d, &g1, &|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), Some(REASON_RULE)), 1, NO_EX))
            || (g2.len() > 1 && group::is_exists(d, &g2, &|d, c| card::is_able_to_remove(d, c, Some(1 - tp), Some(POS_FACEDOWN as u8), Some(REASON_RULE)), 1, NO_EX));
    }
    true
}
/// `s.rmvop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvop(d: &mut Duel, _a: &EffArgs) {
    // local p1=Duel.GetTurnPlayer() --used to make the turn player banish first
    let p1 = duel::get_turn_player(d);
    // local p2=1-Duel.GetTurnPlayer()
    let p2 = 1 - duel::get_turn_player(d);
    // local g1=Duel.GetFieldGroup(p1,LOCATION_MZONE,0)
    let g1 = duel::get_field_group(d, p1, LOCATION_MZONE, 0);
    // local rg1=g1:Filter(Card.IsAbleToRemove,nil,p1,POS_FACEDOWN,REASON_RULE)
    let rg1 = group::filter(d, &g1, &|d, c| card::is_able_to_remove(d, c, Some(p1), Some(POS_FACEDOWN as u8), Some(REASON_RULE)), NO_EX);
    // if #g1>1 and #rg1>0 then
    if g1.len() > 1 && !rg1.is_empty() {
        // local ct=math.min(#g1-1,#rg1)
        let ct = (g1.len() - 1).min(rg1.len());
        // Duel.Hint(HINT_SELECTMSG,p1,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, p1, HINTMSG_REMOVE as u64);
        // local sg=rg1:Select(p1,ct,ct,nil)
        let sg = group::select(d, &rg1, p1, ct as u16, ct as u16, false, NO_EX).nil();
        // Duel.Remove(sg,POS_FACEDOWN,REASON_RULE,PLAYER_NONE,p1)
        duel::remove(d, sg, POS_FACEDOWN as u8, REASON_RULE, Some(PLAYER_NONE as u8), Some(p1));
    }
    // local g2=Duel.GetFieldGroup(p2,LOCATION_MZONE,0)
    let g2 = duel::get_field_group(d, p2, LOCATION_MZONE, 0);
    // local rg2=g2:Filter(Card.IsAbleToRemove,nil,p2,POS_FACEDOWN,REASON_RULE)
    let rg2 = group::filter(d, &g2, &|d, c| card::is_able_to_remove(d, c, Some(p2), Some(POS_FACEDOWN as u8), Some(REASON_RULE)), NO_EX);
    // if #g2>1 and #rg2>0 then
    if g2.len() > 1 && !rg2.is_empty() {
        // local ct=math.min(#g2-1,#rg2)
        let ct = (g2.len() - 1).min(rg2.len());
        // Duel.Hint(HINT_SELECTMSG,p2,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, p2, HINTMSG_REMOVE as u64);
        // local sg=rg2:Select(p2,ct,ct,nil)
        let sg = group::select(d, &rg2, p2, ct as u16, ct as u16, false, NO_EX).nil();
        // Duel.Remove(sg,POS_FACEDOWN,REASON_RULE,PLAYER_NONE,p2)
        duel::remove(d, sg, POS_FACEDOWN as u8, REASON_RULE, Some(PLAYER_NONE as u8), Some(p2));
    }
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_KASHTIRA) and c:GetOwner()==tp and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_KASHTIRA as u16]) && card::get_owner(d, c) == tp && card::is_able_to_hand(d, c, None)
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // if not (c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsSetCard(SET_KASHTIRA)) then return false end
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_KASHTIRA as u16])) {
        return false;
    }
    // local g=c:GetOverlayGroup()
    let g = card::get_overlay_group(d, c);
    // return g:IsExists(s.thfilter,1,nil,tp)
    group::is_exists(d, &g, &|d, cc| thfilter(d, cc, tp), 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let _tc = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_OVERLAY)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_OVERLAY as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local g=tc:GetOverlayGroup()
    let g = card::get_overlay_group(d, tc);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=g:FilterSelect(tp,s.thfilter,1,1,nil,tp):GetFirst()
    let sel = group::filter_select(d, &g, tp, &|d, c| thfilter(d, c, tp), 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    if let Some(sc) = sc {
        // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND)
        //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) then
        if duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) > 0
            && card::is_location(d, sc, LOCATION_HAND)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, None, None, None)
        {
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // if not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
            if !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
                return;
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(sc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
