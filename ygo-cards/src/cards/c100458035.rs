//! Sky Thunder Xenovader△ Yomi
use super::prelude::*;

pub const ID: u32 = 100458035;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_XENOVADER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can reveal this card in your hand; banish it (face-down), and if you do, add 1 of your face-down banished "Xenovader△" cards to your hand, except a Level 5 monster. You can only use this effect of "Sky Thunder Xenovader△ Yomi" once per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    // e1:SetTarget(s.selfbantg)
    effect::set_target(d, e1, Some(cost_target(selfbantg)));
    // e1:SetOperation(s.selfbanop)
    effect::set_operation(d, e1, Some(operation(selfbanop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Normal or Special Summoned: Banish (face-down) the top 5 cards of your Deck
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2a, CATEGORY_REMOVE);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e2a, Some(cost_target(e2a_tg)));
    // e2a:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2a, Some(operation(e2a_op)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Once per turn, when your opponent activates a monster effect (Quick Effect): You can shuffle 1 of your face-down banished "Xenovader△" monsters with the same Attribute into the Deck, and if you do, negate the activation
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TODECK+CATEGORY_NEGATE)
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_NEGATE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetTarget(s.negtg)
    effect::set_target(d, e3, Some(cost_target(negtg)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`: `c:IsFacedown() and c:IsSetCard(SET_XENOVADER) and not c:IsLevel(5) and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::is_set_card(d, c, &[SET_XENOVADER as u16]) && !card::is_level(d, c, &[5]) && card::is_able_to_hand(d, c, None)
}
/// `s.selfbantg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfbantg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove(tp,POS_FACEDOWN) and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, Some(a.tp), Some(POS_FACEDOWN as u8), None)
            && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,tp,POS_FACEDOWN)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, a.tp, POS_FACEDOWN as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_REMOVED as i32);
    true
}
/// `s.selfbanop(e,tp,eg,ep,ev,re,r,rp)`
fn selfbanop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.Remove(c,POS_FACEDOWN,REASON_EFFECT)>0 and c:IsLocation(LOCATION_REMOVED) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::remove(d, CardSet::from_card(c), POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, c, LOCATION_REMOVED)
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_REMOVED,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
/// The target of e2a: banish the top 5 cards of the Deck
fn e2a_tg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 5, a.tp, LOCATION_DECK as i32);
    true
}
/// The operation of e2a: banish the top 5 cards of the Deck face-down
fn e2a_op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetDecktopGroup(tp,5)
    let g = duel::get_decktop_group(d, a.tp, 5);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    }
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    a.ep == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.tdfilter(c,attr)`
fn tdfilter(d: &mut Duel, c: CardId, attr: u32) -> bool {
    // return c:IsFacedown() and c:IsSetCard(SET_XENOVADER) and c:IsAttribute(attr) and c:IsAbleToDeck()
    card::is_facedown(d, c) && card::is_set_card(d, c, &[SET_XENOVADER as u16]) && card::is_attribute(d, c, attr) && card::is_able_to_deck(d, c)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_REMOVED,0,1,nil,re:GetCardAttribute()) end
    if a.chk == Some(0) {
        let re = a.re.nil();
        let attr = chain::effect_get_card(d, re, chain::Prop::Attribute).int() as u32;
        return duel::is_existing_matching_card(d, Some(&|d, c| tdfilter(d, c, attr)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_REMOVED as i32);
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, tp, 0);
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, tp, 0);
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let re = a.re.nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_REMOVED,0,1,1,nil,re:GetCardAttribute()):GetFirst()
    let attr = chain::effect_get_card(d, re, chain::Prop::Attribute).int() as u32;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| tdfilter(d, c, attr)), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else {
        return;
    };
    // Duel.ConfirmCards(1-tp,sc)
    duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
    // if Duel.SendtoDeck(sc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_DECK|LOCATION_EXTRA) then
    if duel::send_to_deck(d, CardSet::from_card(sc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, sc, LOCATION_DECK | LOCATION_EXTRA)
    {
        // Duel.NegateActivation(ev)
        duel::negate_activation(d, a.ev as u8);
    }
}
