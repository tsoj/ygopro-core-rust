//! Enneacraft - Aiza.LEON
use super::prelude::*;

pub const ID: u32 = 82359538;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[COUNTER_EC as u32],
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
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(activity_filter));
    // Return up to 3 cards on the field to the hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e4, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.rthcon)
    effect::set_condition(d, e4, Some(condition(rthcon)));
    // e4:SetCost(Cost.SelfChangePosition(POS_FACEUP_DEFENSE))
    effect::set_cost(d, e4, Some(cost::self_change_position(POS_FACEUP_DEFENSE)));
    // e4:SetTarget(s.rthtg)
    effect::set_target(d, e4, Some(cost_target(rthtg)));
    // e4:SetOperation(s.rthop)
    effect::set_operation(d, e4, Some(operation(rthop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Each time a card(s) is added to your opponent's hand by their card effect, inflict 900 damage to your opponent for each
    // local e5a=Effect.CreateEffect(c)
    let e5a = effect::create_effect(d, c);
    // e5a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e5a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e5a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e5a:SetCode(EVENT_FLIP)
    effect::set_code(d, e5a, EVENT_FLIP);
    // e5a:SetOperation(function(e) e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,3)) end)
    effect::set_operation(d, e5a, Some(lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 3));
    })));
    // c:RegisterEffect(e5a)
    card::register_effect(d, c, e5a, false);
    // local e5b=Effect.CreateEffect(c)
    let e5b = effect::create_effect(d, c);
    // e5b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5b:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e5b, EVENT_TO_HAND);
    // e5b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5b, LOCATION_MZONE);
    // e5b:SetCondition(s.damcon)
    effect::set_condition(d, e5b, Some(condition(damcon)));
    // e5b:SetOperation(s.damop)
    effect::set_operation(d, e5b, Some(operation(damop)));
    // c:RegisterEffect(e5b)
    card::register_effect(d, c, e5b, false);
    // local e5c=Effect.CreateEffect(c)
    let e5c = effect::create_effect(d, c);
    // e5c:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5c, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5c:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e5c, EFFECT_FLAG_DELAY, 0);
    // e5c:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e5c, EVENT_CHAIN_SOLVED);
    // e5c:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5c, LOCATION_MZONE);
    // e5c:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e5c, Some(condition(e5c_condition)));
    // e5c:SetOperation(s.chainsolvedop)
    effect::set_operation(d, e5c, Some(operation(chainsolvedop)));
    // c:RegisterEffect(e5c)
    card::register_effect(d, c, e5c, false);
}
/// `function(c) return c:IsPosition(POS_FACEDOWN_DEFENSE) end` (the SPSUMMON activity filter)
fn activity_filter(d: &mut Duel, a: Args) -> bool {
    card::is_position(d, a.c(0), POS_FACEDOWN_DEFENSE)
}
/// `s.counterop(e,tp,eg,ep,ev,re,r,rp)`
fn counterop(d: &mut Duel, a: &EffArgs) {
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
        effect::set_operation(d, e1, Some(lua_fn(move |d, _a: Args| {
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
        effect::set_operation(d, e2, Some(lua_fn(move |d, _a: Args| {
            effect::reset(d, e1);
        })));
        // e2:SetReset(RESET_CHAIN)
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
}
/// `s.descon(e,tp,...)`
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
/// `s.destg(e,tp,...,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local atk=e:GetHandler():GetAttack()-1
    let h = effect::get_handler(d, a.e).nil();
    let atk = card::get_attack(d, h) - 1;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsAttackBelow(atk) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && card::is_attack_below(d, chkc, atk);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAttackBelow,atk),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| faceup_below(d, cc, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAttackBelow,atk),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, cc| faceup_below(d, cc, atk)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    true
}
/// `aux.FaceupFilter(Card.IsAttackBelow,atk)`
fn faceup_below(d: &mut Duel, c: CardId, atk: i32) -> bool {
    card::is_faceup(d, c) && card::is_attack_below(d, c, atk)
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.Destroy(tc,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
/// `s.spcost(e,tp,...,chk)`
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
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(Card.IsCanBeSpecialSummoned,tp,LOCATION_HAND,0,1,nil,e,0,tp,false,false,POS_FACEDOWN_DEFENSE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, cc| card::is_can_be_special_summoned(d, cc, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)),
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
/// `s.spop(e,tp,...)`
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
        Some(&|d, cc| card::is_can_be_special_summoned(d, cc, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)),
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
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
    }
}
/// `s.rthcon(e,tp,...)`
fn rthcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ch=Duel.GetCurrentChain()-1
    let ch = duel::get_current_chain(d, false) - 1;
    // return e:GetHandler():IsFacedown() and ch>0 and ep==1-tp and Duel.GetChainInfo(ch,CHAININFO_TRIGGERING_CONTROLER)==tp
    let h = effect::get_handler(d, a.e).nil();
    card::is_facedown(d, h)
        && ch > 0
        && a.ep == 1 - tp
        && duel::get_chain_info(d, ch as u8, &[CHAININFO_TRIGGERING_CONTROLER])[0].int() == tp as i64
}
/// `s.rthtg(e,tp,...,chk)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, cc| card::is_able_to_hand(d, cc, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_ALL,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    true
}
/// `s.rthop(e,tp,...)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,3,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| card::is_able_to_hand(d, cc, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 3, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
/// `function(e) return e:GetHandler():HasFlagEffect(id) end`
fn e5c_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, h, ID, None)
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    card::has_flag_effect(d, h, ID, None)
        && a.rp == 1 - tp
        && a.r & REASON_EFFECT > 0
        && group::is_exists(d, &eg, &|d, cc| card::is_controler(d, cc, 1 - tp), 1, NO_EX)
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // local ct=eg:FilterCount(Card.IsControler,nil,1-tp)
    let ct = group::filter_count(d, &eg, &|d, cc| card::is_controler(d, cc, 1 - tp), NO_EX);
    // if Duel.IsChainSolving() then
    if duel::is_chain_solving(d) {
        // If a chain link is resolving, register a flag to deal the damage after it
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // for i=1,ct do c:RegisterFlagEffect(id+1,RESET_CHAIN,0,1) end
        for _ in 0..ct {
            card::register_flag_effect(d, c, ID + 1, RESET_CHAIN, 0, 1, 0, 0);
        }
    } else {
        // If a chain is not resolving, deal the damage right away
        // Duel.Hint(HINT_CARD,1-tp,id)
        duel::hint(d, HINT_CARD as u8, 1 - tp, ID as u64);
        // Duel.Damage(1-tp,ct*900,REASON_EFFECT)
        duel::damage(d, 1 - tp, (ct * 900) as i64, REASON_EFFECT, false, None);
    }
}
/// `s.chainsolvedop(e,tp,...)`
fn chainsolvedop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=e:GetHandler():GetFlagEffect(id+1)
    let c = effect::get_handler(d, a.e).nil();
    let ct = card::get_flag_effect(d, c, ID + 1);
    // if ct>0 then
    if ct > 0 {
        // Duel.Hint(HINT_CARD,1-tp,id)
        duel::hint(d, HINT_CARD as u8, 1 - tp, ID as u64);
        // Duel.Damage(1-tp,ct*900,REASON_EFFECT)
        duel::damage(d, 1 - tp, (ct * 900) as i64, REASON_EFFECT, false, None);
        // e:GetHandler():ResetFlagEffect(id+1)
        card::reset_flag_effect(d, c, ID + 1);
    }
}
