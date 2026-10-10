//! Bait Doll
use super::prelude::*;

pub const ID: u32 = 7165085;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:GetSequence()<5
    card::is_facedown(d, c) && card::get_sequence(d, c) < 5
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_SZONE,LOCATION_SZONE,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_SZONE, LOCATION_SZONE, 1, c.into());
    }
    // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEDOWN)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEDOWN as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, c.into());
    true
}
/// `f(te,tep,eg,ep,ev,re,r,rp)`, or with `chk` as the 9th argument: a function of the activate
/// effect `te` of the trap, called with the arguments of this operation.
fn call_te(d: &mut Duel, f: &LuaFn, te: EffectId, tep: u8, a: &EffArgs, chk: Option<i64>) -> Value {
    let mut args = vec![
        Value::Effect(te),
        Value::Int(tep as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        args.push(Value::Int(chk));
    }
    f.call(d, &args)
}
/// `not f or f(...)`: whether the optional function `f` is absent or returns a true value.
fn opt_ok(d: &mut Duel, f: &Option<LuaFn>, te: EffectId, tep: u8, a: &EffArgs, chk: Option<i64>) -> bool {
    match f {
        None => true,
        Some(f) => call_te(d, f, te, tep, a, chk).truthy(),
    }
}
/// Lua's `f1==f2` for functions: the same function object.
fn same_fn(f1: &Option<LuaFn>, f2: &Option<LuaFn>) -> bool {
    match (f1, f2) {
        (None, None) => true,
        (Some(x), Some(y)) => x.ptr_eq(y),
        _ => false,
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsFaceup() then
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_faceup(d, tc) {
        // s.todeckop(c,e)
        todeckop(d, c, a.e);
        // return
        return;
    }
    // if tc:IsTrap() then
    if card::is_trap(d, tc) {
        // local te=tc:GetActivateEffect()
        let te = card::get_activate_effect(d, tc).first().copied();
        // local tep=tc:GetControler()
        let tep = card::get_controler(d, tc);
        // local condition
        // local cost
        // local target
        // local operation
        let mut condition = None;
        let mut cost = None;
        let mut target_f = None;
        let mut operation_f = None;
        // if te then
        if let Some(te) = te {
            // condition=te:GetCondition()
            condition = effect::get_condition(d, te);
            // cost=te:GetCost()
            cost = effect::get_cost(d, te);
            // target=te:GetTarget()
            target_f = effect::get_target(d, te);
            // operation=te:GetOperation()
            operation_f = effect::get_operation(d, te);
        }
        // local chk=te and te:GetCode()==EVENT_FREE_CHAIN and te:IsActivatable(tep)
        //   and (not condition or condition(te,tep,eg,ep,ev,re,r,rp))
        //   and (not cost or cost(te,tep,eg,ep,ev,re,r,rp,0))
        //   and (not target or target(te,tep,eg,ep,ev,re,r,rp,0))
        let chk = match te {
            None => false,
            Some(te) => {
                effect::get_code(d, te) == EVENT_FREE_CHAIN
                    && effect::is_activatable(d, te, tep, false, false)
                    && opt_ok(d, &condition, te, tep, a, None)
                    && opt_ok(d, &cost, te, tep, a, Some(0))
                    && opt_ok(d, &target_f, te, tep, a, Some(0))
            }
        };
        // Duel.ChangePosition(tc,POS_FACEUP)
        duel::change_position(d, tc, POS_FACEUP as u8, None, None, None, false);
        // Duel.ConfirmCards(tp,tc)
        duel::confirm_cards(d, a.tp, CardSet::from_card(tc));
        // if chk then
        if chk {
            let te = te.nil();
            // Duel.ClearTargetCard()
            duel::clear_target_card(d);
            // e:SetProperty(te:GetProperty())
            let (p1, p2) = effect::get_property(d, te);
            effect::set_property(d, a.e, p1, p2);
            // Duel.Hint(HINT_CARD,0,tc:GetOriginalCode())
            let code = card::get_original_code(d, tc);
            duel::hint(d, HINT_CARD as u8, 0, code as u64);
            // if tc:IsNormalTrap() then
            if card::is_normal_trap(d, tc) {
                // tc:CancelToGrave(false)
                card::cancel_to_grave(d, tc, false);
            }
            // tc:CreateEffectRelation(te)
            card::create_effect_relation(d, tc, te);
            // if cost then cost(te,tep,eg,ep,ev,re,r,rp,1) end
            if let Some(f) = &cost {
                call_te(d, f, te, tep, a, Some(1));
            }
            // if target~=te:GetTarget() then
            let target_now = effect::get_target(d, te);
            if !same_fn(&target_f, &target_now) {
                // target=te:GetTarget()
                target_f = target_now;
            }
            // if target then target(te,tep,eg,ep,ev,re,r,rp,1) end
            if let Some(f) = &target_f {
                call_te(d, f, te, tep, a, Some(1));
            }
            // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
            let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
            // for tg in g:Iter() do tg:CreateEffectRelation(te) end
            for tg in group::iter(d, &g) {
                card::create_effect_relation(d, tg, te);
            }
            // if tc:IsHasEffect(EFFECT_REMAIN_FIELD) then
            if !card::is_has_effect(d, tc, EFFECT_REMAIN_FIELD, None).is_empty() {
                // tc:SetStatus(STATUS_LEAVE_CONFIRMED,false)
                card::set_status(d, tc, STATUS_LEAVE_CONFIRMED, false);
            }
            // if operation~=te:GetOperation() then
            let operation_now = effect::get_operation(d, te);
            if !same_fn(&operation_f, &operation_now) {
                // operation=te:GetOperation()
                operation_f = operation_now;
            }
            // if operation then operation(te,tep,eg,ep,ev,re,r,rp) end
            if let Some(f) = &operation_f {
                call_te(d, f, te, tep, a, None);
            }
            // tc:ReleaseEffectRelation(te)
            card::release_effect_relation(d, tc, te);
            // for tg in g:Iter() do tg:ReleaseEffectRelation(te) end
            for tg in group::iter(d, &g) {
                card::release_effect_relation(d, tg, te);
            }
        } else {
            // if Duel.Destroy(tc,REASON_EFFECT)==0 then
            if duel::destroy(d, tc, REASON_EFFECT, None, None) == 0 {
                // Duel.SendtoGrave(tc,REASON_RULE)
                duel::send_to_grave(d, tc, REASON_RULE, None, None);
            }
        }
    } else {
        // Duel.ConfirmCards(tp,tc)
        duel::confirm_cards(d, a.tp, CardSet::from_card(tc));
    }
    // s.todeckop(c,e)
    todeckop(d, c, a.e);
}
/// `s.todeckop(c,e)`
fn todeckop(d: &mut Duel, c: CardId, e: EffectId) {
    // if c:IsRelateToEffect(e) and e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if card::is_relate_to_effect(d, c, e) && effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        // c:CancelToGrave()
        card::cancel_to_grave(d, c, true);
        // Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
