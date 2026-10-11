//! Eclipse Twins
use super::prelude::*;

pub const ID: u32 = 45852939;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure: 2 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // For this card's Xyz Summon, you can treat Rank 4 monsters you control as Level 4 monsters for material
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e0:SetCode(EFFECT_XYZ_LEVEL)
    effect::set_code(d, e0, EFFECT_XYZ_LEVEL);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e0, LOCATION_MZONE, 0);
    // e0:SetTarget(function(e,c) return c:IsRank(4) end)
    effect::set_target(d, e0, Some(effect_card_fn(|d, _e, c| card::is_rank(d, c, &[4]))));
    // e0:SetValue(function(e,_,rc) return rc==e:GetHandler() and 4 or 0 end)
    effect::set_value_fn(d, e0, lua_fn(xyz_level_value));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // This card can make up to 2 attacks on monsters during each Battle Phase this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function() return Duel.IsAbleToEnterBP() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_able_to_enter_bp(d))));
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Rank 4 or lower Xyz monster from the GY and attach another to it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// e0 value: function(e,_,rc) return rc==e:GetHandler() and 4 or 0 end
fn xyz_level_value(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let rc = a.card(2);
    let h = effect::get_handler(d, e).nil();
    if rc == Some(h) { 4 } else { 0 }
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // return c:GetEffectCount(EFFECT_EXTRA_ATTACK)==0 and c:GetEffectCount(EFFECT_EXTRA_ATTACK_MONSTER)==0
        return card::get_effect_count(d, c, EFFECT_EXTRA_ATTACK) == 0 && card::get_effect_count(d, c, EFFECT_EXTRA_ATTACK_MONSTER) == 0;
    }
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // This card can make up to 2 attacks on monsters during each Battle Phase this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3202)
        effect::set_description(d, e1, 3202);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK_MONSTER);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
// function s.xyzfilter(c,e)
fn xyzfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsRankBelow(4) and c:IsCanBeEffectTarget(e)
    card::is_type(d, c, TYPE_XYZ) && card::is_rank_below(d, c, 4) && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // return sg:IsExists(s.spfilter,1,nil,e,tp,sg)
    (group::is_exists(d, sg, &|d, c| spfilter(d, c, e, tp, sg), 1, NO_EX), false)
}
// function s.spfilter(c,e,tp,sg)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and sg:IsExists(Card.IsCanBeXyzMaterial,1,c,c,tp,REASON_EFFECT)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && group::is_exists(d, sg, &|d, x| card::is_can_be_xyz_material(d, x, Some(c), Some(tp), Some(REASON_EFFECT)), 1, c.into())
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_GRAVE,0,e:GetHandler(),e)
    let h = effect::get_handler(d, e).nil();
    let g = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, e)), tp, LOCATION_GRAVE, 0, h.into());
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg.clone()), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(tg), 1, tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetTargetCards(e)
    let mut g = duel::get_target_cards(d, e).nil();
    // if #g~=2 or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if g.len() != 2 || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=g:FilterSelect(tp,Card.IsCanBeSpecialSummoned,1,1,nil,e,0,tp,false,false):GetFirst()
    let sel = group::filter_select(d, &g, tp, &|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None), 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // g:RemoveCard(sc)
            group::remove_card(&mut g, &CardSet::from_card(sc));
            // Duel.Overlay(sc,g)
            duel::overlay(d, sc, g, false);
        }
    }
}
