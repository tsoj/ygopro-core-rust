//! Steelswarm Origin
//! force effect scripted by edo9300
use super::prelude::*;

pub const ID: u32 = 61888819;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LSWARM], ..ScriptInfo::DEFAULT };

/// `local ALL_EMZ=0x600060` (0x60 for your EMZs, 0x60<<16 for your opponent's EMZs)
const ALL_EMZ: u32 = 0x600060;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_LSWARM),2,2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_LSWARM as u16], sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // force mzone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_FORCE_MZONE)
    effect::set_code(d, e1, EFFECT_FORCE_MZONE);
    // e1:SetRange(LOCATION_EMZONE)
    effect::set_range(d, e1, LOCATION_EMZONE);
    // e1:SetTargetRange(LOCATION_EXTRA,LOCATION_EXTRA)
    effect::set_target_range(d, e1, LOCATION_EXTRA, LOCATION_EXTRA);
    // e1:SetValue(s.fmval)
    effect::set_value_fn(d, e1, lua_fn(fmval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot be target/indestructable
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.indcon)
    effect::set_condition(d, e3, Some(condition(indcon)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_BATTLE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // special summon
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e6, aux::stringid(ID, 0));
    // e6:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e6, CATEGORY_SPECIAL_SUMMON);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e6:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e6, EVENT_DESTROYED);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e6, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetCondition(s.spcon)
    effect::set_condition(d, e6, Some(condition(spcon)));
    // e6:SetTarget(s.sptg)
    effect::set_target(d, e6, Some(cost_target(sptg)));
    // e6:SetOperation(s.spop)
    effect::set_operation(d, e6, Some(operation(spop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.fmval(e,c,fp,rp,r)`
fn fmval(d: &mut Duel, a: Args) -> i64 {
    // return e:GetHandler():GetLinkedZone(rp)|ALL_EMZ
    let h = effect::get_handler(d, a.e(0)).nil();
    (card::get_linked_zone(d, h, Some(a.p(3))) | ALL_EMZ) as i64
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return #(e:GetHandler():GetLinkedGroup():Filter(Card.IsMonster,nil))>0
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_linked_group(d, h);
    !group::filter(d, &g, &|d, c| card::is_monster(d, c), NO_EX).is_empty()
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &cfilter, 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsSetCard(SET_LSWARM) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_level_below(d, c, 4)
        && card::is_set_card(d, c, &[SET_LSWARM as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local ct=eg:FilterCount(s.cfilter,nil)
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &cfilter, NO_EX) as i32;
    // ft=math.min(ft,ct)
    ft = ft.min(ct);
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,ft,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, ft as u16, false, NO_EX).nil();
    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
    duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
}
