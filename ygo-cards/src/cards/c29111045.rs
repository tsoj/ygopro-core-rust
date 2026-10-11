//! Goblin Biker Grand Breakout
use super::prelude::*;

pub const ID: u32 = 29111045;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GOBLIN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Goblin" monster from your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // All face-up monsters your opponent currently controls lose 1000 ATK for each material detached
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(atkcost)));
    effect::set_target(d, e2, Some(cost_target(atktg)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
}
// function s.spcostfilter(c,tp)
fn spcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return Duel.GetMZoneCount(tp,c)>0
    duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.spcostfilter,1,false,nil,nil,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| spcostfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,s.spcostfilter,1,1,false,nil,nil,tp)
    let sg = duel::select_release_group_cost(d, tp, Some(&|d, c| spcostfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GOBLIN) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GOBLIN as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // local cost_chk=e:GetLabel()==1
        let cost_chk = effect::get_label(d, a.e) == 1;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return (cost_chk or Duel.GetLocationCount(tp,LOCATION_MZONE)>0) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
        let e = a.e;
        return (cost_chk || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let e = a.e;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3206)
            effect::set_description(d, e1, 3206);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
        }
    }
}
// function s.atkcostfilter(c)
fn atkcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GOBLIN) and c:IsType(TYPE_XYZ) and c:IsFaceup() and c:GetOverlayCount()>0
    card::is_set_card(d, c, &[SET_GOBLIN as u16]) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c) && card::get_overlay_count(d, c) > 0
}
// function s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(-1)
    effect::set_label(d, a.e, &[-1]);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local xyzg=Duel.GetMatchingGroup(s.atkcostfilter,tp,LOCATION_MZONE,0,nil)
    let xyzg = duel::get_matching_group(d, Some(&atkcostfilter), tp, LOCATION_MZONE, 0, NO_EX);
    if a.chk == Some(0) {
        // return c:IsAbleToRemoveAsCost() and #xyzg>0 and Duel.CheckRemoveOverlayCard(tp,0,0,1,REASON_COST,xyzg)
        return card::is_able_to_remove_as_cost(d, c, None)
            && !xyzg.is_empty()
            && duel::check_remove_overlay_card(d, tp, 0, 0, 1, REASON_COST, Some(&xyzg));
    }
    // Duel.Remove(c,POS_FACEUP,REASON_COST)
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    // local maxct=xyzg:GetSum(Card.GetOverlayCount)
    let maxct = group::get_sum(d, &xyzg, &|d, c| card::get_overlay_count(d, c) as i64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVEXYZ)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVEXYZ as u64);
    // local ct=Duel.RemoveOverlayCard(tp,0,0,1,maxct,REASON_COST,xyzg)
    let ct = duel::remove_overlay_card(d, tp, 0, 0, 1, maxct as u16, REASON_COST, Some(&xyzg));
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
    true
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local atkg=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let atkg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        // local cost_chk=e:GetLabel()==-1
        let cost_chk = effect::get_label(d, a.e) == -1;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return cost_chk and #atkg>0
        return cost_chk && !atkg.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,atkg,#atkg,tp,e:GetLabel()*-1000)
    let val = effect::get_label(d, a.e) * -1000;
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(atkg.clone()), atkg.len() as u8, tp, val as i32);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=e:GetLabel()*-1000
    let atk = effect::get_label(d, a.e) * -1000;
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
