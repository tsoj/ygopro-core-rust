//! T.G. Recipro Dragonfly
use super::prelude::*;

pub const ID: u32 = 62560742;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TG], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro Summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,1)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 1, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Send 1 "T.G." Synchro monster to the GY and special summon the materials
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_TG) and c:IsType(TYPE_SYNCHRO)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TG as u16]) && card::is_type(d, c, TYPE_SYNCHRO)
}
/// `s.mgfilter(c,e,tp,sync)`
fn mgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sync: CardId) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_GRAVE)
    //     and (c:GetReason()&(REASON_MATERIAL+REASON_SYNCHRO))==(REASON_MATERIAL+REASON_SYNCHRO) and c:GetReasonCard()==sync and c:IsType(TYPE_SYNCHRO)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_GRAVE)
        && (card::get_reason(d, c) & (REASON_MATERIAL + REASON_SYNCHRO)) == (REASON_MATERIAL + REASON_SYNCHRO)
        && card::get_reason_card(d, c) == Some(sync)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local mg=tc:GetMaterial()
    let mg = card::get_material(d, tc);
    // local ct=#mg
    let ct = mg.len();
    // local sumtype=tc:GetSummonType()
    let sumtype = card::get_summon_type(d, tc);
    // if Duel.SendtoGrave(tc,REASON_EFFECT)~=0 and sumtype==SUMMON_TYPE_SYNCHRO
    //     and ct>0 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and ct<=Duel.GetLocationCount(tp,LOCATION_MZONE)
    //     and mg:FilterCount(aux.NecroValleyFilter(s.mgfilter),nil,e,tp,tc)==ct
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::send_to_grave(d, tc, REASON_EFFECT, None, None) != 0
        && sumtype == SUMMON_TYPE_SYNCHRO
        && ct > 0
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && (ct as i32) <= duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0
        && group::filter_count(d, &mg, &aux::necro_valley_filter(|d, x| mgfilter(d, x, a.e, tp, tc)), NO_EX) == ct
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(mg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, mg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
