//! Traptrix Sera
use super::prelude::*;

pub const ID: u32 = 73639099;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRAPTRIX, SET_TRAP_HOLE, SET_HOLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link summon method
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 1, Some(1), None, None, None);
    // Unaffected by trap effects, continuous effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetCondition(s.immcon)
    effect::set_condition(d, e1, Some(condition(immcon)));
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon a "Traptrix" monster from deck, optional trigger effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Set 1 "Trap Hole" normal trap card from deck, optional trigger effect
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_SET)
    effect::set_category(d, e5, CATEGORY_SET);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e5:SetCode(EVENT_CHAINING)
    effect::set_code(d, e5, EVENT_CHAINING);
    // e5:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e5, 1, ID, 1, 0);
    // e5:SetCondition(s.setcon)
    effect::set_condition(d, e5, Some(condition(setcon)));
    // e5:SetTarget(s.settg)
    effect::set_target(d, e5, Some(cost_target(settg)));
    // e5:SetOperation(s.setop)
    effect::set_operation(d, e5, Some(operation(setop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`: link material of a non-link "Traptrix" monster
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_TRAPTRIX,lc,sumtype,tp) and not c:IsType(TYPE_LINK,lc,sumtype,tp)
    card::is_set_card_ctx(d, c, &[SET_TRAPTRIX as u16], lc, sumtype, Some(tp))
        && !card::is_type_ctx(d, c, TYPE_LINK, lc, sumtype, Some(tp))
}
/// `s.immcon(e)`: if this card was Link Summoned
fn immcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
/// `s.efilter(e,te)`: unaffected by trap effects
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsTrapEffect()
    effect::is_trap_effect(d, a.e(1))
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`: if a normal trap card is activated
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    // return re:GetActiveType()==TYPE_TRAP and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    effect::get_active_type(d, re) == TYPE_TRAP && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
/// `s.namefilter(c,cd)`
fn namefilter(d: &mut Duel, c: CardId, cd: u32) -> bool {
    // return c:IsCode(cd) and c:IsFaceup()
    card::is_code(d, c, &[cd]) && card::is_faceup(d, c)
}
/// `s.spfilter(c,e,tp)`: a "Traptrix" monster with a different name from the monsters you control
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_TRAPTRIX) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and not Duel.IsExistingMatchingCard(s.namefilter,tp,LOCATION_ONFIELD,0,1,nil,c:GetCode())
    if !(card::is_set_card(d, c, &[SET_TRAPTRIX as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)) {
        return false;
    }
    let code = card::get_code(d, c).first().copied().unwrap_or_default();
    !duel::is_existing_matching_card(d, Some(&|d, x| namefilter(d, x, code)), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`: a "Traptrix" monster effect activates, except this card
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let re = a.re.nil();
    // return rp==tp and re:IsMonsterEffect() and re:GetHandler():IsSetCard(SET_TRAPTRIX) and re:GetHandler()~=e:GetHandler()
    if a.rp != tp || !effect::is_monster_effect(d, re) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    card::is_set_card(d, rh, &[SET_TRAPTRIX as u16]) && Some(rh) != effect::get_handler(d, e)
}
/// `s.setfilter(c)`: a "Trap Hole" normal trap
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_TRAP_HOLE) or c:IsSetCard(SET_HOLE)) and c:IsNormalTrap() and c:IsSSetable()
    (card::is_set_card(d, c, &[SET_TRAP_HOLE as u16]) || card::is_set_card(d, c, &[SET_HOLE as u16]))
        && card::is_normal_trap(d, c)
        && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g:GetFirst())
        let tc = group::get_first(d, &g).nil();
        duel::s_set(d, tp, tc, None, true);
    }
}
