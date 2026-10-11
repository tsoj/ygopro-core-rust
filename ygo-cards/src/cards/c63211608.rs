//! Genex Ally Reliever
use super::prelude::*;

pub const ID: u32 = 63211608;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // counter
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetOperation(s.chop1)
    effect::set_operation(d, e1, Some(operation(chop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_NEGATED)
    effect::set_code(d, e2, EVENT_CHAIN_NEGATED);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetOperation(s.chop2)
    effect::set_operation(d, e2, Some(operation(chop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e1:SetLabelObject(e2)
    effect::set_label_object(d, e1, Value::Effect(e2));
    // special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e3, EVENT_CHAIN_END);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCondition(s.sumcon)
    effect::set_condition(d, e3, Some(condition(sumcon)));
    // e3:SetTarget(s.sumtg)
    effect::set_target(d, e3, Some(cost_target(sumtg)));
    // e3:SetOperation(s.sumop)
    effect::set_operation(d, e3, Some(operation(sumop)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.chop1(e,tp,eg,ep,ev,re,r,rp)`
fn chop1(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():SetLabel(0)
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, e2, &[0]);
}
/// `s.chop2(e,tp,eg,ep,ev,re,r,rp)`
fn chop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if rp==tp or (not re:IsMonsterEffect() and not re:IsHasType(EFFECT_TYPE_ACTIVATE)) then return end
    if a.rp == tp || (!effect::is_monster_effect(d, a.re.nil()) && !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)) {
        return;
    }
    // local de,dp=Duel.GetChainInfo(ev,CHAININFO_DISABLE_REASON,CHAININFO_DISABLE_PLAYER)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_DISABLE_REASON, CHAININFO_DISABLE_PLAYER]);
    let dp_is_tp = matches!(info[1], Value::Int(x) if x == tp as i64);
    // if dp==tp then
    if dp_is_tp {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    }
}
/// `s.sumcon(e,tp,eg,ep,ev,re,r,rp)`
fn sumcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabelObject():GetLabel()~=0
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, e2) != 0
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, c.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
