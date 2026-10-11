//! Bahalutiya, the Grand Radiance
use super::prelude::*;

pub const ID: u32 = 53318263;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 1 tribute
    // local e1=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e1 = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(std::sync::Arc::new(otfilter)), None);
    // local e2=aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e2 = aux::add_normal_set_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(std::sync::Arc::new(otfilter)), None);
    // tribute summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e3, CATEGORY_SUMMON);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e3, EVENT_TO_HAND);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCondition(s.tscon)
    effect::set_condition(d, e3, Some(condition(tscon)));
    // e3:SetTarget(s.tstg)
    effect::set_target(d, e3, Some(cost_target(tstg)));
    // e3:SetOperation(s.tsop)
    effect::set_operation(d, e3, Some(operation(tsop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_REMOVE+CATEGORY_TODECK+CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_REMOVE + CATEGORY_TODECK + CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    // e4:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.thcon)
    effect::set_condition(d, e4, Some(condition(thcon)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.otfilter(c,tp)`
fn otfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_EFFECT) and (c:IsControler(tp) or c:IsFaceup())
    card::is_type(d, c, TYPE_EFFECT) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsPreviousLocation(LOCATION_DECK) and not c:IsReason(REASON_DRAW)
    card::is_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_DECK) && !card::is_reason(d, c, REASON_DRAW)
}
/// `s.tscon(e,tp,eg,ep,ev,re,r,rp)`
fn tscon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ph=Duel.GetCurrentPhase()
    let _ph = duel::get_current_phase(d);
    // return Duel.IsTurnPlayer(1-tp) and (Duel.IsMainPhase())
    //     and eg:IsExists(s.cfilter,1,nil,1-tp)
    if !duel::is_turn_player(d, 1 - tp) || !duel::is_main_phase(d, None) {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, 1 - tp), 1, NO_EX)
}
/// `s.tstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tstg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSummonable(true,nil,1) or c:IsMSetable(true,nil,1) end
    if a.chk == Some(0) {
        return card::is_summonable(d, c, true, None, 1, None) || card::is_m_setable(d, c, true, None, 1, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.tsop(e,tp,eg,ep,ev,re,r,rp)`
fn tsop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local s1=c:IsSummonable(true,nil,1)
        let s1 = card::is_summonable(d, c, true, None, 1, None);
        // local s2=c:IsMSetable(true,nil,1)
        let s2 = card::is_m_setable(d, c, true, None, 1, None);
        // if (s1 and s2 and Duel.SelectPosition(tp,c,POS_FACEUP_ATTACK+POS_FACEDOWN_DEFENSE)==POS_FACEUP_ATTACK) or not s2 then
        if (s1 && s2 && duel::select_position(d, tp, c, (POS_FACEUP_ATTACK + POS_FACEDOWN_DEFENSE) as u8) == POS_FACEUP_ATTACK as u32) || !s2 {
            // Duel.Summon(tp,c,true,nil,1)
            duel::summon(d, tp, c, true, None, 1, None);
        } else {
            // Duel.MSet(tp,c,true,nil,1)
            duel::m_set(d, tp, c, true, None, 1, None);
        }
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_HAND)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_HAND)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local hg=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let hg = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // local ct=#hg
    let ct = hg.len();
    // local dg=Duel.GetDecktopGroup(1-tp,ct)
    let dg = duel::get_decktop_group(d, 1 - tp, ct as u32);
    // if chk==0 then return ct>0 and dg:FilterCount(Card.IsAbleToRemove,nil,tp,POS_FACEDOWN)==ct
    //     and hg:FilterCount(Card.IsAbleToDeck,nil)==ct end
    if a.chk == Some(0) {
        return ct > 0
            && group::filter_count(d, &dg, &|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None), NO_EX) == ct
            && group::filter_count(d, &hg, &|d, c| card::is_able_to_deck(d, c), NO_EX) == ct;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,dg,ct,1-tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(dg), ct as u8, 1 - tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,hg,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(hg), ct as u8, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local hg=Duel.GetFieldGroup(p,LOCATION_HAND,0)
    let hg = duel::get_field_group(d, p, LOCATION_HAND, 0);
    // local ct=#hg
    let ct = hg.len();
    // local dg=Duel.GetDecktopGroup(p,ct)
    let dg = duel::get_decktop_group(d, p, ct as u32);
    // if ct>0 and dg:FilterCount(Card.IsAbleToRemove,nil,tp,POS_FACEDOWN)==ct and Duel.Remove(dg,POS_FACEDOWN,REASON_EFFECT)==ct then
    if ct > 0
        && group::filter_count(d, &dg, &|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None), NO_EX) == ct
        && duel::remove(d, dg, POS_FACEDOWN as u8, REASON_EFFECT, None, None) == ct as i32
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // if Duel.SendtoDeck(hg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 then
        if duel::send_to_deck(d, hg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
            // Duel.SendtoHand(og,p,REASON_EFFECT)
            duel::send_to_hand(d, og, Some(p), REASON_EFFECT, None);
        }
    }
}
