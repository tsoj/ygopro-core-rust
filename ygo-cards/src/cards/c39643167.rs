//! Bunny Ear Enthusiast
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 39643167;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], counter_place_list: &[0x1208], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 Bunny Ears Counter to a monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.countercond)
    effect::set_condition(d, e1, Some(condition(countercond)));
    // e1:SetTarget(s.countertg)
    effect::set_target(d, e1, Some(cost_target(countertg)));
    // e1:SetOperation(s.counterop)
    effect::set_operation(d, e1, Some(operation(counterop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish itself and a monster with a Bunny Ears Counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.countercond(e,tp,eg,ep,ev,re,r,rp)`
fn countercond(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsMonsterEffect() and re:GetActivateLocation()==LOCATION_MZONE and not re:GetHandler():IsCode(id)
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    if effect::get_activate_location(d, re) != LOCATION_MZONE {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    !card::is_code(d, rh, &[ID])
}
/// `s.countertg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn countertg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return re:GetHandler():IsRelateToEffect(re) and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        return card::is_relate_to_effect(d, rh, re) && card::get_flag_effect(d, c, ID) == 0;
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.counterop(e,tp,eg,ep,ev,re,r,rp)`
fn counterop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsRelateToEffect(re) and rc:IsFaceup() and rc:AddCounter(0x1208,1) then
    if card::is_relate_to_effect(d, rc, re) && card::is_faceup(d, rc) && card::add_counter(d, rc, 0x1208, 1, false) {
        // Cannot be destroyed by battle
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetCondition(function(e) return e:GetHandler():GetCounter(0x1208)>0 end)
        effect::set_condition(d, e1, Some(condition(|d, a| {
            let eh = effect::get_handler(d, a.e).nil();
            card::get_counter(d, eh, 0x1208) > 0
        })));
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // rc:RegisterEffect(e1)
        card::register_effect(d, rc, e1, false);
    }
}
/// `s.cfilter(c)`: `return c:IsAbleToRemove() and c:GetCounter(0x1208)>0`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None) && card::get_counter(d, c, 0x1208) > 0
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.cfilter(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && cfilter(d, chkc) && chkc != c;
    }
    // if chk==0 then return c:IsAbleToRemove() and Duel.IsExistingTarget(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None)
            && duel::is_existing_target(d, Some(&cfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c)
    let mut g = duel::select_target(d, tp, Some(&cfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into()).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 2, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not (c:IsRelateToEffect(e) and tc:IsRelateToEffect(e)) then return end
    if !(card::is_relate_to_effect(d, c, e) && card::is_relate_to_effect(d, tc.nil(), e)) {
        return;
    }
    let tc = tc.nil();
    // local rg=Group.FromCards(c,tc)
    let mut rg = CardSet::from_card(c);
    group::add_card(&mut rg, &CardSet::from_card(tc));
    // if #rg>0 then
    if !rg.is_empty() {
        // local turn_chk=Duel.GetTurnCount()
        let turn_chk = duel::get_turn_count(d, None);
        // local reset_count=Duel.GetCurrentPhase()<=PHASE_STANDBY and 2 or 1
        let reset_count: u16 = if duel::get_current_phase(d) <= PHASE_STANDBY { 2 } else { 1 };
        // aux.RemoveUntil(rg,nil,REASON_EFFECT,PHASE_STANDBY,id+100,e,tp,aux.DefaultFieldReturnOp,
        //     function() return Duel.GetTurnCount()==turn_chk+1 end, RESET_PHASE|PHASE_STANDBY, reset_count)
        aux::remove_until(
            d,
            rg,
            None,
            REASON_EFFECT,
            PHASE_STANDBY,
            ID + 100,
            e,
            tp,
            Some(aux::default_field_return()),
            Some(std::sync::Arc::new(move |d: &mut Duel, _g: &CardSet, _a: &EffArgs| duel::get_turn_count(d, None) == turn_chk + 1)),
            Some(RESET_PHASE | PHASE_STANDBY),
            Some(reset_count),
            None,
            None,
        );
    }
}
