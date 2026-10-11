//! Xyz Poseidon Splash
use super::prelude::*;

pub const ID: u32 = 16938770;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy monsters of the declared attribute that are not equipped with an Equip Spell
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Fish, Sea Serpent, or Aqua monster from your GY to either field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.desconfilter(c)`
fn desconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsFaceup() and c:GetOverlayGroup():IsExists(Card.IsType,1,nil,TYPE_XYZ)
    if !(card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)) {
        return false;
    }
    let og = card::get_overlay_group(d, c);
    group::is_exists(d, &og, &|d, x| card::is_type(d, x, TYPE_XYZ), 1, NO_EX)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.desconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&desconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil):Filter(aux.NOT(Card.HasEquipCard),nil)
    let all = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let g = group::filter(d, &all, &|d, c| !card::has_equip_card(d, c), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local attributes=g:GetBitwiseOr(Card.GetAttribute)
    let attributes = group::get_bitwise_or(d, &g, &|d, c| card::get_attribute(d, c) as i64) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTRIBUTE as u64);
    // local decl_attr=Duel.AnnounceAttribute(tp,1,attributes)
    let decl_attr = duel::announce_attribute(d, a.tp, 1, attributes);
    // e:SetLabel(decl_attr)
    effect::set_label(d, a.e, &[decl_attr as i64]);
    // local sg=g:Filter(Card.IsAttribute,nil,decl_attr)
    let sg = group::filter(d, &g, &|d, c| card::is_attribute(d, c, decl_attr), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,tp,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, a.tp, 0);
    true
}
/// `s.desfilter(c,attr)`
fn desfilter(d: &mut Duel, c: CardId, attr: u32) -> bool {
    // return c:IsFaceup() and c:IsAttribute(attr) and not c:HasEquipCard()
    card::is_faceup(d, c) && card::is_attribute(d, c, attr) && !card::has_equip_card(d, c)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local attr=e:GetLabel()
    let attr = effect::get_label(d, a.e) as u32;
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,attr)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, attr)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() and Duel.CheckRemoveOverlayCard(tp,1,0,1,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None) && duel::check_remove_overlay_card(d, a.tp, 1, 0, 1, REASON_COST, None);
    }
    // Duel.Remove(c,POS_FACEUP,REASON_COST)
    duel::remove(d, CardSet::from_card(c), POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.RemoveOverlayCard(tp,1,0,1,1,REASON_COST)
    duel::remove_overlay_card(d, a.tp, 1, 0, 1, 1, REASON_COST, None);
    true
}
/// `s.spfilter(c,e,tp,ftpl,ftopp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ftpl: i32, ftopp: i32) -> bool {
    // return c:IsRace(RACE_FISH|RACE_SEASERPENT|RACE_AQUA) and
    //     ((ftpl>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    //     or (ftopp>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)))
    card::is_race(d, c, RACE_FISH | RACE_SEASERPENT | RACE_AQUA)
        && ((ftpl > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
            || (ftopp > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local ftpl=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ftpl = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        // local ftopp=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
        let ftopp = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0;
        // return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,ftpl,ftopp)
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, ftpl, ftopp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ftpl=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ftpl = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local ftopp=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
    let ftopp = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,ftpl,ftopp):GetFirst()
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, c| spfilter(d, c, a.e, a.tp, ftpl, ftopp)),
        a.tp,
        LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &g);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // local b1=ftpl>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let b1 = ftpl > 0 && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, None, None, None);
    // local b2=ftopp>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    let b2 = ftopp > 0 && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(1 - a.tp), None);
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // local target_player=op==1 and tp or 1-tp
    let target_player = if op == Some(1) { a.tp } else { 1 - a.tp };
    // Duel.SpecialSummon(sc,0,tp,target_player,false,false,POS_FACEUP)
    duel::special_summon(d, CardSet::from_card(sc), 0, a.tp, target_player, false, false, POS_FACEUP as u8, None);
}
