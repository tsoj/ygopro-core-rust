//! Zefraath
use super::prelude::*;

pub const ID: u32 = 29432356;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ZEFRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your face-up Extra Deck) by Tributing all monsters you control, including at least 3 "Zefra" monsters
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetCondition(s.selfspcon)
    effect::set_condition(d, e0, Some(lua_fn(selfspcon)));
    // e0:SetOperation(s.selfspop)
    effect::set_operation(d, e0, Some(lua_fn(selfspop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Add 1 "Zefra" Pendulum Monster from your Deck to your Extra Deck, face-up, and if you do, change this card's Pendulum Scale ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.textg)
    effect::set_target(d, e1, Some(cost_target(textg)));
    // e1:SetOperation(s.texop)
    effect::set_operation(d, e1, Some(operation(texop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // After you Special Summon this card, you can conduct 1 Pendulum Summon of a "Zefra" monster(s) during your Main Phase this turn, in addition to your Pendulum Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetOperation(function(e,tp) Pendulum.GrantAdditionalPendulumSummon(...) end)
    effect::set_operation(d, e2, Some(operation(e2_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Zefra" monster from your Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e3, aux::stringid(ID, 4));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(s.deckspcost)
    effect::set_cost(d, e3, Some(cost_target(deckspcost)));
    // e3:SetTarget(s.decksptg)
    effect::set_target(d, e3, Some(cost_target(decksptg)));
    // e3:SetOperation(s.deckspop)
    effect::set_operation(d, e3, Some(operation(deckspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
    // local rg=Duel.GetReleaseGroup(tp)
    let rg = duel::get_release_group(d, tp, false, false, None);
    let _ = e;
    // return (#g>0 or #rg>0) and g:FilterCount(Card.IsReleasable,nil)==#g and g:FilterCount(Card.IsSetCard,nil,SET_ZEFRA)>=3
    (!g.is_empty() || !rg.is_empty())
        && group::filter_count(d, &g, &|d, cc| card::is_releasable(d, cc, None, None), NO_EX) == g.len()
        && group::filter_count(d, &g, &|d, cc| card::is_set_card(d, cc, &[SET_ZEFRA as u16]), NO_EX) >= 3
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn selfspop(d: &mut Duel, a: Args) {
    let tp = a.p(1);
    // local rg=Duel.GetReleaseGroup(tp)
    let rg = duel::get_release_group(d, tp, false, false, None);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
}
/// `s.texfilter(c,scale)`
fn texfilter(d: &mut Duel, c: CardId, scale: u32) -> bool {
    // return c:IsSetCard(SET_ZEFRA) and c:IsType(TYPE_PENDULUM) and not c:IsScale(scale)
    card::is_set_card(d, c, &[SET_ZEFRA as u16]) && card::is_type(d, c, TYPE_PENDULUM) && !card::is_scale(d, c, &[scale])
}
/// `s.textg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn textg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.texfilter,tp,LOCATION_DECK,0,1,nil,e:GetHandler():GetScale()) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        let scale = card::get_scale(d, c);
        return duel::is_existing_matching_card(d, Some(&|d, cc| texfilter(d, cc, scale)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.texop(e,tp,eg,ep,ev,re,r,rp)`
fn texop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,5))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 5));
    // local sc=Duel.SelectMatchingCard(tp,s.texfilter,tp,LOCATION_DECK,0,1,1,nil,c:GetScale()):GetFirst()
    let scale = card::get_scale(d, c);
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| texfilter(d, cc, scale)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SendtoExtraP(sc,tp,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_EXTRA) then
    if let Some(sc) = sc {
        if duel::send_to_extra_p(d, sc, Some(tp), REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_EXTRA) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LSCALE)
            effect::set_code(d, e1, EFFECT_CHANGE_LSCALE);
            // e1:SetValue(sc:GetLeftScale())
            let left = card::get_left_scale(d, sc);
            effect::set_value(d, e1, left as i64);
            // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CHANGE_RSCALE)
            effect::set_code(d, e2, EFFECT_CHANGE_RSCALE);
            // e2:SetValue(sc:GetRightScale())
            let right = card::get_right_scale(d, sc);
            effect::set_value(d, e2, right as i64);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
        }
    }
}
/// `function(e,tp) Pendulum.GrantAdditionalPendulumSummon(e:GetHandler(), ...) end`
fn e2_op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let filter: pendulum::PendFilter = std::sync::Arc::new(|d, cc, _e, _tp, _a, _b| card::is_set_card(d, cc, &[SET_ZEFRA as u16]));
    pendulum::grant_additional_pendulum_summon(d, c, Some(filter), a.tp, LOCATION_HAND | LOCATION_EXTRA, Some(aux::stringid(ID, 2)), Some(aux::stringid(ID, 3)), ID, None, None);
}
/// `s.deckspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn deckspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,1,false,aux.ReleaseCheckMMZ,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, None, 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,1,1,false,aux.ReleaseCheckMMZ,nil)
    let g = duel::select_release_group_cost(d, tp, None, 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.deckspfilter(c,e,tp)`
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ZEFRA) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ZEFRA as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.decksptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn decksptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        return duel::is_existing_matching_card(d, Some(&|d, cc| deckspfilter(d, cc, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.deckspop(e,tp,eg,ep,ev,re,r,rp)`
fn deckspop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| deckspfilter(d, cc, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
