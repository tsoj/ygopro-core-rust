//! Nopenguin
use super::prelude::*;

pub const ID: u32 = 32623004;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PENGUIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_F)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_F);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // remove
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_TO_HAND_REDIRECT)
    effect::set_code(d, e2, EFFECT_TO_HAND_REDIRECT);
    // e2:SetTargetRange(LOCATION_ONFIELD,LOCATION_ONFIELD)
    effect::set_target_range(d, e2, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(effect_card_fn(rmtg)));
    // e2:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e2, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetHandler():IsSetCard(SET_PENGUIN) and re:IsMonsterEffect()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    card::is_set_card(d, rc, &[SET_PENGUIN as u16]) && effect::is_monster_effect(d, re)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, c.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(e:GetHandler(),0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.rmtg(e,c)`
fn rmtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsReason(REASON_EFFECT) and c:GetReasonEffect() and c:GetReasonEffect():GetHandler():IsSetCard(SET_PENGUIN)
    if !card::is_reason(d, c, REASON_EFFECT) {
        return false;
    }
    let Some(re) = card::get_reason_effect(d, c) else {
        return false;
    };
    let h = effect::get_handler(d, re).nil();
    card::is_set_card(d, h, &[SET_PENGUIN as u16])
}
