//! Slifer the Sky Dragon
use super::prelude::*;

pub const ID: u32 = 10000020;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 3 tribute
    // local e1=aux.AddNormalSummonProcedure(c,true,false,3,3)
    let _e1 = aux::add_normal_summon_procedure(d, c, true, false, Some(3), Some(3), None, None, None, None);
    // local e2=aux.AddNormalSetProcedure(c)
    let _e2 = aux::add_normal_set_procedure(d, c, false, false, None, None, None, None, None, None);
    // summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CANNOT_DISABLE_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_DISABLE_SUMMON);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // summon success
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    // e4:SetOperation(s.sumsuc)
    effect::set_operation(d, e4, Some(operation(sumsuc)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // to grave
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e5, CATEGORY_TOGRAVE);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e5, EVENT_PHASE + PHASE_END);
    // e5:SetCondition(s.tgcon)
    effect::set_condition(d, e5, Some(condition(tgcon)));
    // e5:SetTarget(s.tgtg)
    effect::set_target(d, e5, Some(cost_target(tgtg)));
    // e5:SetOperation(s.tgop)
    effect::set_operation(d, e5, Some(operation(tgop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // atk/def
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e6, EFFECT_UPDATE_ATTACK);
    // e6:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e6, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetValue(s.adval)
    effect::set_value_fn(d, e6, effect_card_value(adval));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=e6:Clone()
    let e7 = effect::clone(d, e6);
    // e7:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e7, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // atkdown
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e8, aux::stringid(ID, 1));
    // e8:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DESTROY)
    effect::set_category(d, e8, CATEGORY_ATKCHANGE + CATEGORY_DESTROY);
    // e8:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e8, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e8:SetRange(LOCATION_MZONE)
    effect::set_range(d, e8, LOCATION_MZONE);
    // e8:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e8, EVENT_SUMMON_SUCCESS);
    // e8:SetCondition(s.atkcon)
    effect::set_condition(d, e8, Some(condition(atkcon)));
    // e8:SetTarget(s.atktg)
    effect::set_target(d, e8, Some(cost_target(atktg)));
    // e8:SetOperation(s.atkop)
    effect::set_operation(d, e8, Some(operation(atkop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
    // local e9=e8:Clone()
    let e9 = effect::clone(d, e8);
    // e9:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e9, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e9)
    card::register_effect(d, c, e9, false);
}
/// `s.sumsuc(e,tp,eg,ep,ev,re,r,rp)`
fn sumsuc(d: &mut Duel, _a: &EffArgs) {
    // Duel.SetChainLimitTillChainEnd(aux.FALSE)
    duel::set_chain_limit_till_chain_end(d, aux_false());
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSpecialSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_special_summoned(d, h)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then Duel.SendtoGrave(c,REASON_EFFECT) end
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
    }
}
/// `s.adval(e,c)`
fn adval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetFieldGroupCount(c:GetControler(),LOCATION_HAND,0)*1000
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, LOCATION_HAND, 0) as i64 * 1000
}
/// `s.atkfilter(c,tp)`
fn atkfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and c:IsPosition(POS_FACEUP_ATTACK)
    card::is_controler(d, c, 1 - tp) && card::is_position(d, c, POS_FACEUP_ATTACK)
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.atkfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    !eg.contains(h) && group::is_exists(d, &eg, &|d, c| atkfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_effect(d, h, a.e);
    }
    // Duel.SetTargetCard(eg:Filter(s.atkfilter,nil,tp))
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| atkfilter(d, c, a.tp), NO_EX);
    duel::set_target_card(d, g);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e):Match(Card.IsFaceup,nil)
    let mut g = duel::get_target_cards(d, a.e).unwrap_or_default();
    group::match_(d, &mut g, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local dg=Group.CreateGroup()
    let mut dg = CardSet::new();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local preatk=tc:GetAttack()
        let preatk = card::get_attack(d, tc);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-2000)
        effect::set_value(d, e1, -2000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // if preatk~=0 and tc:GetAttack()==0 then dg:AddCard(tc) end
        if preatk != 0 && card::get_attack(d, tc) == 0 {
            group::add_card(&mut dg, &CardSet::from_card(tc));
        }
    }
    // if #dg==0 then return end
    if dg.is_empty() {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Destroy(dg,REASON_EFFECT)
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
