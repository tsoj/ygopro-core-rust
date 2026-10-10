//! Wightmare
use super::prelude::*;

pub const ID: u32 = 22339232;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SKULL_SERVANT, 36021814, 40991587, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Name becomes "Skull Servant" while in the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetValue(CARD_SKULL_SERVANT)
    effect::set_value(d, e1, CARD_SKULL_SERVANT as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e2, Some(cost::self_discard()));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsCode(36021814,40991587) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_faceup(d, c) && card::is_code(d, c, &[36021814, 40991587]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // if not (chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and chkc:IsFaceup()) then return false end
        if !(card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && card::is_faceup(d, chkc)) {
            return false;
        }
        // local op=e:GetLabel()
        let op = effect::get_label(d, a.e);
        // return (op==1 and chkc:IsCode(CARD_SKULL_SERVANT,id)) or (op==2 and s.spfilter(chkc,e,tp))
        return (op == 1 && card::is_code(d, chkc, &[CARD_SKULL_SERVANT, ID])) || (op == 2 && spfilter(d, chkc, a.e, a.tp));
    }
    // local b1=Duel.IsExistingTarget(aux.FaceupFilter(Card.IsCode,CARD_SKULL_SERVANT,id),tp,LOCATION_REMOVED,0,1,nil)
    let b1 = duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_SKULL_SERVANT, ID])), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp)
    let b2 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).map_or(0, |x| x as i64);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsCode,CARD_SKULL_SERVANT,id),tp,LOCATION_REMOVED,0,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_SKULL_SERVANT, ID])), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, a.tp, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
        let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Return 1 of your banished "Skull Servant" or "Wightmare" to the GY
        // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_RETURN)
        duel::send_to_grave(d, CardSet::from_card(tc), REASON_EFFECT | REASON_RETURN, None, None);
    } else if op == 2 {
        // Special Summon 1 of your banished "The Lady in Wight" or "King of the Skull Servants"
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
