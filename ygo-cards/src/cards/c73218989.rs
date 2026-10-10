//! Black-Winged Assault Dragon
//! ブラックフェザー・アサルト・ドラゴン
//! scripted by Cybercatman
use super::prelude::*;

pub const ID: u32 = 73218989;
pub const INFO: ScriptInfo = ScriptInfo {
    counter_list: &[COUNTER_FEATHER],
    listed_names: &[CARD_BLACK_WINGED_DRAGON],
    synchro_tuner_required: Some(1),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_FEATHER)
    card::enable_counter_permit(d, c, COUNTER_FEATHER as u16, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner Synchro Monster + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_SYNCHRO),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_type_ctx(d, x, TYPE_SYNCHRO, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:AddMustBeSynchroSummoned()
    card::add_must_be_synchro_summoned(d, c);
    // Must be either Synchro Summoned, or Special Summoned from your Extra Deck by banishing 1 Tuner Synchro Monster and 1 "Black-Winged Dragon" from your face-up Monster Zone and/or GY
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    // e0:SetTarget(s.sptg)
    effect::set_target(d, e0, Some(lua_fn(sptg)));
    // e0:SetOperation(s.spop)
    effect::set_operation(d, e0, Some(operation(spop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Each time your opponent activates a monster effect, place 1 Black Feather Counter on this card when that effect resolves, and if you do, inflict 700 damage to your opponent
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1a:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1a, EVENT_CHAINING);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetOperation(aux.chainreg)
    effect::set_operation(d, e1a, Some(aux::chainreg()));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1b, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1b:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e1b, EVENT_CHAIN_SOLVED);
    // e1b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1b, LOCATION_MZONE);
    // e1b:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp and re:IsMonsterEffect() and e:GetHandler():HasFlagEffect(1) end)
    effect::set_condition(d, e1b, Some(condition(e1b_cond)));
    // e1b:SetOperation(s.damop)
    effect::set_operation(d, e1b, Some(operation(damop)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // During your opponent's turn (Quick Effect): You can Tribute this card with 4 or more Black Feather Counters on it; destroy all cards on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(s.descost)
    effect::set_cost(d, e2, Some(cost_target(descost)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,tp)` (tp is not used)
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCompositeType(TYPE_SYNCHRO|TYPE_TUNER) or c:IsCode(CARD_BLACK_WINGED_DRAGON))
    //   and c:IsFaceup() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true,true)
    (card::is_composite_type(d, c, TYPE_SYNCHRO | TYPE_TUNER) || card::is_code(d, c, &[CARD_BLACK_WINGED_DRAGON]))
        && card::is_faceup(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, true)
}
/// `s.rescon(sg,e,tp)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // if #sg~=2 or Duel.GetLocationCountFromEx(tp,tp,sg,e:GetHandler())==0 then return false end
    if sg.len() != 2 {
        return (false, false);
    }
    let h = effect::get_handler(d, e.nil()).nil();
    if duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Card(h), None).0 == 0 {
        return (false, false);
    }
    // local a,b=sg:GetFirst(),sg:GetNext()
    let v: Vec<CardId> = group::iter(d, sg);
    let (a, b) = (v[0], v[1]);
    // return (a:IsCompositeType(TYPE_SYNCHRO|TYPE_TUNER) and b:IsCode(CARD_BLACK_WINGED_DRAGON))
    //   or (b:IsCompositeType(TYPE_SYNCHRO|TYPE_TUNER) and a:IsCode(CARD_BLACK_WINGED_DRAGON))
    let res = (card::is_composite_type(d, a, TYPE_SYNCHRO | TYPE_TUNER) && card::is_code(d, b, &[CARD_BLACK_WINGED_DRAGON]))
        || (card::is_composite_type(d, b, TYPE_SYNCHRO | TYPE_TUNER) && card::is_code(d, a, &[CARD_BLACK_WINGED_DRAGON]));
    (res, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let e = a.e(0);
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned();
    // if g then
    if let Some(g) = g {
        // Duel.Remove(g,POS_FACEUP,REASON_COST)
        duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp and re:IsMonsterEffect() and e:GetHandler():HasFlagEffect(1) end`
fn e1b_cond(d: &mut Duel, a: &EffArgs) -> bool {
    if a.ep != 1 - a.tp {
        return false;
    }
    if !effect::is_monster_effect(d, a.re.nil()) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, h, 1, None)
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:AddCounter(COUNTER_FEATHER,1) then
    if card::add_counter(d, c, COUNTER_FEATHER as u16, 1, false) {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // Duel.Damage(1-tp,700,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, 700, REASON_EFFECT, false, None);
    }
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetCounter(COUNTER_FEATHER)>=4 and c:IsReleasable() end
    if a.chk == Some(0) {
        return card::get_counter(d, c, COUNTER_FEATHER as u16) >= 4 && card::is_releasable(d, c, None, None);
    }
    // Duel.Release(c,REASON_COST)
    duel::release(d, c, REASON_COST, None);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
