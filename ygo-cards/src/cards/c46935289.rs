//! Herald of Mirage Lights
use super::prelude::*;

pub const ID: u32 = 46935289;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.NOT(aux.FilterBoolFunctionEx(Card.IsType,TYPE_TOKEN)),2,nil,s.spcheck)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| !card::is_type_ctx(d, c, TYPE_TOKEN, sc, st, Some(tp)))),
        2,
        None,
        Some(group_check(spcheck)),
        None,
        None,
    );
    // Negate Spell/Trap or effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetCost(s.discost)
    effect::set_cost(d, e1, Some(cost_target(discost)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.spcheck(g,lc,sumtype,tp)`
fn spcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:CheckSameProperty(Card.GetRace,lc,sumtype,tp) and g:CheckSameProperty(Card.GetAttribute,lc,sumtype,tp)
    group::check_same_property(d, g, &|d, c| card::get_race_ctx(d, c, Some(lc), sumtype, Some(tp))).0
        && group::check_same_property(d, g, &|d, c| card::get_attribute_ctx(d, c, Some(lc), sumtype, Some(tp)) as u64).0
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // return re:IsSpellTrapEffect() and Duel.IsChainNegatable(ev)
    effect::is_spell_trap_effect(d, a.re.nil()) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.disfilter(c)`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FAIRY) and c:IsAbleToGraveAsCost()
    card::is_race(d, c, RACE_FAIRY) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.disfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&disfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.disfilter,1,1,REASON_COST|REASON_DISCARD,nil)
    duel::discard_hand(d, a.tp, Some(&disfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let rh = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rh, a.re.nil()) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and e:GetHandler():IsPreviousControler(tp)
    if a.rp != 1 - a.tp {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_controler(d, h, a.tp)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand() and (c:IsRitualMonster() or c:IsRitualSpell())
    card::is_able_to_hand(d, c, None) && (card::is_ritual_monster(d, c) || card::is_ritual_spell(d, c))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,2,aux.dncheck,1,tp,HINTMSG_ATOHAND)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(2), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_ATOHAND as u64), None, None, false)
        .unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
