//! Double Spell
use super::prelude::*;

pub const ID: u32 = 24096228;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RANK_UP_MAGIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // copy spell
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsDiscardable() and c:IsSpell()
    card::is_discardable(d, c, None, None) && card::is_spell(d, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// The Lua arguments `(e,tp,eg,ep,ev,re,r,rp)` with `e` replaced by `e`, plus `chk` if given.
fn args_with(e: EffectId, a: &EffArgs, chk: Option<i64>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(x) = chk {
        v.push(Value::Int(x));
    }
    v
}
/// `s.filter1(c,e,tp,eg,ep,ev,re,r,rp)`
fn filter1(d: &mut Duel, c: CardId, a: &EffArgs) -> bool {
    // local te=c:CheckActivateEffect(false,false,false)
    let te = card::check_activate_effect(d, c, false, false, false).map(|x| x.0);
    // if c:IsSpell() and te then
    if card::is_spell(d, c) {
        if let Some(te) = te {
            // if c:IsSetCard(SET_RANK_UP_MAGIC) then
            if card::is_set_card(d, c, &[SET_RANK_UP_MAGIC as u16]) {
                // local tg=te:GetTarget()
                // return not tg or tg(e,tp,eg,ep,ev,re,r,rp,0)
                return match effect::get_target(d, te) {
                    None => true,
                    Some(tg) => tg.call(d, &args_with(a.e, a, Some(0))).truthy(),
                };
            } else {
                // return true
                return true;
            }
        }
    }
    // return false
    false
}
/// `s.filter2(c,e,tp,eg,ep,ev,re,r,rp)`
fn filter2(d: &mut Duel, c: CardId, a: &EffArgs) -> bool {
    // local te=c:CheckActivateEffect(false,false,false)
    let te = card::check_activate_effect(d, c, false, false, false).map(|x| x.0);
    // if c:IsSpell() and not c:IsType(TYPE_EQUIP|TYPE_CONTINUOUS) and te then
    if card::is_spell(d, c) && !card::is_type(d, c, TYPE_EQUIP | TYPE_CONTINUOUS) {
        if let Some(te) = te {
            // if c:IsSetCard(SET_RANK_UP_MAGIC) then
            if card::is_set_card(d, c, &[SET_RANK_UP_MAGIC as u16]) {
                // local tg=te:GetTarget()
                // return not tg or tg(e,tp,eg,ep,ev,re,r,rp,0)
                return match effect::get_target(d, te) {
                    None => true,
                    Some(tg) => tg.call(d, &args_with(a.e, a, Some(0))).truthy(),
                };
            } else {
                // return true
                return true;
            }
        }
    }
    // return false
    false
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local b=e:GetHandler():IsLocation(LOCATION_HAND)
        let h = effect::get_handler(d, a.e).nil();
        let b = card::is_location(d, h, LOCATION_HAND);
        // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
        let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
        // if (b and ft>1) or (not b and ft>0) then
        if (b && ft > 1) || (!b && ft > 0) {
            // return Duel.IsExistingTarget(s.filter1,tp,0,LOCATION_GRAVE,1,e:GetHandler(),e,tp,eg,ep,ev,re,r,rp)
            return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, a)), tp, 0, LOCATION_GRAVE, 1, h.into());
        } else {
            // return Duel.IsExistingTarget(s.filter2,tp,0,LOCATION_GRAVE,1,e:GetHandler(),e,tp,eg,ep,ev,re,r,rp)
            return duel::is_existing_target(d, Some(&|d, c| filter2(d, c, a)), tp, 0, LOCATION_GRAVE, 1, h.into());
        }
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
        // Duel.SelectTarget(tp,s.filter1,tp,0,LOCATION_GRAVE,1,1,nil,e,tp,eg,ep,ev,re,r,rp)
        duel::select_target(d, tp, Some(&|d, c| filter1(d, c, a)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    } else {
        // Duel.SelectTarget(tp,s.filter2,tp,0,LOCATION_GRAVE,1,1,nil,e,tp,eg,ep,ev,re,r,rp)
        duel::select_target(d, tp, Some(&|d, c| filter2(d, c, a)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not tc or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local tpe=tc:GetType()
    let tpe = card::get_type(d, tc);
    // local te=tc:GetActivateEffect()
    let te = card::get_activate_effect(d, tc).first().copied().nil();
    // local tg=te:GetTarget()
    let tg = effect::get_target(d, te);
    // local co=te:GetCost()
    let co = effect::get_cost(d, te);
    // local op=te:GetOperation()
    let op = effect::get_operation(d, te);
    // e:SetCategory(te:GetCategory())
    let cat = effect::get_category(d, te);
    effect::set_category(d, a.e, cat);
    // e:SetProperty(te:GetProperty())
    let (p1, p2) = effect::get_property(d, te);
    effect::set_property(d, a.e, p1, p2);
    // Duel.ClearTargetCard()
    duel::clear_target_card(d);
    // if (tpe&TYPE_EQUIP+TYPE_CONTINUOUS)~=0 or tc:IsHasEffect(EFFECT_REMAIN_FIELD) then
    if (tpe & (TYPE_EQUIP + TYPE_CONTINUOUS)) != 0 || !card::is_has_effect(d, tc, EFFECT_REMAIN_FIELD, None).is_empty() {
        // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    } else if (tpe & TYPE_FIELD) != 0 {
        // Duel.MoveToField(tc,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
    }
    // tc:CreateEffectRelation(te)
    card::create_effect_relation(d, tc, te);
    // if co then co(te,tp,eg,ep,ev,re,r,rp,1) end
    if let Some(co) = co {
        co.call(d, &args_with(te, a, Some(1)));
    }
    // if tg then
    if let Some(tg) = tg {
        // if tc:IsSetCard(SET_RANK_UP_MAGIC) then tg(e,tp,eg,ep,ev,re,r,rp,1)
        if card::is_set_card(d, tc, &[SET_RANK_UP_MAGIC as u16]) {
            tg.call(d, &args_with(a.e, a, Some(1)));
        } else {
            // else tg(te,tp,eg,ep,ev,re,r,rp,1) end
            tg.call(d, &args_with(te, a, Some(1)));
        }
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // for etc in aux.Next(g) do etc:CreateEffectRelation(te) end
    for etc in group::iter(d, &g) {
        card::create_effect_relation(d, etc, te);
    }
    // if op then
    if let Some(op) = op {
        // if tc:IsSetCard(SET_RANK_UP_MAGIC) then op(e,tp,eg,ep,ev,re,r,rp)
        if card::is_set_card(d, tc, &[SET_RANK_UP_MAGIC as u16]) {
            op.call(d, &args_with(a.e, a, None));
        } else {
            // else op(te,tp,eg,ep,ev,re,r,rp) end
            op.call(d, &args_with(te, a, None));
        }
    }
    // tc:ReleaseEffectRelation(te)
    card::release_effect_relation(d, tc, te);
    // for etc in aux.Next(g) do etc:ReleaseEffectRelation(te) end
    for etc in group::iter(d, &g) {
        card::release_effect_relation(d, etc, te);
    }
}
