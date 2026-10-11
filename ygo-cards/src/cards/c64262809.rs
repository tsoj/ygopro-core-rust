//! Dragon Ice
//! ドラゴン・アイス
use super::prelude::*;

pub const ID: u32 = 64262809;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[64262809], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // There can only be 1 "Dragon Ice" on the field
    // c:SetUniqueOnField(1,1,id)
    card::set_unique_on_field(d, c, 1, 1, card::UniqueCode::Code(ID), None);
    // When your opponent Special Summons a monster (except during the Damage Step): You can discard 1 card; Special Summon this card from your hand or Graveyard
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCondition(function(e,tp,eg) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.Discard(function(c,e,tp) ... end))
    effect::set_cost(d, e1, Some(cost::discard(Some(std::sync::Arc::new(discard_filter)), false, None, None, None)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The filter of `Cost.Discard`: `function(c,e,tp) ... end`
fn discard_filter(d: &mut Duel, c: CardId, e: EffectId, _tp: u8) -> bool {
    // local dragon_ice=e:GetHandler()
    let dragon_ice = effect::get_handler(d, e).nil();
    // return dragon_ice:IsLocation(LOCATION_GRAVE) or dragon_ice:IsAbleToGraveAsCost() or c~=dragon_ice
    card::is_location(d, dragon_ice, LOCATION_GRAVE) || card::is_able_to_grave_as_cost(d, dragon_ice) || c != dragon_ice
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // c:CreateEffectRelation(e)
    card::create_effect_relation(d, c, a.e);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `function(e,tp,eg) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg();
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX)
}
