//! N.As.H. Knight
//! Ｎ・Ａｓ・Ｈ Ｋｎｉｇｈｔ
use super::prelude::*;

pub const ID: u32 = 34876719;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,5,2)
    xyz::add_procedure(d, c, None, Some(5), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetCondition(s.indcon)
    effect::set_condition(d, e1, Some(condition(indcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach 1 "Number" monster and 1 other monster to itself
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.attcon)
    effect::set_condition(d, e2, Some(condition(attcon)));
    // e2:SetCost(Cost.DetachFromSelf(2,2,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(2), Some(cost::Count::N(2)), None)));
    // e2:SetTarget(s.atttg)
    effect::set_target(d, e2, Some(cost_target(atttg)));
    // e2:SetOperation(s.attop)
    effect::set_operation(d, e2, Some(operation(attop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_NUMBER),0,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NUMBER as u16])), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.attcon(e,tp,eg,ep,ev,re,r,rp)`
fn attcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
/// `s.attfilter1(c,e)`
fn attfilter1(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // local no=c.xyz_number
    let no = crate::card_info(d, c).xyz_number;
    // return no and no>=101 and no<=107 and c:IsSetCard(SET_NUMBER) and c:IsType(TYPE_XYZ) and not c:IsImmuneToEffect(e)
    match no {
        Some(no) if no >= 101 && no <= 107 => {
            card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_type(d, c, TYPE_XYZ) && !card::is_immune_to_effect(d, c, e)
        }
        _ => false,
    }
}
/// `s.atttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.attfilter1,tp,LOCATION_EXTRA,0,1,nil,e)
    //     and e:GetHandler():IsType(TYPE_XYZ) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| attfilter1(d, c, a.e)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX)
            && card::is_type(d, effect::get_handler(d, a.e).nil(), TYPE_XYZ);
    }
    true
}
/// `s.attfilter2(c,e)`
fn attfilter2(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and not c:IsType(TYPE_TOKEN) and not c:IsImmuneToEffect(e)
    card::is_faceup(d, c) && !card::is_type(d, c, TYPE_TOKEN) && !card::is_immune_to_effect(d, c, e)
}
/// `s.attop(e,tp,eg,ep,ev,re,r,rp)`
fn attop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
    // local exg=Duel.SelectMatchingCard(tp,s.attfilter1,tp,LOCATION_EXTRA,0,1,1,nil,e)
    let exg = duel::select_matching_card(d, a.tp, Some(&|d, x| attfilter1(d, x, a.e)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #exg==0 then return end
    if exg.is_empty() {
        return;
    }
    // Duel.Overlay(c,exg)
    duel::overlay(d, c, exg, false);
    // Attach 1 other face-up monster
    // local g=Duel.GetMatchingGroup(s.attfilter2,tp,LOCATION_MZONE,LOCATION_MZONE,c,e)
    let g = duel::get_matching_group(d, Some(&|d, x| attfilter2(d, x, a.e)), a.tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
        // local tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel);
        // if not tc then return end
        let Some(tc) = tc else {
            return;
        };
        // Duel.HintSelection(tc,true)
        duel::hint_selection(d, tc, true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Overlay(c,tc,true)
        duel::overlay(d, c, tc, true);
    }
}
