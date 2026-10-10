//! Future Fusion
use super::prelude::*;

pub const ID: u32 = 77565204;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.reg)
    effect::set_cost(d, e1, Some(cost_target(reg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Turn 1
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.tgcon)
    effect::set_condition(d, e2, Some(condition(tgcon)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Turn 2
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.proccon)
    effect::set_condition(d, e3, Some(condition(proccon)));
    // e3:SetOperation(s.procop)
    effect::set_operation(d, e3, Some(operation(procop)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Destroy2
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e5, EVENT_LEAVE_FIELD);
    // e5:SetCondition(s.descon2)
    effect::set_condition(d, e5, Some(condition(descon2)));
    // e5:SetOperation(s.desop2)
    effect::set_operation(d, e5, Some(operation(desop2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.reg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:SetTurnCounter(0)
    card::set_turn_counter(d, c, 0);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE_START|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE_START | PHASE_STANDBY);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // c:CreateEffectRelation(e1)
    card::create_effect_relation(d, c, e1);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetTurnCounter()
    let mut ct = card::get_turn_counter(d, c);
    // if not c:IsRelateToEffect(e) or ct>=2 then
    if !card::is_relate_to_effect(d, c, a.e) || ct >= 2 {
        // c:SetTurnCounter(0)
        card::set_turn_counter(d, c, 0);
        // e:Reset()
        effect::reset(d, a.e);
        return;
    }
    // if Duel.IsTurnPlayer(1-tp) then return end
    if duel::is_turn_player(d, 1 - a.tp) {
        return;
    }
    // ct=ct+1
    ct += 1;
    // c:SetTurnCounter(ct)
    card::set_turn_counter(d, c, ct as u16);
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and e:GetHandler():GetTurnCounter()==1
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, a.tp) && card::get_turn_counter(d, h) == 1
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.filter1(c,e)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsMonster() and c:IsAbleToGrave() and not c:IsImmuneToEffect(e)
    card::is_monster(d, c) && card::is_able_to_grave(d, c, None) && !card::is_immune_to_effect(d, c, e)
}
/// `s.filter2(c,m)`
fn filter2(d: &mut Duel, c: CardId, m: &CardSet) -> bool {
    // return c:IsFusionSummonableCard() and c:CheckFusionMaterial(m)
    card::is_fusion_summonable_card(d, c, 0) && card::check_fusion_material(d, c, Some(m), None, None)
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local mg=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_DECK,0,nil,e)
    let mg = duel::get_matching_group(d, Some(&|d, x| filter1(d, x, e)), tp, LOCATION_DECK, 0, NO_EX);
    // local sg=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_EXTRA,0,nil,mg)
    let sg = duel::get_matching_group(d, Some(&|d, x| filter2(d, x, &mg)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local tg=sg:Select(tp,1,1,nil)
        let tg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
        // local tc=tg:GetFirst()
        let tc = group::get_first(d, &tg).nil();
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - tp, tc);
        // local code=tc:GetCode()
        let code = card::get_code(d, tc).first().copied().unwrap_or(0) as i64;
        // local mat=Duel.SelectFusionMaterial(tp,tc,mg)
        let mat = duel::select_fusion_material(d, tp, tc, &mg, None, None);
        // mat:KeepAlive()
        // Duel.SendtoGrave(mat,REASON_EFFECT)
        duel::send_to_grave(d, mat.clone(), REASON_EFFECT, None, None);
        // for mc in aux.Next(mat) do
        for mc in group::iter(d, &mat) {
            // mc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, mc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
        // e:SetLabel(code)
        effect::set_label(d, e, &[code]);
        // e:SetLabelObject(mat)
        effect::set_label_object(d, e, Value::Group(mat));
    }
}
/// `s.proccon(e,tp,eg,ep,ev,re,r,rp)`
fn proccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and e:GetHandler():GetTurnCounter()==2
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, a.tp) && card::get_turn_counter(d, h) == 2
}
/// `s.procfilter(c,code,e,tp)`
fn procfilter(d: &mut Duel, c: CardId, code: u32, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(code) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_FUSION,tp,false,false)
    card::is_code(d, c, &[code])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION, tp, false, false, None, None, None)
}
/// `s.procop(e,tp,eg,ep,ev,re,r,rp)`
fn procop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local code=e:GetLabelObject():GetLabel()
    let le = effect::get_label_object(d, e).as_effect().nil();
    let code = effect::get_label(d, le) as u32;
    // local mat=e:GetLabelObject():GetLabelObject()
    let mat = match effect::get_label_object(d, le) {
        Value::Group(g) => Some(g),
        _ => None,
    };
    // if not mat or #mat==0 then return end
    let mat = match mat {
        Some(m) if !m.is_empty() => m,
        _ => return,
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.procfilter,tp,LOCATION_EXTRA,0,1,1,nil,code,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| procfilter(d, x, code, e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else {
        return;
    };
    // tc:SetStatus(STATUS_FUTURE_FUSION,true)
    card::set_status(d, tc, STATUS_FUTURE_FUSION, true);
    // for mc in aux.Next(mat) do
    for mc in group::iter(d, &mat) {
        // if mc:GetFlagEffect(id)>0 then
        if card::get_flag_effect(d, mc, ID) > 0 {
            // mc:SetReason(REASON_EFFECT+REASON_FUSION+REASON_MATERIAL)
            card::set_reason(d, mc, REASON_EFFECT + REASON_FUSION + REASON_MATERIAL, false);
        }
    }
    // tc:SetMaterial(mat)
    card::set_material(d, tc, mat.clone());
    // Duel.SpecialSummon(tc,SUMMON_TYPE_FUSION,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, tc, SUMMON_TYPE_FUSION, tp, tp, false, false, POS_FACEUP as u8, None);
    // tc:CompleteProcedure()
    card::complete_procedure(d, tc);
    // tc:SetStatus(STATUS_FUTURE_FUSION,false)
    card::set_status(d, tc, STATUS_FUTURE_FUSION, false);
    // c:SetCardTarget(tc)
    card::set_card_target(d, c, tc);
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    // if tc and tc:IsLocation(LOCATION_MZONE) then
    if let Some(tc) = card::get_first_card_target(d, h) {
        if card::is_location(d, tc, LOCATION_MZONE) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// `s.descon2(e,tp,eg,ep,ev,re,r,rp)`
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let Some(tc) = card::get_first_card_target(d, h) else {
        return false;
    };
    // return tc and eg:IsContains(tc) and tc:IsReason(REASON_DESTROY)
    let eg = a.eg.clone().nil();
    group::is_contains(&eg, tc) && card::is_reason(d, tc, REASON_DESTROY)
}
/// `s.desop2(e,tp,eg,ep,ev,re,r,rp)`
fn desop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
