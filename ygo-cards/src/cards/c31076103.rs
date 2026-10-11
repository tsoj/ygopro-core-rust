//! The First Sarcophagus
use super::prelude::*;

pub const ID: u32 = 31076103;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[4081094, 78697395, ID, 25343280], ..ScriptInfo::DEFAULT };

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
    // Place "The Second Sarcophagus" or "The Third Sarcophagus" in your Spell/Trap Zone
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) and e:GetHandler():GetFlagEffect(id)<2 end)
    effect::set_condition(d, e2, Some(condition(plcon)));
    // e2:SetTarget(s.pltg)
    effect::set_target(d, e2, Some(cost_target(pltg)));
    // e2:SetOperation(s.plop)
    effect::set_operation(d, e2, Some(operation(plop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send them to the GY if any leave the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.tgcon)
    effect::set_condition(d, e3, Some(condition(tgcon)));
    // e3:SetOperation(s.tgop)
    effect::set_operation(d, e3, Some(operation(tgop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetOperation(s.tgop)
    effect::set_operation(d, e4, Some(operation(tgop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) and e:GetHandler():GetFlagEffect(id)<2 end`
fn plcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && card::get_flag_effect(d, effect::get_handler(d, a.e).nil(), ID) < 2
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.plfilter(c,code)`
fn plfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and not c:IsForbidden()
    card::is_code(d, c, &[code]) && !card::is_forbidden(d, c)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(25343280) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[25343280]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // local code=not c:HasFlagEffect(id) and 4081094 or 78697395
    let code: u32 = if !card::has_flag_effect(d, c.nil(), ID, None) { 4081094 } else { 78697395 };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local pc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,code):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d: &mut Duel, x| plfilter(d, x, code)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let pc = group::get_first(d, &sel);
    // if not (pc and Duel.MoveToField(pc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)) then return end
    let Some(pc) = pc else {
        return;
    };
    if !duel::move_to_field(d, pc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
        return;
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
    card::register_flag_effect(d, c.nil(), ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,id),tp,LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,4081094),tp,LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,78697395),tp,LOCATION_ONFIELD,0,1,nil) then
    if duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d: &mut Duel, x| card::is_code(d, x, &[ID]))), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d: &mut Duel, x| card::is_code(d, x, &[4081094]))), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d: &mut Duel, x| card::is_code(d, x, &[78697395]))), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
    {
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,id,4081094,78697395),tp,LOCATION_ONFIELD,0,nil)
        let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d: &mut Duel, x| card::is_code(d, x, &[ID, 4081094, 78697395]))), a.tp, LOCATION_ONFIELD, 0, NO_EX);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SendtoGrave(g,REASON_EFFECT)~=#g or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        let n = g.len();
        if duel::send_to_grave(d, g, REASON_EFFECT, None, None) != n as i32
            || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&|d: &mut Duel, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if sc and Duel.SpecialSummon(sc,0,tp,tp,true,false,POS_FACEUP)>0 then sc:CompleteProcedure() end
        if let Some(sc) = sc {
            if duel::special_summon(d, CardSet::from_card(sc), 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) > 0 {
                card::complete_procedure(d, sc);
            }
        }
    }
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(aux.FaceupFilter(Card.IsCode,4081094,78697395),1,nil)
    group::is_exists(d, &a.eg.clone().nil(), &aux::faceup_filter(|d: &mut Duel, x| card::is_code(d, x, &[4081094, 78697395])), 1, NO_EX)
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,4081094,78697395),tp,LOCATION_ONFIELD,0,nil):Merge(e:GetHandler())
    let mut g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d: &mut Duel, x| card::is_code(d, x, &[4081094, 78697395]))), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    group::add_card(&mut g, &CardSet::from_card(effect::get_handler(d, a.e).nil()));
    // Duel.SendtoGrave(g,REASON_EFFECT)
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
}
