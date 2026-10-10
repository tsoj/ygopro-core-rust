//! Metalfoes Fusion
use super::prelude::*;

pub const ID: u32 = 73594093;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_METALFOES],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // c:RegisterEffect(Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_METALFOES)))
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_METALFOES as u16]))),
            ..Default::default()
        },
    );
    card::register_effect(d, c, e1, false);
    // draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return e:GetHandler():IsAbleToDeck() and Duel.IsPlayerCanDraw(tp,1)
        return card::is_able_to_deck(d, h) && duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoDeck(c,nil,SEQ_DECKTOP,REASON_EFFECT)~=0 and c:IsLocation(LOCATION_DECK) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_deck(d, c, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) != 0
        && card::is_location(d, c, LOCATION_DECK)
    {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
}
