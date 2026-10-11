//! Darklord Eveningstar
use super::prelude::*;

pub const ID: u32 = 10136446;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARKLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 2 Level 6 or higher DARK Fairy monsters
    // Fusion.AddProcMixN(c,true,true,s.matfilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))), 2)]);
    // Your opponent cannot target this card with card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 "Darklord" Spell and 1 "Darklord" Trap from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e2, Some(condition(fusion_cond)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Apply the effect of 1 "Darklord" Spell/Trap in your GY, then shuffle it into the Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e3, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e3:SetTarget(s.copytg)
    effect::set_target(d, e3, Some(cost_target(copytg)));
    // e3:SetOperation(s.copyop)
    effect::set_operation(d, e3, Some(operation(copyop)));
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,fc,sumtype,sump)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, sump: u8) -> bool {
    // return c:IsLevelAbove(6) and c:IsAttribute(ATTRIBUTE_DARK,fc,sumtype,sump) and c:IsRace(RACE_FAIRY,fc,sumtype,sump)
    card::is_level_above(d, c, 6)
        && card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, Some(fc), sumtype, Some(sump))
        && card::is_race_ctx(d, c, RACE_FAIRY as u64, Some(fc), sumtype, Some(sump))
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end`
fn fusion_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DARKLORD) and c:IsSpellTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_DARKLORD as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.copyfilter(c)`
fn copyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DARKLORD) and c:IsSpellTrap() and c:IsAbleToDeck() and c:CheckActivateEffect(false,true,false)~=nil
    card::is_set_card(d, c, &[SET_DARKLORD as u16])
        && card::is_spell_trap(d, c)
        && card::is_able_to_deck(d, c)
        && card::check_activate_effect(d, c, false, true, false).is_some()
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    if a.chk == Some(0) {
        // local stzone_chk=Duel.GetLocationCount(tp,LOCATION_SZONE)>=2
        let stzone_chk = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 >= 2;
        // local g=Duel.GetMatchingGroup(s.setfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| setfilter(d, c)), tp, LOCATION_DECK, 0, NO_EX);
        // return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon(stzone_chk),0)
        return g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon(stzone_chk)), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    true
}
/// `s.rescon(stzone_chk)`: the returned `function(sg,e,tp,mg)`
fn rescon(stzone_chk: bool) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| {
        // return #sg==2 and (stzone_chk or sg:IsExists(Card.IsFieldSpell,1,nil)) and sg:IsExists(Card.IsSpell,1,nil) and sg:IsExists(Card.IsTrap,1,nil)
        let res = sg.len() == 2
            && (stzone_chk || group::is_exists(d, sg, &|d, c| card::is_field_spell(d, c), 1, NO_EX))
            && group::is_exists(d, sg, &|d, c| card::is_spell(d, c), 1, NO_EX)
            && group::is_exists(d, sg, &|d, c| card::is_trap(d, c), 1, NO_EX);
        (res, false)
    }
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local stzone_chk=Duel.GetLocationCount(tp,LOCATION_SZONE)>=2
    let stzone_chk = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 >= 2;
    // local g=Duel.GetMatchingGroup(s.setfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| setfilter(d, c)), tp, LOCATION_DECK, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon(stzone_chk),1,tp,HINTMSG_SET)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon(stzone_chk)), 1, tp, Some(HINTMSG_SET as u64), None, None, false).unwrap_or_default();
    // if #sg==2 then Duel.SSet(tp,sg) end
    if sg.len() == 2 {
        duel::s_set(d, tp, sg, None, true);
    }
}
/// `s.copytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn copytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.copyfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::get_controler(d, chkc) == tp && copyfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.copyfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| copyfilter(d, c)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.copyfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| copyfilter(d, c)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, tp, 0);
    true
}
/// `s.copyop(e,tp,eg,ep,ev,re,r,rp)`
fn copyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local te,ceg,cep,cev,cre,cr,crp=tc:CheckActivateEffect(false,true,true)
    // if not te then return end
    let Some((te, _)) = card::check_activate_effect(d, tc, false, true, true) else {
        return;
    };
    // local tg=te:GetTarget()
    let tg = effect::get_target(d, te);
    // local op=te:GetOperation()
    let op = effect::get_operation(d, te);
    // if tg then tg(te,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,1) end
    if let Some(tg) = tg {
        let _ = tg.call(d, &[Value::Effect(te), Value::Int(tp as i64), Value::Group(CardSet::new()), Value::Int(PLAYER_NONE as i64), Value::Int(0), Value::Effect(e), Value::Int(REASON_EFFECT as i64), Value::Int(PLAYER_NONE as i64), Value::Int(1)]);
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // tc:CreateEffectRelation(te)
    card::create_effect_relation(d, tc, te);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // for etc in aux.Next(g) do etc:CreateEffectRelation(te) end
    for etc in group::iter(d, &g) {
        card::create_effect_relation(d, etc, te);
    }
    // if op then op(te,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,1) end
    if let Some(op) = op {
        let _ = op.call(d, &[Value::Effect(te), Value::Int(tp as i64), Value::Group(CardSet::new()), Value::Int(PLAYER_NONE as i64), Value::Int(0), Value::Effect(e), Value::Int(REASON_EFFECT as i64), Value::Int(PLAYER_NONE as i64), Value::Int(1)]);
    }
    // tc:ReleaseEffectRelation(te)
    card::release_effect_relation(d, tc, te);
    // for etc in aux.Next(g) do etc:ReleaseEffectRelation(te) end
    for etc in group::iter(d, &g) {
        card::release_effect_relation(d, etc, te);
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.SendtoDeck(te:GetHandler(),nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    let h = effect::get_handler(d, te).nil();
    duel::send_to_deck(d, h, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
