//! Clown Crew Flair
//! scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 42759961;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // You can reveal this card in your hand; add 1 non-Ritual "Clown Crew" card from your Deck to your hand, then discard 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Tributed: You can activate 1 of these effects (but you can only use each of these effects of "Clown Crew Flare" once per turn)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.listed_series={SET_CLOWN_CREW}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CLOWN_CREW], ..ScriptInfo::DEFAULT };

/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CLOWN_CREW) and not c:IsRitualMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_CLOWN_CREW as u16]) && !card::is_ritual_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 and g:GetFirst():IsLocation(LOCATION_HAND) then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        let first = group::get_first(d, &g).nil();
        if card::is_location(d, first, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD,nil)
            duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        }
    }
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsRitualMonster,Card.IsFaceup,Card.IsAbleToDeck),tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil)
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_ritual_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            LOCATION_MZONE | LOCATION_GRAVE,
            1,
            NO_EX,
        );
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsEffectMonster,Card.IsCanTurnSet),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let b2 = !duel::has_flag_effect(d, a.tp, ID + 1, None)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_effect_monster(d, c) && card::is_can_turn_set(d, c)),
            a.tp,
            LOCATION_MZONE,
            LOCATION_MZONE,
            1,
            NO_EX,
        );
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |x| x as i64)]);
    // if op==1 then
    if op == Some(1) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_TODECK)
        effect::set_category(d, a.e, CATEGORY_TODECK);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_ALL,LOCATION_MZONE|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_ALL as u8, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    // elseif op==2 then
    } else if op == Some(2) {
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_POSITION + CATEGORY_SET);
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,PLAYER_EITHER,POS_FACEDOWN_DEFENSE)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, PLAYER_EITHER as u8, POS_FACEDOWN_DEFENSE as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // --● Shuffle all Ritual Monsters on the field and in the GYs into the Deck
        // local g=Duel.GetMatchingGroup(aux.AND(Card.IsRitualMonster,Card.IsFaceup,Card.IsAbleToDeck),tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,nil)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_ritual_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            LOCATION_MZONE | LOCATION_GRAVE,
            NO_EX,
        );
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    // elseif op==2 then
    } else if op == 2 {
        // --● Change 1 Effect Monster on the field to face-down Defense Position
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.AND(Card.IsEffectMonster,Card.IsCanTurnSet),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(
            d,
            a.tp,
            Some(&|d, c| card::is_effect_monster(d, c) && card::is_can_turn_set(d, c)),
            a.tp,
            LOCATION_MZONE,
            LOCATION_MZONE,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
