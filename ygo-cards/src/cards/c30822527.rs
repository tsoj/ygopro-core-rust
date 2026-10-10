//! Decode Talker Extended
use super::prelude::*;

pub const ID: u32 = 30822527;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[1861629],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // Name becomes "Decode Talker" while on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(1861629)
    effect::set_value(d, e1, 1861629);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains 500 ATK for each monster it points to
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Make a second attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    // e3:SetCondition(s.regcon)
    effect::set_condition(d, e3, Some(condition(regcon)));
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCondition(s.regcon2)
    effect::set_condition(d, e4, Some(condition(regcon2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e5:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e5, EVENT_CUSTOM + ID);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.racon)
    effect::set_condition(d, e5, Some(condition(racon)));
    // e5:SetTarget(s.ratg)
    effect::set_target(d, e5, Some(cost_target(ratg)));
    // e5:SetOperation(s.raop)
    effect::set_operation(d, e5, Some(operation(raop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetLinkedGroup():FilterCount(Card.IsMonster,nil)*500
    let g = card::get_linked_group(d, c);
    group::filter_count(d, &g, &card::is_monster, NO_EX) as i64 * 500
}
/// `s.cfilter(c,tp,zone)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8, zone: u32) -> bool {
    // local seq=c:GetPreviousSequence()
    let mut seq = card::get_previous_sequence(d, c);
    // if not c:IsPreviousControler(tp) then seq=seq+16 end
    if !card::is_previous_controler(d, c, tp) {
        seq += 16;
    }
    // return c:IsPreviousLocation(LOCATION_MZONE) and bit.extract(zone,seq)~=0
    card::is_previous_location(d, c, LOCATION_MZONE) && ((zone >> seq) & 1) != 0
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp,e:GetHandler():GetLinkedZone())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, None);
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp, zone), 1, NO_EX)
}
/// `s.cfilter2(c,tp,zone)`
fn cfilter2(d: &mut Duel, c: CardId, tp: u8, zone: u32) -> bool {
    // return not c:IsReason(REASON_BATTLE) and s.cfilter(c,tp,zone)
    !card::is_reason(d, c, REASON_BATTLE) && cfilter(d, c, tp, zone)
}
/// `s.regcon2(e,tp,eg,ep,ev,re,r,rp)`
fn regcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter2,1,nil,tp,e:GetHandler():GetLinkedZone())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, None);
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter2(d, c, tp, zone), 1, NO_EX)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, 0, 0);
}
/// `s.racon(e,tp,eg,ep,ev,re,r,rp)`
fn racon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let _ph = duel::get_current_phase(d);
    // local p=Duel.GetTurnPlayer()
    let p = duel::get_turn_player(d);
    // return p==tp and Duel.IsBattlePhase()
    p == a.tp && duel::is_battle_phase(d, None)
}
/// `s.ratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():GetEffectCount(EFFECT_EXTRA_ATTACK)==0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::get_effect_count(d, h, EFFECT_EXTRA_ATTACK) == 0;
    }
    true
}
/// `s.raop(e,tp,eg,ep,ev,re,r,rp)`
fn raop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Can make a second attack
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3201)
        effect::set_description(d, e1, 3201);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
