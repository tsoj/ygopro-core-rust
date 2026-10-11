//! Fairy Tail - Rochka
//! Scripted by DyXel
use super::prelude::*;

pub const ID: u32 = 65953423;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Opponent looks at top 3 cards of your deck, choose 1 to add to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.exctg)
    effect::set_target(d, e1, Some(cost_target(exctg)));
    // e1:SetOperation(s.excop)
    effect::set_operation(d, e1, Some(operation(excop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When battling, send itself to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_CONFIRM)
    effect::set_code(d, e2, EVENT_BATTLE_CONFIRM);
    // e2:SetCondition(s.sgycon)
    effect::set_condition(d, e2, Some(condition(sgycon)));
    // e2:SetTarget(s.sgytg)
    effect::set_target(d, e2, Some(cost_target(sgytg)));
    // e2:SetOperation(s.sgyop)
    effect::set_operation(d, e2, Some(operation(sgyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.exctg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn exctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetDecktopGroup(tp,3)
        let g = duel::get_decktop_group(d, tp, 3);
        // return #g>=3 and g:FilterCount(Card.IsAbleToHand,nil)>0
        return g.len() >= 3 && group::filter_count(d, &g, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,1-tp,500)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, 1 - tp, 500);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_DECK as i32);
    true
}
/// `s.excop(e,tp,eg,ep,ev,re,r,rp)`
fn excop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.Recover(1-tp,500,REASON_EFFECT)<=0 then return end
    if duel::recover(d, 1 - tp, 500, REASON_EFFECT, false, None) == 0 {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.ConfirmDecktop(tp,3)
    let _ = duel::confirm_decktop(d, tp, 3);
    // local g=Duel.GetDecktopGroup(tp,3)
    let g = duel::get_decktop_group(d, tp, 3);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_ATOHAND as u64);
        // local sc=g:Select(1-tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel).nil();
        // if sc:IsAbleToHand() then
        if card::is_able_to_hand(d, sc, None) {
            // Duel.SendtoHand(sc,nil,REASON_EFFECT)
            duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
        } else {
            // Duel.SendtoGrave(sc,REASON_RULE)
            duel::send_to_grave(d, CardSet::from_card(sc), REASON_RULE, None, None);
        }
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
    }
}
/// `s.sgycon(e,tp,eg,ep,ev,re,r,rp)`
fn sgycon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsRelateToBattle() and bc and bc:IsFaceup() and bc:IsRelateToBattle()
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    match bc {
        None => false,
        Some(bc) => card::is_faceup(d, bc) && card::is_relate_to_battle(d, bc),
    }
}
/// `s.sgytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sgytg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGrave() end
    if a.chk == Some(0) {
        return card::is_able_to_grave(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.sgyop(e,tp,eg,ep,ev,re,r,rp)`
fn sgyop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.SendtoGrave(c,REASON_EFFECT) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_grave(d, CardSet::from_card(c), REASON_EFFECT, None, None);
    }
}
