//! Atlantis, City of the Sea Dragon
use super::prelude::*;

pub const ID: u32 = 38391684;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON, CARD_UMI], listed_series: &[SET_DAEDALUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // While a monster that mentions "Atlantis, the Dragon City" is on the field, reduce the Levels of all monsters in both players' hands and fields by 1
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,LOCATION_HAND|LOCATION_MZONE)
    effect::set_target_range(d, e1, (LOCATION_HAND | LOCATION_MZONE) as u32, (LOCATION_HAND | LOCATION_MZONE) as u32);
    // e1:SetCondition(function() return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.ListsCode,CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON),0,LOCATION_MZONE,LOCATION_MZONE,1,nil) end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetValue(-1)
    effect::set_value(d, e1, -1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you Link Summon a "Daedalus" monster, you can reduce the material requirement listed on it by 1 and Link Summon it with 1 less material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON)
    effect::set_code(d, e2, CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card is in your GY and you control "Umi": You can place this card face-up on your field. You can only use this effect of "Atlantis, the Dragon City" once per turn.
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(...)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetTarget(...)
    effect::set_target(d, e3, Some(cost_target(e3_target)));
    // e3:SetOperation(...)
    effect::set_operation(d, e3, Some(operation(e3_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `aux.FaceupFilter(Card.ListsCode,CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON)`
fn faceup_lists_atlantis(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::lists_code(d, c, &[CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON])
}
/// `function() return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.ListsCode,...),0,LOCATION_MZONE,LOCATION_MZONE,1,nil) end`
fn e1_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&faceup_lists_atlantis), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `aux.FaceupFilter(Card.IsCode,CARD_UMI)`
fn faceup_umi(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_UMI])
}
/// `e3:SetCondition(function(e,tp,...) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_UMI),tp,LOCATION_ONFIELD,0,1,nil) end)`
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&faceup_umi), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `e3:SetTarget(function(e,tp,...,chk) ... end)`
fn e3_target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:CheckUniqueOnField(tp) end
    if a.chk == Some(0) {
        return card::check_unique_on_field(d, c, a.tp, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `e3:SetOperation(function(e,tp,...) ... end)`
fn e3_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
    }
}
