//! D/D Extra Surveyor
use super::prelude::*;

pub const ID: u32 = 74387963;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DD, SET_DDD],
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon procedure
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Banish 2 cards from your Pendulum Zone and make 1 "D/D/D" monster able to make a second attack during each Battle Phase this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.rmvcon)
    effect::set_condition(d, e1, Some(condition(rmvcon)));
    // e1:SetTarget(s.rmvtg)
    effect::set_target(d, e1, Some(cost_target(rmvtg)));
    // e1:SetOperation(s.rmvop)
    effect::set_operation(d, e1, Some(operation(rmvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 face-up "D/D" Pendulum Monster from your Extra Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e2, Some(cost::self_discard()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish cards from the top of your opponent's Deck then you can make 1 "D/D" monster you control gain 200 ATK for each card banished
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_REMOVE)
    effect::set_code(d, e3, EVENT_REMOVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetTarget(s.rmvatktg)
    effect::set_target(d, e3, Some(cost_target(rmvatktg)));
    // e3:SetOperation(s.rmvatkop)
    effect::set_operation(d, e3, Some(operation(rmvatkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rmvconfilter(c,tp)`
fn rmvconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsPreviousControler(1-tp) and c:IsPreviousLocation(LOCATION_ONFIELD)
    //   and c:IsPreviousPosition(POS_FACEUP) and c:IsMonsterCard()
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_previous_controler(d, c, 1 - tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_monster_card(d, c)
}
/// `s.rmvcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.rmvconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| rmvconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DDD) and c:IsFaceup() and not c:IsHasEffect(EFFECT_EXTRA_ATTACK)
    card::is_set_card(d, c, &[SET_DDD as u16]) && card::is_faceup(d, c) && card::is_has_effect(d, c, EFFECT_EXTRA_ATTACK, None).is_empty()
}
/// `s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_PZONE,0,2,nil)
    //   and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_PZONE, 0, 2, NO_EX)
            && duel::is_existing_target(d, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_PZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_PZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 2, a.tp, 0);
    true
}
/// `s.rmvop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_PZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_PZONE, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.Remove(g,POS_FACEUP,REASON_EFFECT)==2 and tc:IsRelateToEffect(e) then
    if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) == 2 && card::is_relate_to_effect(d, tc.nil(), a.e) {
        let tc = tc.nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3201)
        effect::set_description(d, e1, 3201);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DD) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DD as u16]) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `aux.FaceupFilter(Card.IsType,TYPE_PENDULUM)`
fn faceup_pendulum(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM)
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_DD)`
fn faceup_dd(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DD as u16])
}
/// `s.rmvatktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmvatktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_PENDULUM),tp,0,LOCATION_EXTRA,nil)
    let ct = duel::get_matching_group_count(d, Some(&faceup_pendulum), a.tp, 0, LOCATION_EXTRA, NO_EX);
    // local g=Duel.GetDecktopGroup(1-tp,ct)
    let g = duel::get_decktop_group(d, 1 - a.tp, ct as u32);
    // if chk==0 then return ct>0 and g:FilterCount(Card.IsAbleToRemove,nil)==ct end
    if a.chk == Some(0) {
        return ct > 0 && group::filter_count(d, &g, &|d, c| card::is_able_to_remove(d, c, None, None, None), NO_EX) == ct;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,ct,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), ct as u8, a.tp, 0);
    true
}
/// `s.rmvatkop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvatkop(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_PENDULUM),tp,0,LOCATION_EXTRA,nil)
    let ct = duel::get_matching_group_count(d, Some(&faceup_pendulum), a.tp, 0, LOCATION_EXTRA, NO_EX);
    // local g=Duel.GetDecktopGroup(1-tp,ct)
    let g = duel::get_decktop_group(d, 1 - a.tp, ct as u32);
    // if ct==0 or #g==0 or ct>#g then return end
    if ct == 0 || g.is_empty() || ct > g.len() {
        return;
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // local atk=Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    let atk = duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    // if atk>0 and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_DD),tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if atk > 0
        && duel::is_existing_matching_card(d, Some(&faceup_dd), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsSetCard,SET_DD),tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&faceup_dd), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if sc then
        if let Some(sc) = sc {
            // Duel.HintSelection(sc)
            duel::hint_selection(d, sc, true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // sc:UpdateAttack(200*atk,RESET_EVENT|RESETS_STANDARD,e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            card::update_attack(d, sc, 200 * atk as i64, Some(RESET_EVENT | RESETS_STANDARD), Some(h), None);
        }
    }
}
