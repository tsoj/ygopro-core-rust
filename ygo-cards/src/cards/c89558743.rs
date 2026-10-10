//! Small World
use super::prelude::*;

pub const ID: u32 = 89558743;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish and search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.singleprop(c,d)`: `c` matches `mc` in exactly one of Race, Attribute, Level, ATK, DEF
fn singleprop(d: &mut Duel, c: CardId, mc: CardId) -> bool {
    // local ct=0
    let mut ct = 0;
    // if c:IsRace(d:GetRace()) then ct=ct+1 end
    let race = card::get_race(d, mc);
    if card::is_race(d, c, race) {
        ct += 1;
    }
    // if c:IsAttribute(d:GetAttribute()) then ct=ct+1 end
    let attribute = card::get_attribute(d, mc);
    if card::is_attribute(d, c, attribute) {
        ct += 1;
    }
    // if c:IsLevel(d:GetLevel()) then ct=ct+1 end
    let level = card::get_level(d, mc);
    if card::is_level(d, c, &[level]) {
        ct += 1;
    }
    // if c:IsAttack(d:GetTextAttack()) and d:IsAttack(c:GetTextAttack()) then ct=ct+1 end
    let atk = card::get_text_attack(d, mc);
    if card::is_attack(d, c, &[atk as u32]) && card::is_attack(d, mc, &[card::get_text_attack(d, c) as u32]) {
        ct += 1;
    }
    // if c:IsDefense(d:GetTextDefense()) and d:IsDefense(c:GetTextDefense()) then ct=ct+1 end
    let def = card::get_text_defense(d, mc);
    if card::is_defense(d, c, &[def as u32]) && card::is_defense(d, mc, &[card::get_text_defense(d, c) as u32]) {
        ct += 1;
    }
    // return ct==1
    ct == 1
}
/// `s.thfilter(c,mc)`
fn thfilter(d: &mut Duel, c: CardId, mc: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToHand() and s.singleprop(c,mc)
    card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && singleprop(d, c, mc)
}
/// `s.showfilter(c,tp,mc)`
fn showfilter(d: &mut Duel, c: CardId, tp: u8, mc: CardId) -> bool {
    // return c:IsMonster() and not c:IsPublic() and c:IsAbleToRemove() and s.singleprop(c,mc)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,c,c)
    card::is_monster(d, c)
        && !card::is_public(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
        && singleprop(d, c, mc)
        && duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, c)), tp, LOCATION_DECK, 0, 1, c.into())
}
/// `s.revfilter(c,tp)`
fn revfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and not c:IsPublic() and c:IsAbleToRemove()
    //     and Duel.IsExistingMatchingCard(s.showfilter,tp,LOCATION_DECK,0,1,nil,tp,c)
    card::is_monster(d, c)
        && !card::is_public(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| showfilter(d, x, tp, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.revfilter,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| revfilter(d, x, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 2, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.revfilter,tp,LOCATION_HAND,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| revfilter(d, x, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if not rc then return end
    let Some(rc) = group::get_first(d, &g) else {
        return;
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.showfilter,tp,LOCATION_DECK,0,1,1,nil,tp,rc):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| showfilter(d, x, tp, rc)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else {
        return;
    };
    // Duel.ConfirmCards(1-tp,rc)
    duel::confirm_cards(d, 1 - tp, rc);
    // Duel.ConfirmCards(1-tp,sc)
    duel::confirm_cards(d, 1 - tp, sc);
    // if Duel.Remove(rc,POS_FACEDOWN,REASON_EFFECT)>0 and rc:IsLocation(LOCATION_REMOVED) and rc:IsFacedown() then
    if duel::remove(d, rc, POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, rc, LOCATION_REMOVED)
        && card::is_facedown(d, rc)
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local th=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,sc,sc)
        let th = duel::select_matching_card(d, tp, Some(&|d, x| thfilter(d, x, sc)), tp, LOCATION_DECK, 0, 1, 1, false, sc.into()).nil();
        // if #th>0 then
        if !th.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.SendtoHand(th,nil,REASON_EFFECT)>0 and th:GetFirst():IsLocation(LOCATION_HAND) then
            if duel::send_to_hand(d, th.clone(), None, REASON_EFFECT, None) > 0 {
                let first = group::get_first(d, &th).nil();
                if card::is_location(d, first, LOCATION_HAND) {
                    // Duel.ConfirmCards(1-tp,th)
                    duel::confirm_cards(d, 1 - tp, th);
                    // Duel.Remove(sc,POS_FACEDOWN,REASON_EFFECT)
                    duel::remove(d, sc, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
                }
            }
        }
    }
}
