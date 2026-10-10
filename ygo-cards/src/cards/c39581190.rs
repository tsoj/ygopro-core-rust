//! Subterror Nemesis Archer
use super::prelude::*;

pub const ID: u32 = 39581190;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUBTERROR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Shuffle an opponent's face-down Defense Position monster into the Deck
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.tdcon)
    effect::set_condition(d, e1, Some(condition(tdcon)));
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Subterror" monster from your Deck
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SUBTERROR)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SUBTERROR as u16])
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local d=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // return c==Duel.GetAttacker() and d and d:IsPosition(POS_FACEDOWN_DEFENSE)
    //   and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,c)
    if duel::get_attacker(d) != Some(c) {
        return false;
    }
    let Some(t) = t else { return false };
    card::is_position(d, t, POS_FACEDOWN_DEFENSE)
        && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d).nil();
    // if chk==0 then return d:IsAbleToDeck() end
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, t);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,d,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(t)), 1, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, _a: &EffArgs) {
    // local d=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d).nil();
    // if d:IsRelateToBattle() and d:IsPosition(POS_FACEDOWN_DEFENSE) then
    if card::is_relate_to_battle(d, t) && card::is_position(d, t, POS_FACEDOWN_DEFENSE) {
        // Duel.SendtoDeck(d,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, CardSet::from_card(t), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsReason(REASON_DESTROY) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_reason(d, c, REASON_DESTROY) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SUBTERROR) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_DEFENSE)
    card::is_set_card(d, c, &[SET_SUBTERROR as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else { return };
    // if Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_DEFENSE)~=0 then
    if duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_DEFENSE as u8, None) != 0 {
        // if tc:IsFacedown() then
        if card::is_facedown(d, tc) {
            // Duel.ConfirmCards(1-tp,tc)
            duel::confirm_cards(d, 1 - tp, CardSet::from_card(tc));
        }
    }
}
