//! Underworld Goddess of the Closed World
use super::prelude::*;

pub const ID: u32 = 98127546;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),4)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 4, None, None, None, None);
    // You can also use 1 monster your opponent controls as material to Link Summon this card
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_PLAYER_TARGET|EFFECT_FLAG_CANNOT_DISABLE|EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e0, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e0:SetCode(EFFECT_EXTRA_MATERIAL)
    effect::set_code(d, e0, EFFECT_EXTRA_MATERIAL);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetTargetRange(1,1)
    effect::set_target_range(d, e0, 1, 1);
    // e0:SetOperation(s.extracon)
    effect::set_operation(d, e0, Some(lua_fn(extracon)));
    // e0:SetValue(s.extraval)
    effect::set_value_fn(d, e0, lua_fn(extraval));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Negate the effects of all face-up monsters your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(lua_fn(e1_cond)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Unaffected by your opponent's activated effects, unless they target this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.immval)
    effect::set_value_fn(d, e2, lua_fn(immval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Negate the activation of an opponent's effect that includes Special Summoning a monster(s) from the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e3, CATEGORY_NEGATE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.spnegcon)
    effect::set_condition(d, e3, Some(condition(spnegcon)));
    // e3:SetTarget(s.spnegtg)
    effect::set_target(d, e3, Some(cost_target(spnegtg)));
    // e3:SetOperation(function(e,tp,eg,ep,ev) Duel.NegateActivation(ev) end)
    effect::set_operation(d, e3, Some(operation(spnegop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// `s.curgroup=nil` at load time: a variable that is nil until extraval sets it (the duel's default)

/// Key of `s.curgroup` in the duel's script variables
const CURGROUP: u64 = 0;

/// `s.curgroup`: the group that `extraval` gave out, or nil
fn curgroup(d: &Duel) -> Option<CardSet> {
    match s_var(d, ID, CURGROUP) {
        Value::Group(g) => Some(g),
        _ => None,
    }
}
/// `s.closed_sky_filter(c)`: `not (c:HasFlagEffect(71818935) and #c:GetCardTarget()>0)`
fn closed_sky_filter(d: &mut Duel, c: CardId) -> bool {
    !(card::has_flag_effect(d, c, 71818935, None) && card::get_card_target(d, c).len() > 0)
}
/// `s.extracon(c,e,tp,sg,mg,lc,og,chk)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    // if not s.curgroup then return true end
    let Some(cg) = curgroup(d) else {
        return true;
    };
    // local g=s.curgroup:Filter(s.closed_sky_filter,nil)
    let g = group::filter(d, &cg, &closed_sky_filter, NO_EX);
    // local max_count=1
    let mut max_count = 1;
    // local must_include=Group.CreateGroup()
    let mut must_include = group::create_group(&[]);
    let tp = a.p(2);
    let sg = a.g(3);
    let lc = a.card(5);
    // local effs={Duel.GetPlayerEffect(tp,EFFECT_EXTRA_MATERIAL)}
    let effs = duel::get_player_effect(d, tp, EFFECT_EXTRA_MATERIAL);
    // for _,eff in ipairs(effs) do
    for eff in effs {
        // if not eff:GetOwner():IsCode(id) then
        let owner = effect::get_owner(d, eff).nil();
        if !card::is_code(d, owner, &[ID]) {
            // if #(eff:GetValue()(0,SUMMON_TYPE_LINK,eff,tp,lc))>0 then
            let value = effect::get_value(d, eff);
            let r = value.as_fn().nil().call(
                d,
                &[Value::Int(0), Value::Int(SUMMON_TYPE_LINK as i64), Value::Effect(eff), Value::Int(tp as i64), Value::card(lc)],
            );
            let n = match r {
                Value::Group(g) => g.len(),
                _ => crate::lua_error("attempt to get length of a nil value"),
            };
            if n > 0 {
                // local handler=eff:GetHandler()
                let handler = effect::get_handler(d, eff).nil();
                // must_include:Merge(handler)
                group::add_card(&mut must_include, &CardSet::from_card(handler));
                // if #(sg&must_include)>0 or lc==handler then
                if group::intersection(&sg, &must_include).len() > 0 || lc == Some(handler) {
                    // max_count=max_count+1
                    max_count += 1;
                }
            }
        }
    }
    // return #(sg&g)<=max_count
    group::intersection(&sg, &g).len() <= max_count
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
        // if summon_type~=SUMMON_TYPE_LINK or sc~=e:GetHandler() then
        let h = effect::get_handler(d, e).nil();
        if summon_type != SUMMON_TYPE_LINK as i64 || sc != Some(h) {
            // return Group.CreateGroup()
            return Value::Group(group::create_group(&[]));
        }
        // s.curgroup=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
        // s.curgroup:KeepAlive()
        set_s_var(d, ID, CURGROUP, Value::Group(g.clone()));
        // return s.curgroup
        return Value::Group(g);
    } else if chk == 2 {
        // if s.curgroup then s.curgroup:DeleteGroup() end
        // s.curgroup=nil
        set_s_var(d, ID, CURGROUP, Value::Nil);
    }
    Value::Nil
}
/// `function(e) return e:GetHandler():IsLinkSummoned() end`
fn e1_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_link_summoned(d, h)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_negatable_monster), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&card::is_negatable_monster), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // tc:NegateEffects(c)
        card::negate_effects(d, tc, c, None, false, None);
    }
}
/// `s.immval(e,re)`
fn immval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not (re:IsActivated() and c:IsLinkSummoned() and e:GetOwnerPlayer()==1-re:GetOwnerPlayer()) then return false end
    if !(effect::is_activated(d, re) && card::is_link_summoned(d, c) && effect::get_owner_player(d, e) == 1 - effect::get_owner_player(d, re)) {
        return false;
    }
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return true end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return true;
    }
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    // return not g or not g:IsContains(c)
    match info.first() {
        Some(Value::Group(g)) => !g.contains(c),
        _ => true,
    }
}
/// `s.spnegcon(e,tp,eg,ep,ev,re,r,rp)`
fn spnegcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) or rp==tp or not Duel.IsChainNegatable(ev) then return false end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_status(d, h, STATUS_BATTLE_DESTROYED) || a.rp == a.tp || !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    // local ex1,tg1,ct1,p1,loc1=Duel.GetOperationInfo(ev,CATEGORY_SPECIAL_SUMMON)
    let info1 = duel::get_operation_info(d, a.ev, CATEGORY_SPECIAL_SUMMON);
    // local ex2,tg2,ct2,p2,loc2=Duel.GetPossibleOperationInfo(ev,CATEGORY_SPECIAL_SUMMON)
    let info2 = duel::get_possible_operation_info(d, a.ev, CATEGORY_SPECIAL_SUMMON);
    // local res1=((loc1 or 0)&LOCATION_GRAVE>0) or (tg1 and tg1:IsExists(function(c) return c:IsLocation(LOCATION_GRAVE) and c:IsMonster() end,1,nil))
    let res1 = grave_summon_info(d, info1);
    // local res2=((loc2 or 0)&LOCATION_GRAVE>0) or (tg2 and tg2:IsExists(function(c) return c:IsLocation(LOCATION_GRAVE) and c:IsMonster() end,1,nil))
    let res2 = grave_summon_info(d, info2);
    // return res1 or res2
    res1 || res2
}
/// `((loc or 0)&LOCATION_GRAVE>0) or (tg and tg:IsExists(...))` of an operation info (`None`: ex is false)
fn grave_summon_info(d: &mut Duel, info: Option<ygo_core::duel::OpTarget>) -> bool {
    let Some(info) = info else {
        return false;
    };
    (info.op_param & LOCATION_GRAVE as i32) > 0
        || info.op_cards.is_some_and(|tg| group::is_exists(d, &tg, &|d, c| card::is_location(d, c, LOCATION_GRAVE) && card::is_monster(d, c), 1, NO_EX))
}
/// `s.spnegtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spnegtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    true
}
/// `function(e,tp,eg,ep,ev) Duel.NegateActivation(ev) end`
fn spnegop(d: &mut Duel, a: &EffArgs) {
    duel::negate_activation(d, a.ev as u8);
}
