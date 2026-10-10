//! Nadir Servant
use super::prelude::*;

pub const ID: u32 = 1984618;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], listed_series: &[SET_DOGMATIKA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,tp,necro)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8, necro: bool) -> bool {
    // return c:IsAbleToGrave() and Duel.IsExistingMatchingCard(necro and aux.NecroValleyFilter(s.thfilter) or s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,c:GetAttack())
    if !card::is_able_to_grave(d, c, None) {
        return false;
    }
    let atk = card::get_attack(d, c) as i64;
    if necro {
        duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, x| thfilter(d, x, atk))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
    } else {
        duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, atk)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
    }
}
/// `s.thfilter(c,atk)`
fn thfilter(d: &mut Duel, c: CardId, atk: i64) -> bool {
    // return (c:IsCode(CARD_ALBAZ) or (c:IsMonster() and c:IsSetCard(SET_DOGMATIKA))) and c:GetAttack()<=atk and c:IsAbleToHand()
    (card::is_code(d, c, &[CARD_ALBAZ]) || (card::is_monster(d, c) && card::is_set_card(d, c, &[SET_DOGMATIKA as u16])))
        && (card::get_attack(d, c) as i64) <= atk
        && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_EXTRA,0,1,nil,tp,false) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, tp, false)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_EXTRA)
    card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_EXTRA,0,1,1,nil,tp,true)
    let g1 = duel::select_matching_card(d, tp, Some(&|d, c| tgfilter(d, c, tp, true)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g1>0 and Duel.SendtoGrave(g1,REASON_EFFECT)>0 then
    if !g1.is_empty() && duel::send_to_grave(d, g1.clone(), REASON_EFFECT, None, None) > 0 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // if og:GetFirst():IsLocation(LOCATION_GRAVE) then
        let first = group::get_first(d, &og).nil();
        if card::is_location(d, first, LOCATION_GRAVE) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local g2=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,g1:GetFirst():GetAttack())
            let f1 = group::get_first(d, &g1).nil();
            let atk = card::get_attack(d, f1) as i64;
            let g2 = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, x| thfilter(d, x, atk))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            // if #g2>0 then
            if !g2.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(g2,nil,REASON_EFFECT)
                duel::send_to_hand(d, g2.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,g2)
                duel::confirm_cards(d, 1 - tp, g2);
            }
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(e:GetHandler(),EFFECT_FLAG_OATH,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, h, EFFECT_FLAG_OATH, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
}
