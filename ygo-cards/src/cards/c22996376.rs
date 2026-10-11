//! Behemoth the King of All Animals
use super::prelude::*;

pub const ID: u32 = 22996376;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Normal Summon/Set with 1 Tribute
    // local e1=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),nil,s.otop)
    let _e1 = aux::add_normal_summon_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE),
        Some(aux::stringid(ID, 0)),
        None,
        Some(std::sync::Arc::new(otop)),
    );
    // local e2=aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),nil,s.otop)
    let _e2 = aux::add_normal_set_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE),
        Some(aux::stringid(ID, 0)),
        None,
        Some(std::sync::Arc::new(otop)),
    );
    // Return Beast monsters from your GY to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(function(e) return e:GetHandler():IsTributeSummoned() end)
    effect::set_condition(d, e3, Some(condition(tributed_cond)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():IsTributeSummoned() end`
fn tributed_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, c)
}
/// `s.otop(g,e,tp,eg,ep,ev,re,r,rp,c,minc,zone,relzone,exeff)`
fn otop(d: &mut Duel, _g: &CardSet, a: &EffArgs) {
    // (c is the summoned monster, the argument after rp)
    let c = a.extra.first().and_then(|v| v.as_card()).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(2000)
    effect::set_value(d, e1, 2000);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE&~RESET_TOFIELD)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_BEAST) and c:IsAbleToHand()
    card::is_race(d, c, RACE_BEAST) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && thfilter(d, chkc);
    }
    // local ct=e:GetHandler():GetMaterialCount()
    let c = effect::get_handler(d, a.e).nil();
    let ct = card::get_material_count(d, c) as u16;
    // if chk==0 then return ct>0 and Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,ct,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, ct, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,ct,ct,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, ct, ct, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, ct as u8, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e):Match(Card.IsRace,nil,RACE_BEAST)
    let mut g = duel::get_target_cards(d, a.e).nil();
    group::match_(d, &mut g, &|d, c| card::is_race(d, c, RACE_BEAST), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
