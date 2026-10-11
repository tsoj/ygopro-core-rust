//! Enneacraft - Asta.PIXEA
use super::prelude::*;

pub const ID: u32 = 28454232;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[COUNTER_EC],
    listed_series: &[SET_ENNEACRAFT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_EC,LOCATION_PZONE)
    card::enable_counter_permit(d, c, COUNTER_EC as u16, Some(LOCATION_PZONE), None);
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Each time a monster(s) is flipped face-up, place 1 E.C. Counter on this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_FLIP)
    effect::set_code(d, e1, EVENT_FLIP);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetOperation(s.counterop)
    effect::set_operation(d, e1, Some(operation(counterop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 monster your opponent controls with less ATK than this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 monster from your hand in face-down Defense Position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCost(Cost.AND(Cost.SelfReveal,s.spcost))
    effect::set_cost(d, e3, Some(cost::and(vec![cost::self_reveal(), cost_target(spcost)])));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return c:IsPosition(POS_FACEDOWN_DEFENSE) end)
    duel::add_custom_activity_counter(
        d,
        ID,
        ACTIVITY_SPSUMMON,
        lua_fn(|d, a| card::is_position(d, a.c(0), POS_FACEDOWN_DEFENSE)),
    );
    // Negate an opponent's effect that targets a card(s) you control, then you can banish 1 random card from your opponent's hand, face-down
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_DISABLE+CATEGORY_REMOVE)
    effect::set_category(d, e4, CATEGORY_DISABLE + CATEGORY_REMOVE);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e4, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.discon)
    effect::set_condition(d, e4, Some(condition(discon)));
    // e4:SetCost(Cost.SelfChangePosition(POS_FACEUP_DEFENSE))
    effect::set_cost(d, e4, Some(cost::self_change_position(POS_FACEUP_DEFENSE)));
    // e4:SetTarget(s.distg)
    effect::set_target(d, e4, Some(cost_target(distg)));
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(operation(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // After this card was flipped face-up, while it is in the Monster Zone, your opponent cannot target cards in the field or GY with card effects
    // local e5a=Effect.CreateEffect(c)
    let e5a = effect::create_effect(d, c);
    // e5a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e5a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e5a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e5a:SetCode(EVENT_FLIP)
    effect::set_code(d, e5a, EVENT_FLIP);
    // e5a:SetOperation(function(e) e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,3)) end)
    effect::set_operation(d, e5a, Some(operation(e5a_op)));
    // c:RegisterEffect(e5a)
    card::register_effect(d, c, e5a, false);
    // local e5b=Effect.CreateEffect(c)
    let e5b = effect::create_effect(d, c);
    // e5b:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5b, EFFECT_TYPE_FIELD);
    // e5b:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e5b, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e5b:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e5b, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e5b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5b, LOCATION_MZONE);
    // e5b:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e5b, Some(condition(e5b_cond)));
    // e5b:SetTargetRange(LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE)
    effect::set_target_range(d, e5b, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE);
    // e5b:SetValue(aux.tgoval)
    effect::set_value_fn(d, e5b, aux::tgoval());
    // c:RegisterEffect(e5b)
    card::register_effect(d, c, e5b, false);
}
// function(e) e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,3)) end
fn e5a_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 3));
}
// function(e) return e:GetHandler():HasFlagEffect(id) end
fn e5b_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, h, ID, None)
}
// s.counterop(e,tp,eg,ep,ev,re,r,rp)
fn counterop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // c:AddCounter(COUNTER_EC,1)
        card::add_counter(d, c, COUNTER_EC as u16, 1, false);
    } else {
        // Place 1 Sinquisition Counter on this card at the end of the Chain Link
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
        // e1:SetRange(LOCATION_PZONE)
        effect::set_range(d, e1, LOCATION_PZONE);
        // e1:SetOperation(function() c:AddCounter(COUNTER_EC,1) end)
        effect::set_operation(d, e1, Some(operation(move |d, _a| {
            card::add_counter(d, c, COUNTER_EC as u16, 1, false);
        })));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // Reset "e1" at the end of the Chain Link
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
        // e2:SetOperation(function() e1:Reset() end)
        // (e1 is kept as the label object of e2)
        effect::set_label_object(d, e2, Value::Effect(e1));
        effect::set_operation(d, e2, Some(operation(e2_op)));
        // e2:SetReset(RESET_CHAIN)
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
    }
}
// function() e1:Reset() end
fn e2_op(d: &mut Duel, a: &EffArgs) {
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::reset(d, e1);
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local pc=Duel.GetFirstMatchingCard(nil,tp,LOCATION_PZONE,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let pc = duel::get_first_matching_card(d, None, a.tp, LOCATION_PZONE, 0, h.into());
    // return pc and pc:IsSetCard(SET_ENNEACRAFT)
    match pc {
        Some(pc) => card::is_set_card(d, pc, &[SET_ENNEACRAFT as u16]),
        None => false,
    }
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local atk=e:GetHandler():GetAttack()-1
    let h = effect::get_handler(d, a.e).nil();
    let atk = card::get_attack(d, h) - 1;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsAttackBelow(atk) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp)
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_faceup(d, chkc)
            && card::is_attack_below(d, chkc, atk);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAttackBelow,atk),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&aux::faceup_filter(move |d, c| card::is_attack_below(d, c, atk))), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAttackBelow,atk),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(
        d,
        tp,
        Some(&aux::faceup_filter(move |d, c| card::is_attack_below(d, c, atk))),
        tp,
        0,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0;
    }
    // You cannot Special Summon the turn you activate this effect, except in face-down Defense Position
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e1, aux::stringid(ID, 4));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_FORCE_SPSUMMON_POSITION)
    effect::set_code(d, e1, EFFECT_FORCE_SPSUMMON_POSITION);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(POS_FACEDOWN_DEFENSE)
    effect::set_value(d, e1, POS_FACEDOWN_DEFENSE as i64);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(Card.IsCanBeSpecialSummoned,tp,LOCATION_HAND,0,1,nil,e,0,tp,false,false,POS_FACEDOWN_DEFENSE) end
    if a.chk == Some(0) {
        let e = a.e;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)),
                tp,
                LOCATION_HAND,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsCanBeSpecialSummoned,tp,LOCATION_HAND,0,1,1,nil,e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)),
        tp,
        LOCATION_HAND,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
    }
}
// function s.disconfilter(c,tp)
fn disconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsOnField() and c:IsControler(tp)
    card::is_on_field(d, c) && card::is_controler(d, c, tp)
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    // if not (rp==1-tp and e:GetHandler():IsFacedown() and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and Duel.IsChainDisablable(ev)) then return false end
    if !(a.rp == 1 - tp
        && card::is_facedown(d, h)
        && effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0)
        && duel::is_chain_disablable(d, a.ev as u8))
    {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // return tg and tg:IsExists(s.disconfilter,1,nil,tp)
    match tg {
        Some(tg) => group::is_exists(d, &tg, &|d, c| disconfilter(d, c, tp), 1, NO_EX),
        None => false,
    }
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_HAND as i32);
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local rc=re:GetHandler()
    let _rc = effect::get_handler(d, a.re.nil()).nil();
    // if not Duel.NegateEffect(ev) then return end
    if !duel::negate_effect(d, a.ev as u8) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil,tp,POS_FACEDOWN)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)),
        tp,
        0,
        LOCATION_HAND,
        NO_EX,
    );
    // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,5)) then return end
    if group::get_count(&g) == 0 || !duel::select_yes_no(d, tp, aux::stringid(ID, 5)) {
        return;
    }
    // local rg=g:RandomSelect(tp,1)
    let rg = group::random_select(d, &g, tp, 1);
    // if #rg>0 then
    if group::get_count(&rg) > 0 {
        // Duel.Remove(rg,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, rg, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    }
}
