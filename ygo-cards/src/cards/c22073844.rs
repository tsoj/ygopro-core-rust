//! Dogmatika Nexus
use super::prelude::*;

pub const ID: u32 = 22073844;
/// `local TYPE_FULL=TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ+TYPE_LINK`
const TYPE_FULL: u32 = TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ + TYPE_LINK;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(0)
    effect::set_value(d, e1, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon itsef from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy monsters and inflict damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e3, EVENT_BATTLE_START);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.rmfilter(c,e)
fn rmfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsMonster() and c:IsType(TYPE_FULL) and c:IsAbleToRemove() and c:IsCanBeEffectTarget(e)
    card::is_monster(d, c) && card::is_type(d, c, TYPE_FULL) && card::is_able_to_remove(d, c, None, None, None) && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, cc| rmfilter(d, cc, e)), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    //   and #g>=4 end --aux.SelectUnselectGroup(g,e,tp,4,4,s.spcheck,0) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
            && g.len() >= 4;
    }
    // local tg=g:Select(tp,4,4,nil) --aux.SelectUnselectGroup(g,e,tp,4,4,s.spcheck,1,tp,HINTMSG_REMOVE)
    let tg = group::select(d, &g, tp, 4, 4, false, NO_EX).nil();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tg,4,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(tg), 4, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tg=Duel.GetTargetCards(e)
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,true,false,POS_FACEUP)~=0 and #tg>0 then
    if card::is_relate_to_effect(d, c, e) && duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, true, false, POS_FACEUP as u8, None) != 0 {
        let tg = duel::get_target_cards(d, e).nil();
        if !tg.is_empty() {
            // c:CompleteProcedure()
            card::complete_procedure(d, c);
            // Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    // return bc and bc:IsSpecialSummoned()
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_special_summoned(d, bc),
        None => false,
    }
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAttackPos,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_attack_pos(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAttackPos,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_attack_pos(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)~=0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) != 0 {
        // local ct=Duel.GetOperatedGroup():FilterCount(Card.IsType,nil,TYPE_FULL)
        let operated = duel::get_operated_group(d);
        let ct = group::filter_count(d, &operated, &|d, c| card::is_type(d, c, TYPE_FULL), NO_EX);
        // if ct>0 then
        if ct > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,ct*800,REASON_EFFECT)
            duel::damage(d, 1 - tp, (ct * 800) as i64, REASON_EFFECT, false, None);
        }
    }
}
