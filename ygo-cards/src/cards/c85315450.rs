//! Radiant Typhoon Fonix, the Great Flame
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 85315450;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MYSTICAL_SPACE_TYPHOON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand if a Quick-Play Spell Card is activated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and re:IsActiveType(TYPE_QUICKPLAY) end)
    effect::set_condition(d, e1, Some(condition(act_qp_cond)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle up to 2 cards your opponent controls into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and re:IsActiveType(TYPE_QUICKPLAY) end)
    effect::set_condition(d, e2, Some(condition(act_qp_cond)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon this card from your GY if "Mystical Space Typhoon" is activated
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsCode(CARD_MYSTICAL_SPACE_TYPHOON) end)
    effect::set_condition(d, e3, Some(condition(mst_act_cond)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and re:IsActiveType(TYPE_QUICKPLAY) end`
fn act_qp_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && effect::is_spell_effect(d, re)
        && effect::is_active_type(d, re, TYPE_QUICKPLAY)
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsCode(CARD_MYSTICAL_SPACE_TYPHOON) end`
fn mst_act_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && {
        let rc = effect::get_handler(d, re).nil();
        card::is_code(d, rc, &[CARD_MYSTICAL_SPACE_TYPHOON])
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        // return chkc:IsOnField() and chkc:IsControler(1-tp) and chkc:IsAbleToDeck()
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp) && card::is_able_to_deck(d, chkc);
    }
    if a.chk == Some(0) {
        // return Duel.IsExistingTarget(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,1,nil)
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,1,2,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, tp, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, &tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
