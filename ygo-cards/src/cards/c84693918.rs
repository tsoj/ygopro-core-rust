//! Solfachord Solfegia
//! ドレミコード・ソルフェージア
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 84693918;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_GRANSOLFACHORD, SET_SOLFACHORD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Negate a monster effect activated by your opponent, then destroy this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return not Duel.IsExistingMatchingCard(aux.NOT(aux.FaceupFilter(Card.IsSetCard,SET_SOLFACHORD)),tp,LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(d, e2, Some(condition(selfsp_cond)));
    // e2:SetTarget(s.selfsptg)
    effect::set_target(d, e2, Some(cost_target(selfsptg)));
    // e2:SetOperation(s.selfspop)
    effect::set_operation(d, e2, Some(operation(selfspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Solfachord" monster from your hand, except "Solfachord Solfeggia"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `Cost.SelfTribute(e,tp,eg,ep,ev,re,r,rp,chk)` called from the target with the same arguments
fn self_tribute_call(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    let args = [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(chk),
    ];
    cost::self_tribute().call(d, &args).truthy()
}
/// `function(e,tp) return not Duel.IsExistingMatchingCard(aux.NOT(aux.FaceupFilter(Card.IsSetCard,SET_SOLFACHORD)),tp,LOCATION_MZONE,0,1,nil) end`
fn selfsp_cond(d: &mut Duel, a: &EffArgs) -> bool {
    !duel::is_existing_matching_card(
        d,
        Some(&|d, c| !(card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SOLFACHORD as u16]))),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return rp==1-tp and re:IsMonsterEffect() and Duel.IsChainDisablable(ev) and not Duel.HasFlagEffect(tp,id)
    //     and Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)==LOCATION_MZONE
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_GRANSOLFACHORD),tp,LOCATION_MZONE,0,1,nil)
    if a.rp != 1 - tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    if !duel::is_chain_disablable(d, a.ev as u8) {
        return false;
    }
    if duel::get_flag_effect(d, tp, ID) != 0 {
        return false;
    }
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION]).first().map_or(0, |v| v.int());
    if loc != LOCATION_MZONE as i64 {
        return false;
    }
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GRANSOLFACHORD as u16])),
        tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not Duel.SelectEffectYesNo(tp,c,aux.Stringid(id,2)) then return end
    if !duel::select_effect_yes_no(d, tp, c, Some(aux::stringid(ID, 2))) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // if Duel.NegateEffect(ev) then
    if duel::negate_effect(d, a.ev as u8) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.spfilter(c,e,tp,exc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, exc: Option<CardId>) -> bool {
    // if not (c:IsSetCard(SET_SOLFACHORD) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_SOLFACHORD as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
    {
        return false;
    }
    // if c:IsLocation(LOCATION_HAND|LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_HAND | LOCATION_GRAVE) {
        // return Duel.GetMZoneCount(tp,exc)>0
        duel::get_m_zone_count(d, tp, exc.into(), None, None, None).0 > 0
    // elseif c:IsLocation(LOCATION_EXTRA) then
    } else if card::is_location(d, c, LOCATION_EXTRA) {
        // return c:IsFaceup() and Duel.GetLocationCountFromEx(tp,tp,exc,c)>0
        card::is_faceup(d, c) && duel::get_location_count_from_ex(d, tp, Some(tp), exc.into(), duel::ExtraCard::Card(c), None).0 > 0
    } else {
        // (the Lua function falls off the end: nil, false)
        false
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local cost_chk=Cost.SelfTribute(e,tp,eg,ep,ev,re,r,rp,0)
    let cost_chk = self_tribute_call(d, a, 0);
    // local exc=cost_chk and e:GetHandler() or nil
    let exc = if cost_chk { Some(effect::get_handler(d, a.e).nil()) } else { None };
    // local location=cost_chk and LOCATION_HAND|LOCATION_GRAVE|LOCATION_EXTRA or LOCATION_HAND
    let location = if cost_chk { LOCATION_HAND | LOCATION_GRAVE | LOCATION_EXTRA } else { LOCATION_HAND };
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,location,0,1,nil,e,tp,exc) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, exc)), tp, location, 0, 1, NO_EX);
    }
    // local hand_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    let hand_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    // if cost_chk and (not hand_chk or Duel.SelectYesNo(tp,aux.Stringid(id,3))) then
    if cost_chk && (!hand_chk || duel::select_yes_no(d, tp, aux::stringid(ID, 3))) {
        // Cost.SelfTribute(e,tp,eg,ep,ev,re,r,rp,1)
        self_tribute_call(d, a, 1);
        // e:SetLabel(LOCATION_HAND|LOCATION_GRAVE|LOCATION_EXTRA)
        effect::set_label(d, a.e, &[(LOCATION_HAND | LOCATION_GRAVE | LOCATION_EXTRA) as i64]);
    } else {
        // e:SetLabel(LOCATION_HAND)
        effect::set_label(d, a.e, &[LOCATION_HAND as i64]);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,e:GetLabel())
    let label = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, label as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,e:GetLabel(),0,1,1,nil,e,tp)
    let loc = effect::get_label(d, a.e) as u32;
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp, None))),
        tp,
        loc,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
