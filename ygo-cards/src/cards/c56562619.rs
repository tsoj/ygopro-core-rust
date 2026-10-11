//! Black Dragon Ninja
use super::prelude::*;

pub const ID: u32 = 56562619;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NINJA, SET_NINJITSU_ART], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon Condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.splimit)
    effect::set_value_fn(d, e1, lua_fn(splimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Remove monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.rmcost)
    effect::set_cost(d, e2, Some(cost_target(rmcost)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special summon when leaving the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // clear
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    // e4:SetOperation(s.clearop)
    effect::set_operation(d, e4, Some(operation(clearop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local ng=Group.CreateGroup()
    // ng:KeepAlive()
    // e2:SetLabelObject(ng)
    // e3:SetLabelObject(ng)
    // e4:SetLabelObject(ng)
    // e5:SetLabelObject(ng)
    // The group lives in the label object of e2; e3, e4 and e5 point at e2 (see `ng_holder`).
    effect::set_label_object(d, e2, Value::Group(CardSet::new()));
    effect::set_label_object(d, e3, Value::Effect(e2));
    effect::set_label_object(d, e4, Value::Effect(e2));
    effect::set_label_object(d, e5, Value::Effect(e2));
}
/// The effect whose label object holds the shared group `ng` (`e` itself, or the effect it points at).
fn ng_holder(d: &mut Duel, e: EffectId) -> EffectId {
    effect::get_label_object(d, e).as_effect().unwrap_or(e)
}
/// `e:GetLabelObject()` for the shared group `ng`.
fn ng_get(d: &mut Duel, e: EffectId) -> CardSet {
    let h = ng_holder(d, e);
    effect::get_label_object(d, h).as_group().cloned().nil()
}
/// Stores the shared group `ng` again after a change.
fn ng_set(d: &mut Duel, e: EffectId, g: CardSet) {
    let h = ng_holder(d, e);
    effect::set_label_object(d, h, Value::Group(g));
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let se = a.e(1);
    // return (se:IsMonsterEffect() and se:GetHandler():IsSetCard(SET_NINJA)) or se:GetHandler():IsSetCard(SET_NINJITSU_ART)
    if effect::is_monster_effect(d, se) {
        let h = effect::get_handler(d, se).nil();
        if card::is_set_card(d, h, &[SET_NINJA as u16]) {
            return true;
        }
    }
    let h = effect::get_handler(d, se).nil();
    card::is_set_card(d, h, &[SET_NINJITSU_ART as u16])
}
/// `s.cfilter1(c,tp)`
fn cfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:IsMonster() and c:IsSetCard(SET_NINJA) and c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingMatchingCard(s.cfilter2,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,c,c)
    (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_NINJA as u16])
        && card::is_able_to_grave_as_cost(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| cfilter2(d, x, c)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, c.into())
}
/// `s.cfilter2(c,cc)`
fn cfilter2(d: &mut Duel, c: CardId, cc: CardId) -> bool {
    // return (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:IsSetCard(SET_NINJITSU_ART) and c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingTarget(s.rmfilter,0,LOCATION_MZONE,LOCATION_MZONE,1,c,cc)
    (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_set_card(d, c, &[SET_NINJITSU_ART as u16])
        && card::is_able_to_grave_as_cost(d, c)
        && duel::is_existing_target(d, Some(&|d, x| rmfilter(d, x, cc)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// `s.rmfilter(c,cc)`
fn rmfilter(d: &mut Duel, c: CardId, cc: CardId) -> bool {
    // return c~=cc and c:IsAbleToRemove()
    c != cc && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter1,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter1(d, c, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.cfilter1,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,tp)
    let tp = a.tp;
    let mut g1 = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter1(d, c, tp)), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local cc=g1:GetFirst()
    let cc = group::get_first(d, &g1).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g2=Duel.SelectMatchingCard(tp,s.cfilter2,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,cc,cc)
    let g2 = duel::select_matching_card(d, a.tp, Some(&|d, x| cfilter2(d, x, cc)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, cc.into()).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SendtoGrave(g1,REASON_COST)
    duel::send_to_grave(d, g1, REASON_COST, None, None);
    true
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_REMOVED) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e)
            && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0
            && card::is_location(d, tc, LOCATION_REMOVED)
        {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
            // e:GetLabelObject():AddCard(tc)
            let mut g = ng_get(d, a.e);
            group::add_card(&mut g, &CardSet::from_card(tc));
            ng_set(d, a.e, g);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousPosition(POS_FACEUP) and not c:IsLocation(LOCATION_DECK)
    card::is_previous_position(d, c, POS_FACEUP) && !card::is_location(d, c, LOCATION_DECK)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetFlagEffect(id)~=0
    //     and (c:IsCanBeSpecialSummoned(e,0,tp,false,false) or c:IsCanBeSpecialSummoned(e,0,tp,false,false,1-tp))
    card::get_flag_effect(d, c, ID) != 0
        && (card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            || card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(1 - tp), None, None))
}
/// `s.spfilter1(c,e,tp)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetFlagEffect(id)~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:GetOwner()==tp
    card::get_flag_effect(d, c, ID) != 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::get_owner(d, c) == tp
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetFlagEffect(id)~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp) and c:GetOwner()==1-tp
    card::get_flag_effect(d, c, ID) != 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
        && card::get_owner(d, c) == 1 - tp
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=e:GetLabelObject():Filter(s.spfilter,nil,e,tp)
    let rg = ng_get(d, a.e);
    let (e, tp) = (a.e, a.tp);
    let g = group::filter(d, &rg, &|d, c| spfilter(d, c, e, tp), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,#g,0,0)
    let n = group::get_count(&g) as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), n, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local ft2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
    let ft2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
    // local rg=e:GetLabelObject()
    let rg = ng_get(d, e);
    // if (ft1<=0 and ft2<=0) or #rg<=0 then return end
    if (ft1 <= 0 && ft2 <= 0) || group::get_count(&rg) == 0 {
        return;
    }
    // local sg=nil
    let mut sg: Option<CardSet> = None;
    // local sg1=rg:Filter(s.spfilter1,nil,e,tp)
    let mut sg1 = group::filter(d, &rg, &|d, c| spfilter1(d, c, e, tp), NO_EX);
    // local sg2=rg:Filter(s.spfilter2,nil,e,tp)
    let mut sg2 = group::filter(d, &rg, &|d, c| spfilter2(d, c, e, tp), NO_EX);
    // local gc1=#sg1
    let gc1 = group::get_count(&sg1);
    // local gc2=#sg2
    let gc2 = group::get_count(&sg2);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        if ft1 <= 0 && gc2 > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // sg=sg2:Select(tp,1,1,nil)
            sg = group::select(d, &sg2, tp, 1, 1, false, NO_EX);
        } else if ft2 <= 0 && gc1 > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // sg=sg1:Select(tp,1,1,nil)
            sg = group::select(d, &sg1, tp, 1, 1, false, NO_EX);
        } else if (gc1 > 0 && ft1 > 0) || (gc2 > 0 && ft2 > 0) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // sg=rg:FilterSelect(tp,s.spfilter,1,1,nil,e,tp)
            sg = group::filter_select(d, &rg, tp, &|d, c| spfilter(d, c, e, tp), 1, 1, false, NO_EX);
        }
        // if sg~=nil then
        if let Some(sg) = sg {
            // Duel.SpecialSummon(sg,0,tp,sg:GetFirst():GetOwner(),false,false,POS_FACEUP)
            let first = group::get_first(d, &sg).nil();
            let owner = card::get_owner(d, first);
            duel::special_summon(d, sg, 0, tp, owner, false, false, POS_FACEUP as u8, None);
        }
        // rg:Clear()
        ng_set(d, e, CardSet::new());
        // return
        return;
    }
    // if gc1>0 and ft1>0 then
    if gc1 > 0 && ft1 > 0 {
        // if #sg1>ft1 then
        if group::get_count(&sg1) as i32 > ft1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // sg1=sg1:Select(tp,ft1,ft1,nil)
            sg1 = group::select(d, &sg1, tp, ft1 as u16, ft1 as u16, false, NO_EX).nil();
        }
        // for sc in aux.Next(sg1) do
        for sc in group::iter(d, &sg1) {
            // Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // if gc2>0 and ft2>0 then
    if gc2 > 0 && ft2 > 0 {
        // if #sg2>ft2 then
        if group::get_count(&sg2) as i32 > ft2 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // sg2=sg2:Select(tp,ft2,ft2,nil)
            sg2 = group::select(d, &sg2, tp, ft2 as u16, ft2 as u16, false, NO_EX).nil();
        }
        // for sc in aux.Next(sg2) do
        for sc in group::iter(d, &sg2) {
            // Duel.SpecialSummonStep(sc,0,tp,1-tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, sc, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // rg:Clear()
    ng_set(d, e, CardSet::new());
}
/// `s.clearop(e,tp,eg,ep,ev,re,r,rp)`
fn clearop(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():Clear()
    ng_set(d, a.e, CardSet::new());
}
