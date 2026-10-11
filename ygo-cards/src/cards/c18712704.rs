//! Infernity Force
//! インフェルニティ・フォース
use super::prelude::*;

pub const ID: u32 = 18712704;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INFERNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local att=eg:GetFirst()
    let g = a.eg.clone().nil();
    let att = group::get_first(d, &g);
    // return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0 and att:IsFaceup() and att:IsSetCard(SET_INFERNITY)
    duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) == 0
        && card::is_faceup(d, att.nil())
        && card::is_set_card(d, att.nil(), &[SET_INFERNITY as u16])
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_INFERNITY) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_INFERNITY as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tg=Duel.GetAttacker()
    let tg = duel::get_attacker(d);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return tg:IsOnField() and tg:IsDestructable() and tg:IsCanBeEffectTarget(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let tg = tg.nil();
        return card::is_on_field(d, tg)
            && card::is_destructable(d, tg, None)
            && card::is_can_be_effect_target(d, tg, Some(a.e))
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.nil());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tg.nil())), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ex,sg=Duel.GetOperationInfo(0,CATEGORY_SPECIAL_SUMMON)
    let sg = duel::get_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON).and_then(|o| o.op_cards).nil();
    // local ex,dg=Duel.GetOperationInfo(0,CATEGORY_DESTROY)
    let dg = duel::get_operation_info(d, 0, CATEGORY_DESTROY).and_then(|o| o.op_cards).nil();
    // local sc=sg:GetFirst()
    let sc = group::get_first(d, &sg);
    // local dc=dg:GetFirst()
    let dc = group::get_first(d, &dg).nil();
    // if dc:IsRelateToEffect(e) and dc:IsAttackPos() then
    if card::is_relate_to_effect(d, dc, a.e) && card::is_attack_pos(d, dc) {
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
        // if sc:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) then
        let sc = sc.nil();
        if card::is_relate_to_effect(d, sc, a.e)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, None, None, None)
        {
            // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
