//! Basiltrice, Familiar of the Evil Eye
use super::prelude::*;

pub const ID: u32 = 65351555;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EVIL_EYE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Link Summon 1 "Evil Eye" monster during opponent's turn
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
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(lnkcond)));
    // e2:SetTarget(s.lnktg)
    effect::set_target(d, e2, Some(cost_target(lnktg)));
    // e2:SetOperation(s.lnkop)
    effect::set_operation(d, e2, Some(operation(lnkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(_,tp) return Duel.IsTurnPlayer(1-tp) end` (the second effect's condition)
fn lnkcond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_EVIL_EYE),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_EVIL_EYE as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
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
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Banish it if it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EVIL_EYE) and c:IsEquipSpell()
    card::is_set_card(d, c, &[SET_EVIL_EYE as u16]) && card::is_equip_spell(d, c)
}
/// `s.lnkfilter(c)`
fn lnkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EVIL_EYE) and c:IsLinkSummonable()
    card::is_set_card(d, c, &[SET_EVIL_EYE as u16]) && card::is_link_summonable(d, c, None, None, 0, 0)
}
/// `s.lnktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lnktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local e1,e2 = s.tempregister(e,tp,eg,ep,ev,re,r,rp,chk)
    let (e1, e2) = tempregister(d, a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=Duel.IsExistingMatchingCard(s.lnkfilter,tp,LOCATION_EXTRA,0,1,nil)
        let res = duel::is_existing_matching_card(d, Some(&lnkfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // e1:Reset()
        effect::reset(d, e1);
        // e2:Reset()
        effect::reset(d, e2);
        // return res
        return res;
    }
    // e1:Reset()
    effect::reset(d, e1);
    // e2:Reset()
    effect::reset(d, e2);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.lnkop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lnkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1,e2 = s.tempregister(e,tp,eg,ep,ev,re,r,chk)
    let (e1, e2) = tempregister(d, a.e, tp);
    // local g=Duel.GetMatchingGroup(s.lnkfilter,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&lnkfilter), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        match sc {
            // if not sc then
            None => {
                // e1:Reset()
                effect::reset(d, e1);
                // e2:Reset()
                effect::reset(d, e2);
                // return
            }
            Some(sc) => {
                // Duel.LinkSummon(tp,sc)
                duel::link_summon(d, tp, sc, None, None, 0, 0);
                // Manually reset e1 and e2 when the monster would be Link Summoned
                // local e3=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e3 = effect::create_effect(d, h);
                // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
                // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e3:SetCode(EVENT_SPSUMMON)
                effect::set_code(d, e3, EVENT_SPSUMMON);
                // e3:SetOperation(s.resetop)
                effect::set_operation(d, e3, Some(operation(resetop)));
                // e3:SetLabelObject({e1,e2})
                effect::set_label_object(d, e3, Value::Multi(vec![Value::Effect(e1), Value::Effect(e2)]));
                // Duel.RegisterEffect(e3,tp)
                duel::register_effect(d, e3, tp);
            }
        }
    }
}
/// `s.resetop(e,tp,eg,ep,ev,re,r,rp)`
fn resetop(d: &mut Duel, a: &EffArgs) {
    // local e1,e2=table.unpack(e:GetLabelObject())
    let parts = match effect::get_label_object(d, a.e) {
        Value::Multi(v) => v,
        _ => Vec::new(),
    };
    let e1 = parts.first().and_then(|v| v.as_effect()).nil();
    let e2 = parts.get(1).and_then(|v| v.as_effect()).nil();
    // e1:Reset()
    effect::reset(d, e1);
    // e2:Reset()
    effect::reset(d, e2);
    // e:Reset()
    effect::reset(d, a.e);
}
/// `s.tempregister(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tempregister(d: &mut Duel, e: EffectId, tp: u8) -> (EffectId, EffectId) {
    // Can treat "Evil Eye" Equip cards as Link Material if using this effect
    let h = effect::get_handler(d, e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_MATERIAL)
    effect::set_code(d, e1, EFFECT_EXTRA_MATERIAL);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET|EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetOperation(aux.TRUE)
    effect::set_operation(d, e1, Some(lua_fn(|_d, _a| true)));
    // e1:SetValue(s.extraval)
    effect::set_value_fn(d, e1, lua_fn(extraval));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_ADD_TYPE)
    effect::set_code(d, e2, EFFECT_ADD_TYPE);
    // e2:SetTargetRange(LOCATION_SZONE,0)
    effect::set_target_range(d, e2, LOCATION_SZONE, 0);
    // e2:SetCondition(s.addtypecon)
    effect::set_condition(d, e2, Some(condition(addtypecon)));
    // e2:SetTarget(aux.TargetBoolFunction(s.eqfilter))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| eqfilter(d, c))));
    // e2:SetValue(TYPE_MONSTER)
    effect::set_value(d, e2, TYPE_MONSTER as i64);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // return e1,e2
    (e1, e2)
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_EVIL_EYE) and c:IsSpell() and c:IsType(TYPE_EQUIP)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_EVIL_EYE as u16]) && card::is_spell(d, c) && card::is_type(d, c, TYPE_EQUIP)
}
/// `s.extraval(chk,summon_type,e,...)`
fn extraval(d: &mut Duel, a: Args) -> Value {
    let chk = a.int(0);
    let summon_type = a.int(1);
    let e = a.e(2);
    if chk == 0 {
        // local tp,sc=...
        let tp = a.p(3);
        let sc = a.card(4);
        // if summon_type~=SUMMON_TYPE_LINK or not (sc and sc:IsSetCard(SET_EVIL_EYE)) then
        let sc_evil = match sc {
            Some(s) => card::is_set_card(d, s, &[SET_EVIL_EYE as u16]),
            None => false,
        };
        if summon_type != SUMMON_TYPE_LINK as i64 || !sc_evil {
            // return Group.CreateGroup()
            return Value::Group(CardSet::new());
        }
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // return Duel.GetMatchingGroup(s.matfilter,tp,LOCATION_ONFIELD,0,nil)
        return Value::Group(duel::get_matching_group(d, Some(&matfilter), tp, LOCATION_ONFIELD, 0, NO_EX));
    } else if chk == 2 {
        // Duel.ResetFlagEffect(e:GetHandlerPlayer(),id)
        let p = effect::get_handler_player(d, e);
        duel::reset_flag_effect(d, p, ID);
    }
    Value::Nil
}
/// `s.addtypecon(e,tp,eg,ep,ev,re,r,rp)`
fn addtypecon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(e:GetHandlerPlayer(),id)>0
    let p = effect::get_handler_player(d, a.e);
    duel::get_flag_effect(d, p, ID) > 0
}
