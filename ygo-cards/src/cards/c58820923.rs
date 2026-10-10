//! Number 95: Galaxy-Eyes Dark Matter Dragon
use super::prelude::*;

pub const ID: u32 = 58820923;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(95), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,9,3,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(d, c, None, Some(9), 3, Some(xyz::alter_filter(ovfilter)), Some(aux::stringid(ID, 0)), None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // xyzlimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_XYZ_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // remove
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.rmcon)
    effect::set_condition(d, e2, Some(condition(rmcon)));
    // e2:SetCost(s.rmcost)
    effect::set_cost(d, e2, Some(cost_target(rmcost)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // multi attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ovfilter(c,tp,xyzc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GALAXY_EYES,xyzc,SUMMON_TYPE_XYZ,tp) and c:IsType(TYPE_XYZ,xyzc,SUMMON_TYPE_XYZ,tp)
    card::is_faceup(d, c)
        && card::is_set_card_ctx(d, c, &[SET_GALAXY_EYES as u16], Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, c)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsAbleToGraveAsCost()
    card::is_race(d, c, RACE_DRAGON) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_DECK,0,nil)
    let mut g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return g:GetClassCount(Card.GetCode)>2 end
    if a.chk == Some(0) {
        return group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) > 2;
    }
    // local tg=Group.CreateGroup()
    let mut tg = group::create_group(&[]);
    // for i=1,3 do
    for _ in 1..=3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // g:Remove(Card.IsCode,nil,sg:GetFirst():GetCode())
        let first = group::get_first(d, &sg).nil();
        let codes = card::get_code(d, first);
        group::remove(d, &mut g, &|d, c| card::is_code(d, c, &codes), NO_EX);
        // tg:Merge(sg)
        group::add_card(&mut tg, &sg);
    }
    // Duel.SendtoGrave(tg,REASON_COST)
    duel::send_to_grave(d, tg, REASON_COST, None, None);
    true
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>2
    //     and not Duel.IsPlayerAffectedByEffect(1-tp,30459350) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 2 && duel::is_player_affected_by_effect(d, 1 - a.tp, 30459350).is_none();
    }
    true
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove()
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(1-tp,30459350) then return end
    if duel::is_player_affected_by_effect(d, 1 - a.tp, 30459350).is_some() {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.rmfilter,1-tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&rmfilter), 1 - a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g>2 then
    if group::get_count(&g) > 2 {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_REMOVE as u64);
        // local sg=g:Select(1-tp,3,3,nil)
        let sg = group::select(d, &g, 1 - a.tp, 3, 3, false, NO_EX).nil();
        // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsAbleToEnterBP()
    duel::is_able_to_enter_bp(d)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():GetEffectCount(EFFECT_EXTRA_ATTACK)==0
    //     and e:GetHandler():GetEffectCount(EFFECT_EXTRA_ATTACK_MONSTER)==0 end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::get_effect_count(d, c, EFFECT_EXTRA_ATTACK) == 0 && card::get_effect_count(d, c, EFFECT_EXTRA_ATTACK_MONSTER) == 0;
    }
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK_MONSTER);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
