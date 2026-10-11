//! Radiant Typhoon Varuroon, the Vibrant Vortex
//! 絢嵐渦麗ヴァルルーン
use super::prelude::*;

pub const ID: u32 = 53927851;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MYSTICAL_SPACE_TYPHOON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and re:IsActiveType(TYPE_QUICKPLAY) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let re = a.re.nil();
        effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && effect::is_active_type(d, re, TYPE_QUICKPLAY)
    })));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate a monster effect activated by the opponent, then if you have 2 or more "Mystical Space Typhoon" in your GY, you can destroy that monster.
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon this card from your GY if "Mystical Space Typhoon" is activated
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsCode(CARD_MYSTICAL_SPACE_TYPHOON) end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let re = a.re.nil();
        effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && {
            let h = effect::get_handler(d, re).nil();
            card::is_code(d, h, &[CARD_MYSTICAL_SPACE_TYPHOON])
        }
    })));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
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
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect() and Duel.IsChainDisablable(ev)
    //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_MYSTICAL_SPACE_TYPHOON)
    let re = a.re.nil();
    a.rp == 1 - a.tp
        && effect::is_monster_effect(d, re)
        && duel::is_chain_disablable(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_MYSTICAL_SPACE_TYPHOON])), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,re:GetHandler(),1,tp,0)
    let re = a.re.nil();
    let rh = effect::get_handler(d, re);
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, rh.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if Duel.NegateEffect(ev) and rc:IsRelateToEffect(re) and rc:IsDestructable()
    //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,2,nil,CARD_MYSTICAL_SPACE_TYPHOON)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::negate_effect(d, a.ev as u8)
        && card::is_relate_to_effect(d, rc, re)
        && card::is_destructable(d, rc, None)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_MYSTICAL_SPACE_TYPHOON])), a.tp, LOCATION_GRAVE, 0, 2, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(eg,REASON_EFFECT)
        duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    }
}
