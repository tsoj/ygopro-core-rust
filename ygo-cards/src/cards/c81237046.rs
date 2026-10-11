//! Enneacraft - Archa.TAIL
use super::prelude::*;

pub const ID: u32 = 81237046;

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
    // Apply "Monsters and "Enneacraft" Spells you control cannot be destroyed by card effects for the rest of this turn
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e4, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.indescon)
    effect::set_condition(d, e4, Some(condition(indescon)));
    // e4:SetCost(Cost.SelfChangePosition(POS_FACEUP_DEFENSE))
    effect::set_cost(d, e4, Some(cost::self_change_position(POS_FACEUP_DEFENSE)));
    // e4:SetOperation(s.indesop)
    effect::set_operation(d, e4, Some(operation(indesop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // After this card was flipped face-up, while it is in the Monster Zone, each time a monster(s) is sent to your opponent's GY, inflict 900 damage to your opponent
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
    // e5b:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e5b, EVENT_TO_GRAVE);
    // e5b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5b, LOCATION_MZONE);
    // e5b:SetCondition(function(e,tp,eg) return e:GetHandler():HasFlagEffect(id) and eg:IsExists(s.damconfilter,1,nil,tp) end)
    effect::set_condition(d, e5b, Some(condition(e5b_condition)));
    // e5b:SetOperation(s.damop)
    effect::set_operation(d, e5b, Some(operation(damop)));
    // c:RegisterEffect(e5b)
    card::register_effect(d, c, e5b, false);
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
/// `s.indescon(e,tp,eg,ep,ev,re,r,rp)`
fn indescon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if rp==tp or Duel.HasFlagEffect(tp,id) or e:GetHandler():IsFaceup() then return false end
    if a.rp == tp || duel::has_flag_effect(d, tp, ID, None) || card::is_faceup(d, h) {
        return false;
    }
    // if re:IsHasCategory(CATEGORY_NEGATE) and Duel.GetChainInfo(ev-1,CHAININFO_TRIGGERING_EFFECT):IsHasType(EFFECT_TYPE_ACTIVATE) then return false end
    let re = a.re.nil();
    if effect::is_has_category(d, re, CATEGORY_NEGATE) {
        let te = duel::get_chain_info(d, (a.ev - 1) as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
        if effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE) {
            return false;
        }
    }
    // local ex,tg,tc=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    // return ex and tg~=nil and tg:FilterCount(Card.IsOnField,nil)>0
    match duel::get_operation_info(d, a.ev, CATEGORY_DESTROY) {
        Some(info) => match info.op_cards {
            Some(tg) => group::filter_count(d, &tg, &|d, cc| card::is_on_field(d, cc), NO_EX) > 0,
            None => false,
        },
        None => false,
    }
}
/// `s.indesop(e,tp,...)`
fn indesop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.HasFlagEffect(tp,id) then return end
    if duel::has_flag_effect(d, tp, ID, None) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,5))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 5)), 0, None);
    // Monsters and "Enneacraft" Spells you control cannot be destroyed by card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e1, LOCATION_ONFIELD, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_MZONE) or (c:IsFaceup() and c:IsSpell() and c:IsSetCard(SET_ENNEACRAFT)) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, cc| {
        card::is_location(d, cc, LOCATION_MZONE) || (card::is_faceup(d, cc) && card::is_spell(d, cc) && card::is_set_card(d, cc, &[SET_ENNEACRAFT as u16]))
    })));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():HasFlagEffect(id) and eg:IsExists(s.damconfilter,1,nil,tp) end`
fn e5b_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    card::has_flag_effect(d, h, ID, None) && group::is_exists(d, &eg, &|d, cc| damconfilter(d, cc, tp), 1, NO_EX)
}
/// `s.damconfilter(c,tp)`: `return c:IsMonster() and c:IsControler(1-tp)`
fn damconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_monster(d, c) && card::is_controler(d, c, 1 - tp)
}
/// `s.damop(e,tp,...)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_CARD,1-tp,id)
    duel::hint(d, HINT_CARD as u8, 1 - tp, ID as u64);
    // Duel.Damage(1-tp,900,REASON_EFFECT)
    duel::damage(d, 1 - tp, 900, REASON_EFFECT, false, None);
}
