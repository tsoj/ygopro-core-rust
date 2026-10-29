//! Odd-Eyes Pendulum Dragon of the Four Heavenly Dragons
use super::prelude::*;

pub const ID: u32 = 75787708;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // search and destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY|CATEGORY_TOHAND|CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_DESTROY | CATEGORY_TOHAND | CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e) return e:GetHandler():IsPendulumSummoned() end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_pendulum_summoned(d, h)
    })));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // double damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e3:SetCondition(function(e) return e:GetHandler():IsPendulumSummoned() end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_pendulum_summoned(d, h)
    })));
    // e3:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e3, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // to hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOHAND|CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_TOHAND | CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(s.thcon2)
    effect::set_condition(d, e4, Some(condition(thcon2)));
    // e4:SetTarget(s.thtg2)
    effect::set_target(d, e4, Some(cost_target(thtg2)));
    // e4:SetOperation(s.thop2)
    effect::set_operation(d, e4, Some(operation(thop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPendulumMonster() and c:IsAttackBelow(1500) and c:IsAbleToHand()
    card::is_pendulum_monster(d, c) && card::is_attack_below(d, c, 1500) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.Destroy(c,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, c, e) && duel::destroy(d, c, REASON_EFFECT, None, None) != 0 {
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
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.actlimit)
    effect::set_value_fn(d, e1, lua_fn(actlimit));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    // e0:SetLabelObject(e1)
    effect::set_label_object(d, e0, Value::Effect(e1));
    // e0:SetOperation(s.checkop)
    effect::set_operation(d, e0, Some(operation(checkop)));
    // e0:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e0, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e0,tp)
    duel::register_effect(d, e0, tp);
}
// s.actlimit(e,re)
fn actlimit(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re);
    // return rc and rc:IsLocation(LOCATION_MZONE) and re:IsMonsterEffect()
    match rc {
        Some(rc) => card::is_location(d, rc, LOCATION_MZONE) && effect::is_monster_effect(d, re),
        None => false,
    }
}
// s.psfilter(c,tp)
fn psfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsPendulumSummoned()
    card::is_summon_player(d, c, tp) && card::is_pendulum_summoned(d, c)
}
// s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local ef=e:GetLabelObject()
    let ef = effect::get_label_object(d, a.e).as_effect();
    // if ef and eg and eg:IsExists(s.psfilter,1,nil,tp) then
    if let (Some(ef), Some(eg)) = (ef, a.eg.clone()) {
        if group::is_exists(d, &eg, &|d, c| psfilter(d, c, a.tp), 1, NO_EX) {
            // ef:Reset()
            effect::reset(d, ef);
            // e:Reset()
            effect::reset(d, a.e);
        }
    }
}
// s.atkval(e,c)
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local pg=Duel.GetFieldGroup(e:GetHandlerPlayer(),LOCATION_PZONE,0)
    let p = effect::get_handler_player(d, e);
    let pg = duel::get_field_group(d, p, LOCATION_PZONE, 0);
    // if #pg~=2 then return 0 end
    if pg.len() != 2 {
        return 0;
    }
    // local pc1,pc2=pg:GetFirst(),pg:GetNext()
    let order = group::iter(d, &pg);
    let pc1 = order.first().copied().nil();
    let pc2 = order.get(1).copied().nil();
    // return math.abs(pc1:GetScale()-pc2:GetScale())*300
    let s1 = card::get_scale(d, pc1) as i64;
    let s2 = card::get_scale(d, pc2) as i64;
    (s1 - s2).abs() * 300
}
// s.thcon2(e,tp,eg,ep,ev,re,r,rp)
fn thcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return r&(REASON_BATTLE|REASON_EFFECT)>0 and e:GetHandler():IsPreviousLocation(LOCATION_MZONE)
    (a.r & (REASON_BATTLE | REASON_EFFECT)) > 0 && {
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_MZONE)
    }
}
// s.thtg2(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // if rp==1-tp then e:SetLabel(1) else e:SetLabel(0) end
    if a.rp == 1 - a.tp {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,LOCATION_HAND)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_HAND as i32);
    }
    true
}
// s.thop2(e,tp,eg,ep,ev,re,r,rp)
fn thop2(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoHand(c,nil,REASON_EFFECT)~=0
    //     and e:GetLabel()==1 and c:IsLocation(LOCATION_HAND) then
    if card::is_relate_to_effect(d, c, e)
        && duel::send_to_hand(d, c, None, REASON_EFFECT, None) != 0
        && effect::get_label(d, e) == 1
        && card::is_location(d, c, LOCATION_HAND)
    {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
