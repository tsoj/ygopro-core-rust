//! Return of the Monarchs
use super::prelude::*;

pub const ID: u32 = 61466310;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(s.sumlimit)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // search
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND | CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Lizard lock
    // aux.addContinuousLizardCheck(c,LOCATION_SZONE)
    aux::add_continuous_lizard_check(d, c, LOCATION_SZONE, None, None, None);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and eg:GetFirst():IsTributeSummoned()
    let ec = group::get_first(d, &a.eg.clone().nil()).nil();
    a.ep == a.tp && card::is_tribute_summoned(d, ec)
}
/// `s.filter(c,code)`
fn filter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return (c:GetAttack()==2400 or c:GetAttack()==2800) and c:GetDefense()==1000 and c:GetCode()~=code and c:IsAbleToHand()
    (card::get_attack(d, c) == 2400 || card::get_attack(d, c) == 2800)
        && card::get_defense(d, c) == 1000
        && card::get_code(d, c)[0] != code
        && card::is_able_to_hand(d, c, None)
}
/// `s.filter2(c,atk,code)`
fn filter2(d: &mut Duel, c: CardId, atk: i32, code: u32) -> bool {
    // return c:GetAttack()==atk and c:GetDefense()==1000 and c:GetCode()~=code and c:IsAbleToHand()
    card::get_attack(d, c) == atk && card::get_defense(d, c) == 1000 && card::get_code(d, c)[0] != code && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,eg:GetFirst():GetCode()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        if !card::is_relate_to_effect(d, h, a.e) {
            return false;
        }
        let ec = group::get_first(d, &a.eg.clone().nil()).nil();
        let code = card::get_code(d, ec)[0];
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, code)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // local t1=Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK,0,1,nil,2400,eg:GetFirst():GetCode())
    let ec = group::get_first(d, &a.eg.clone().nil()).nil();
    let code = card::get_code(d, ec)[0];
    let t1 = duel::is_existing_matching_card(d, Some(&|d, c| filter2(d, c, 2400, code)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local t2=Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK,0,1,nil,2800,eg:GetFirst():GetCode())
    let ec = group::get_first(d, &a.eg.clone().nil()).nil();
    let code = card::get_code(d, ec)[0];
    let t2 = duel::is_existing_matching_card(d, Some(&|d, c| filter2(d, c, 2800, code)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // if t1 and t2 then e:SetLabel(Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2)))
    if t1 && t2 {
        let v = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
        effect::set_label(d, a.e, &[v as i64]);
    }
    // elseif t1 then e:SetLabel(Duel.SelectOption(tp,aux.Stringid(id,1)))
    else if t1 {
        let v = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]);
        effect::set_label(d, a.e, &[v as i64]);
    }
    // else e:SetLabel(Duel.SelectOption(tp,aux.Stringid(id,2))+1) end
    else {
        let v = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]);
        effect::set_label(d, a.e, &[v as i64 + 1]);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // eg:GetFirst():CreateEffectRelation(e)
    let ec = group::get_first(d, &a.eg.clone().nil()).nil();
    card::create_effect_relation(d, ec, a.e);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local ec=eg:GetFirst()
    let ec = group::get_first(d, &a.eg.clone().nil()).nil();
    // if ec:IsFacedown() or not ec:IsRelateToEffect(e) then return end
    if card::is_facedown(d, ec) || !card::is_relate_to_effect(d, ec, a.e) {
        return;
    }
    // local atk=e:GetLabel()==0 and 2400 or 2800
    let atk = if effect::get_label(d, a.e) == 0 { 2400 } else { 2800 };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_DECK,0,1,1,nil,atk,ec:GetCode())
    let code = card::get_code(d, ec)[0];
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter2(d, c, atk, code)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
