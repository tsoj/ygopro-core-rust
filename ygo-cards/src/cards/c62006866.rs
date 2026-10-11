//! Zubababa Knight
//! ズバババナイト
use super::prelude::*;

pub const ID: u32 = 62006866;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_ZUBABA, SET_GAGAGA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Gagaga" monster from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.gathcon)
    effect::set_condition(d, e3, Some(condition(gathcon)));
    // e3:SetTarget(s.gathtg)
    effect::set_target(d, e3, Some(cost_target(gathtg)));
    // e3:SetOperation(s.gathop)
    effect::set_operation(d, e3, Some(operation(gathop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_REMOVE)
    effect::set_code(d, e4, EVENT_REMOVE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.zbthfilter(c)`
fn zbthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZUBABA) and c:IsMonster() and c:IsAbleToHand() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_ZUBABA as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsDefensePos() and c:IsFaceup()
    card::is_level_below(d, c, 4) && card::is_defense_pos(d, c) && card::is_faceup(d, c)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local b1=Duel.IsExistingMatchingCard(s.zbthfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&zbthfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=#g>0
    let b2 = !g.is_empty();
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.unwrap_or(0) as i64]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_LVCHANGE)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_LVCHANGE);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Add 1 "Zubaba" monster from your Deck to your hand, except "Zubababa Knight", and if you do, this card's Level becomes that added monster's
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.zbthfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&zbthfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 then
        if let Some(sc) = sc {
            if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 {
                // Duel.ConfirmCards(1-tp,sc)
                duel::confirm_cards(d, 1 - a.tp, sc);
                // local lv=sc:GetLevel()
                let lv = card::get_level(d, sc);
                // local c=e:GetHandler()
                let c = effect::get_handler(d, a.e).nil();
                // if c:IsRelateToEffect(e) and c:IsFaceup() and not c:IsLevel(lv) then
                if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && !card::is_level(d, c, &[lv]) {
                    // This card's Level becomes the added monster's
                    // local e1=Effect.CreateEffect(c)
                    let e1 = effect::create_effect(d, c);
                    // e1:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    // e1:SetCode(EFFECT_CHANGE_LEVEL)
                    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
                    // e1:SetValue(lv)
                    effect::set_value(d, e1, lv as i64);
                    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
                    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
                    // c:RegisterEffect(e1)
                    card::register_effect(d, c, e1, false);
                }
            }
        }
    } else if op == 2 {
        // Destroy 1 Level 4 or lower Defense Position monster your opponent controls
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,s.desfilter,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&desfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
/// `s.gathcon(e,tp,eg,ep,ev,re,r,rp)`
fn gathcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_COST) and re:IsActivated() and re:IsActiveType(TYPE_XYZ) and c:IsPreviousLocation(LOCATION_OVERLAY)
    let re = a.re.nil();
    card::is_reason(d, c, REASON_COST)
        && effect::is_activated(d, re)
        && effect::is_active_type(d, re, TYPE_XYZ)
        && card::is_previous_location(d, c, LOCATION_OVERLAY)
}
/// `s.gathfilter(c)`
fn gathfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GAGAGA) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GAGAGA as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.gathtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gathtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.gathfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&gathfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.gathop(e,tp,eg,ep,ev,re,r,rp)`
fn gathop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gathfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&gathfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
