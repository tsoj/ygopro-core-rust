//! Evil Eye Mesmerism
use super::prelude::*;

pub const ID: u32 = 42899204;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_EVIL_EYE_SELENE], listed_series: &[SET_EVIL_EYE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate and use effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // e2:SetCondition(aux.PersistentTgCon)
    effect::set_condition(d, e2, Some(aux::persistent_tg_con()));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // control
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_SET_CONTROL)
    effect::set_code(d, e3, EFFECT_SET_CONTROL);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e3, Some(aux::persistent_target_filter()));
    // e3:SetValue(s.tg)
    effect::set_value_fn(d, e3, effect_card_value(tg));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // treats
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_ADD_SETCODE)
    effect::set_code(d, e4, EFFECT_ADD_SETCODE);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    // e4:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e4, Some(aux::persistent_target_filter()));
    // e4:SetCondition(s.sccon)
    effect::set_condition(d, e4, Some(condition(sccon)));
    // e4:SetValue(SET_EVIL_EYE)
    effect::set_value(d, e4, SET_EVIL_EYE as i64);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // destroy
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e5, EVENT_LEAVE_FIELD);
    // e5:SetCondition(s.descon)
    effect::set_condition(d, e5, Some(condition(descon)));
    // e5:SetOperation(s.desop)
    effect::set_operation(d, e5, Some(operation(desop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.cfilter1(c)`
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_EVIL_EYE)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_EVIL_EYE as u16])
}
/// `s.cfilter2(c,e,tp,eg)`
fn cfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, eg: &CardSet) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:IsCanBeEffectTarget(e) and c:IsControlerCanBeChanged() and c:IsPosition(POS_FACEUP_ATTACK)
    //     and eg:IsExists(s.cfilter3,1,nil,c:GetAttack())
    if !(card::is_summon_player(d, c, 1 - tp)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_controler_can_be_changed(d, c, false, None)
        && card::is_position(d, c, POS_FACEUP_ATTACK))
    {
        return false;
    }
    let atk = card::get_attack(d, c);
    group::is_exists(d, eg, &|d, x| cfilter3(d, x, atk), 1, NO_EX)
}
/// `s.cfilter3(c,atk)`
fn cfilter3(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:GetAttack()>atk
    card::get_attack(d, c) > atk
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.cfilter1,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter1), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chkc then return s.cfilter2(chkc,e,tp,g) and eg:IsContains(chkc) end
    if let Some(chkc) = a.chkc() {
        let eg = a.eg.clone().nil();
        return cfilter2(d, chkc, a.e, a.tp, &g) && group::is_contains(&eg, chkc);
    }
    // if chk==0 then return eg and eg:IsExists(s.cfilter2,1,nil,e,tp,g) end
    if a.chk == Some(0) {
        return match a.eg.clone() {
            Some(eg) => group::is_exists(d, &eg, &|d, x| cfilter2(d, x, a.e, a.tp, &g), 1, NO_EX),
            None => false,
        };
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local tg=eg:FilterSelect(tp,s.cfilter2,1,1,nil,e,tp,g)
    let eg = a.eg.clone().nil();
    let tg = group::filter_select(d, &eg, a.tp, &|d, x| cfilter2(d, x, a.e, a.tp, &g), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(tg), 1, 0, 0);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS):GetFirst()
    let targets = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS as u32])[0].as_group().cloned().nil();
    let tc = group::get_first(d, &targets);
    // if c:IsRelateToEffect(re) and tc and tc:IsRelateToEffect(re) then
    if card::is_relate_to_effect(d, c, a.re.nil()) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.re.nil()) {
                // c:SetCardTarget(tc)
                card::set_card_target(d, c, tc);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_SET_CONTROL)
                effect::set_code(d, e1, EFFECT_SET_CONTROL);
                // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_OWNER_RELATE)
                effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_OWNER_RELATE, 0);
                // e1:SetRange(LOCATION_MZONE)
                effect::set_range(d, e1, LOCATION_MZONE);
                // e1:SetValue(tp)
                effect::set_value(d, e1, a.tp as i64);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
                effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
                // e1:SetCondition(s.con)
                effect::set_condition(d, e1, Some(condition(con)));
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    }
}
/// `s.con(e)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetOwner()
    let c = effect::get_owner(d, a.e).nil();
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // return c:IsHasCardTarget(h)
    card::is_has_card_target(d, c, h)
}
/// `s.tg(e,c)`
fn tg(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetHandlerPlayer()
    effect::get_handler_player(d, e) as i64
}
/// `s.sccon(e)`
fn sccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_EVIL_EYE_SELENE),e:GetHandlerPlayer(),LOCATION_SZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[CARD_EVIL_EYE_SELENE])), p, LOCATION_SZONE, 0, 1, NO_EX)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_DESTROY_CONFIRMED) then return false end
    if card::is_status(d, c, STATUS_DESTROY_CONFIRMED) {
        return false;
    }
    // local tc=c:GetFirstCardTarget()
    // return tc and eg:IsContains(tc)
    match card::get_first_card_target(d, c) {
        Some(tc) => group::is_contains(&a.eg.clone().nil(), tc),
        None => false,
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT, None, None);
}
