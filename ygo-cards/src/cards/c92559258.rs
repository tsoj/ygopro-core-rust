//! Servant of Endymion
use super::prelude::*;

pub const ID: u32 = 92559258;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // c:EnableCounterPermit(COUNTER_SPELL,LOCATION_PZONE|LOCATION_MZONE)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, Some(LOCATION_PZONE | LOCATION_MZONE), None);
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Place a Spell Counter on itself each time a Spell resolves
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon itself and a monster from the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Can attack direct while it has Spell Counters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
    // e3:SetCondition(function(e) return e:GetHandler():GetCounter(COUNTER_SPELL)>0 end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::get_counter(d, c, COUNTER_SPELL as u16) > 0
    })));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Place 1 Spell Counter on each card that can hold Spell Counters
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e4, CATEGORY_COUNTER);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e4, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e4:SetCost(s.ctcost2)
    effect::set_cost(d, e4, Some(cost_target(ctcost2)));
    // e4:SetTarget(s.cttg2)
    effect::set_target(d, e4, Some(cost_target(cttg2)));
    // e4:SetOperation(s.ctop2)
    effect::set_operation(d, e4, Some(operation(ctop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Spell Counter check
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_DESTROY)
    effect::set_code(d, e5, EVENT_DESTROY);
    // e5:SetOperation(s.ctchk)
    effect::set_operation(d, e5, Some(operation(ctchk)));
    // e5:SetLabel(0)
    effect::set_label(d, e5, &[0]);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Place itself in the Pendulum Zone
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e6, aux::stringid(ID, 2));
    // e6:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e6:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e6, EFFECT_FLAG_DELAY, 0);
    // e6:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e6, EVENT_DESTROYED);
    // e6:SetLabelObject(e5)
    effect::set_label_object(d, e6, Value::Effect(e5));
    // e6:SetCondition(s.pencon)
    effect::set_condition(d, e6, Some(condition(pencon)));
    // e6:SetTarget(s.pentg)
    effect::set_target(d, e6, Some(cost_target(pentg)));
    // e6:SetOperation(s.penop)
    effect::set_operation(d, e6, Some(operation(penop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and re:GetHandler()~=c then
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && effect::get_handler(d, re) != Some(c) {
        // c:AddCounter(COUNTER_SPELL,1)
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
    }
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanRemoveCounter(tp,COUNTER_SPELL,3,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, tp, COUNTER_SPELL as u16, 3, REASON_COST);
    }
    // c:RemoveCounter(tp,COUNTER_SPELL,3,REASON_COST)
    card::remove_counter(d, c, tp, COUNTER_SPELL as u16, 3, REASON_COST);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanAddCounter(COUNTER_SPELL,1,false,LOCATION_MZONE) and c:IsAttackAbove(1000)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, LOCATION_MZONE)
        && card::is_attack_above(d, c, 1000)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>=2 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, tc| spfilter(d, tc, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none();
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,2,tp,LOCATION_PZONE|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 2, tp, (LOCATION_PZONE | LOCATION_DECK) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and (Duel.GetLocationCount(tp,LOCATION_MZONE)>=2))
    //     or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    let ok = card::is_relate_to_effect(d, c, e)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2;
    if !ok || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, tc| spfilter(d, tc, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // g:AddCard(c)
        let mut g = g;
        group::add_card(&mut g, &CardSet::from_card(c));
        // if Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)==2 then
        if duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) == 2 {
            // g:ForEach(Card.AddCounter,COUNTER_SPELL,1)
            for tc in group::iter(d, &g) {
                card::add_counter(d, tc, COUNTER_SPELL as u16, 1, false);
            }
        }
    }
}
/// `s.ctcost2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ctcost2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `aux.FaceupFilter(Card.IsCanAddCounter,COUNTER_SPELL,1)`
fn faceup_can_add_spell(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, 0)
}
/// `s.cttg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCanAddCounter,COUNTER_SPELL,1),tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&faceup_can_add_spell), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCanAddCounter,COUNTER_SPELL,1),tp,LOCATION_ONFIELD,0,nil)
    let g = duel::get_matching_group(d, Some(&faceup_can_add_spell), tp, LOCATION_ONFIELD, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,g,#g,tp,0)
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, Some(g), n as u8, tp, 0);
    true
}
/// `s.ctop2(e,tp,eg,ep,ev,re,r,rp)`
fn ctop2(d: &mut Duel, _a: &EffArgs) {
    let tp = _a.tp;
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCanAddCounter,COUNTER_SPELL,1),tp,LOCATION_ONFIELD,0,nil)
    let g = duel::get_matching_group(d, Some(&faceup_can_add_spell), tp, LOCATION_ONFIELD, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // g:ForEach(Card.AddCounter,COUNTER_SPELL,1)
        for tc in group::iter(d, &g) {
            card::add_counter(d, tc, COUNTER_SPELL as u16, 1, false);
        }
    }
}
/// `s.ctchk(e,tp,eg,ep,ev,re,r,rp)`
fn ctchk(d: &mut Duel, a: &EffArgs) {
    // e:SetLabel(e:GetHandler():GetCounter(COUNTER_SPELL))
    let c = effect::get_handler(d, a.e).nil();
    let n = card::get_counter(d, c, COUNTER_SPELL as u16);
    effect::set_label(d, a.e, &[n as i64]);
}
/// `s.pencon(e,tp,eg,ep,ev,re,r,rp)`
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return r&REASON_EFFECT+REASON_BATTLE~=0 and c:IsPreviousLocation(LOCATION_MZONE) and c:IsFaceup()
    (a.r & (REASON_EFFECT + REASON_BATTLE)) != 0 && card::is_previous_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)
}
/// `s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=e:GetLabelObject():GetLabel()
    let e5 = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, e5);
    // local c=e:GetHandler() (only used below)
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.CheckPendulumZones(tp)
    //     and (ct==0 or e:GetHandler():IsCanAddCounter(COUNTER_SPELL,ct,false,LOCATION_PZONE)) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, tp) && (ct == 0 || card::is_can_add_counter(d, c, COUNTER_SPELL as u16, ct as u16, false, LOCATION_PZONE));
    }
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.CheckPendulumZones(tp) then return false end
    if !duel::check_pendulum_zones(d, tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=e:GetLabelObject():GetLabel()
    let e5 = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, e5);
    // if c:IsRelateToEffect(e) and Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true) then
    if card::is_relate_to_effect(d, c, a.e) && duel::move_to_field(d, c, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // c:AddCounter(COUNTER_SPELL,ct)
        card::add_counter(d, c, COUNTER_SPELL as u16, ct as u16, false);
    }
}
