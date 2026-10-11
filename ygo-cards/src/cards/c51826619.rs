//! Magnet Warrior Sigma Plus
use super::prelude::*;

pub const ID: u32 = 51826619;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGNET_WARRIOR], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // While you control an EARTH monster, your opponent's monsters that can attack must attack EARTH monsters
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    // e1a:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e1a, EFFECT_MUST_ATTACK);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1a, 0, LOCATION_MZONE);
    // e1a:SetCondition(function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_EARTH),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(d, e1a, Some(condition(cond1a)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e1b, EFFECT_MUST_ATTACK_MONSTER);
    // e1b:SetValue(function(e,c) return c:IsAttribute(ATTRIBUTE_EARTH) end)
    effect::set_value_fn(d, e1b, effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_EARTH)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // While your opponent controls an EARTH monster, you choose the attack targets for your opponent's attacks
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_PATRICIAN_OF_DARKNESS)
    effect::set_code(d, e2, EFFECT_PATRICIAN_OF_DARKNESS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCondition(function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_EARTH),e:GetHandlerPlayer(),0,LOCATION_MZONE,1,nil) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add to your hand, or Special Summon, 1 Level 4 or lower "Magnet Warrior" monster in your GY, except "Magnet Warrior Sigma Plus"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.thsptg)
    effect::set_target(d, e3, Some(cost_target(thsptg)));
    // e3:SetOperation(s.thspop)
    effect::set_operation(d, e3, Some(operation(thspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_EARTH),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil) end`
fn cond1a(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH)), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_EARTH),e:GetHandlerPlayer(),0,LOCATION_MZONE,1,nil) end`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH)), p, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.thspfilter(c,e,tp,mmz_chk)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // return c:IsLevelBelow(4) and c:IsSetCard(SET_MAGNET_WARRIOR) and not c:IsCode(id)
    card::is_level_below(d, c, 4) && card::is_set_card(d, c, &[SET_MAGNET_WARRIOR as u16]) && !card::is_code(d, c, &[ID])
        // and (c:IsAbleToHand() or (mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
        && (card::is_able_to_hand(d, c, None) || (mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.thspfilter(chkc,e,tp,mmz_chk) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && thspfilter(d, chkc, e, tp, mmz_chk);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,mmz_chk) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| thspfilter(d, c, e, tp, mmz_chk)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local g=Duel.SelectTarget(tp,s.thspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,mmz_chk)
    let g = duel::select_target(d, tp, Some(&|d, c| thspfilter(d, c, e, tp, mmz_chk)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, g.clone(), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
/// `s.thspop(e,tp,eg,ep,ev,re,r,rp)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    let tp = a.tp;
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // aux.ToHandOrElse(tc,tp, function() ... end, function() Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end, aux.Stringid(id,2))
        let check = |d: &mut Duel, _c: CardId| {
            duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, tc, a.e, 0, tp, false, false, None, None, None)
        };
        let oper = |d: &mut Duel, _g: &CardSet| {
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            0
        };
        aux::to_hand_or_else(d, tc, tp, Some(&check), Some(&oper), Some(aux::stringid(ID, 2)));
    }
}
