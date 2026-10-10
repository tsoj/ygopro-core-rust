//! Trickstar Light Arena
use super::prelude::*;

pub const ID: u32 = 63492244;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRICKSTAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // lock
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let ec = group::get_first(d, &eg).nil();
    // return ec:IsSetCard(SET_TRICKSTAR) and ec:IsLinkSummoned() and ec:IsSummonPlayer(tp)
    card::is_set_card(d, ec, &[SET_TRICKSTAR as u16]) && card::is_link_summoned(d, ec) && card::is_summon_player(d, ec, a.tp)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsControler(tp) and c:IsSetCard(SET_TRICKSTAR)
    //     and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_location(d, c, LOCATION_GRAVE)
        && card::is_controler(d, c, tp)
        && card::is_set_card(d, c, &[SET_TRICKSTAR as u16])
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
/// (the Lua `c` in `mg:IsExists(s.spfilter,1,c,e,tp)` is the global nil here)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local mg=eg:GetFirst():GetMaterial()
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    let mg = card::get_material(d, first);
    // if chkc then return mg:IsContains(chkc) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&mg, chkc) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and mg:IsExists(s.spfilter,1,c,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && group::is_exists(d, &mg, &|d, x| spfilter(d, x, a.e, a.tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=mg:FilterSelect(tp,s.spfilter,1,1,nil,e,tp)
    let g = group::filter_select(d, &mg, a.tp, &|d, x| spfilter(d, x, a.e, a.tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:GetSequence()<5
    card::is_facedown(d, c) && card::get_sequence(d, c) < 5
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_SZONE) and s.cfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_SZONE) && cfilter(d, chkc);
    }
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.cfilter,tp,0,LOCATION_SZONE,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&cfilter), a.tp, 0, LOCATION_SZONE, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // Duel.SelectTarget(tp,s.cfilter,tp,0,LOCATION_SZONE,1,1,e:GetHandler())
    duel::select_target(d, a.tp, Some(&cfilter), a.tp, 0, LOCATION_SZONE, 1, 1, false, h.into());
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsFacedown() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_facedown(d, tc.nil()) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        let tc = tc.nil();
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc);
        // e:SetLabelObject(tc)
        effect::set_label_object(d, a.e, Value::card(Some(tc)));
        // c:ResetFlagEffect(id)
        card::reset_flag_effect(d, c, ID);
        // tc:ResetFlagEffect(id)
        card::reset_flag_effect(d, tc, ID);
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c) as i64;
        // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,fid)
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid, 0);
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,fid)
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid, 0);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_OWNER_RELATE)
        effect::set_property(d, e1, EFFECT_FLAG_OWNER_RELATE, 0);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::card(Some(tc)));
        // e1:SetCondition(s.rcon)
        effect::set_condition(d, e1, Some(condition(rcon)));
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // End of e1
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // e2:SetLabel(fid)
        effect::set_label(d, e2, &[fid]);
        // e2:SetLabelObject(e1)
        effect::set_label_object(d, e2, Value::effect(Some(e1)));
        // e2:SetCondition(s.rstcon)
        effect::set_condition(d, e2, Some(condition(rstcon)));
        // e2:SetOperation(s.rstop)
        effect::set_operation(d, e2, Some(operation(rstop)));
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        // return to hand
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e3:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
        // e3:SetCountLimit(1)
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        // e3:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
        // e3:SetLabel(fid)
        effect::set_label(d, e3, &[fid]);
        // e3:SetLabelObject(tc)
        effect::set_label_object(d, e3, Value::card(Some(tc)));
        // e3:SetCondition(s.agcon)
        effect::set_condition(d, e3, Some(condition(agcon)));
        // e3:SetOperation(s.agop)
        effect::set_operation(d, e3, Some(operation(agop)));
        // Duel.RegisterEffect(e3,tc:GetControler())
        let p = card::get_controler(d, tc);
        duel::register_effect(d, e3, p);
        // activate check
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e4:SetCode(EVENT_CHAINING)
        effect::set_code(d, e4, EVENT_CHAINING);
        // e4:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
        // e4:SetLabel(fid)
        effect::set_label(d, e4, &[fid]);
        // e4:SetLabelObject(e3)
        effect::set_label_object(d, e4, Value::effect(Some(e3)));
        // e4:SetOperation(s.rstop2)
        effect::set_operation(d, e4, Some(operation(rstop2)));
        // Duel.RegisterEffect(e4,tp)
        duel::register_effect(d, e4, a.tp);
    }
}
/// `s.rcon(e)`
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetOwner():IsHasCardTarget(e:GetHandler()) and e:GetHandler():GetFlagEffect(id)~=0
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_has_card_target(d, owner, h) && card::get_flag_effect(d, h, ID) != 0
}
/// `s.rstcon(e,tp,eg,ep,ev,re,r,rp)`
fn rstcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let tc = effect::get_label_object(d, e1).as_card().nil();
    let lbl = effect::get_label(d, a.e);
    // if tc:GetFlagEffectLabel(id)==e:GetLabel() and c:GetFlagEffectLabel(id)==e:GetLabel() then
    if card::get_flag_effect_label(d, tc, ID).first().copied() == Some(lbl) && card::get_flag_effect_label(d, c, ID).first().copied() == Some(lbl) {
        // return not c:IsDisabled()
        !card::is_disabled(d, c)
    } else {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        false
    }
}
/// `s.rstop(e,tp,eg,ep,ev,re,r,rp)`
fn rstop(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, a.e).as_effect().nil();
    // te:Reset()
    effect::reset(d, te);
    // Duel.HintSelection(Group.FromCards(e:GetHandler()))
    let h = effect::get_handler(d, a.e).nil();
    duel::hint_selection(d, CardSet::from(h), true);
}
/// `s.agcon(e,tp,eg,ep,ev,re,r,rp)`
fn agcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    let lbl = effect::get_label(d, a.e);
    // if tc:GetFlagEffectLabel(id)==e:GetLabel() and c:GetFlagEffectLabel(id)==e:GetLabel() then
    if card::get_flag_effect_label(d, tc, ID).first().copied() == Some(lbl) && card::get_flag_effect_label(d, c, ID).first().copied() == Some(lbl) {
        // return not c:IsDisabled()
        !card::is_disabled(d, c)
    } else {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        false
    }
}
/// `s.agop(e,tp,eg,ep,ev,re,r,rp)`
fn agop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.SendtoHand(tc,nil,REASON_RULE)
    duel::send_to_hand(d, tc, None, REASON_RULE, None);
}
/// `s.rstop2(e,tp,eg,ep,ev,re,r,rp)`
fn rstop2(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // local c=e:GetOwner()
    let c = effect::get_owner(d, a.e).nil();
    let lbl = effect::get_label(d, a.e);
    // if tc:GetFlagEffectLabel(id)~=e:GetLabel() or tc==c then return end
    if card::get_flag_effect_label(d, tc, ID).first().copied() != Some(lbl) || tc == c {
        return;
    }
    // c:CancelCardTarget(tc)
    card::cancel_card_target(d, c, tc);
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, a.e).as_effect();
    // tc:ResetFlagEffect(id)
    card::reset_flag_effect(d, tc, ID);
    // if te then te:Reset() end
    if let Some(te) = te {
        effect::reset(d, te);
    }
}
