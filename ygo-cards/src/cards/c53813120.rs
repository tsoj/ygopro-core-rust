//! Radiant Typhoon Mandate
//! 絢嵐たる権能
use super::prelude::*;

pub const ID: u32 = 53813120;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MYSTICAL_SPACE_TYPHOON], listed_series: &[SET_RADIANT_TYPHOON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Once per turn: You can target 3 Quick-Play Spells in your GY, including a "Radiant Typhoon" card; apply the following effects in sequence
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW+CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per Chain, when "Mystical Space Typhoon" is activated: You can target 1 face-up card your opponent controls; negate its effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsCode(CARD_MYSTICAL_SPACE_TYPHOON) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let re = a.re.nil();
        effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && {
            let h = effect::get_handler(d, re).nil();
            card::is_code(d, h, &[CARD_MYSTICAL_SPACE_TYPHOON])
        }
    })));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card is destroyed by the effect of "Mystical Space Typhoon": You can Set this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re and re:GetHandler():IsCode(CARD_MYSTICAL_SPACE_TYPHOON) end)
    effect::set_condition(d, e3, Some(condition(|d, a| is_mst_activation(d, a.re))));
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(cost_target(settg)));
    // e3:SetOperation(s.setop)
    effect::set_operation(d, e3, Some(operation(setop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `re and re:GetHandler():IsCode(CARD_MYSTICAL_SPACE_TYPHOON)`
fn is_mst_activation(d: &mut Duel, re: Option<EffectId>) -> bool {
    match re {
        Some(re) => {
            let h = effect::get_handler(d, re).nil();
            card::is_code(d, h, &[CARD_MYSTICAL_SPACE_TYPHOON])
        }
        None => false,
    }
}
/// `s.tdfilter(c,e)`
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsQuickPlaySpell() and c:IsAbleToDeck() and c:IsCanBeEffectTarget(e)
    card::is_quick_play_spell(d, c) && card::is_able_to_deck(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_RADIANT_TYPHOON)
    let res = group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_RADIANT_TYPHOON as u16]), 1, NO_EX);
    (res, false)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| tdfilter(d, c, a.e)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #g>=3 and g:IsExists(Card.IsSetCard,1,nil,SET_RADIANT_TYPHOON) end
    if a.chk == Some(0) {
        return g.len() >= 3 && group::is_exists(d, &g, &|d, c| card::is_set_card(d, c, &[SET_RADIANT_TYPHOON as u16]), 1, NO_EX);
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_TODECK)
    let rs: aux::Rescon = &rescon;
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(rs), 1, a.tp, Some(HINTMSG_TODECK as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,#tg,tp,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,nil,0,tp,300)
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, None, 0, a.tp, 300);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DEFCHANGE,nil,0,tp,300)
    duel::set_possible_operation_info(d, 0, CATEGORY_DEFCHANGE, None, 0, a.tp, 300);
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local break_chk=false
    let mut break_chk = false;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // --● Shuffle them into the Deck, then draw 1 card
    // if #tg>0 and Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 then
    if tg.len() > 0 && duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        // break_chk=true
        break_chk = true;
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
    // if break_chk then Duel.BreakEffect() end
    if break_chk {
        duel::break_effect(d);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,3))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
    // --● All WIND monsters you control gain 300 ATK/DEF this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsAttribute(ATTRIBUTE_WIND) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, cc| card::is_attribute(d, cc, ATTRIBUTE_WIND))));
    // e1:SetValue(300)
    effect::set_value(d, e1, 300);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and chkc:IsNegatable() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && card::is_negatable(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_negatable(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), 1, a.tp, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // tc:NegateEffects(e:GetHandler(),nil,true)
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, None, true, None);
    }
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    // if c:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_GRAVE) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SSet(tp,c)
        duel::s_set(d, a.tp, CardSet::from_card(c), None, true);
    }
}
