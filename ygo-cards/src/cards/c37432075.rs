//! Invocation - "Sword"
//! 召喚魔術－「剣」
//! scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 37432075;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_INVOKED, SET_ALEISTER],
    listed_names: &[CARD_INVOCATION],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 Fusion Monster from your Extra Deck, using monsters you control. If Summoning an "Invoked" Fusion Monster this way, you can also return banished monsters to the GY as material
    // local e1=Fusion.CreateSummonEff({
    //     handler=c,
    //     matfilter=Fusion.OnFieldMat,
    //     extrafil=s.fextra,
    // })
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            matfilter: Some(fusion::on_field_mat()),
            extrafil: Some(Arc::new(fextra)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase, if this card is in your GY: You can target 1 "Aleister" monster or "Invocation" in your GY; shuffle this card into the Deck, and if you do, add the targeted card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.checkmat(tp,sg,fc)
fn checkmat(d: &mut Duel, a: Args) -> bool {
    // return fc:IsSetCard(SET_INVOKED) or not sg:IsExists(Card.IsLocation,1,nil,LOCATION_REMOVED)
    let sg = a.g(1);
    let fc = a.c(2);
    card::is_set_card(d, fc, &[SET_INVOKED as u16])
        || !group::is_exists(d, &sg, &|d, c| card::is_location(d, c, LOCATION_REMOVED), 1, NO_EX)
}
// function s.fextra(e,tp,mg)
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsFaceup),tp,LOCATION_REMOVED,LOCATION_REMOVED,nil),s.checkmat
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_monster(d, c) && card::is_faceup(d, c)),
        tp,
        LOCATION_REMOVED,
        LOCATION_REMOVED,
        NO_EX,
    );
    (Some(g), Some(lua_fn(checkmat)))
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsSetCard(SET_ALEISTER) and c:IsMonster()) or c:IsCode(CARD_INVOCATION)) and c:IsAbleToHand()
    ((card::is_set_card(d, c, &[SET_ALEISTER as u16]) && card::is_monster(d, c)) || card::is_code(d, c, &[CARD_INVOCATION]))
        && card::is_able_to_hand(d, c, None)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToDeck() and Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, c) && duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and c:IsLocation(LOCATION_DECK)
    //     and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_DECK)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc.nil(), None, REASON_EFFECT, None);
    }
}
