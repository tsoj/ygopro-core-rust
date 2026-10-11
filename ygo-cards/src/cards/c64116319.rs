//! Toon Harpie Lady
use super::prelude::*;

pub const ID: u32 = 64116319;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TOON_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot attack the turn it was summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.atklimit)
    effect::set_operation(d, e1, Some(operation(atklimit)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Can attack directly
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e4, EFFECT_DIRECT_ATTACK);
    // e4:SetCondition(s.dircon)
    effect::set_condition(d, e4, Some(condition(dircon)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Special summon itself from hand
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_HAND)
    effect::set_range(d, e5, LOCATION_HAND);
    // e5:SetCountLimit(1,id)
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    // e5:SetCondition(s.spcon)
    effect::set_condition(d, e5, Some(condition(spcon)));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.atklimit(e,tp,eg,ep,ev,re,r,rp)`
fn atklimit(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,15259703),tp,LOCATION_ONFIELD,0,1,nil)
    //     and not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_TOON),tp,0,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TOON_WORLD])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_TOON)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,15259703),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TOON_WORLD])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
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
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 and #g>0
    if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && !g.is_empty()
        // and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_TOON),tp,LOCATION_MZONE,0,1,c)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_type(d, x, TYPE_TOON)), tp, LOCATION_MZONE, 0, 1, c.into())
        // and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
