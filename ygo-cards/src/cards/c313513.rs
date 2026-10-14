//! Spell Gear
use super::prelude::*;

pub const ID: u32 = 313513;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ANCIENT_GEAR_GOLEM], listed_series: &[SET_ANCIENT_GEAR], ..ScriptInfo::DEFAULT };

/// `s[0]` in the Lua script: the list of the reset effects (`s[0]` as an array), stored as script
/// variable 0.
const LIST: u64 = 0;
/// `s[0][e]` in the Lua script: the pair `{e1,e2}` of each reset effect `e`, stored as script
/// variable 1 (flattened to `[e, e1, e2]` triples).
const PAIRS: u64 = 1;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    //   s[0]={}
    //   aux.AddValuesReset(function() ... end)
    // end)
    aux::global_check(d, ID, |d| {
        set_s_var(d, ID, LIST, Value::Multi(vec![]));
        set_s_var(d, ID, PAIRS, Value::Multi(vec![]));
        aux::add_values_reset(d, values_reset);
    });
}
/// The list `s[0]`
fn list(d: &Duel) -> Vec<EffectId> {
    match s_var(d, ID, LIST) {
        Value::Multi(v) => v.iter().filter_map(|x| x.as_effect()).collect(),
        _ => Vec::new(),
    }
}
fn set_list(d: &mut Duel, l: &[EffectId]) {
    set_s_var(d, ID, LIST, Value::Multi(l.iter().map(|e| Value::Effect(*e)).collect()));
}
/// The pairs `s[0][e]={e1,e2}`
fn pairs(d: &Duel) -> Vec<(EffectId, EffectId, EffectId)> {
    match s_var(d, ID, PAIRS) {
        Value::Multi(v) => v
            .chunks(3)
            .filter_map(|t| Some((t.first()?.as_effect()?, t.get(1)?.as_effect()?, t.get(2)?.as_effect()?)))
            .collect(),
        _ => Vec::new(),
    }
}
fn set_pairs(d: &mut Duel, p: &[(EffectId, EffectId, EffectId)]) {
    let v = p.iter().flat_map(|(a, b, c)| [Value::Effect(*a), Value::Effect(*b), Value::Effect(*c)]).collect();
    set_s_var(d, ID, PAIRS, Value::Multi(v));
}
/// `aux.AddValuesReset(function() for _,te in ipairs(s[0]) do ... end end)`
fn values_reset(d: &mut Duel) {
    // for _,te in ipairs(s[0]) do
    let mut i = 0;
    loop {
        // (ipairs reads the table fresh at every index: `s.reset` may remove entries)
        let Some(&te) = list(d).get(i) else { break };
        i += 1;
        // if Duel.GetTurnPlayer()==te:GetOwnerPlayer() then
        let owner = effect::get_owner_player(d, te);
        if duel::get_turn_player(d) == owner {
            // s.reset(te,te:GetOwnerPlayer(),nil,0,0,nil,0,0)
            reset(d, &EffArgs { e: te, tp: owner, eg: None, ep: 0, ev: 0, re: None, r: 0, rp: 0, chk: None, extra: vec![] });
        }
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ANCIENT_GEAR) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ANCIENT_GEAR as u16]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local tg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_ONFIELD,0,nil)
    let tg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-3 and #tg>=3 and aux.SelectUnselectGroup(tg,e,tp,3,3,aux.ChkfMMZ(1),0)
        let ok = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > -3 && tg.len() >= 3;
        return ok
            && matches!(
                aux::select_unselect_group(d, &tg, Some(a.e), a.tp, Some(3), Some(3), Some(&aux::chkf_mmz(1)), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=aux.SelectUnselectGroup(tg,e,tp,3,3,aux.ChkfMMZ(1),1,tp,HINTMSG_TOGRAVE)
    let g = aux::select_unselect_group(d, &tg, Some(a.e), a.tp, Some(3), Some(3), Some(&aux::chkf_mmz(1)), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return true;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // (ygoai override lua_fixes/c313513.lua: no e1:SetReset(RESET_SELF_TURN|RESET_PHASE|PHASE_END,2); the core
    // deleted e1 before s.reset's turn-end cleanup, which then raised on it; s.reset alone ends it)
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_MSET)
    effect::set_code(d, e2, EFFECT_CANNOT_MSET);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local e4=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e4 = effect::create_effect(d, h);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetReset(RESET_SELF_TURN|RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e4, RESET_SELF_TURN | RESET_PHASE | PHASE_END, 2);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, a.tp);
    // local descnum=tp==c:GetOwner() and 0 or 1
    let descnum = if a.tp == card::get_owner(d, c) { 0 } else { 1 };
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetDescription(aux.Stringid(id,descnum))
    effect::set_description(d, e3, aux::stringid(ID, descnum));
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetCode(1082946)
    effect::set_code(d, e3, 1082946);
    // e3:SetOwnerPlayer(tp)
    effect::set_owner_player(d, e3, a.tp);
    // e3:SetLabel(0)
    effect::set_label(d, e3, &[0]);
    // e3:SetOperation(s.reset)
    effect::set_operation(d, e3, Some(operation(reset)));
    // (ygoai override lua_fixes/c313513.lua: no e3:SetReset(RESET_SELF_TURN|RESET_PHASE|PHASE_END,2); the core
    // deleted e3 before s.reset's turn-end cleanup, which then raised on it; s.reset alone ends it)
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // table.insert(s[0],e3)
    let mut l = list(d);
    l.push(e3);
    set_list(d, &l);
    // s[0][e3]={e1,e2}
    let mut p = pairs(d);
    p.push((e3, e1, e2));
    set_pairs(d, &p);
    true
}
/// `s.reset(e,tp,eg,ep,ev,re,r,rp)`
fn reset(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local label=e:GetLabel()
    let label = effect::get_label(d, a.e);
    // label=label+1
    let label = label + 1;
    // e:SetLabel(label)
    effect::set_label(d, a.e, &[label]);
    // if ev==1082946 then c:SetTurnCounter(label) end
    if a.ev == 1082946 {
        card::set_turn_counter(d, c, label as u16);
    }
    // c:SetTurnCounter(0)
    card::set_turn_counter(d, c, 0);
    // if label==2 then
    if label == 2 {
        // local e1,e2=table.unpack(s[0][e])
        let Some(&(_, e1, e2)) = pairs(d).iter().find(|(k, _, _)| *k == a.e) else {
            crate::lua_error("bad argument #1 to 'unpack' (table expected, got nil)")
        };
        // e:Reset()
        effect::reset(d, a.e);
        // if e1 and not e1:IsDeleted() then e1:Reset() end
        // (ygoai override lua_fixes/c313513.lua: e1/e2 are oath effects, deleted by the core when the
        // activation was negated; the original raised here and left e in s[0])
        if !effect::is_deleted(d, e1) {
            effect::reset(d, e1);
        }
        // if e2 and not e2:IsDeleted() then e2:Reset() end
        if !effect::is_deleted(d, e2) {
            effect::reset(d, e2);
        }
        // s[0][e]=nil
        let mut p = pairs(d);
        p.retain(|(k, _, _)| *k != a.e);
        set_pairs(d, &p);
        // for i,te in ipairs(s[0]) do if te==e then table.remove(s[0],i) break end end
        let mut l = list(d);
        if let Some(i) = l.iter().position(|te| *te == a.e) {
            l.remove(i);
            set_list(d, &l);
        }
    }
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_ANCIENT_GEAR_GOLEM) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[CARD_ANCIENT_GEAR_GOLEM]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==0 and Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
        if effect::get_label(d, a.e) == 0 && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp)
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.dfilter(c)`
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() or c:GetCode()~=CARD_ANCIENT_GEAR_GOLEM
    card::is_facedown(d, c) || card::get_code(d, c)[0] != CARD_ANCIENT_GEAR_GOLEM
}
/// `s.spcheck(sg,e,tp,mg)`
fn spcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_HAND)<2 and sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)<2
    let res = group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX) < 2
        && group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) < 2;
    (res, false)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if ft>2 then ft=2 end
    if ft > 2 {
        ft = 2;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or g:FilterCount(Card.IsLocation,nil,LOCATION_HAND)<=0
    //   or g:FilterCount(Card.IsLocation,nil,LOCATION_DECK)<=0 then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
        || group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX) <= 0
        || group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) <= 0
    {
        ft = 1;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,ft,ft,s.spcheck,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(ft as usize), Some(ft as usize), Some(&spcheck), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if Duel.SpecialSummon(sg,0,tp,tp,true,false,POS_FACEUP)>0 then
    if duel::special_summon(d, sg, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) > 0 {
        // local dg=Duel.GetMatchingGroup(s.dfilter,tp,LOCATION_MZONE,0,nil)
        let dg = duel::get_matching_group(d, Some(&dfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
        // if #dg>0 then
        if !dg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
