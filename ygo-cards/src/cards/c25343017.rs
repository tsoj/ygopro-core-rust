//! Hushed Psychic Cleric
use super::prelude::*;

pub const ID: u32 = 25343017;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to defense
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.potg)
    effect::set_target(d, e1, Some(cost_target(potg)));
    // e1:SetOperation(s.poop)
    effect::set_operation(d, e1, Some(operation(poop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    // c:RegisterEffect(e2)
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // remove
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(s.rmcost)
    effect::set_cost(d, e3, Some(cost_target(rmcost)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // special summon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCondition(s.spcon)
    effect::set_condition(d, e4, Some(condition(spcon)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local ng=Group.CreateGroup()
    // ng:KeepAlive()
    // e4:SetLabelObject(ng)
    effect::set_label_object(d, e4, Value::Group(CardSet::new()));
    // e3:SetLabelObject(e4)
    effect::set_label_object(d, e3, Value::Effect(e4));
}
/// `s.potg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn potg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return e:GetHandler():IsAttackPos() end
    if a.chk == Some(0) {
        return card::is_attack_pos(d, h);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.poop(e,tp,eg,ep,ev,re,r,rp)`
fn poop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsFaceup() and c:IsAttackPos() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_attack_pos(d, c) && card::is_relate_to_effect(d, c, e) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsAbleToGraveAsCost,1,1,REASON_COST)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), 1, 1, REASON_COST, NO_EX);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PSYCHIC) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_PSYCHIC as u64) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,e:GetHandler())
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if tc and tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)~=0 and c:IsRelateToEffect(e) then
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    if duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // e4 = e:GetLabelObject()
    let e4 = effect::get_label_object(d, e).as_effect().nil();
    // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
    card::register_flag_effect(d, tc, ID, RESET_EVENT + RESETS_STANDARD, 0, 0, 0, 0);
    // e:GetLabelObject():SetLabel(1)
    effect::set_label(d, e4, &[1]);
    // if c:GetFlagEffect(id)==0 then
    if card::get_flag_effect(d, c, ID) == 0 {
        // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~(RESET_LEAVE|RESET_TEMP_REMOVE|RESET_TOGRAVE|RESET_TURN_SET)),0,0)
        card::register_flag_effect(d, c, ID, RESET_EVENT + (RESETS_STANDARD & !(RESET_LEAVE + RESET_TEMP_REMOVE + RESET_TOGRAVE + RESET_TURN_SET)), 0, 0, 0, 0);
        // e:GetLabelObject():GetLabelObject():Clear()
        effect::set_label_object(d, e4, Value::Group(CardSet::new()));
    }
    // e:GetLabelObject():GetLabelObject():AddCard(tc)
    let mut ng = effect::get_label_object(d, e4).as_group().cloned().nil();
    group::add_card(&mut ng, &CardSet::from_card(tc));
    effect::set_label_object(d, e4, Value::Group(ng));
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local rg=e:GetLabelObject()
    // local act=e:GetLabel()
    let act = effect::get_label(d, e);
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    let h = effect::get_handler(d, e).nil();
    // if act==1 and e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) and e:GetHandler():GetFlagEffect(id) then return true
    if act == 1 && card::is_previous_location(d, h, LOCATION_ONFIELD) && card::get_flag_effect(d, h, ID) != 0 {
        return true;
    }
    // else rg:Clear() return false end
    effect::set_label_object(d, e, Value::Group(CardSet::new()));
    false
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetFlagEffect(id)~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::get_flag_effect(d, c, ID) != 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local rg=e:GetLabelObject()
    let mut rg = effect::get_label_object(d, e).as_group().cloned().nil();
    // if chkc then return rg:IsContains(chkc) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&rg, chkc) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if rg:IsExists(s.spfilter,1,nil,e,tp) then return true
        if group::is_exists(d, &rg, &|d, c| spfilter(d, c, e, tp), 1, NO_EX) {
            return true;
        }
        // else rg:Clear() return false end
        effect::set_label_object(d, e, Value::Group(CardSet::new()));
        return false;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=rg:FilterSelect(tp,s.spfilter,1,1,nil,e,tp)
    let sg = group::filter_select(d, &rg, tp, &|d, c| spfilter(d, c, e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // rg:Clear()
    rg = CardSet::new();
    effect::set_label_object(d, e, Value::Group(rg));
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    let Some(tc) = duel::first_target(d) else { return };
    if card::is_relate_to_effect(d, tc, e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
