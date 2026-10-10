//! Formud Skipper
use super::prelude::*;

pub const ID: u32 = 50366775;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reveal 1 Link Monster in your Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.revtg)
    effect::set_target(d, e1, Some(cost_target(revtg)));
    // e1:SetOperation(s.revop)
    effect::set_operation(d, e1, Some(operation(revop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search 1 Level 5 or higher Cyberse monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsLocation(LOCATION_GRAVE) and r==REASON_LINK end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_location(d, c, LOCATION_GRAVE) && a.r == REASON_LINK
    })));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.revfilter(c,code)
fn revfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsLinkMonster() and not c:IsCode(code)
    card::is_link_monster(d, c) && !card::is_code(d, c, &[code])
}
// function s.revtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn revtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local code=e:GetHandler():GetCode(nil,SUMMON_TYPE_LINK,tp)
    let h = effect::get_handler(d, a.e).nil();
    let code = card::get_code_ctx(d, h, None, SUMMON_TYPE_LINK as u64, Some(a.tp))[0];
    // if chk==0 then return Duel.IsExistingMatchingCard(s.revfilter,tp,LOCATION_EXTRA,0,1,nil,code) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| revfilter(d, c, code)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    true
}
// function s.revop(e,tp,eg,ep,ev,re,r,rp)
fn revop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local code=c:GetCode(nil,SUMMON_TYPE_LINK,tp)
    let code = card::get_code_ctx(d, c, None, SUMMON_TYPE_LINK as u64, Some(a.tp))[0];
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.revfilter,tp,LOCATION_EXTRA,0,1,1,nil,code):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| revfilter(d, x, code)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &g);
    // if not rc then return end
    let Some(rc) = rc else { return };
    // Duel.ConfirmCards(1-tp,rc)
    duel::confirm_cards(d, 1 - a.tp, rc);
    // local code1,code2=rc:GetOriginalCodeRule()
    let codes = card::get_original_code_rule(d, rc);
    // Can be treated as the revealed monster's name, Type, and Attribute, when used as Link Material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_FORMUD_SKIPPER)
    effect::set_code(d, e1, EFFECT_FORMUD_SKIPPER);
    // e1:SetLabel(ASSUME_CODE,code1,ASSUME_RACE,rc:GetRace(),ASSUME_ATTRIBUTE,rc:GetAttribute())
    let race = card::get_race(d, rc);
    let attribute = card::get_attribute(d, rc);
    effect::set_label(
        d,
        e1,
        &[ASSUME_CODE as i64, codes[0] as i64, ASSUME_RACE as i64, race as i64, ASSUME_ATTRIBUTE as i64, attribute as i64],
    );
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:IsLevelAbove(5) and c:IsAbleToHand()
    card::is_race(d, c, RACE_CYBERSE) && card::is_level_above(d, c, 5) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
