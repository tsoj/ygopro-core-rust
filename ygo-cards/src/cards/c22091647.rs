//! Immortal Phoenix Gearfried
use super::prelude::*;

pub const ID: u32 = 22091647;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon from hand by banishing 1 equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip 1 face-up monster on field to this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.eqcon)
    effect::set_condition(d, e2, Some(condition(eqcon)));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.AddEREquipLimit(c,s.eqcon,function(ec,_,tp) return ec:IsControler(1-tp) end,s.equipop,e2)
    aux::add_er_equip_limit(d, c, Some(condition(eqcon)), lua_fn(equip_limit), lua_fn(equipop_lua), e2, None, None, None);
    // Send equip to negate monster effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e3:SetCondition(s.ngcon)
    effect::set_condition(d, e3, Some(condition(ngcon)));
    // e3:SetCost(s.ngcost)
    effect::set_cost(d, e3, Some(cost_target(ngcost)));
    // e3:SetTarget(s.ngtg)
    effect::set_target(d, e3, Some(cost_target(ngtg)));
    // e3:SetOperation(s.ngop)
    effect::set_operation(d, e3, Some(operation(ngop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(c,e,tp,tc)` of `s.equipop`
fn equipop_lua(d: &mut Duel, a: Args) -> bool {
    let c = a.card(0).nil();
    let e = a.e(1);
    let tp = a.int(2) as u8;
    let tc = a.card(3).nil();
    equipop(d, c, e, tp, tc);
    true
}
/// `function(ec,_,tp) return ec:IsControler(1-tp) end`
fn equip_limit(d: &mut Duel, a: Args) -> bool {
    let ec = a.card(0).nil();
    let tp = a.int(2) as u8;
    card::is_controler(d, ec, 1 - tp)
}
/// Check for equip spell to banish
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemoveAsCost() and c:IsEquipSpell() and (c:IsFaceup() or c:IsLocation(LOCATION_GRAVE))
    card::is_able_to_remove_as_cost(d, c, None) && card::is_equip_spell(d, c) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE))
}
/// Cost of banishing from field/GY
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local h=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return h:GetFlagEffect(id)==0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_SZONE|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, h, ID) == 0
            && duel::is_existing_matching_card(d, Some(&spfilter), tp, LOCATION_SZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_SZONE|LOCATION_GRAVE,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&spfilter), tp, LOCATION_SZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// Activation legality
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local h=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// Special Summon from hand
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // Duel.SpecialSummon(c,1,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 1, tp, tp, false, false, POS_FACEUP as u8, None);
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetHandler():GetEquipGroup():Filter(Card.HasFlagEffect,nil,id)
    let h = effect::get_handler(d, a.e).nil();
    let eq = card::get_equip_group(d, h);
    let g = group::filter(d, &eq, &|d, c| card::has_flag_effect(d, c, ID, None), NO_EX);
    // return Duel.GetAttacker()==e:GetHandler() and #g==0
    duel::get_attacker(d) == Some(h) && group::get_count(&g) == 0
}
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and (c:IsControler(tp) or c:IsAbleToChangeControler())
    //   and c:CheckUniqueOnField(tp) and not c:IsForbidden()
    card::is_faceup(d, c)
        && (card::is_controler(d, c, tp) || card::is_able_to_change_controler(d, c))
        && card::check_unique_on_field(d, c, tp, None, None)
        && !card::is_forbidden(d, c)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler(),tp)
    //   and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, h.into())
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, 0, 0);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc,id) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_OWNER_RELATE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OWNER_RELATE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) or Duel.GetLocationCount(tp,LOCATION_SZONE)<1 then return end
    if card::is_facedown(d, c)
        || !card::is_relate_to_effect(d, c, e)
        || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 < 1
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| eqfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into()).nil();
    // if tc then s.equipop(c,e,tp,tc) end
    if let Some(tc) = group::get_first(d, &g) {
        equipop(d, c, e, tp, tc);
    }
}
/// Monster effect is activated, and this card isn't about to die
fn ngcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and re:IsMonsterEffect()
    //   and Duel.IsChainNegatable(ev)
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// Check for equip spell
fn ngfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EQUIP) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EQUIP) && card::is_able_to_grave_as_cost(d, c)
}
/// Cost of sending equip to GY
fn ngcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ngfilter,tp,LOCATION_SZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&ngfilter), tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.ngfilter,tp,LOCATION_SZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&ngfilter), tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
fn ngtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) and re:GetHandler():IsDestructable() then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) && card::is_destructable(d, rh, None) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// Negate the activation of monster effect and destroy it
fn ngop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
