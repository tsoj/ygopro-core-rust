//! Galaxy-Eyes Photon Change Dragon
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 54725177;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_GALAXYEYES_P_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --You can Special Summon this card (from your hand) by Tributing 2 face-up monsters on either field with 2000 or more ATK, including a monster you control. You can only Special Summon "Galaxy-Eyes Photon Change Dragon" once per turn this way
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0},EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(lua_fn(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(lua_fn(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --During the Main Phase (Quick Effect): You can banish this card you control; Special Summon 1 "Galaxy-Eyes Photon Dragon" from your Deck or GY, then you can banish 1 monster your opponent controls that was Special Summoned from the Extra Deck until the End Phase. You can only use this effect of "Galaxy-Eyes Photon Change Dragon" once per turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.gepdsptg)
    effect::set_target(d, e2, Some(cost_target(gepdsptg)));
    // e2:SetOperation(s.gepdspop)
    effect::set_operation(d, e2, Some(operation(gepdspop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.selfspcostfilter(c)`
fn selfspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttackAbove(2000) and c:IsFaceup() and c:IsReleasable()
    card::is_attack_above(d, c, 2000) && card::is_faceup(d, c) && card::is_releasable(d, c, None, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0 and sg:IsExists(Card.IsControler,1,nil,tp)
    let res = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0
        && group::is_exists(d, sg, &|d, x| card::is_controler(d, x, tp), 1, NO_EX);
    (res, false)
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.selfspcostfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&selfspcostfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // return #g>0 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    !g.is_empty() && matches!(aux::select_unselect_group(d, &g, Some(a.e(0)), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=Duel.GetMatchingGroup(s.selfspcostfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&selfspcostfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_RELEASE,nil,nil,true)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true).unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn selfspop(d: &mut Duel, a: Args) -> Value {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e(0)).as_group().cloned();
    // if g then
    if let Some(g) = g {
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
    }
    Value::Nil
}
/// `s.gepdspfilter(c,e,tp)`
fn gepdspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_GALAXYEYES_P_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_GALAXYEYES_P_DRAGON]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.gepdsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gepdsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetMZoneCount(tp,e:GetHandler())>0
    //   and Duel.IsExistingMatchingCard(s.gepdspfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, e).nil();
        return duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| gepdspfilter(d, x, e, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}
/// `s.rmvfilter(c)`
fn rmvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSummonLocation(LOCATION_EXTRA) and c:IsAbleToRemove()
    card::is_summon_location(d, c, LOCATION_EXTRA) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.gepdspop(e,tp,eg,ep,ev,re,r,rp)`
fn gepdspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.gepdspfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, x| gepdspfilter(d, x, e, tp))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0
    //   and Duel.IsExistingMatchingCard(s.rmvfilter,tp,0,LOCATION_MZONE,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty()
        && duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::is_existing_matching_card(d, Some(&rmvfilter), tp, 0, LOCATION_MZONE, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local rg=Duel.SelectMatchingCard(tp,s.rmvfilter,tp,0,LOCATION_MZONE,1,1,nil)
        let rg = duel::select_matching_card(d, tp, Some(&rmvfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #rg>0 then
        if !rg.is_empty() {
            // Duel.HintSelection(rg)
            duel::hint_selection(d, rg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // --Banish 1 monster your opponent controls that was Special Summoned from the Extra Deck until the End Phase
            // aux.RemoveUntil(rg,nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp)
            aux::remove_until(d, rg, None, REASON_EFFECT, PHASE_END, ID, e, tp, Some(aux::default_field_return()), None, None, None, None, None);
        }
    }
}
