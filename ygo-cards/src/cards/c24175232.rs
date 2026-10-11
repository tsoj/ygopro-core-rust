//! ネフティスの蒼凰神
//! Cerulean Sacred Phoenix of Nephthys
use super::prelude::*;

pub const ID: u32 = 24175232;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEPHTHYS], listed_names: &[23459650], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);

    // spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_operation(d, e2, Some(operation(spr)));
    card::register_effect(d, c, e2, false);

    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_STANDBY);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsSetCard(SET_NEPHTHYS) and (c:IsFaceup() or not c:IsOnField())
    card::is_set_card(d, c, &[SET_NEPHTHYS as u16]) && (card::is_faceup(d, c) || !card::is_on_field(d, c))
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0
            && duel::is_existing_matching_card(d, Some(&desfilter), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 2, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, NO_EX);
    // local og=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let og = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
    // if #g*#og==0 then return end
    if g.is_empty() || og.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local sg=g:Select(tp,1,#og,nil)
    let sg = group::select(d, &g, a.tp, 1, og.len() as u16, false, NO_EX).nil();
    // local oc=Duel.Destroy(sg,REASON_EFFECT)
    let oc = duel::destroy(d, sg, REASON_EFFECT, None, None);
    // if oc==0 then return end
    if oc == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local sg2=og:Select(tp,oc,oc,nil)
    let sg2 = group::select(d, &og, a.tp, oc as u16, oc as u16, false, NO_EX).nil();
    // Duel.Destroy(sg2,REASON_EFFECT)
    duel::destroy(d, sg2, REASON_EFFECT, None, None);
}
fn spr(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if r & REASON_DESTROY==0 or r & REASON_BATTLE+REASON_EFFECT ==0 then return end
    if a.r & REASON_DESTROY == 0 || a.r & (REASON_BATTLE + REASON_EFFECT) == 0 {
        return;
    }
    // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_STANDBY) then
    if duel::is_turn_player(d, a.tp) && duel::is_phase(d, PHASE_STANDBY) {
        // e:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None);
        effect::set_label(d, a.e, &[turn as i64]);
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,0,2)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 0, 2, 0, 0);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 0, 1, 0, 0);
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and e:GetLabelObject():GetLabel()~=Duel.GetTurnCount()
    //     and tp==Duel.GetTurnPlayer() and c:GetFlagEffect(id)>0
    if !card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None) {
        return false;
    }
    let spr_e = effect::get_label_object(d, a.e).as_effect().nil();
    let label = effect::get_label(d, spr_e);
    if label == duel::get_turn_count(d, None) as i64 {
        return false;
    }
    a.tp == duel::get_turn_player(d) && card::get_flag_effect(d, c, ID) > 0
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // c:ResetFlagEffect(id)
    card::reset_flag_effect(d, c, ID);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,1,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 1, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
