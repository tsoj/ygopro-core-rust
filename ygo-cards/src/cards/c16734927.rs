//! Ectoplasmic Fortification
//! 霊魂プラズマ
use super::prelude::*;

pub const ID: u32 = 16734927;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[80749819, CARD_CALL_OF_THE_HAUNTED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsLevel(6) and c:IsRace(RACE_ZOMBIE)) or c:IsCode(80749819)) and c:IsAbleToHand()
    ((card::is_level(d, c, &[6]) && card::is_race(d, c, RACE_ZOMBIE)) || card::is_code(d, c, &[80749819])) && card::is_able_to_hand(d, c, None)
}
/// `aux.FaceupFilter(Card.IsRace,RACE_ZOMBIE)`
fn faceup_zombie(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_ZOMBIE)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_ZOMBIE),tp,LOCATION_MZONE,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&faceup_zombie), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).map_or(0, |v| v as i64);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE + CATEGORY_DRAW);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // --● If you control no monsters: Add 1 Level 6 Zombie monster or 1 "Call of the Forgotten" from your Deck to your hand
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
    // elseif op==2 then
    } else if op == 2 {
        // --● Zombie monsters you control gain 400 ATK, then you can draw cards equal to the number of "Call of the Haunted" you control
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_ZOMBIE),tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&faceup_zombie), tp, LOCATION_MZONE, 0, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e);
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // --Zombie monsters you control gain 400 ATK
            // tc:UpdateAttack(400,RESET_EVENT|RESETS_STANDARD,c)
            card::update_attack(d, tc, 400, Some(RESET_EVENT | RESETS_STANDARD), c, None);
        }
        // local draw_count=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsCode,CARD_CALL_OF_THE_HAUNTED),tp,LOCATION_ONFIELD,0,nil)
        let draw_count = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_CALL_OF_THE_HAUNTED])), tp, LOCATION_ONFIELD, 0, NO_EX);
        // if draw_count>0 and Duel.IsPlayerCanDraw(tp,draw_count) and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if draw_count > 0 && duel::is_player_can_draw(d, tp, draw_count as u32) && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,draw_count,REASON_EFFECT)
            duel::draw(d, tp, draw_count as u32, REASON_EFFECT);
        }
    }
}
