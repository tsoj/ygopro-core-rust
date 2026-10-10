//! Thunder Dragon Thunderstormech
use super::prelude::*;

pub const ID: u32 = 12081875;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THUNDER_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ Thunder monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_THUNDER),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_THUNDER, sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // Apply 1 "Thunder Dragon" monster's effect that discards itself to activate, then place that monster on the top or bottom of your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.applytg)
    effect::set_target(d, e1, Some(cost_target(applytg)));
    // e1:SetOperation(s.applyop)
    effect::set_operation(d, e1, Some(operation(applyop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a Thunder monster(s) you control would be destroyed by battle or card effect, you can banish 3 cards from your GY instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e2, lua_fn(|d, a: Args| {
        let e = a.e(0);
        let c = a.c(1);
        let hp = effect::get_handler_player(d, e);
        repfilter(d, c, hp)
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
/// `s.runfn(fn,eff,tp,chk)`: `not fn or fn(eff,tp,Group.CreateGroup(),PLAYER_NONE,0,nil,REASON_EFFECT,PLAYER_NONE,chk)`
fn runfn(d: &mut Duel, f: Option<LuaFn>, eff: EffectId, tp: u8, chk: i64) -> bool {
    match f {
        None => true,
        Some(f) => f
            .call(
                d,
                &[
                    Value::Effect(eff),
                    Value::Int(tp as i64),
                    Value::Group(CardSet::new()),
                    Value::Int(PLAYER_NONE as i64),
                    Value::Int(0),
                    Value::Nil,
                    Value::Int(REASON_EFFECT as i64),
                    Value::Int(PLAYER_NONE as i64),
                    Value::Int(chk),
                ],
            )
            .truthy(),
    }
}
/// `s.applyfilter(c,e,tp)`
fn applyfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_THUNDER_DRAGON) and c:IsMonster()
    //     and c:IsAbleToDeck() and c:IsFaceup()) then return false end
    if !(card::is_set_card(d, c, &[SET_THUNDER_DRAGON as u16]) && card::is_monster(d, c) && card::is_able_to_deck(d, c) && card::is_faceup(d, c)) {
        return false;
    }
    // for _,eff in ipairs({c:GetOwnEffects()}) do
    for eff in card::get_own_effects(d, c) {
        // if eff:HasSelfDiscardCost() and s.runfn(eff:GetCondition(),eff,tp,0) and s.runfn(eff:GetTarget(),eff,tp,0) then return true end
        if effect::has_self_discard_cost(d, eff) {
            let cond = effect::get_condition(d, eff);
            if runfn(d, cond, eff, tp, 0) {
                let targ = effect::get_target(d, eff);
                if runfn(d, targ, eff, tp, 0) {
                    return true;
                }
            }
        }
    }
    // end (falls through: nil, which is false)
    let _ = e;
    false
}
/// `s.applytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn applytg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.applyfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| applyfilter(d, c, e, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.applyfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_target(d, tp, Some(&|d, c| applyfilter(d, c, e, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // local effs={}
    let mut effs: Vec<EffectId> = Vec::new();
    // local options={}
    let mut options: Vec<(bool, u64)> = Vec::new();
    // for _,eff in ipairs({tc:GetOwnEffects()}) do
    for eff in card::get_own_effects(d, tc) {
        // if eff:HasSelfDiscardCost() then
        if effect::has_self_discard_cost(d, eff) {
            // table.insert(effs,eff)
            effs.push(eff);
            // local eff_chk=s.runfn(eff:GetCondition(),eff,tp,0) and s.runfn(eff:GetTarget(),eff,tp,0)
            let cond = effect::get_condition(d, eff);
            let mut eff_chk = runfn(d, cond, eff, tp, 0);
            if eff_chk {
                let targ = effect::get_target(d, eff);
                eff_chk = runfn(d, targ, eff, tp, 0);
            }
            // table.insert(options,{eff_chk,eff:GetDescription()})
            let desc = effect::get_description(d, eff);
            options.push((eff_chk, desc));
        }
    }
    // local op=#options==1 and 1 or Duel.SelectEffect(tp,table.unpack(options))
    let op = if options.len() == 1 { Some(1) } else { duel::select_effect(d, tp, &options) };
    // local te=effs[op]
    let te = effs[op.nil() - 1];
    // Duel.Hint(HINT_OPSELECTED,1-tp,te:GetDescription())
    let te_desc = effect::get_description(d, te);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, te_desc);
    // Duel.ClearTargetCard()
    duel::clear_target_card(d);
    // tc:CreateEffectRelation(e)
    card::create_effect_relation(d, tc, e);
    // e:SetLabel(te:GetLabel())
    let te_label = effect::get_label(d, te);
    effect::set_label(d, e, &[te_label]);
    // e:SetLabelObject(te:GetLabelObject())
    let te_lo = effect::get_label_object(d, te);
    effect::set_label_object(d, e, te_lo);
    // local targ_fn=te:GetTarget()
    let targ_fn = effect::get_target(d, te);
    // if targ_fn then
    if targ_fn.is_some() {
        // s.runfn(targ_fn,e,tp,1)
        runfn(d, targ_fn, e, tp, 1);
        // te:SetLabel(e:GetLabel())
        let l = effect::get_label(d, e);
        effect::set_label(d, te, &[l]);
        // te:SetLabelObject(e:GetLabelObject())
        let lo = effect::get_label_object(d, e);
        effect::set_label_object(d, te, lo);
        // Duel.ClearOperationInfo(0)
        duel::clear_operation_info(d, 0);
    }
    // e:SetLabelObject(te)
    effect::set_label_object(d, e, Value::Effect(te));
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(tc)), 1, tp, 0);
    true
}
/// `s.applyop(e,tp,eg,ep,ev,re,r,rp)`
fn applyop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local te=e:GetLabelObject()
    // if not te then return end
    let Some(te) = effect::get_label_object(d, e).as_effect() else {
        return;
    };
    // local tc=te:GetHandler()
    let tc = effect::get_handler(d, te).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local op=te:GetOperation()
    let op = effect::get_operation(d, te);
    // if op then
    if let Some(op) = op {
        // e:SetLabel(te:GetLabel())
        let l = effect::get_label(d, te);
        effect::set_label(d, e, &[l]);
        // e:SetLabelObject(te:GetLabelObject())
        let lo = effect::get_label_object(d, te);
        effect::set_label_object(d, e, lo);
        // op(e,tp,eg,ep,ev,re,r,rp)
        let re = match a.re {
            Some(re) => Value::Effect(re),
            None => Value::Nil,
        };
        op.call(
            d,
            &[
                Value::Effect(e),
                Value::Int(tp as i64),
                Value::Group(a.eg()),
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                re,
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
            ],
        );
    }
    // if tc:IsAbleToDeck() then
    if card::is_able_to_deck(d, tc) {
        // local opt=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 and Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2)) or 0
        let opt = if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0 {
            duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
        } else {
            0
        };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoDeck(tc,nil,opt,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, opt, REASON_EFFECT, None);
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    // e:SetLabelObject(nil)
    effect::set_label_object(d, e, Value::Nil);
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_THUNDER) and c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    //     and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_race(d, c, RACE_THUNDER)
        && card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.repcostfilter(c)`
fn repcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp) and Duel.IsExistingMatchingCard(s.repcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,3,nil) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| repfilter(d, c, tp), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&repcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 3, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.repcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,3,3,nil)
        let g = duel::select_matching_card(d, tp, Some(&repcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 3, 3, false, NO_EX).nil();
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
