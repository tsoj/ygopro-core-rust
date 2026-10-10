//! Ursarctic Polari
use super::prelude::*;

pub const ID: u32 = 27693363;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_URSARCTIC], listed_names: &[CARD_URSARCTIC_BIG_DIPPER], ..ScriptInfo::DEFAULT };

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
    effect::set_operation(d, e0, Some(lua_fn(sprop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Activate 1 "Ursarctic Big Dipper" from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.acttg)
    effect::set_target(d, e1, Some(cost_target(acttg)));
    // e1:SetOperation(s.actop)
    effect::set_operation(d, e1, Some(operation(actop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add to your hand, or Special Summon, 1 "Ursarctic" monster from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.Replaceable(s.thspcost,s.extracon))
    effect::set_cost(d, e2, Some(cost::replaceable(cost_target(thspcost), Some(lua_fn(extracon)))));
    // e2:SetTarget(s.thsptg)
    effect::set_target(d, e2, Some(cost_target(thsptg)));
    // e2:SetOperation(s.thspop)
    effect::set_operation(d, e2, Some(operation(thspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sprfilter(c)`
fn sprfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToGraveAsCost() and c:HasLevel()
    card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c) && card::has_level(d, c)
}
/// `s.sprfilter2(c,tp,mc,sc)`
fn sprfilter2(d: &mut Duel, c: CardId, tp: u8, mc: CardId, sc: CardId) -> bool {
    // local sg=Group.FromCards(c,mc)
    let mut sg = CardSet::from_card(c);
    group::add_card(&mut sg, &CardSet::from_card(mc));
    // return (math.abs((c:GetLevel()-mc:GetLevel()))==1) and not c:IsType(TYPE_TUNER)
    //     and Duel.GetLocationCountFromEx(tp,tp,sg,sc)>0
    let diff = card::get_level(d, c) as i64 - card::get_level(d, mc) as i64;
    diff.abs() == 1
        && !card::is_type(d, c, TYPE_TUNER)
        && duel::get_location_count_from_ex(d, tp, Some(tp), (&sg).into(), duel::ExtraCard::Card(sc), None).0 > 0
}
/// `s.sprfilter1(c,tp,g,sc)`: the `g` argument is shadowed by a new group in Lua
fn sprfilter1(d: &mut Duel, c: CardId, tp: u8, _g: &CardSet, sc: CardId) -> bool {
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return c:IsType(TYPE_TUNER) and g:IsExists(s.sprfilter2,1,c,tp,c,sc)
    card::is_type(d, c, TYPE_TUNER) && group::is_exists(d, &g, &|d, x| sprfilter2(d, x, tp, c, sc), 1, c.into())
}
/// `s.sprcon(e,c)`
fn sprcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return g:IsExists(s.sprfilter1,1,nil,tp,g,c)
    group::is_exists(d, &g, &|d, x| sprfilter1(d, x, tp, &g, c), 1, NO_EX)
}
/// `s.sprtg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sprtg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g1=g:Filter(s.sprfilter1,nil,tp,g,c)
    let g1 = group::filter(d, &g, &|d, x| sprfilter1(d, x, tp, &g, c), NO_EX);
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
/// `s.sprop(e,tp,eg,ep,ev,re,r,rp,c)`
fn sprop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
}
/// `s.actfilter(c,tp)`
fn actfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_URSARCTIC_BIG_DIPPER) and c:GetActivateEffect() and c:GetActivateEffect():IsActivatable(tp,true,true)
    if !card::is_code(d, c, &[CARD_URSARCTIC_BIG_DIPPER]) {
        return false;
    }
    match card::get_activate_effect(d, c).first().copied() {
        Some(ae) => effect::is_activatable(d, ae, tp, true, true),
        None => false,
    }
}
/// `s.acttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.actfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| actfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.actop(e,tp,eg,ep,ev,re,r,rp)`
fn actop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.actfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| actfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // Duel.ActivateFieldSpell(sc,e,tp,eg,ep,ev,re,r,rp)
        duel::activate_field_spell(d, Some(sc), a, None);
    }
}
/// `s.thspcostfilter(c,e,tp)`
fn thspcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelAbove(7) and Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,Duel.GetMZoneCount(tp,c)>0)
    card::is_level_above(d, c, 7) && {
        let mmz = duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0;
        duel::is_existing_matching_card(d, Some(&|d, x| thspfilter(d, x, e, tp, mmz)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
    }
}
/// `s.thspfilter(c,e,tp,mmz_chk)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // return (c:IsSetCard(SET_URSARCTIC) and c:IsMonster()) and (c:IsAbleToHand() or (mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_set_card(d, c, &[SET_URSARCTIC as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || (mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.thspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    let f = |d: &mut Duel, c: CardId| thspcostfilter(d, c, e, tp);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.thspcostfilter,1,false,nil,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&f), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.thspcostfilter,1,1,false,nil,nil,e,tp)
    let g = duel::select_release_group_cost(d, tp, Some(&f), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thspop(e,tp,eg,ep,ev,re,r,rp)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,mmz_chk):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thspfilter(d, c, e, tp, mmz_chk)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else { return };
    // aux.ToHandOrElse(sc,tp,
    //     function() return mmz_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
    //     function() Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
    //     aux.Stringid(id,3))
    aux::to_hand_or_else(
        d,
        sc,
        tp,
        Some(&|d, _c| mmz_chk && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None)),
        Some(&|d, _g| duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None)),
        Some(aux::stringid(ID, 3)),
    );
}
/// `s.extracon(base,e,tp,eg,ep,ev,re,r,rp,c)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let tp = a.p(2);
    let c = a.card(9);
    // return Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_GRAVE,0,1,c,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
    let mmz = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    let ex = match c {
        Some(c) => c.into(),
        None => NO_EX,
    };
    duel::is_existing_matching_card(d, Some(&|d, x| thspfilter(d, x, e, tp, mmz)), tp, LOCATION_GRAVE, 0, 1, ex)
}
