//! Spell Shattering Sword
//! 魔法効果の剣
use super::prelude::*;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_LIGHT_AND_DARKNESS_RITUAL], ..ScriptInfo::DEFAULT };

pub const ID: u32 = 77456448;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects;
    // ● Destroy all face-up Spells your opponent controls
    // ● Target 1 face-up monster your opponent controls; show 1 "Light and Darkness Ritual" from your hand or GY, then change that monster's ATK to 0, also negate its effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E|TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E | TIMING_DAMAGE_STEP));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card in its owner's possession is destroyed by an opponent's card: You can destroy 1 card in your opponent's hand (at random) or field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and e:GetHandler():IsPreviousControler(tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        a.rp == 1 - a.tp && card::is_previous_controler(d, h, a.tp)
    })));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2, Some(operation(e2_operation)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// The target of `e2` (`function(e,tp,eg,ep,ev,re,r,rp,chk)`)
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(nil,tp,0,LOCATION_HAND|LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_HAND | LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_HAND|LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    true
}
/// The operation of `e2` (`function(e,tp,eg,ep,ev,re,r,rp)`)
fn e2_operation(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local hand_group=Duel.GetMatchingGroup(nil,tp,0,LOCATION_HAND,nil)
    let hand_group = duel::get_matching_group(d, None, tp, 0, LOCATION_HAND, NO_EX);
    // local field_group=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let field_group = duel::get_matching_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // local option_1=#hand_group>0
    let option_1 = !hand_group.is_empty();
    // local option_2=#field_group>0
    let option_2 = !field_group.is_empty();
    // if not (option_1 or option_2) then return end
    if !(option_1 || option_2) {
        return;
    }
    // local choice=(option_1 and option_2 and Duel.SelectEffect(tp,{option_1,aux.Stringid(id,4)},{option_2,aux.Stringid(id,5)}))
    //     or (option_1 and 1) or (option_2 and 2)
    let choice: usize = if option_1 && option_2 {
        duel::select_effect(d, tp, &[(option_1, aux::stringid(ID, 4)), (option_2, aux::stringid(ID, 5))]).nil()
    } else if option_1 {
        1
    } else {
        2
    };
    // local desg=Group.CreateGroup()
    let desg = if choice == 1 {
        // desg=hand_group:RandomSelect(tp,1)
        group::random_select(d, &hand_group, tp, 1)
    } else if choice == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // desg=field_group:Select(tp,1,1,nil)
        let desg = group::select(d, &field_group, tp, 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(desg)
        duel::hint_selection(d, desg.clone(), true);
        desg
    } else {
        CardSet::new()
    };
    // if #desg>0 then
    if !desg.is_empty() {
        // Duel.Destroy(desg,REASON_EFFECT)
        duel::destroy(d, desg, REASON_EFFECT, None, None);
    }
}
/// `s.disfilter(c)`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return (not c:IsAttack(0) or c:IsNegatableMonster()) and c:IsFaceup()
    (!card::is_attack(d, c, &[0]) || card::is_negatable_monster(d, c)) && card::is_faceup(d, c)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // Destroy all face-up Spells your opponent controls
    // local opp_spells=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSpell),tp,0,LOCATION_ONFIELD,nil)
    let opp_spells = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_spell(d, c))), tp, 0, LOCATION_ONFIELD, NO_EX);
    // local option_1=#opp_spells>0 and not Duel.IsDamageStep()
    let option_1 = !opp_spells.is_empty() && !duel::is_damage_step(d, None);
    // Target 1 face-up monster your opponent controls; show 1 "Light and Darkness Ritual" from your hand or GY, then change that monster's ATK to 0, also negate its effects
    // local option_2=Duel.IsExistingTarget(s.disfilter,tp,0,LOCATION_MZONE,1,nil) and aux.StatChangeDamageStepCondition()
    //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,CARD_LIGHT_AND_DARKNESS_RITUAL)
    let option_2 = duel::is_existing_target(d, Some(&disfilter), tp, 0, LOCATION_MZONE, 1, NO_EX)
        && aux::stat_change_damage_step_condition(d)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_LIGHT_AND_DARKNESS_RITUAL])), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return option_1 or option_2 end
    if a.chk == Some(0) {
        return option_1 || option_2;
    }
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp,{option_1,aux.Stringid(id,2)},{option_2,aux.Stringid(id,3)})
    let choice = duel::select_effect(d, tp, &[(option_1, aux::stringid(ID, 2)), (option_2, aux::stringid(ID, 3))]);
    chain::set_effect_data(d, e, None, chain::field::CARD, choice.map_or(Value::Nil, |x| Value::Int(x as i64)));
    // if cd.choice==1 then
    if choice == Some(1) {
        // Destroy all face-up Spells your opponent controls
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e, CATEGORY_DESTROY);
        // e:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
        effect::set_property(d, e, EFFECT_FLAG_DAMAGE_STEP, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,opp_spells,#opp_spells,tp,0)
        let n = opp_spells.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(opp_spells), n, tp, 0);
    } else if choice == Some(2) {
        // Target 1 face-up monster your opponent controls; ...
        // e:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DISABLE)
        effect::set_category(d, e, CATEGORY_ATKCHANGE + CATEGORY_DISABLE);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
        // local g=Duel.SelectTarget(tp,s.disfilter,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_target(d, tp, Some(&disfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, g.clone(), 1, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local cd=e:GetChainData()
    let choice = chain::effect_data(d, e, None, chain::field::CARD).int();
    // if cd.choice==1 then
    if choice == 1 {
        // local opp_spells=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSpell),tp,0,LOCATION_ONFIELD,nil)
        let opp_spells = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_spell(d, c))), tp, 0, LOCATION_ONFIELD, NO_EX);
        // if #opp_spells>0 then
        if !opp_spells.is_empty() {
            // Duel.Destroy(opp_spells,REASON_EFFECT)
            duel::destroy(d, opp_spells, REASON_EFFECT, None, None);
        }
    } else if choice == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local sc=Duel.SelectMatchingCard(tp,Card.IsCode,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,CARD_LIGHT_AND_DARKNESS_RITUAL):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_code(d, c, &[CARD_LIGHT_AND_DARKNESS_RITUAL])), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if not sc then return end
        let Some(sc) = sc else {
            return;
        };
        // if sc:IsLocation(LOCATION_HAND) then
        if card::is_location(d, sc, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
        } else {
            // Duel.HintSelection(sc)
            duel::hint_selection(d, CardSet::from_card(sc), true);
        }
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
        if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, e).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Change that monster's ATK to 0
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(0)
            effect::set_value(d, e1, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // Also negate its effects
            // tc:NegateEffects(c)
            card::negate_effects(d, tc, c, None, false, None);
        }
    }
}
