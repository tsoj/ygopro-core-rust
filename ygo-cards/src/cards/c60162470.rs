//! Zero Extra Link
use super::prelude::*;

pub const ID: u32 = 60162470;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddPersistentProcedure(c,0,s.filter)
    aux::add_persistent_procedure(
        d,
        c,
        Some(0),
        Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        false,
    );
    // Activate it by targeting a Link Monster co-linked to a monster in the Extra Monster Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e1, Some(aux::persistent_target_filter()));
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // A monster summoned with the target gains the same ATK that monster gained via this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD_P);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetCondition(s.matcon)
    effect::set_condition(d, e3, Some(condition(matcon)));
    // e3:SetOperation(s.matop)
    effect::set_operation(d, e3, Some(operation(matop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy this card after damage calculation
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_BATTLED)
    effect::set_code(d, e4, EVENT_BATTLED);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTarget(s.descon)
    effect::set_target(d, e4, Some(cost_target(descon)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.exfilter(c)`
fn exfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLinkMonster() and c:IsInExtraMZone()
    card::is_faceup(d, c) && card::is_link_monster(d, c) && card::is_in_extra_m_zone(d, c, None)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLinkMonster() and c:GetMutualLinkedGroup():IsExists(s.exfilter,1,nil)
    if !(card::is_faceup(d, c) && card::is_link_monster(d, c)) {
        return false;
    }
    let mg = card::get_mutual_linked_group(d, c);
    group::is_exists(d, &mg, &exfilter, 1, NO_EX)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLinkMonster()
    card::is_faceup(d, c) && card::is_link_monster(d, c)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local val=Duel.GetMatchingGroupCount(s.cfilter,e:GetHandlerPlayer(),LOCATION_MZONE,LOCATION_MZONE,nil)*800
    let p = effect::get_handler_player(d, e);
    let val = duel::get_matching_group_count(d, Some(&cfilter), p, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i64 * 800;
    // e:SetLabel(val)
    effect::set_label(d, e, &[val]);
    // return val
    val
}
/// `s.atkfilter(c,tc)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLinkMonster() and c:IsLinkSummoned() and c:HasFlagEffect(id)
    card::is_link_monster(d, c) && card::is_link_summoned(d, c) && card::has_flag_effect(d, c, ID, None)
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.atkfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &atkfilter, 1, NO_EX)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:Filter(s.atkfilter,nil):GetFirst()
    let eg = a.eg.clone().nil();
    let f = group::filter(d, &eg, &atkfilter, NO_EX);
    let tc = group::get_first(d, &f);
    // if tc then
    if let Some(tc) = tc {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(e:GetLabelObject():GetLabel())
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        let val = effect::get_label(d, lo);
        effect::set_value(d, e1, val);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.matfilter(c,ec)`
fn matfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return ec:IsHasCardTarget(c) and c:IsReason(REASON_MATERIAL|REASON_LINK)
    card::is_has_card_target(d, ec, c) && card::is_reason(d, c, REASON_MATERIAL | REASON_LINK)
}
/// `s.matcon(e,tp,eg,ep,ev,re,r,rp)`
fn matcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.matfilter,1,nil,e:GetHandler())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &eg, &|d, c| matfilter(d, c, h), 1, NO_EX)
}
/// `s.matop(e,tp,eg,ep,ev,re,r,rp)`
fn matop(d: &mut Duel, a: &EffArgs) {
    // eg:Filter(s.matfilter,nil,e:GetHandler()):GetFirst():GetReasonCard():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~RESET_TOFIELD,0,1)
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    let f = group::filter(d, &eg, &|d, c| matfilter(d, c, h), NO_EX);
    let tc = group::get_first(d, &f).nil();
    let rc = card::get_reason_card(d, tc).nil();
    card::register_flag_effect(d, rc, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 0, 1, 0, 0);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsHasCardTarget(Duel.GetAttacker())
    let h = effect::get_handler(d, a.e).nil();
    // (a nil attacker is not a card target: IsHasCardTarget(nil) is false)
    match duel::get_attacker(d) {
        Some(at) => card::is_has_card_target(d, h, at),
        None => false,
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
        duel::destroy(d, h, REASON_EFFECT, None, None);
    }
}
