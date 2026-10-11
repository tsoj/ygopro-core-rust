//! Wonko, Noble Knight of the Forest
use super::prelude::*;

pub const ID: u32 = 17832359;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_FAIRY_TALE_PROLOGUE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Fairy Tale Prologue: Journey's Dawn" from your Deck to your hand, or, if "Fairy Tale Prologue: Journey's Dawn" is on your field or in your GY, you can draw 1 card instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // While a card is in a Field Zone, your opponent's monsters cannot target monsters for attacks, except this one
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCondition(function() return Duel.IsExistingMatchingCard(nil,0,LOCATION_FZONE,LOCATION_FZONE,1,nil) end)
    effect::set_condition(d, e3, Some(condition(|d, _a| duel::is_existing_matching_card(d, None, 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX))));
    // e3:SetValue(function(e,c) return c~=e:GetHandler() end)
    effect::set_value_fn(d, e3, effect_card_fn(|d, e, c| {
        let h = effect::get_handler(d, e).nil();
        c != h
    }));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Make the monster that destroyed this card by battle lose 500 ATK
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYED);
    // e4:SetCondition(function(e) return e:GetHandler():GetReasonCard():IsRelateToBattle() end)
    effect::set_condition(d, e4, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        let rc = card::get_reason_card(d, h).nil();
        card::is_relate_to_battle(d, rc)
    })));
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_FAIRY_TALE_PROLOGUE) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_FAIRY_TALE_PROLOGUE]) && card::is_able_to_hand(d, c, None)
}
/// `aux.FaceupFilter(Card.IsCode,CARD_FAIRY_TALE_PROLOGUE)`
fn faceup_prologue(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_FAIRY_TALE_PROLOGUE])
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    //   or (Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_FAIRY_TALE_PROLOGUE),tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil)
    //   and Duel.IsPlayerCanDraw(tp,1)) end
    if a.chk == Some(0) {
        if duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX) {
            return true;
        }
        return duel::is_existing_matching_card(d, Some(&faceup_prologue), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX) && duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local search_chk=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let search_chk = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local draw_chk=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_FAIRY_TALE_PROLOGUE),tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil)
    //   and Duel.IsPlayerCanDraw(tp,1)
    let draw_chk = duel::is_existing_matching_card(d, Some(&faceup_prologue), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX) && duel::is_player_can_draw(d, tp, 1);
    // if search_chk and not (draw_chk and Duel.SelectYesNo(tp,aux.Stringid(id,2))) then
    if search_chk && !(draw_chk && duel::select_yes_no(d, tp, aux::stringid(ID, 2))) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    } else {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
/// `s.atkop(e,tp,...)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetReasonCard()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_reason_card(d, h).nil();
    // if bc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, bc) {
        // It loses 500 ATK
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-500)
        effect::set_value(d, e1, -500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
    }
}
