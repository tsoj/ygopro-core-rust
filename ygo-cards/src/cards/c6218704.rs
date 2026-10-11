//! Odd-Eyes Arcray Dragon
use super::prelude::*;

pub const ID: u32 = 6218704;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ZARC], miracle_synchro_fusion: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 4 Dragon monsters (1 Fusion, 1 Synchro, 1 Xyz, and 1 Pendulum)
    // Fusion.AddProcMix(c,true,true,s.matfilter(TYPE_FUSION),s.matfilter(TYPE_SYNCHRO),s.matfilter(TYPE_XYZ),s.matfilter(TYPE_PENDULUM))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![matfilter(TYPE_FUSION), matfilter(TYPE_SYNCHRO), matfilter(TYPE_XYZ), matfilter(TYPE_PENDULUM)],
    );
    // Must be either Fusion Summoned or Special Summoned by alternate procedure
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(s.splimit)
    effect::set_value_fn(d, e0, lua_fn(splimit));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Alternate Special Summon procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e1, LOCATION_EXTRA);
    // e1:SetCondition(s.hspcon)
    effect::set_condition(d, e1, Some(lua_fn(hspcon)));
    // e1:SetTarget(s.hsptg)
    effect::set_target(d, e1, Some(lua_fn(hsptg)));
    // e1:SetOperation(s.hspop)
    effect::set_operation(d, e1, Some(lua_fn(hspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place 1 Pendulum Monster in the Pendulum Zone
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCondition(function(e) return e:GetHandler():IsSummonLocation(LOCATION_EXTRA) end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetTarget(s.pltg)
    effect::set_target(d, e3, Some(cost_target(pltg)));
    // e3:SetOperation(s.plop)
    effect::set_operation(d, e3, Some(operation(plop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Place itself into pendulum zone
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetCondition(s.pencon)
    effect::set_condition(d, e4, Some(condition(pencon)));
    // e4:SetTarget(s.pentg)
    effect::set_target(d, e4, Some(cost_target(pentg)));
    // e4:SetOperation(s.penop)
    effect::set_operation(d, e4, Some(operation(penop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.matfilter(typ)`: `function(c,fc,sumtype,tp) return c:IsRace(RACE_DRAGON,fc,sumtype,tp) and c:IsType(typ,fc,sumtype,tp) end`
fn matfilter(typ: u32) -> fusion::Mat {
    fusion::Mat::F(fusion::fus_mat(move |d, c, fc, st, tp| {
        card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp)) && card::is_type_ctx(d, c, typ, Some(fc), st, Some(tp))
    }))
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsLocation(LOCATION_EXTRA) or aux.fuslimit(e,se,sp,st) or aux.penlimit(e,se,sp,st)
    let h = effect::get_handler(d, a.e(0)).nil();
    !card::is_location(d, h, LOCATION_EXTRA) || aux::fuslimit().call(d, a.0).truthy() || aux::penlimit().call(d, a.0).truthy()
}
/// `s.hspfilter(c,tp,sc)`
fn hspfilter(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // return c:IsCode(CARD_ZARC) and c:IsLevel(12) and c:IsAttribute(ATTRIBUTE_DARK) and Duel.GetLocationCountFromEx(tp,tp,c,sc)>0
    card::is_code(d, c, &[CARD_ZARC])
        && card::is_level(d, c, &[12])
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(sc), None).0 > 0
}
/// `s.hspcon(e,c)`
fn hspcon(d: &mut Duel, a: Args) -> bool {
    // if not c then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // if c:IsFaceup() then return false end
    if card::is_faceup(d, c) {
        return false;
    }
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(tp,s.hspfilter,1,false,1,true,c,tp,nil,nil,nil,tp,c)
    duel::check_release_group(
        d,
        tp,
        Some(&|d, x| hspfilter(d, x, tp, c)),
        1,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(1), cancelable: false, check_field: true, to_check: Some(c), toplayer: Some(tp), zone: None, use_oppo: false },
        NO_EX,
        false,
    )
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let (tp, c) = (a.p(1), a.c(9));
    // local g=Duel.SelectReleaseGroup(tp,s.hspfilter,1,1,false,true,true,c,tp,nil,false,nil,tp,c)
    let g = duel::select_release_group(
        d,
        tp,
        Some(&|d, x| hspfilter(d, x, tp, c)),
        1,
        1,
        duel::ReleaseArgs { use_hand: Some(false), max: None, cancelable: true, check_field: true, to_check: Some(c), toplayer: Some(tp), zone: None, use_oppo: false },
        NO_EX,
        false,
    );
    // if not g then return false end
    let Some(g) = g else {
        return false;
    };
    // g:KeepAlive()
    // e:SetLabelObject(g)
    effect::set_label_object(d, a.e(0), Value::Group(g));
    // return true
    true
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn hspop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e(0)).as_group().cloned();
    // if not g then return end
    let Some(g) = g else {
        return;
    };
    // Duel.Release(g,REASON_COST|REASON_MATERIAL)
    duel::release(d, g, REASON_COST | REASON_MATERIAL, None);
    // g:DeleteGroup()
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldCard(tp,LOCATION_PZONE,0) and Duel.GetFieldCard(tp,LOCATION_PZONE,1)
    duel::get_field_card(d, a.tp, LOCATION_PZONE, 0).is_some() && duel::get_field_card(d, a.tp, LOCATION_PZONE, 1).is_some()
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_PZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_PZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_PZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_PZONE, 0, NO_EX);
    // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,3)) then return end
    if g.is_empty() || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local sc=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if not sc then return end
    let Some(sc) = sc else {
        return;
    };
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.SendtoDeck(sc,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_EXTRA)
    //     and sc:IsCanBeSpecialSummoned(e,0,tp,true,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,sc)>0
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
    if duel::send_to_deck(d, CardSet::from_card(sc), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, sc, LOCATION_EXTRA)
        && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, true, false, None, None, None)
        && duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::Card(sc), None).0 > 0
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 4))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sc,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    }
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_PENDULUM) and not c:IsForbidden()
    card::is_type(d, c, TYPE_PENDULUM) && !card::is_forbidden(d, c)
}
/// `function(e) return e:GetHandler():IsSummonLocation(LOCATION_EXTRA) end`
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_summon_location(d, h, LOCATION_EXTRA)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp) and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp) && duel::is_existing_matching_card(d, Some(&plfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.CheckPendulumZones(tp) then return false end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&plfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc then
    if let Some(tc) = tc {
        // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.pencon(e,tp,eg,ep,ev,re,r,rp)`
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFaceup() and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_faceup(d, c) && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.CheckPendulumZones(tp) then return false end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
