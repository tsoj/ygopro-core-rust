//! Magister of Endymion
use super::prelude::*;

pub const ID: u32 = 66104644;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // c:EnableCounterPermit(COUNTER_SPELL,LOCATION_PZONE|LOCATION_MZONE)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, Some(LOCATION_PZONE | LOCATION_MZONE), None);
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Add counter to itself
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
    // Special Summon from Extra deck
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
    // e2:SetCost(s.spcost1)
    effect::set_cost(d, e2, Some(cost_target(spcost1)));
    // e2:SetTarget(s.sptg1)
    effect::set_target(d, e2, Some(cost_target(sptg1)));
    // e2:SetOperation(s.spop1)
    effect::set_operation(d, e2, Some(operation(spop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place Spell counters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e3, CATEGORY_COUNTER);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetCondition(s.ctcon)
    effect::set_condition(d, e3, Some(lua_fn(ctcon)));
    // e3:SetOperation(s.ctop2)
    effect::set_operation(d, e3, Some(operation(ctop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon from deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.spcon2)
    effect::set_condition(d, e4, Some(condition(spcon2)));
    // e4:SetCost(s.spcost2)
    effect::set_cost(d, e4, Some(cost_target(spcost2)));
    // e4:SetTarget(s.sptg2)
    effect::set_target(d, e4, Some(cost_target(sptg2)));
    // e4:SetOperation(s.spop2)
    effect::set_operation(d, e4, Some(operation(spop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Counter check
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e0:SetCode(EVENT_DESTROY)
    effect::set_code(d, e0, EVENT_DESTROY);
    // e0:SetOperation(s.ctchk)
    effect::set_operation(d, e0, Some(operation(ctchk)));
    // e0:SetLabel(0)
    effect::set_label(d, e0, &[0]);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Place itself in the `Pendulum Zone when destroyed
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e5, aux::stringid(ID, 2));
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY, 0);
    // e5:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e5, EVENT_DESTROYED);
    // e5:SetLabelObject(e0)
    effect::set_label_object(d, e5, Value::Effect(e0));
    // e5:SetCondition(s.pencon)
    effect::set_condition(d, e5, Some(condition(pencon)));
    // e5:SetTarget(s.pentg)
    effect::set_target(d, e5, Some(cost_target(pentg)));
    // e5:SetOperation(s.penop)
    effect::set_operation(d, e5, Some(operation(penop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and re:GetHandler()~=c then
    let re = a.re.nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && effect::get_handler(d, re) != Some(c) {
        // c:AddCounter(COUNTER_SPELL,1)
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
    }
}
/// `s.spcost1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost1(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanRemoveCounter(tp,COUNTER_SPELL,3,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, a.tp, COUNTER_SPELL as u16, 3, REASON_COST);
    }
    // c:RemoveCounter(tp,COUNTER_SPELL,3,REASON_COST)
    card::remove_counter(d, c, a.tp, COUNTER_SPELL as u16, 3, REASON_COST);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanAddCounter(COUNTER_SPELL,1,false,LOCATION_MZONE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, LOCATION_MZONE)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg1(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.GetLocationCountFromEx(tp)>0
    //   and Duel.GetUsableMZoneCount(tp)>1
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(s.spfilter,e,tp),tp,LOCATION_EXTRA,0,1,nil,e,tp)
    //   and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0 > 0
            && duel::get_usable_m_zone_count(d, tp, None) > 1
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, cc| card::is_faceup(d, cc) && spfilter(d, cc, a.e, tp)),
                tp,
                LOCATION_EXTRA,
                0,
                1,
                NO_EX,
            )
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none();
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, 0, 0);
    true
}
/// `s.spop1(e,tp,eg,ep,ev,re,r,rp)`
fn spop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // if not c:IsRelateToEffect(e) or not c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   or Duel.GetLocationCount(tp,LOCATION_MZONE)<1
    //   or Duel.GetLocationCountFromEx(tp)<1
    //   or Duel.GetUsableMZoneCount(tp)<2
    //   or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if !card::is_relate_to_effect(d, c, a.e)
        || !card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
        || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1
        || duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0 < 1
        || duel::get_usable_m_zone_count(d, tp, None) < 2
        || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(s.spfilter,e,tp),tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, cc| card::is_faceup(d, cc) && spfilter(d, cc, a.e, tp)),
        tp,
        LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        let mut g = g;
        // g:AddCard(c)
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
/// `s.ctcon(e)`
fn ctcon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsCanAddCounter(COUNTER_SPELL,1,false,LOCATION_MZONE)
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_can_add_counter(d, h, COUNTER_SPELL as u16, 1, false, LOCATION_MZONE)
}
/// `s.ctop2(e,tp,eg,ep,ev,re,r,rp)`
fn ctop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then c:AddCounter(COUNTER_SPELL,1) end
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
    }
}
/// `s.spcon2(e,tp,eg,ep,ev,re,r,rp)`
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.spcost2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,COUNTER_SPELL,3,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_SPELL as u16, 3, REASON_COST);
    }
    // Duel.RemoveCounter(tp,1,0,COUNTER_SPELL,3,REASON_COST)
    duel::remove_counter(d, a.tp, 1, 0, COUNTER_SPELL as u16, 3, REASON_COST);
    true
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, cc| spfilter(d, cc, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.ctchk(e,tp,eg,ep,ev,re,r,rp)`
fn ctchk(d: &mut Duel, a: &EffArgs) {
    // e:SetLabel(e:GetHandler():GetCounter(COUNTER_SPELL))
    let h = effect::get_handler(d, a.e).nil();
    let n = card::get_counter(d, h, COUNTER_SPELL as u16);
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
    // local ct=e:GetLabelObject():GetLabel()
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, lo);
    // if chk==0 then return Duel.CheckPendulumZones(tp)
    //   and (ct==0 or e:GetHandler():IsCanAddCounter(COUNTER_SPELL,ct,false,LOCATION_PZONE)) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::check_pendulum_zones(d, a.tp)
            && (ct == 0 || card::is_can_add_counter(d, h, COUNTER_SPELL as u16, ct as u16, false, LOCATION_PZONE));
    }
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.CheckPendulumZones(tp) then return false end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=e:GetLabelObject():GetLabel()
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, lo);
    // if c:IsRelateToEffect(e) and Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true) and ct>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None)
        && ct > 0
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // c:AddCounter(COUNTER_SPELL,ct)
        card::add_counter(d, c, COUNTER_SPELL as u16, ct as u16, false);
    }
}
