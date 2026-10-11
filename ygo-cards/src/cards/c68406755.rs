//! Triamid Sphinx
use super::prelude::*;

pub const ID: u32 = 68406755;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_TRIAMID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
    // spsummon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.splimit)
    effect::set_value_fn(d, e1, lua_fn(splimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon
    // e2:SetDescription(aux.Stringid(id,0))
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atk
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetCondition(s.efcon)
    effect::set_condition(d, e3, Some(condition(efcon)));
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // must be attacked
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e5, EFFECT_MUST_ATTACK_MONSTER);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e5, 0, LOCATION_MZONE);
    // e5:SetCondition(s.efcon)
    effect::set_condition(d, e5, Some(condition(efcon)));
    // e5:SetValue(s.attg)
    effect::set_value_fn(d, e5, effect_card_fn(attg));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return se:GetHandler():IsSetCard(SET_TRIAMID)
    let se = a.e(1);
    let h = effect::get_handler(d, se).nil();
    card::is_set_card(d, h, &[SET_TRIAMID as u16])
}
/// `s.spfilter(c,tp)`
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT)
    //     and c:IsSetCard(SET_TRIAMID) and not c:IsCode(id)
    //     and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEUP)
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_set_card(d, c, &[SET_TRIAMID as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEUP)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| spfilter(d, x, a.tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_TRIAMID)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TRIAMID as u16])
}
/// `s.efcon(e)`
fn efcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,e:GetHandler())
    let p = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&cfilter), p, LOCATION_ONFIELD, 0, 1, h.into())
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=Duel.GetMatchingGroup(Card.IsType,c:GetControler(),LOCATION_GRAVE,0,nil,TYPE_FIELD)
    let p = card::get_controler(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_type(d, x, TYPE_FIELD)), p, LOCATION_GRAVE, 0, NO_EX);
    // return g:GetClassCount(Card.GetCode)*500
    group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64) as i64 * 500
}
/// `s.attg(e,c)`
fn attg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsCode(id)
    card::is_code(d, c, &[ID])
}
