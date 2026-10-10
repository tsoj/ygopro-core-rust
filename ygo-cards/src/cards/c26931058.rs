//! Formation Union
use super::prelude::*;

pub const ID: u32 = 26931058;

pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_UNION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.unioneqfilter(c,tp)`
fn unioneqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_UNION) and c:IsFaceup() and Duel.IsExistingTarget(s.eqfilter,tp,LOCATION_MZONE,0,1,c,c)
    card::is_type(d, c, TYPE_UNION) && card::is_faceup(d, c) && duel::is_existing_target(d, Some(&|d, x| eqfilter(d, x, c)), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.eqfilter(c,ec)`
fn eqfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return ec:CheckUnionTarget(c) and aux.CheckUnionEquip(ec,c) and c:IsFaceup()
    card::check_union_target(d, ec, c) && aux::check_union_equip(d, ec, c) && card::is_faceup(d, c)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsHasEffect(EFFECT_UNION_STATUS) and c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK)
    !card::is_has_effect(d, c, EFFECT_UNION_STATUS, None).is_empty()
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_ATTACK as u8), None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return e:GetLabel()==2 and chkc:IsControler(tp) and chkc:IsLocation(LOCATION_STZONE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return effect::get_label(d, a.e) == 2
            && card::is_controler(d, chkc, a.tp)
            && card::is_location(d, chkc, LOCATION_STZONE)
            && spfilter(d, chkc, a.e, a.tp);
    }
    // local b1=Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and Duel.IsExistingTarget(s.unioneqfilter,tp,LOCATION_MZONE,0,1,nil,tp)
    let b1 = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, x| unioneqfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_STZONE,0,1,nil,e,tp)
    let b2 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_STZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_EQUIP)
        effect::set_category(d, a.e, CATEGORY_EQUIP);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        // local ec=Duel.SelectTarget(tp,s.unioneqfilter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
        let g = duel::select_target(d, a.tp, Some(&|d, x| unioneqfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let ec = group::get_first(d, &g).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
        // Duel.SelectTarget(tp,s.eqfilter,tp,LOCATION_MZONE,0,1,1,ec,ec)
        duel::select_target(d, a.tp, Some(&|d, x| eqfilter(d, x, ec)), a.tp, LOCATION_MZONE, 0, 1, 1, false, ec.into());
        // e:SetLabelObject(ec)
        effect::set_label_object(d, a.e, Value::Card(ec));
        // Duel.SetOperationInfo(0,CATEGORY_EQUIP,ec,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(ec)), 1, a.tp, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_STZONE,0,1,1,nil,e,tp)
        let g = duel::select_target(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_STZONE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Equip 1 Union monster you control to 1 appropriate monster you control
        // local ec=e:GetLabelObject()
        let ec = effect::get_label_object(d, a.e).as_card().nil();
        // if not (ec:IsRelateToEffect(e) and ec:IsControler(tp)) then return end
        if !(card::is_relate_to_effect(d, ec, a.e) && card::is_controler(d, ec, a.tp)) {
            return;
        }
        // local tc=(Duel.GetTargetCards(e)-ec):GetFirst()
        let mut targets = duel::get_target_cards(d, a.e).nil();
        group::remove_card(&mut targets, &CardSet::from_card(ec));
        let tc = group::get_first(d, &targets);
        // if not (tc and tc:IsFaceup() and Duel.GetLocationCount(tp,LOCATION_SZONE)>0) then
        let ok = match tc {
            Some(t) => card::is_faceup(d, t) && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0,
            None => false,
        };
        if !ok {
            // Duel.SendtoGrave(ec,REASON_RULE,PLAYER_NONE,PLAYER_NONE)
            duel::send_to_grave(d, ec, REASON_RULE, Some(PLAYER_NONE as u8), Some(PLAYER_NONE as u8));
        } else if aux::check_union_equip(d, ec, tc.nil()) && duel::equip(d, a.tp, ec, tc.nil(), true, false) {
            // aux.SetUnionState(ec)
            aux::set_union_state(d, ec);
        }
    } else if op == 2 {
        // Special Summon 1 equipped Union Monster Card from your Spell & Trap Zone in Attack Position
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_ATTACK)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None);
        }
    }
}
