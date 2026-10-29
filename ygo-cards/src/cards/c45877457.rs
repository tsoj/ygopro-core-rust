//! Magikey Mechmortar - Garesglasser
use super::prelude::*;

pub const ID: u32 = 45877457;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGIKEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Gains 300 ATK for each different Attribute in your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the activation of an opponent's monster effect and destroy it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Magikey" monster from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=Duel.GetMatchingGroup(Card.IsMonster,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil)
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&card::is_monster), p, LOCATION_GRAVE, 0, NO_EX);
    // return g:GetClassCount(Card.GetAttribute)*300
    group::get_class_count(d, &g, &|d, x| card::get_attribute(d, x) as i64) as i64 * 300
}
// function s.negcon(e,tp,eg,ep,ev,re,r,rp)
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRitualSummoned() then return false end
    if !card::is_ritual_summoned(d, c) {
        return false;
    }
    // local mat=c:GetMaterial()
    let mat = card::get_material(d, c);
    // if not (mat:GetBinClassCount(Card.GetAttribute)>=2 and Duel.IsChainNegatable(ev) and rp==1-tp and not c:IsStatus(STATUS_BATTLE_DESTROYED)) then return false end
    let bin = group::get_bin_class_count(d, &mat, &|d, x| card::get_attribute(d, x) as i64);
    if !(bin >= 2 && duel::is_chain_negatable(d, a.ev as u8) && a.rp == 1 - tp && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)) {
        return false;
    }
    // local trig_typ,trig_att=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_TYPE,CHAININFO_TRIGGERING_ATTRIBUTE)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_TYPE, CHAININFO_TRIGGERING_ATTRIBUTE]);
    let trig_typ = info[0].int();
    let trig_att = info[1].int();
    // local g=Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&card::is_monster), tp, LOCATION_GRAVE, 0, NO_EX);
    // return #g>0 and trig_typ&TYPE_MONSTER>0 and g:IsExists(Card.IsAttribute,1,nil,trig_att)
    !g.is_empty()
        && (trig_typ & TYPE_MONSTER as i64) > 0
        && group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, trig_att as u32), 1, NO_EX)
}
// function s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,rc,1,1-tp,rc:GetLocation())
    let loc = card::get_location(d, rc) as i32;
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, Some(CardSet::from_card(rc)), 1, 1 - tp, loc);
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,rc,1,1-tp,rc:GetLocation())
        let loc = card::get_location(d, rc) as i32;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(rc)), 1, 1 - tp, loc);
    }
    true
}
// function s.negop(e,tp,eg,ep,ev,re,r,rp)
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    if duel::negate_activation(d, a.ev as u8) {
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGIKEY) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MAGIKEY as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsRitualSummoned()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_ritual_summoned(d, c)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
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
}
