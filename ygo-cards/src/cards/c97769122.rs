//! バリアンズ・カオス・ドロー
//! Barian's Chaos Draw
use super::prelude::*;

pub const ID: u32 = 97769122;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NUMBER, SET_SEVENTH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DRAW)
    effect::set_code(d, e1, EVENT_DRAW);
    // e1:SetCondition(s.regcon)
    effect::set_condition(d, e1, Some(condition(regcon)));
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.activate)
    effect::set_operation(d, e2, Some(operation(activate)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.IsPhase(PHASE_DRAW) and c:IsReason(REASON_RULE)
    duel::is_phase(d, PHASE_DRAW) && card::is_reason(d, c, REASON_RULE)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_PUBLIC)
        effect::set_code(d, e1, EFFECT_PUBLIC);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_MAIN1)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_MAIN1, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_MAIN1,EFFECT_FLAG_CLIENT_HINT,1,0,66)
        card::register_flag_effect(
            d,
            c,
            ID,
            RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_MAIN1,
            EFFECT_FLAG_CLIENT_HINT,
            1,
            0,
            66,
        );
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_MAIN1)
    duel::is_phase(d, PHASE_MAIN1)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return e:GetHandler():GetFlagEffect(id)~=0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::get_flag_effect(d, h, ID) != 0;
    }
    true
}
/// `s.copfilter(c)`
fn copfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToGraveAsCost() and c:IsSetCard(SET_SEVENTH) and c:IsNormalSpell()
    //     and c:CheckActivateEffect(true,true,false)~=nil
    card::is_able_to_grave_as_cost(d, c)
        && card::is_set_card(d, c, &[SET_SEVENTH as u16])
        && card::is_normal_spell(d, c)
        && card::check_activate_effect(d, c, true, true, false).is_some()
}
/// `s.xyzfilter(c,tp,sg,g)`
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8, sg: &CardSet, g: &CardSet) -> bool {
    // return c:IsSetCard(SET_NUMBER) and c:IsXyzSummonable(sg,sg+g)
    //     and Duel.GetLocationCountFromEx(tp,tp,sg+g,c)>0
    let sum = group::union(sg, g);
    card::is_set_card(d, c, &[SET_NUMBER as u16])
        && card::is_xyz_summonable(d, c, Some(sg.clone()), Some(sum.clone()), 0, 0)
        && duel::get_location_count_from_ex(d, tp, Some(tp), (&sum).into(), duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
    // return Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_EXTRA,0,1,nil,tp,sg,g)
    (duel::is_existing_matching_card(d, Some(&|d, x| xyzfilter(d, x, tp, sg, &g)), tp, LOCATION_EXTRA, 0, 1, NO_EX), false)
}
/// `Card.IsCanBeSpecialSummoned(e,0,tp,false,false)` as a filter
fn can_be_sp(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// The number of zones the Chain Draw may summon into (`math.min(2,Duel.GetLocationCount(...))`, 1 with Blue-Eyes Spirit).
fn summon_zones(d: &mut Duel, tp: u8) -> i32 {
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0.min(2);
    if ft > 0 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    ft
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then
    if a.chkc().is_some() {
        // local te=e:GetLabelObject()
        let _te = effect::get_label_object(d, a.e);
        // return tg and tg(e,tp,eg,ep,ev,re,r,rp,0,chkc) end   (`tg` is not defined here: nil)
        return false;
    }
    // e:SetCategory(0)
    effect::set_category(d, a.e, 0);
    // local mg=Duel.GetMatchingGroup(Card.IsCanBeSpecialSummoned,tp,LOCATION_DECK,0,nil,e,0,tp,false,false)
    let mg = duel::get_matching_group(d, Some(&|d, x| can_be_sp(d, x, a.e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // local b1=e:GetLabel()==100 and Duel.IsExistingMatchingCard(s.copfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = effect::get_label(d, a.e) == 100 && duel::is_existing_matching_card(d, Some(&copfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local ft=math.min(2,Duel.GetLocationCount(tp,LOCATION_MZONE))
    // if ft>0 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    let ft = summon_zones(d, tp);
    // local b2=Duel.IsPlayerCanSpecialSummonCount(tp,2) and ft>0 and #mg>0 and aux.SelectUnselectGroup(mg,e,tp,1,ft,s.rescon,0)
    let b2 = duel::is_player_can_special_summon_count(d, tp, 2)
        && ft > 0
        && !mg.is_empty()
        && matches!(
            aux::select_unselect_group(d, &mg, Some(a.e), tp, Some(1), Some(ft as usize), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=b1 or b2
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return res
        return b1 || b2;
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil() as i64;
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.copfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&copfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).unwrap_or_default();
        // if not Duel.SendtoGrave(g,REASON_COST) then return end   (a number is always true)
        duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
        // local te=g:GetFirst():CheckActivateEffect(true,true,false)
        let tc = group::get_first(d, &g).nil();
        let te = card::check_activate_effect(d, tc, true, true, false).nil().0;
        // e:SetLabel(te:GetLabel())
        let te_label = effect::get_labels(d, te);
        effect::set_label(d, a.e, &te_label);
        // e:SetLabelObject(te:GetLabelObject())
        let te_obj = effect::get_label_object(d, te);
        effect::set_label_object(d, a.e, te_obj);
        // local tg=te:GetTarget()
        // if tg then tg(e,tp,eg,ep,ev,re,r,rp,1) end
        if let Some(tg) = effect::get_target(d, te) {
            tg.call(d, &chain_args(a, 1));
        }
        // te:SetLabel(e:GetLabel())
        let label = effect::get_labels(d, a.e);
        effect::set_label(d, te, &label);
        // te:SetLabelObject(e:GetLabelObject())
        let obj = effect::get_label_object(d, a.e);
        effect::set_label_object(d, te, obj);
        // e:SetLabelObject(te)
        effect::set_label_object(d, a.e, Value::Effect(te));
        // Duel.ClearOperationInfo(0)
        duel::clear_operation_info(d, 0);
    } else {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    }
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    true
}
/// The arguments `(e,tp,eg,ep,ev,re,r,rp, extra)` of an effect function, as Lua values.
fn chain_args(a: &EffArgs, extra: i64) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(extra),
    ]
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // local te=e:GetLabelObject()
        if let Value::Effect(te) = effect::get_label_object(d, a.e) {
            // e:SetLabel(te:GetLabel())
            let label = effect::get_labels(d, te);
            effect::set_label(d, a.e, &label);
            // e:SetLabelObject(te:GetLabelObject())
            let obj = effect::get_label_object(d, te);
            effect::set_label_object(d, a.e, obj);
            // local op=te:GetOperation()
            // if op then op(e,tp,eg,ep,ev,re,r,rp) end
            if let Some(opf) = effect::get_operation(d, te) {
                let mut args = chain_args(a, 0);
                args.truncate(8);
                opf.call(d, &args);
            }
            // te:SetLabel(e:GetLabel())
            let label = effect::get_labels(d, a.e);
            effect::set_label(d, te, &label);
            // te:SetLabelObject(e:GetLabelObject())
            let obj = effect::get_label_object(d, a.e);
            effect::set_label_object(d, te, obj);
        }
    } else if op == 2 {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
            return;
        }
        // local mg=Duel.GetMatchingGroup(Card.IsCanBeSpecialSummoned,tp,LOCATION_DECK,0,nil,e,0,tp,false,false)
        let mg = duel::get_matching_group(d, Some(&|d, x| can_be_sp(d, x, a.e, tp)), tp, LOCATION_DECK, 0, NO_EX);
        // if #mg==0 then return end
        if mg.is_empty() {
            return;
        }
        // local ft=math.min(2,Duel.GetLocationCount(tp,LOCATION_MZONE))
        // if ft>0 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        let ft = summon_zones(d, tp);
        // local sg=aux.SelectUnselectGroup(mg,e,tp,1,ft,s.rescon,1,tp,HINTMSG_SPSUMMON,s.rescon)
        let sg = aux::select_unselect_group(
            d,
            &mg,
            Some(a.e),
            tp,
            Some(1),
            Some(ft as usize),
            Some(&rescon),
            1,
            tp,
            Some(HINTMSG_SPSUMMON as u64),
            Some(&rescon),
            None,
            false,
        )
        .unwrap_or_default();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // for tc in aux.Next(sg) do
        for tc in group::iter(d, &sg) {
            // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
            if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                // Negate their effects
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_DISABLE)
                effect::set_code(d, e1, EFFECT_DISABLE);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
                // local e2=e1:Clone()
                let e2 = effect::clone(d, e1);
                // e2:SetCode(EFFECT_DISABLE_EFFECT)
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                // tc:RegisterEffect(e2)
                card::register_effect(d, tc, e2, false);
            }
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // local fg=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
        let fg = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local xyzg=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_EXTRA,0,1,1,nil,tp,sg,fg)
        let xyzg = duel::select_matching_card(
            d,
            tp,
            Some(&|d, x| xyzfilter(d, x, tp, &sg, &fg)),
            tp,
            LOCATION_EXTRA,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .unwrap_or_default();
        // if #xyzg>0 then
        if !xyzg.is_empty() {
            // Duel.XyzSummon(tp,xyzg:GetFirst(),sg)
            let x = group::get_first(d, &xyzg).nil();
            duel::xyz_summon(d, tp, x, Some(sg.clone()), None, 0, 0);
        }
    }
}
