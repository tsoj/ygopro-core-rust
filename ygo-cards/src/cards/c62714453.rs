//! Ursarctic Polar Star
use super::prelude::*;

pub const ID: u32 = 62714453;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_URSARCTIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your Extra Deck) by sending 2 monsters you control with a Level difference of 1 to the GY (1 Tuner and 1 non-Tuner)
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetCondition(s.sprcon)
    effect::set_condition(d, e0, Some(lua_fn(sprcon)));
    // e0:SetTarget(s.sprtg)
    effect::set_target(d, e0, Some(lua_fn(sprtg)));
    // e0:SetOperation(s.sprop)
    effect::set_operation(d, e0, Some(operation(sprop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon 1 Level 7 "Ursarctic" Synchro Monster from your Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.Replaceable(s.spcost,s.extracon))
    effect::set_cost(d, e1, Some(cost::replaceable(cost_target(spcost), Some(lua_fn(extracon)))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.sprfilter(c)
fn sprfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToGraveAsCost() and c:HasLevel()
    card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c) && card::has_level(d, c)
}
// function s.sprfilter1(c,tp,g,sc)
fn sprfilter1(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return c:IsType(TYPE_TUNER) and g:IsExists(s.sprfilter2,1,c,tp,c,sc)
    card::is_type(d, c, TYPE_TUNER) && group::is_exists(d, &g, &|d, x| sprfilter2(d, x, tp, c, sc), 1, c.into())
}
// function s.sprfilter2(c,tp,mc,sc)
fn sprfilter2(d: &mut Duel, c: CardId, tp: u8, mc: CardId, sc: CardId) -> bool {
    // local sg=Group.FromCards(c,mc)
    let mut sg = CardSet::from_card(c);
    group::add_card(&mut sg, &CardSet::from_card(mc));
    // return (math.abs((c:GetLevel()-mc:GetLevel()))==1) and not c:IsType(TYPE_TUNER) and Duel.GetLocationCountFromEx(tp,tp,sg,sc)>0
    let diff = card::get_level(d, c) as i64 - card::get_level(d, mc) as i64;
    diff.abs() == 1 && !card::is_type(d, c, TYPE_TUNER) && duel::get_location_count_from_ex(d, tp, Some(tp), (&sg).into(), duel::ExtraCard::Card(sc), None).0 > 0
}
// function s.sprcon(e,c)
fn sprcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return g:IsExists(s.sprfilter1,1,nil,tp,g,c)
    group::is_exists(d, &g, &|d, x| sprfilter1(d, x, tp, c), 1, NO_EX)
}
// function s.sprtg(e,tp,eg,ep,ev,re,r,rp,chk,c)
fn sprtg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g1=g:Filter(s.sprfilter1,nil,tp,g,c)
    let g1 = group::filter(d, &g, &|d, x| sprfilter1(d, x, tp, c), NO_EX);
    // local mg1=aux.SelectUnselectGroup(g1,e,tp,1,1,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let mut mg1 = aux::select_unselect_group(d, &g1, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true).unwrap_or_default();
    // if #mg1>0 then
    if !mg1.is_empty() {
        // local mc=mg1:GetFirst()
        let mc = group::get_first(d, &mg1).nil();
        // local g2=g:Filter(s.sprfilter2,mc,tp,mc,c,mc:GetLevel())
        let g2 = group::filter(d, &g, &|d, x| sprfilter2(d, x, tp, mc, c), mc.into());
        // local mg2=aux.SelectUnselectGroup(g2,e,tp,1,1,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
        let mg2 = aux::select_unselect_group(d, &g2, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true).unwrap_or_default();
        // mg1:Merge(mg2)
        group::add_card(&mut mg1, &mg2);
    }
    // if #mg1==2 then
    if mg1.len() == 2 {
        // mg1:KeepAlive()
        // e:SetLabelObject(mg1)
        effect::set_label_object(d, e, Value::Group(mg1));
        // return true
        return true;
    }
    // return false
    false
}
// function s.sprop(e,tp,eg,ep,ev,re,r,rp,c)
fn sprop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, a.e).as_group().cloned() else { return };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
}
// function s.spcheck(sg,tp,exg,e)
fn spcheck(d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet, e: EffectId) -> (bool, bool) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return sg:IsExists(s.ursfilter,1,c) and sg:IsContains(c)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sg+c)
    let ok = group::is_exists(d, sg, &ursfilter, 1, c.into()) && group::is_contains(sg, c) && {
        let mut sgc = sg.clone();
        group::add_card(&mut sgc, &CardSet::from_card(c));
        duel::is_existing_matching_card(d, Some(&|d: &mut Duel, x: CardId| spfilter(d, x, e, tp, Some(&sgc))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
    };
    (ok, false)
}
// function s.ursfilter(c)
fn ursfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_URSARCTIC) and c:IsLevel(8)
    card::is_set_card(d, c, &[SET_URSARCTIC as u16]) && card::is_level(d, c, &[8])
}
// function s.spfilter(c,e,tp,mustg)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mustg: Option<&CardSet>) -> bool {
    // return c:IsLevel(7) and c:IsSetCard(SET_URSARCTIC) and c:IsType(TYPE_SYNCHRO) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,mustg,c)>0
    let ex: Exclude = match mustg {
        Some(g) => g.into(),
        None => NO_EX,
    };
    card::is_level(d, c, &[7])
        && card::is_set_card(d, c, &[SET_URSARCTIC as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,2,2,true,s.spcheck,nil,e) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, None, 2, 2, true, Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| spcheck(d, sg, tp, exg, e)), NO_EX);
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,nil,2,2,true,s.spcheck,nil,e)
    let sg = duel::select_release_group_cost(d, a.tp, None, 2, 2, true, Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| spcheck(d, sg, tp, exg, e)), NO_EX);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d: &mut Duel, c: CardId| spfilter(d, c, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,true,false,POS_FACEUP)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) > 0 {
            // It gains this effect: ● Your opponent cannot activate the effects of monsters with a Level, that were Special Summoned from the Extra Deck.
            // local e1=Effect.CreateEffect(sc)
            let e1 = effect::create_effect(d, sc);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
            effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetTargetRange(0,1)
            effect::set_target_range(d, e1, 0, 1);
            // e1:SetValue(s.aclimit)
            effect::set_value_fn(d, e1, lua_fn(aclimit));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1,true)
            card::register_effect(d, sc, e1, true);
            // if not sc:IsType(TYPE_EFFECT) then
            if !card::is_type(d, sc, TYPE_EFFECT) {
                // local e2=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e2 = effect::create_effect(d, h);
                // e2:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                // e2:SetCode(EFFECT_ADD_TYPE)
                effect::set_code(d, e2, EFFECT_ADD_TYPE);
                // e2:SetValue(TYPE_EFFECT)
                effect::set_value(d, e2, TYPE_EFFECT as i64);
                // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                // sc:RegisterEffect(e2,true)
                card::register_effect(d, sc, e2, true);
            }
            // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
            card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
        }
    }
}
// function s.aclimit(e,re,tp)
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect() and rc:HasLevel() and rc:IsSummonLocation(LOCATION_EXTRA) and rc:IsLocation(LOCATION_MZONE)
    effect::is_monster_effect(d, re) && card::has_level(d, rc) && card::is_summon_location(d, rc, LOCATION_EXTRA) && card::is_location(d, rc, LOCATION_MZONE)
}
// function s.extracon(base,e,tp,eg,ep,ev,re,r,rp)
fn extracon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let tp = a.p(2);
    // return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
    duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c: CardId| spfilter(d, c, e, tp, None)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
