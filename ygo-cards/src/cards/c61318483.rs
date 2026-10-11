//! Ghostrick Jackfrost
use super::prelude::*;

pub const ID: u32 = 61318483;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GHOSTRICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    // e1:SetCondition(s.sumcon)
    effect::set_condition(d, e1, Some(condition(sumcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // turn set
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // pos
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_POSITION + CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.sfilter(c)`
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GHOSTRICK)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GHOSTRICK as u16])
}
/// `s.sumcon(e)`
fn sumcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.sfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(d, Some(&|d, c| sfilter(d, c)), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanTurnSet() and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::is_can_turn_set(d, c) && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET),0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,c,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Duel.ChangePosition(c,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, c, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    // return at:GetControler()~=tp and Duel.GetAttackTarget()==nil
    let at = duel::get_attacker(d).nil();
    card::get_controler(d, at) != a.tp && duel::get_attack_target(d).is_none()
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return at:IsCanTurnSet() and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE) end
    if a.chk == Some(0) {
        let at = at.nil();
        return card::is_can_turn_set(d, at)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,at,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, at.map(CardSet::from_card), 1, tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if at:IsControler(1-tp) and at:IsRelateToBattle() and at:IsFaceup() and Duel.ChangePosition(at,POS_FACEDOWN_DEFENSE)>0 then
    if card::is_controler(d, at, 1 - tp)
        && card::is_relate_to_battle(d, at)
        && card::is_faceup(d, at)
        && duel::change_position(d, at, POS_FACEDOWN_DEFENSE as u8, None, None, None, false) > 0
    {
        // if not c:IsRelateToEffect(e) then return end
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)~=0 then
        if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) != 0 {
            // Duel.ConfirmCards(1-tp,c)
            duel::confirm_cards(d, 1 - tp, c);
        }
    }
}
