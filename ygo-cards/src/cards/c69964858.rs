//! Witchcrafter Pupils
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 69964858;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WITCHCRAFTER], material_setcode: &[SET_WITCHCRAFTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Witchcrafter" monster + 1 Spellcaster monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_WITCHCRAFTER),aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_WITCHCRAFTER as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_SPELLCASTER, Some(fc), st, Some(tp)))),
        ],
    );
    // During the Main or Battle Phase (Quick Effect): You can activate 1 of these effects;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() or Duel.IsBattlePhase() end)
    effect::set_condition(d, e1, Some(condition(main_or_battle_cond)));
    // e1:SetCost(Cost.Choice(
    //     --● Add 1 "Witchcrafter" Spell from your Deck to your hand
    //     {aux.TRUE,aux.Stringid(id,2),function(e,tp) return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end},
    //     --● Reveal 1 "Witchcrafter" Normal or Quick-Play Spell in your hand; apply that Spell's activation effect
    //     {Cost.Reveal(s.revealfilter,nil,1,1,function(e,tp,og) e:GetChainData().revealed_card=og:GetFirst() end),aux.Stringid(id,3),nil}
    // ))
    effect::set_cost(
        d,
        e1,
        Some(cost::choice(vec![
            cost::CostChoice { cost: aux_true(), desc: aux::stringid(ID, 2), check: Some(cost_target(thcheck)) },
            cost::CostChoice {
                cost: cost::reveal(
                    Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8| revealfilter(d, c))),
                    false,
                    Some(cost::Count::N(1)),
                    Some(cost::Count::N(1)),
                    Some(std::sync::Arc::new(|d: &mut Duel, e: EffectId, _tp: u8, og: &CardSet| {
                        // e:GetChainData().revealed_card=og:GetFirst()
                        let first = group::get_first(d, og).map_or(Value::Nil, Value::Card);
                        chain::set_effect_data(d, e, None, chain::field::CARD, first);
                    })),
                    None,
                ),
                desc: aux::stringid(ID, 3),
                check: None,
            },
        ])),
    );
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_MAIN_END | TIMING_BATTLE_START | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your End Phase: You can return 1 of your banished "Witchcrafter" cards to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(turn_player_cond)));
    // e2:SetTarget(s.rtgtg)
    effect::set_target(d, e2, Some(cost_target(rtgtg)));
    // e2:SetOperation(s.rtgop)
    effect::set_operation(d, e2, Some(operation(rtgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function() return Duel.IsMainPhase() or Duel.IsBattlePhase() end`
fn main_or_battle_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None) || duel::is_battle_phase(d, None)
}
/// `function(e,tp) return Duel.IsTurnPlayer(tp) end`
fn turn_player_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_WITCHCRAFTER) and c:IsSpell() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16]) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// The check of the first Cost.Choice option: `function(e,tp) return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end`
fn thcheck(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.revealfilter(c)`
fn revealfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_WITCHCRAFTER) and (c:IsNormalSpell() or c:IsQuickPlaySpell())
    //     and c:CheckActivateEffect(true,true,false)~=nil
    //     and c:CheckActivateEffect(true,true,false):GetOperation()~=nil
    if !(card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16]) && (card::is_normal_spell(d, c) || card::is_quick_play_spell(d, c))) {
        return false;
    }
    if card::check_activate_effect(d, c, true, true, false).is_none() {
        return false;
    }
    match card::check_activate_effect(d, c, true, true, false) {
        Some((te, _)) => effect::get_operation(d, te).is_some(),
        None => false,
    }
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local cd=e:GetChainData()
    // if cd.cost_choice==1 then
    let choice = chain::effect_data(d, e, None, chain::field::COST_CHOICE).int();
    if choice == 1 {
        // --● Add 1 "Witchcrafter" Spell from your Deck to your hand
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // elseif cd.cost_choice==2 then
    } else if choice == 2 {
        // --● Reveal 1 "Witchcrafter" Normal or Quick-Play Spell in your hand; apply that Spell's activation effect
        // e:SetCategory(0)
        effect::set_category(d, e, 0);
        // local te,ceg,cep,cev,cre,cr,crp=cd.revealed_card:CheckActivateEffect(true,true,true)
        let revealed = chain::effect_data(d, e, None, chain::field::CARD).as_card().nil();
        let (te, evt) = card::check_activate_effect(d, revealed, true, true, true).nil();
        let (ceg, cep, cev, cre, cr, crp) = match &evt {
            Some(ev) => (
                ev.event_cards.clone().map_or(Value::Nil, Value::Group),
                Value::Int(ev.event_player as i64),
                Value::Int(ev.event_value as i64),
                ev.reason_effect.map_or(Value::Nil, Value::Effect),
                Value::Int(ev.reason as i64),
                Value::Int(ev.reason_player as i64),
            ),
            None => (Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil),
        };
        // Duel.ClearTargetCard()
        duel::clear_target_card(d);
        // local tg=te:GetTarget()
        let tg = effect::get_target(d, te);
        // e:SetProperty(te:GetProperty())
        let (p1, p2) = effect::get_property(d, te);
        effect::set_property(d, e, p1, p2);
        // e:SetLabel(te:GetLabel())
        let label = effect::get_labels(d, te);
        effect::set_label(d, e, &label);
        // e:SetLabelObject(te:GetLabelObject())
        let label_object = effect::get_label_object(d, te);
        effect::set_label_object(d, e, label_object);
        // if tg then tg(e,tp,ceg,cep,cev,cre,cr,crp,1) end
        if let Some(tg) = tg {
            tg.call(d, &[Value::Effect(e), Value::Int(tp as i64), ceg, cep, cev, cre, cr, crp, Value::Int(1)]);
        }
        // e:SetCategory(0)
        effect::set_category(d, e, 0);
        // Duel.ClearOperationInfo(0)
        duel::clear_operation_info(d, 0);
        // cd.pupils_target_effect=te
        chain::set_effect_data(d, e, None, chain::field::CARD + 1, Value::Effect(te));
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local cd=e:GetChainData()
    // if cd.cost_choice==1 then
    let choice = chain::effect_data(d, e, None, chain::field::COST_CHOICE).int();
    if choice == 1 {
        // --● Add 1 "Witchcrafter" Spell from your Deck to your hand
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    // elseif cd.cost_choice==2 then
    } else if choice == 2 {
        // --● Reveal 1 "Witchcrafter" Normal or Quick-Play Spell in your hand; apply that Spell's activation effect
        // local te=cd.pupils_target_effect
        // if not te then return end
        let Some(te) = chain::effect_data(d, e, None, chain::field::CARD + 1).as_effect() else {
            return;
        };
        // local op=te:GetOperation()
        // if op then op(e,tp,eg,ep,ev,re,r,rp) end
        if let Some(op) = effect::get_operation(d, te) {
            let eg = a.eg.clone().map_or(Value::Nil, Value::Group);
            let re = a.re.map_or(Value::Nil, Value::Effect);
            op.call(
                d,
                &[Value::Effect(e), Value::Int(tp as i64), eg, Value::Int(a.ep as i64), Value::Int(a.ev as i64), re, Value::Int(a.r as i64), Value::Int(a.rp as i64)],
            );
        }
    }
}
/// `s.rtgfilter(c)`
fn rtgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_WITCHCRAFTER) and c:IsFaceup() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16]) && card::is_faceup(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.rtgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rtgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rtgfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rtgfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_REMOVED as i32);
    true
}
/// `s.rtgop(e,tp,eg,ep,ev,re,r,rp)`
fn rtgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rtgfilter,tp,LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&rtgfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
