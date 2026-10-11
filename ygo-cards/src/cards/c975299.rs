//! B.E.F. Zelos
use super::prelude::*;

pub const ID: u32 = 975299;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BES],
    listed_names: &[66947414],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // ATK increase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_BES))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_BES as u16]))));
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // DEF increase
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Prevent destruction by opponent's effect
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    // e4:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_BES))
    effect::set_target(d, e4, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_BES as u16]))));
    // e4:SetValue(aux.indoval)
    effect::set_value_fn(d, e4, aux::indoval());
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Prevent effect target
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e5, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e5:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e5, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e5, LOCATION_MZONE, 0);
    // e5:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_BES))
    effect::set_target(d, e5, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_BES as u16]))));
    // e5:SetValue(aux.tgoval)
    effect::set_value_fn(d, e5, aux::tgoval());
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Special Summon from hand
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e6, aux::stringid(ID, 1));
    // e6:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e6, CATEGORY_SPECIAL_SUMMON);
    // e6:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e6, EFFECT_TYPE_IGNITION);
    // e6:SetRange(LOCATION_FZONE)
    effect::set_range(d, e6, LOCATION_FZONE);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetTarget(s.sptg)
    effect::set_target(d, e6, Some(cost_target(sptg)));
    // e6:SetOperation(s.spop)
    effect::set_operation(d, e6, Some(operation(spop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // Add counters
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e7, aux::stringid(ID, 2));
    // e7:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e7, CATEGORY_COUNTER);
    // e7:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e7:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e7, EVENT_SUMMON_SUCCESS);
    // e7:SetRange(LOCATION_FZONE)
    effect::set_range(d, e7, LOCATION_FZONE);
    // e7:SetCondition(s.ctcon)
    effect::set_condition(d, e7, Some(condition(ctcon)));
    // e7:SetTarget(s.cttg)
    effect::set_target(d, e7, Some(cost_target(cttg)));
    // e7:SetOperation(s.ctop)
    effect::set_operation(d, e7, Some(operation(ctop)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // local e8=e7:Clone()
    let e8 = effect::clone(d, e7);
    // e8:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e8, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(66947414) and c:IsAbleToHand()
    card::is_code(d, c, &[66947414]) && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_BES) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_BES as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let e = a.e;
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.ctfilter(c,tp)`
fn ctfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_BES) and c:IsControler(tp)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BES as u16]) && card::is_controler(d, c, tp)
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg and eg:IsExists(s.ctfilter,1,nil,tp)
    let tp = a.tp;
    match a.eg.clone() {
        None => false,
        Some(eg) => group::is_exists(d, &eg, &|d, c| ctfilter(d, c, tp), 1, NO_EX),
    }
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ec=eg:FilterCount(s.ctfilter,nil,tp)
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let ec = group::filter_count(d, &eg, &|d, c| ctfilter(d, c, tp), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,ec,0,0x1f)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, ec as u8, 0, 0x1f);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=eg:Filter(s.ctfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, x| ctfilter(d, x, tp), NO_EX);
    // local tc=g:GetFirst()
    let _tc = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // tc:AddCounter(0x1f,1)
        card::add_counter(d, tc, 0x1f, 1, false);
    }
}
