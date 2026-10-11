//! ZS - Ouroboros Sage
use super::prelude::*;

pub const ID: u32 = 32281491;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIA, SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 non-LIGHT "Number" monster from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NUMBER) and c:IsAttributeExcept(ATTRIBUTE_LIGHT)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_NUMBER as u16])
        && card::is_attribute_except(d, c, ATTRIBUTE_LIGHT, None, 0, None)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>1
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_UTOPIA),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 1
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_UTOPIA as u16]))), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Can only declare one attack for the rest of the turn
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e0, EVENT_ATTACK_ANNOUNCE);
    // e0:SetCondition(function(e) return Duel.IsTurnPlayer(e:GetHandlerPlayer()) end)
    effect::set_condition(d, e0, Some(condition(|d, a| {
        let hp = effect::get_handler_player(d, a.e);
        duel::is_turn_player(d, hp)
    })));
    // e0:SetOperation(s.limitop)
    effect::set_operation(d, e0, Some(operation(limitop)));
    // e0:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e0, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e0,tp)
    duel::register_effect(d, e0, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let tc = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &tc);
    // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // Negate its effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // if Duel.GetLocationCount(tp,LOCATION_SZONE)>=2 and c:IsRelateToEffect(e) then
            if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 >= 2 && card::is_relate_to_effect(d, c, a.e) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
                // local uc=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsSetCard,SET_UTOPIA),tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
                let uc = duel::select_matching_card(d, a.tp, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_UTOPIA as u16]))), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
                let uc = group::get_first(d, &uc);
                // if uc then s.equipop(c,uc,e,tp,tc) end
                if let Some(uc) = uc {
                    equipop(d, c, uc, a.e, a.tp, tc);
                }
            }
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.limitop(e,tp,eg,ep,ev,re,r,rp)`
fn limitop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.equipop(c,uc,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, uc: CardId, e: EffectId, tp: u8, tc: CardId) {
    // local g=Group.FromCards(c,uc)
    let mut g = CardSet::from_card(c);
    group::add_card(&mut g, &CardSet::from_card(uc));
    // for ec in g:Iter() do
    for ec in group::iter(d, &g) {
        // if not aux.EquipAndLimitRegister(ec,e,tp,tc) then return end
        if !aux::equip_and_limit_register(d, ec, e, tp, tc, None, None) {
            return;
        }
        // Increase ATK
        // local e1=Effect.CreateEffect(ec)
        let e1 = effect::create_effect(d, ec);
        // e1:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1700)
        effect::set_value(d, e1, 1700);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // ec:RegisterEffect(e1)
        card::register_effect(d, ec, e1, false);
        // local e2=Effect.CreateEffect(ec)
        let e2 = effect::create_effect(d, ec);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
        // e2:SetValue(true)
        effect::set_value(d, e2, 1);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // ec:RegisterEffect(e2)
        card::register_effect(d, ec, e2, false);
    }
    // Double the equipped monster's ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE + CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d);
    // return Duel.GetAttacker()==ec and ec:GetAttack()>0 and bc and bc:IsControler(1-tp)
    if duel::get_attacker(d) != ec {
        return false;
    }
    let ec = ec.nil();
    if card::get_attack(d, ec) <= 0 {
        return false;
    }
    match bc {
        Some(bc) => card::is_controler(d, bc, 1 - a.tp),
        None => false,
    }
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,e:GetHandler():GetEquipTarget(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, ec.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetEquipTarget()
    let tc = card::get_equip_target(d, c).nil();
    // Double its ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetValue(tc:GetAttack()*2)
    let atk = card::get_attack(d, tc);
    effect::set_value(d, e1, (atk * 2) as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // Destroy it during the End Phase
    // aux.DelayedOperation(tc,PHASE_END,id,e,tp,function(ag) Duel.Destroy(ag,REASON_EFFECT) end,nil,0)
    aux::delayed_operation(d, tc, PHASE_END, ID, a.e, a.tp, Some(std::sync::Arc::new(destroy_oper)), None, Some(0), None, None, None);
}
/// `function(ag) Duel.Destroy(ag,REASON_EFFECT) end`
fn destroy_oper(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::destroy(d, ag.clone(), REASON_EFFECT, None, None);
    true
}
