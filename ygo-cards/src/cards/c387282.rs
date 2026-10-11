//! Gagaga Sister
use super::prelude::*;

pub const ID: u32 = 387282;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GAGAGA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Gagaga" Spell/Trap Card from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change the Level of this card and 1 other "Gagaga" monster you control to be their combined current Levels
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GAGAGA) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GAGAGA as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
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
    if g.len() > 0 {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// function s.lvfilter(c,lv)
fn lvfilter(d: &mut Duel, c: CardId, lv: i64) -> bool {
    // local tc_lv=c:GetLevel()
    let tc_lv = card::get_level(d, c) as i64;
    // local total_lv=tc_lv+lv
    let total_lv = tc_lv + lv;
    // return c:IsSetCard(SET_GAGAGA) and c:HasLevel() and c:IsFaceup() and not (total_lv==lv and total_lv==tc_lv)
    card::is_set_card(d, c, &[SET_GAGAGA as u16]) && card::has_level(d, c) && card::is_faceup(d, c) && !(total_lv == lv && total_lv == tc_lv)
}
// function s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c) as i64;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc~=c and s.lvfilter(chkc,lv) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && chkc != c && lvfilter(d, chkc, lv);
    }
    // if chk==0 then return c:HasLevel() and Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_MZONE,0,1,c,lv) end
    if a.chk == Some(0) {
        return card::has_level(d, c) && duel::is_existing_target(d, Some(&|d, x| lvfilter(d, x, lv)), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_MZONE,0,1,1,c,lv):GetFirst()
    let sel = duel::select_target(d, a.tp, Some(&|d, x| lvfilter(d, x, lv)), a.tp, LOCATION_MZONE, 0, 1, 1, false, c.into()).nil();
    let tc = group::get_first(d, &sel).nil();
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,Group.FromCards(c,tc),2,tp,lv+tc:GetLevel())
    let mut g = CardSet::from_card(c);
    group::add_card(&mut g, &CardSet::from_card(tc));
    let param = lv + card::get_level(d, tc) as i64;
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(g), 2, a.tp, param as i32);
    true
}
// function s.lvop(e,tp,eg,ep,ev,re,r,rp)
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // local lv=c:GetLevel()+tc:GetLevel()
            let lv = card::get_level(d, c) as i64 + card::get_level(d, tc) as i64;
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetValue(lv)
            effect::set_value(d, e1, lv);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
