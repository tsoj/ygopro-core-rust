//! 王神鳥シムルグ
//! Simorgh, Bird of Sovereignty
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 72330894;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIMORGH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,2,3,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(3), Some(group_check(lcheck)), None, None);
    // Cannot be Link Material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Opponent cannot target Winged-Beast monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.indtg)
    effect::set_target(d, e2, Some(effect_card_fn(indtg)));
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destruction replacement
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetTarget(s.desreptg)
    effect::set_target(d, e3, Some(cost_target(desreptg)));
    // e3:SetOperation(s.desrepop)
    effect::set_operation(d, e3, Some(operation(desrepop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon 1 Winged-Beast monster from the Deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsRace,1,nil,RACE_WINGEDBEAST,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_race_ctx(d, c, RACE_WINGEDBEAST, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.indtg(e,c)`
fn indtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local oc=e:GetHandler()
    let oc = effect::get_handler(d, e).nil();
    // return c==oc or (c:IsRace(RACE_WINGEDBEAST) and oc:GetLinkedGroup():IsContains(c))
    c == oc || (card::is_race(d, c, RACE_WINGEDBEAST) && {
        let lg = card::get_linked_group(d, oc);
        group::is_contains(&lg, c)
    })
}
/// `s.repfilter(c,e)`
fn repfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SIMORGH) and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SIMORGH as u16])
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return c:IsReason(REASON_BATTLE) and c:IsOnField() and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.repfilter,tp,LOCATION_ONFIELD,0,1,c,e) end
    if a.chk == Some(0) {
        return card::is_reason(d, c, REASON_BATTLE)
            && card::is_on_field(d, c)
            && card::is_faceup(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, x| repfilter(d, x, e)), tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.repfilter,tp,LOCATION_ONFIELD,0,1,1,c,e)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| repfilter(d, x, e)), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
        // Duel.SetTargetCard(g)
        duel::set_target_card(d, g.clone());
        // g:GetFirst():SetStatus(STATUS_DESTROY_CONFIRMED,true)
        let first = group::get_first(d, &g).nil();
        card::set_status(d, first, STATUS_DESTROY_CONFIRMED, true);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.desrepop(e,tp,eg,ep,ev,re,r,rp)`
fn desrepop(d: &mut Duel, _a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // g:GetFirst():SetStatus(STATUS_DESTROY_CONFIRMED,false)
    let first = group::get_first(d, &g).nil();
    card::set_status(d, first, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(g,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, g, REASON_EFFECT | REASON_REPLACE, None, None);
}
/// `s.spfilter(c,e,tp,lv)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsRace(RACE_WINGEDBEAST) and c:IsLevelBelow(lv) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_WINGEDBEAST)
        && card::is_level_below(d, c, lv)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `local ct=5*2-Duel.GetMatchingGroupCount(nil,0,LOCATION_STZONE,LOCATION_STZONE,nil)`
fn spell_trap_count(d: &mut Duel) -> i64 {
    10 - duel::get_matching_group_count(d, None, 0, LOCATION_STZONE, LOCATION_STZONE, NO_EX) as i64
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let ct = spell_trap_count(d);
    // if chk==0 then return ct>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and
    //     Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp,ct) end
    if a.chk == Some(0) {
        return ct > 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, ct as u32)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local ct=5*2-Duel.GetMatchingGroupCount(nil,0,LOCATION_STZONE,LOCATION_STZONE,nil)
    let ct = spell_trap_count(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp,ct)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, ct as u32)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
