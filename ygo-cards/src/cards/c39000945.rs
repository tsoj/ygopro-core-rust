//! Reflection of Endymion
use super::prelude::*;

pub const ID: u32 = 39000945;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[COUNTER_SPELL],
    listed_series: &[SET_ENDYMION],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL,LOCATION_PZONE|LOCATION_MZONE)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, Some(LOCATION_PZONE | LOCATION_MZONE), None);
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Special Summon itself and from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetOperation(aux.chainreg)
    effect::set_operation(d, e2, Some(aux::chainreg()));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    // e3:SetRange(LOCATION_PZONE)
    effect::set_range(d, e3, LOCATION_PZONE);
    // e3:SetOperation(s.acop)
    effect::set_operation(d, e3, Some(operation(acop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // return targets to the hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Register when destroyed by battle
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e5, EVENT_LEAVE_FIELD_P);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e5:SetOperation(s.regop)
    effect::set_operation(d, e5, Some(operation(regop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Add 1 "Endymion" card to the hand
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e6, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e6:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e6:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e6, EVENT_BATTLE_DESTROYED);
    // e6:SetCondition(s.srcon)
    effect::set_condition(d, e6, Some(condition(srcon)));
    // e6:SetTarget(s.srtg)
    effect::set_target(d, e6, Some(cost_target(srtg)));
    // e6:SetOperation(s.srop)
    effect::set_operation(d, e6, Some(operation(srop)));
    // e6:SetLabelObject(e5)
    effect::set_label_object(d, e6, Value::Effect(e5));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanRemoveCounter(tp,COUNTER_SPELL,3,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, h, a.tp, COUNTER_SPELL as u16, 3, REASON_COST);
    }
    // e:GetHandler():RemoveCounter(tp,COUNTER_SPELL,3,REASON_COST)
    card::remove_counter(d, h, a.tp, COUNTER_SPELL as u16, 3, REASON_COST);
    true
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanAddCounter(COUNTER_SPELL,1,false,LOCATION_MZONE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, LOCATION_MZONE)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    //   and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, cc| cfilter(d, cc, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none();
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,2,tp,LOCATION_PZONE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 2, tp, (LOCATION_PZONE | LOCATION_HAND) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if (Duel.GetLocationCount(tp,LOCATION_MZONE)<=1)
    //   or not c:IsRelateToEffect(e)
    //   or not c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 1
        || !card::is_relate_to_effect(d, c, a.e)
        || !card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
        || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| cfilter(d, cc, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let mut g = g;
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // if #g>1 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)==2 then
    if g.len() > 1 && duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) == 2 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local ct=g:GetFirst()
        let _ct = group::get_first(d, &g);
        // for ct in aux.Next(g) do ct:AddCounter(COUNTER_SPELL,1) end
        for ct in group::iter(d, &g) {
            card::add_counter(d, ct, COUNTER_SPELL as u16, 1, false);
        }
    }
}
/// `s.acop(e,tp,eg,ep,ev,re,r,rp)`
fn acop(d: &mut Duel, a: &EffArgs) {
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and e:GetHandler():GetFlagEffect(1)>0 then
    let re = a.re.nil();
    let h = effect::get_handler(d, a.e).nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && card::get_flag_effect(d, h, 1) > 0 {
        // e:GetHandler():AddCounter(COUNTER_SPELL,1)
        card::add_counter(d, h, COUNTER_SPELL as u16, 1, false);
    }
}
/// `s.thfilter(c,e)`
fn thfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:GetCounter(COUNTER_SPELL)>0 and e:GetHandler():IsCanAddCounter(COUNTER_SPELL,c:GetCounter(COUNTER_SPELL),false,LOCATION_MZONE)
    let n = card::get_counter(d, c, COUNTER_SPELL as u16);
    if n <= 0 {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    card::is_can_add_counter(d, h, COUNTER_SPELL as u16, n as u16, false, LOCATION_MZONE)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_ONFIELD,0,1,c,e)
    //   and Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| thfilter(d, cc, a.e)), a.tp, LOCATION_ONFIELD, 0, 1, c.into())
            && duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g1=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g1 = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g2=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_ONFIELD,0,1,1,c,e)
    let g2 = duel::select_target(d, a.tp, Some(&|d, cc| thfilter(d, cc, a.e)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    // e:SetLabelObject(g2:GetFirst())
    let value = match group::get_first(d, &g2) {
        Some(tc) => Value::Card(tc),
        None => Value::Nil,
    };
    effect::set_label_object(d, a.e, value);
    // g1:Merge(g2)
    let mut g1 = g1;
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g1,2,0,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g1), 2, 0, LOCATION_ONFIELD as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let mut tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, a.e).as_card().nil();
    // local oc=(tg-sc):GetFirst()
    let mut rest = tg.clone();
    group::remove_card(&mut rest, &CardSet::from_card(sc));
    let oc = group::get_first(d, &rest);
    // if sc and sc:IsControler(1-tp) then tg:RemoveCard(sc) end
    if card::is_controler(d, sc, 1 - tp) {
        group::remove_card(&mut tg, &CardSet::from_card(sc));
    }
    // if oc and oc:IsControler(tp) then tg:RemoveCard(oc) end
    if let Some(oc) = oc {
        if card::is_controler(d, oc, tp) {
            group::remove_card(&mut tg, &CardSet::from_card(oc));
        }
    }
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local ct=0
    let mut ct: i64 = 0;
    // if tg:IsContains(sc) then ct=sc:GetCounter(COUNTER_SPELL) end
    if group::is_contains(&tg, sc) {
        ct = card::get_counter(d, sc, COUNTER_SPELL as u16) as i64;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if #tg>0 and Duel.SendtoHand(tg,nil,REASON_EFFECT)>0 and ct>0 and tg:IsExists(Card.IsLocation,1,nil,LOCATION_HAND)
    //   and c:IsRelateToEffect(e) then
    if duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None) > 0
        && ct > 0
        && group::is_exists(d, &tg, &|d, cc| card::is_location(d, cc, LOCATION_HAND), 1, NO_EX)
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // c:AddCounter(COUNTER_SPELL,ct)
        card::add_counter(d, c, COUNTER_SPELL as u16, ct as u16, false);
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetCounter(COUNTER_SPELL)
    let ct = card::get_counter(d, c, COUNTER_SPELL as u16);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
}
/// `s.srcon(e,tp,eg,ep,ev,re,r,rp)`
fn srcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=e:GetLabelObject():GetLabel()
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, lo);
    // return ct>0 and c:IsReason(REASON_BATTLE)
    ct > 0 && card::is_reason(d, c, REASON_BATTLE)
}
/// `s.srfilter(c)`
fn srfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand() and c:IsSetCard(SET_ENDYMION)
    card::is_able_to_hand(d, c, None) && card::is_set_card(d, c, &[SET_ENDYMION as u16])
}
/// `s.srtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn srtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.srfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&srfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.srop(e,tp,eg,ep,ev,re,r,rp)`
fn srop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.srfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&srfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
