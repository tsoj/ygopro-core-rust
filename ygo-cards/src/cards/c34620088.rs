//! Gimmick Puppet Shadow Feeler
use super::prelude::*;

pub const ID: u32 = 34620088;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle indestructable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.GlobalEffect()
        let ge1 = effect::global_effect(d);
        // ge1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD);
        // ge1:SetCode(EFFECT_TO_GRAVE_REDIRECT)
        effect::set_code(d, ge1, EFFECT_TO_GRAVE_REDIRECT);
        // ge1:SetTargetRange(LOCATION_OVERLAY,LOCATION_OVERLAY)
        effect::set_target_range(d, ge1, LOCATION_OVERLAY, LOCATION_OVERLAY);
        // ge1:SetTarget(aux.TargetBoolFunction(Card.IsCode,id))
        effect::set_target(d, ge1, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[ID]))));
        // ge1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, ge1, LOCATION_REMOVED as i64);
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and eg:GetFirst():IsControler(1-tp) and Duel.GetAttackTarget()==nil
    if a.ep != a.tp {
        return false;
    }
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    card::is_controler(d, first, 1 - a.tp) && duel::get_attack_target(d).is_none()
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,0,0,tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 1000);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP_ATTACK)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None) != 0 {
        // Duel.Damage(tp,1000,REASON_EFFECT)
        duel::damage(d, a.tp, 1000, REASON_EFFECT, false, None);
    }
}
