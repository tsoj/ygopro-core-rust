//! Officiating Reverie
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 49828011;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is in your hand: You can discard 1 other card; Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.Discard(nil,true))
    effect::set_cost(d, e1, Some(cost::discard(None, true, None, None, None)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If an Xyz Monster(s) is sent to the GY, while this card is in your GY (except during the Damage Step): You can banish this card; Special Summon 1 Zombie monster from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.gyspcon)
    effect::set_condition(d, e2, Some(lua_fn(gyspcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.gysptg)
    effect::set_target(d, e2, Some(cost_target(gysptg)));
    // e2:SetOperation(s.gyspop)
    effect::set_operation(d, e2, Some(operation(gyspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // During the Standby Phase of the next turn after this card was banished: You can target 1 Zombie Xyz Monster you control; attach this banished card to it
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_STANDBY);
    // e3:SetRange(LOCATION_REMOVED)
    effect::set_range(d, e3, LOCATION_REMOVED);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function(e) return Duel.GetTurnCount()==e:GetHandler():GetTurnID()+1 end)
    effect::set_condition(d, e3, Some(lua_fn(e3_cond)));
    // e3:SetTarget(s.attachtg)
    effect::set_target(d, e3, Some(cost_target(attachtg)));
    // e3:SetOperation(s.attachop)
    effect::set_operation(d, e3, Some(operation(attachop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.gyspcon(e,tp,eg,ep,ev,re,r,rp)`
fn gyspcon(d: &mut Duel, a: Args) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(Card.IsXyzMonster,1,nil)
    let eg = a.g(2);
    let h = effect::get_handler(d, a.e(0)).nil();
    !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, c| card::is_xyz_monster(d, c), 1, NO_EX)
}
/// `s.gyspfilter(c,e,tp)`
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_ZOMBIE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.gysptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.gyspfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler(),e,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| gyspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.gyspop(e,tp,eg,ep,ev,re,r,rp)`
fn gyspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gyspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| gyspfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.xyzfilter(c,tp,hc)`
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8, hc: CardId) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsXyzMonster() and c:IsFaceup() and hc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT)
    card::is_race(d, c, RACE_ZOMBIE)
        && card::is_xyz_monster(d, c)
        && card::is_faceup(d, c)
        && card::is_can_be_xyz_material(d, hc, Some(c), Some(tp), Some(REASON_EFFECT))
}
/// `function(e) return Duel.GetTurnCount()==e:GetHandler():GetTurnID()+1 end`
fn e3_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    duel::get_turn_count(d, None) == card::get_turn_id(d, h) as i32 + 1
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.xyzfilter(chkc,tp,c) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && xyzfilter(d, chkc, tp, c);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil,tp,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| xyzfilter(d, x, tp, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil,tp,c)
    let _ = duel::select_target(d, tp, Some(&|d, x| xyzfilter(d, x, tp, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
            // Duel.Overlay(tc,c)
            duel::overlay(d, tc, c, false);
        }
    }
}
