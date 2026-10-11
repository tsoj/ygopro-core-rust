//! Tellarknight Constellar Caduceus
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 58858807;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT, SET_CONSTELLAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Add 1 "tellarknight" and/or 1 "Constellar" card from your GY to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This effect becomes the effect of the banished "tellarknight" or "Constellar" monster that activates when it is Normal Summoned
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.AND(s.applycost,Cost.DetachFromSelf(1)))
    effect::set_cost(d, e2, Some(cost::and(vec![cost_target(applycost), cost::detach_from_self(cost::Count::N(1), None, None)])));
    // e2:SetTarget(s.applytg)
    effect::set_target(d, e2, Some(cost_target(applytg)));
    // e2:SetOperation(s.applyop)
    effect::set_operation(d, e2, Some(operation(applyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsXyzSummoned() end` (of e1)
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h)
}
/// `s.thfilter(c,e)`
fn thfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsSetCard({SET_TELLARKNIGHT,SET_CONSTELLAR}) and c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16, SET_CONSTELLAR as u16]) && card::is_able_to_hand(d, c, None) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return #sg==1 or (sg:IsExists(Card.IsSetCard,1,nil,SET_TELLARKNIGHT) and sg:IsExists(Card.IsSetCard,1,nil,SET_CONSTELLAR))
    if sg.len() == 1 {
        return (true, false);
    }
    let a = group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16]), 1, NO_EX);
    let b = group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_CONSTELLAR as u16]), 1, NO_EX);
    (a && b, false)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, a.e)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon,1,tp,HINTMSG_ATOHAND)
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,#tg,tp,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), n, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #tg>0 then Duel.SendtoHand(tg,nil,REASON_EFFECT) end
    if !tg.is_empty() {
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
/// The arguments of a `(e,tp,eg,ep,ev,re,r,rp,chk)` call made by this script, `e` being `e`
/// (`f(e,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,0)`).
fn call_condition_target(d: &mut Duel, f: &LuaFn, eff: EffectId, tp: u8, e: EffectId) -> bool {
    f.call(
        d,
        &[
            Value::Effect(eff),
            Value::Int(tp as i64),
            Value::Group(CardSet::new()),
            Value::Int(PLAYER_NONE as i64),
            Value::Int(0),
            Value::Effect(e),
            Value::Int(REASON_EFFECT as i64),
            Value::Int(PLAYER_NONE as i64),
            Value::Int(0),
        ],
    )
    .truthy()
}
/// `(con==nil or con(eff,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,0))`
fn check_fn(d: &mut Duel, f: &Option<LuaFn>, eff: EffectId, tp: u8, e: EffectId) -> bool {
    match f {
        None => true,
        Some(f) => call_condition_target(d, f, eff, tp, e),
    }
}
/// `s.rmfilter(c,e,tp)`
fn rmfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsSetCard({SET_TELLARKNIGHT,SET_CONSTELLAR}) and c:IsMonster() and c:IsAbleToRemoveAsCost()) then return false end
    if !(card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16, SET_CONSTELLAR as u16]) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)) {
        return false;
    }
    // local effs={c:GetOwnEffects()}
    // for _,eff in ipairs(effs) do
    for eff in card::get_own_effects(d, c) {
        // if eff:GetCode()==EVENT_SUMMON_SUCCESS then
        if effect::get_code(d, eff) == EVENT_SUMMON_SUCCESS {
            // local con=eff:GetCondition()
            let con = effect::get_condition(d, eff);
            // local tg=eff:GetTarget()
            let tg = effect::get_target(d, eff);
            // if (con==nil or con(...)) and (tg==nil or tg(...)) then return true end
            if check_fn(d, &con, eff, tp, e) && check_fn(d, &tg, eff, tp, e) {
                return true;
            }
        }
    }
    // return false
    false
}
/// `s.applycost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn applycost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| rmfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| rmfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &g).nil();
    // Duel.Remove(rc,POS_FACEUP,REASON_COST)
    duel::remove(d, rc, POS_FACEUP as u8, REASON_COST, None, None);
    // local available_effs={}
    let mut available_effs: Vec<Value> = Vec::new();
    // local effs={rc:GetOwnEffects()}
    // for _,eff in ipairs(effs) do
    for eff in card::get_own_effects(d, rc) {
        // if eff:GetCode()==EVENT_SUMMON_SUCCESS then
        if effect::get_code(d, eff) == EVENT_SUMMON_SUCCESS {
            // local con=eff:GetCondition()
            let con = effect::get_condition(d, eff);
            // local tg=eff:GetTarget()
            let tg = effect::get_target(d, eff);
            // if (con==nil or con(...)) and (tg==nil or tg(...)) then table.insert(available_effs,eff) end
            if check_fn(d, &con, eff, a.tp, a.e) && check_fn(d, &tg, eff, a.tp, a.e) {
                available_effs.push(Value::Effect(eff));
            }
        }
    }
    // e:SetLabelObject(available_effs)
    effect::set_label_object(d, a.e, Value::Multi(available_effs));
    true
}
/// The effects of the list kept in the label object of `e` (`e:SetLabelObject(available_effs)`).
fn list_effects(v: Value) -> Vec<EffectId> {
    match v {
        Value::Multi(v) => v.into_iter().filter_map(|x| x.as_effect()).collect(),
        _ => Vec::new(),
    }
}
/// `s.applytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn applytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local eff=e:GetLabelObject()
        // return eff and eff:GetTarget() and eff:GetTarget()(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        let lo = effect::get_label_object(d, a.e);
        let Some(eff) = lo.as_effect() else {
            return false;
        };
        let Some(tg) = effect::get_target(d, eff) else {
            return false;
        };
        let eg = a.eg.clone().map_or(Value::Nil, Value::Group);
        return tg
            .call(
                d,
                &[
                    Value::Effect(a.e),
                    Value::Int(a.tp as i64),
                    eg,
                    Value::Int(a.ep as i64),
                    Value::Int(a.ev as i64),
                    Value::effect(a.re),
                    Value::Int(a.r as i64),
                    Value::Int(a.rp as i64),
                    Value::Int(0),
                    Value::Card(chkc),
                ],
            )
            .truthy();
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local eff=nil
    // local available_effs=e:GetLabelObject()
    let available_effs = list_effects(effect::get_label_object(d, a.e));
    // if #available_effs>1 then
    let eff = if available_effs.len() > 1 {
        // local available_effs_desc={}
        // for _,eff in ipairs(available_effs) do table.insert(available_effs_desc,eff:GetDescription()) end
        let descs: Vec<u64> = available_effs.iter().map(|&x| effect::get_description(d, x)).collect();
        // local op=Duel.SelectOption(tp,table.unpack(available_effs_desc))
        let op = duel::select_option(d, a.tp, true, &descs);
        // eff=available_effs[op+1]
        available_effs[op as usize]
    } else {
        // eff=available_effs[1]
        available_effs.first().copied().nil()
    };
    // Duel.Hint(HINT_OPSELECTED,1-tp,eff:GetDescription())
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, effect::get_description(d, eff));
    // e:SetLabel(eff:GetLabel())
    let lbl = effect::get_label(d, eff);
    effect::set_label(d, a.e, &[lbl]);
    // e:SetLabelObject(eff:GetLabelObject())
    let eff_lo = effect::get_label_object(d, eff);
    effect::set_label_object(d, a.e, eff_lo);
    // e:SetProperty(eff:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and EFFECT_FLAG_CARD_TARGET or 0)
    let prop = if effect::is_has_property(d, eff, EFFECT_FLAG_CARD_TARGET, 0) { EFFECT_FLAG_CARD_TARGET } else { 0 };
    effect::set_property(d, a.e, prop, 0);
    // local tg=eff:GetTarget()
    // if tg then tg(e,tp,eg,ep,ev,re,r,rp,1) end
    if let Some(tg) = effect::get_target(d, eff) {
        let eg = a.eg.clone().map_or(Value::Nil, Value::Group);
        tg.call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(a.tp as i64),
                eg,
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                Value::effect(a.re),
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
                Value::Int(1),
            ],
        );
    }
    // eff:SetLabel(e:GetLabel())
    let e_lbl = effect::get_label(d, a.e);
    effect::set_label(d, eff, &[e_lbl]);
    // eff:SetLabelObject(e:GetLabelObject())
    let e_lo = effect::get_label_object(d, a.e);
    effect::set_label_object(d, eff, e_lo);
    // e:SetLabelObject(eff)
    effect::set_label_object(d, a.e, Value::Effect(eff));
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
/// `s.applyop(e,tp,eg,ep,ev,re,r,rp)`
fn applyop(d: &mut Duel, a: &EffArgs) {
    // local eff=e:GetLabelObject()
    // if not eff then return end
    let Some(eff) = effect::get_label_object(d, a.e).as_effect() else {
        return;
    };
    // e:SetLabel(eff:GetLabel())
    let lbl = effect::get_label(d, eff);
    effect::set_label(d, a.e, &[lbl]);
    // e:SetLabelObject(eff:GetLabelObject())
    let lo = effect::get_label_object(d, eff);
    effect::set_label_object(d, a.e, lo);
    // local op=eff:GetOperation()
    // if op then op(e,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE) end
    if let Some(op) = effect::get_operation(d, eff) {
        op.call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(a.tp as i64),
                Value::Group(CardSet::new()),
                Value::Int(PLAYER_NONE as i64),
                Value::Int(0),
                Value::Effect(a.e),
                Value::Int(REASON_EFFECT as i64),
                Value::Int(PLAYER_NONE as i64),
            ],
        );
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // e:SetLabelObject(nil)
    effect::set_label_object(d, a.e, Value::Nil);
}
