//! Muckraker From the Underworld
use super::prelude::*;

pub const ID: u32 = 71607202;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // --Cannot be used as Link Material the turn it is Link Summoned
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
    effect::set_code(d, e0, EFFECT_CANNOT_BE_LINK_MATERIAL);
    // e0:SetCondition(s.cannotlinkcon)
    effect::set_condition(d, e0, Some(condition(cannotlinkcon)));
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // --Destruction replacement by Tributing 1 Fiend monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // e1:SetOperation(s.repop)
    effect::set_operation(d, e1, Some(operation(repop)));
    // e1:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e1, lua_fn(repvalue));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Special Summon 1 Fiend monster from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_HANDES)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_HANDES);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cannotlinkcon(e)`
fn cannotlinkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsStatus(STATUS_SPSUMMON_TURN) and c:IsLinkSummoned()
    card::is_status(d, c, STATUS_SPSUMMON_TURN) && card::is_link_summoned(d, c)
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_MZONE) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    //     and not c:IsReason(REASON_REPLACE)
    card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.trbfilter(c,e)`
fn trbfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsReleasableByEffect() and not c:IsImmuneToEffect(e)
    //     and not c:IsStatus(STATUS_DESTROY_CONFIRMED|STATUS_BATTLE_DESTROYED)
    card::is_race(d, c, RACE_FIEND)
        && card::is_releasable_by_effect(d, c, None, None)
        && !card::is_immune_to_effect(d, c, e)
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED | STATUS_BATTLE_DESTROYED)
}
/// `e:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end)`
fn repvalue(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let c = a.c(1);
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local eg = the event group (nil is an error in Lua when indexed)
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp)
    //     and Duel.IsExistingMatchingCard(s.trbfilter,tp,LOCATION_MZONE,0,1,eg,e) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| repfilter(d, c, a.tp), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| trbfilter(d, c, a.e)), a.tp, LOCATION_MZONE, 0, 1, (&eg).into());
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, a.tp, h, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESREPLACE as u64);
        // local tg=Duel.SelectMatchingCard(tp,s.trbfilter,tp,LOCATION_MZONE,0,1,1,eg,e)
        let tg = duel::select_matching_card(d, a.tp, Some(&|d, c| trbfilter(d, c, a.e)), a.tp, LOCATION_MZONE, 0, 1, 1, false, (&eg).into()).nil();
        // Duel.SetTargetCard(tg)
        duel::set_target_card(d, tg);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,1-tp,id)
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // Duel.Release(tc,REASON_EFFECT|REASON_REPLACE)
    duel::release(d, tc, REASON_EFFECT | REASON_REPLACE, None);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_FIEND) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_FIEND) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil,REASON_EFFECT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)>0 and tc:IsRelateToEffect(e) then
    if duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0 && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(tc.nil()), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    // --Cannot Special Summon monsters, except Fiend monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsRace(RACE_FIEND) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_FIEND))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
