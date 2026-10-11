//! Ｗ：Ｐファンシーボール
//! W:P Fancy Ball
use super::prelude::*;

pub const ID: u32 = 4993187;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ Effect Monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_type_ctx(d, x, TYPE_EFFECT, sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // When your opponent activates a monster effect on the field or GY (Quick Effect): You can banish this card that was Special Summoned this turn until the End Phase; negate that effect, and if you do, banish it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetCost(s.discost)
    effect::set_cost(d, e1, Some(cost_target(discost)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your opponent's Main Phase, you can (Quick Effect): Immediately after this effect resolves, Link Summon 1 Link Monster, and if this card you control would be used as material for that Link Summon, you can also use 1 Link-2 or lower monster your opponent controls
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
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsMainPhase(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_main_phase(d, Some(1 - a.tp)))));
    // e2:SetTarget(s.linktg)
    effect::set_target(d, e2, Some(cost_target(linktg)));
    // e2:SetOperation(s.linkop)
    effect::set_operation(d, e2, Some(operation(linkop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not e:GetHandler():IsStatus(STATUS_SPSUMMON_TURN) then return false end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_status(d, h, STATUS_SPSUMMON_TURN) {
        return false;
    }
    // local trig_loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let trig_loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int() as u32;
    // return ep==1-tp and re:IsMonsterEffect() and trig_loc&(LOCATION_MZONE|LOCATION_GRAVE)>0 and Duel.IsChainDisablable(ev)
    a.ep as i32 == 1 - a.tp as i32
        && effect::is_monster_effect(d, a.re.nil())
        && (trig_loc & (LOCATION_MZONE | LOCATION_GRAVE)) > 0
        && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None);
    }
    // Banish this card until the End Phase
    // aux.RemoveUntil(c,nil,REASON_COST,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp)
    aux::remove_until(
        d,
        c,
        None,
        REASON_COST,
        PHASE_END,
        ID,
        a.e,
        a.tp,
        Some(aux::default_field_return()),
        None,
        None,
        None,
        None,
        None,
    );
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // local relation=rc:IsRelateToEffect(re)
    let relation = card::is_relate_to_effect(d, rc, re);
    // if chk==0 then return rc:IsAbleToRemove(tp) or (not relation and Duel.IsPlayerCanRemove(tp)) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, rc, Some(a.tp), None, None)
            || (!relation && duel::is_player_can_remove(d, a.tp, None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    if relation {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, a.tp, 0);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,rc:GetPreviousLocation())
        let prev = card::get_previous_location(d, rc) as i32;
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, prev);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_effect(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, a.eg(), POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
/// `s.linktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn linktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_EXTRA_MATERIAL)
        effect::set_code(d, e1, EFFECT_EXTRA_MATERIAL);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetOperation(s.extracon)
        effect::set_operation(d, e1, Some(lua_fn(extracon)));
        // e1:SetValue(s.extraval)
        effect::set_value_fn(d, e1, lua_fn(extraval));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local res=Duel.IsExistingMatchingCard(Card.IsLinkSummonable,tp,LOCATION_EXTRA,0,1,nil)
        let res = duel::is_existing_matching_card(d, Some(&|d, x| card::is_link_summonable(d, x, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // e1:Reset()
        effect::reset(d, e1);
        // return res
        return res;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetChainLimit(function(re,rp,tp) return not (re:IsMonsterEffect() and re:GetHandler():IsLinkMonster()) end)
    duel::set_chain_limit(
        d,
        lua_fn(|d, a: Args| {
            let re = a.e(0);
            let monster = effect::is_monster_effect(d, re);
            let rh = effect::get_handler(d, re).nil();
            !(monster && card::is_link_monster(d, rh))
        }),
    );
    true
}
/// `s.linkop(e,tp,eg,ep,ev,re,r,rp)`
fn linkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=nil
    let mut e1: Option<EffectId> = None;
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // If this card you control would be used as material for that Link Summon, you can also use 1 Link-2 or lower monster your opponent controls
        // e1=Effect.CreateEffect(c)
        let x = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, x, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, x, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_EXTRA_MATERIAL)
        effect::set_code(d, x, EFFECT_EXTRA_MATERIAL);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, x, LOCATION_MZONE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, x, 1, 0);
        // e1:SetOperation(s.extracon)
        effect::set_operation(d, x, Some(lua_fn(extracon)));
        // e1:SetValue(s.extraval)
        effect::set_value_fn(d, x, lua_fn(extraval));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, x, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, x, false);
        e1 = Some(x);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,Card.IsLinkSummonable,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let sg = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| card::is_link_summonable(d, x, None, None, 0, 0)),
        tp,
        LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .unwrap_or_default();
    let sc = group::get_first(d, &sg);
    if let Some(sc) = sc {
        // Duel.LinkSummon(tp,sc)
        duel::link_summon(d, tp, sc, None, None, 0, 0);
        if let Some(e1) = e1 {
            // local eff_code=Duel.GetCurrentChain()==1 and EVENT_SPSUMMON or EVENT_SPSUMMON_SUCCESS
            let eff_code = if duel::get_current_chain(d, false) == 1 { EVENT_SPSUMMON } else { EVENT_SPSUMMON_SUCCESS };
            // Reset e1
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e2:SetCode(eff_code)
            effect::set_code(d, e2, eff_code);
            // e2:SetOperation(function(e) e1:Reset() e:Reset() end)
            effect::set_operation(
                d,
                e2,
                Some(operation(move |d, a| {
                    effect::reset(d, e1);
                    effect::reset(d, a.e);
                })),
            );
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, tp);
        }
    } else if let Some(e1) = e1 {
        // e1:Reset()
        effect::reset(d, e1);
    }
}
/// `s.extracon(c,e,tp,sg,mg,lc,og,chk)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let tp = a.p(2);
    let sg = a.g(3);
    let lc = a.card(5);
    // if not sg:IsContains(e:GetHandler()) then return false end
    let h = effect::get_handler(d, e).nil();
    if !sg.contains(h) {
        return false;
    }
    // local g=Duel.GetMatchingGroup(Card.IsLinkBelow,tp,0,LOCATION_MZONE,nil,2)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_link_below(d, x, 2)), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return true end
    if g.is_empty() {
        return true;
    }
    // local max_count=1
    let mut max_count: usize = 1;
    // local must_include=Group.CreateGroup()
    let mut must_include = group::create_group(&[]);
    // local effs={Duel.GetPlayerEffect(tp,EFFECT_EXTRA_MATERIAL)}
    let effs = duel::get_player_effect(d, tp, EFFECT_EXTRA_MATERIAL);
    // for _,eff in ipairs(effs) do
    for eff in effs {
        // if not eff:GetOwner():IsCode(id) then
        let owner = effect::get_owner(d, eff).nil();
        if !card::is_code(d, owner, &[ID]) {
            // if #(eff:GetValue()(0,SUMMON_TYPE_LINK,eff,tp,lc))>0 then
            let res = match effect::get_value(d, eff) {
                Value::Fn(f) => f.call(d, &[Value::Int(0), Value::Int(SUMMON_TYPE_LINK as i64), Value::Effect(eff), Value::Int(tp as i64), a.v(5).clone()]),
                _ => crate::lua_error("attempt to call a non-function value"),
            };
            if value_len(res) > 0 {
                // local handler=eff:GetHandler()
                let handler = effect::get_handler(d, eff).nil();
                // must_include:Merge(handler)
                group::add_card(&mut must_include, &CardSet::from_card(handler));
                // if #(sg&must_include)>0 or lc==handler then
                if !group::intersection(&sg, &must_include).is_empty() || lc == Some(handler) {
                    // max_count=max_count+1
                    max_count += 1;
                }
            }
        }
    }
    // return #(sg&g)>0 and #(sg&g)<=max_count
    let sg_g = group::intersection(&sg, &g).len();
    sg_g > 0 && sg_g <= max_count
}
/// `#v` of a Lua value (a group, or a table of values).
fn value_len(v: Value) -> usize {
    match v {
        Value::Group(g) => g.len(),
        Value::Multi(v) => v.len(),
        _ => crate::lua_error("attempt to get length of a non-table value"),
    }
}
/// `s.extraval(chk,summon_type,e,...)`
fn extraval(d: &mut Duel, a: Args) -> Value {
    let chk = a.int(0);
    let summon_type = a.int(1);
    // if chk==0 then
    if chk == 0 {
        // local tp,sc=...
        let tp = a.p(3);
        // if summon_type~=SUMMON_TYPE_LINK then return Group.CreateGroup()
        if summon_type != SUMMON_TYPE_LINK as i64 {
            return Value::Group(group::create_group(&[]));
        }
        // return Duel.GetMatchingGroup(Card.IsLinkBelow,tp,0,LOCATION_MZONE,nil,2)
        Value::Group(duel::get_matching_group(d, Some(&|d, x| card::is_link_below(d, x, 2)), tp, 0, LOCATION_MZONE, NO_EX))
    } else if chk == 1 {
        // local sg,sc,tp=...
        let sg = a.g(3);
        let tp = a.p(5);
        // if summon_type&SUMMON_TYPE_LINK==SUMMON_TYPE_LINK and #sg>0 then
        if (summon_type & SUMMON_TYPE_LINK as i64) == SUMMON_TYPE_LINK as i64 && !sg.is_empty() {
            // Duel.Hint(HINT_CARD,tp,id)
            duel::hint(d, HINT_CARD as u8, tp, ID as u64);
        }
        Value::Nil
    } else {
        Value::Nil
    }
}
