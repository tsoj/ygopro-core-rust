//! Destined Duel
use super::prelude::*;

pub const ID: u32 = 48128081;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this is activated: Each player can Special Summon 1 monster from their hand in Attack Position, then if there are any other face-up monsters on the field, change all of them to face-down Defense Position. For the rest of this turn after this card resolves, you cannot Normal or Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn, when your monster declares an attack: You can negate the attack, and if you do, destroy 1 face-down card your opponent controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.GetAttacker():IsControler(tp) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e2, Some(cost_target(e2_tg)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2, Some(operation(e2_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Once per turn, during your opponent's End Phase: Return this card to the hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e3, Some(cost_target(e3_tg)));
    // e3:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e3, Some(operation(e3_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_EITHER,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_EITHER as u8, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_POSITION,nil,1,PLAYER_ALL,POS_FACEDOWN_DEFENSE)
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, None, 1, PLAYER_ALL as u8, POS_FACEDOWN_DEFENSE as i32);
    true
}
// function(e,tp,eg,ep,ev,re,r,rp) return Duel.GetAttacker():IsControler(tp) end
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, a.tp)
}
// function(e,tp,eg,ep,ev,re,r,rp,chk) ... end  (the e2 target)
fn e2_tg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFacedown,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    true
}
// function(e,tp,eg,ep,ev,re,r,rp) ... end  (the e2 operation)
fn e2_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.NegateAttack() then
    if duel::negate_attack(d) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsFacedown,tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
// function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsTurnPlayer(1-tp) end
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
// function(e,tp,eg,ep,ev,re,r,rp,chk) ... end  (the e3 target)
fn e3_tg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
// function(e,tp,eg,ep,ev,re,r,rp) ... end  (the e3 operation)
fn e3_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
// function s.spfilter(c,e,sp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, sp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,sp,false,false,POS_FACEUP_ATTACK)
    card::is_can_be_special_summoned(d, c, e, 0, sp, false, false, Some(POS_FACEUP_ATTACK as u8), None, None)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local sg=Group.CreateGroup()
    let mut sg = CardSet::new();
    // local turn_player=Duel.GetTurnPlayer()
    let turn_player = duel::get_turn_player(d);
    // local step=turn_player==0 and 1 or -1
    // for sp=turn_player,1-turn_player,step do
    let order: [u8; 2] = if turn_player == 0 { [0, 1] } else { [1, 0] };
    for sp in order {
        // if Duel.GetLocationCount(sp,LOCATION_MZONE,sp)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter,sp,LOCATION_HAND,0,1,nil,e,sp)
        //     and Duel.SelectYesNo(sp,aux.Stringid(id,3)) then
        if duel::get_location_count(d, sp, LOCATION_MZONE as u8, Some(sp), None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, sp)), sp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::select_yes_no(d, sp, aux::stringid(ID, 3))
        {
            // Duel.Hint(HINT_SELECTMSG,sp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, sp, HINTMSG_SPSUMMON as u64);
            // local sc=Duel.SelectMatchingCard(sp,s.spfilter,sp,LOCATION_HAND,0,1,1,nil,e,sp):GetFirst()
            let sel = duel::select_matching_card(d, sp, Some(&|d, c| spfilter(d, c, e, sp)), sp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            // if sc then
            if let Some(sc) = group::get_first(d, &sel) {
                // sg:AddCard(sc)
                group::add_card(&mut sg, &CardSet::from_card(sc));
                // Duel.SpecialSummonStep(sc,0,sp,sp,false,false,POS_FACEUP_ATTACK)
                duel::special_summon_step(d, sc, 0, sp, sp, false, false, POS_FACEUP_ATTACK as u8, None);
            }
        }
    }
    // if Duel.SpecialSummonComplete()>0 and Duel.IsExistingMatchingCard(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,1,sg) then
    if duel::special_summon_complete(d) > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_can_turn_set(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, (&sg).into())
    {
        // local posg=Duel.GetMatchingGroup(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,sg)
        let posg = duel::get_matching_group(d, Some(&|d, c| card::is_can_turn_set(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, (&sg).into());
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.ChangePosition(posg,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, posg, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e1a, aux::stringid(ID, 4));
    // e1a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    // e1a:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1a, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1a:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e1a, EFFECT_CANNOT_SUMMON);
    // e1a:SetTargetRange(1,0)
    effect::set_target_range(d, e1a, 1, 0);
    // e1a:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1a, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1a,tp)
    duel::register_effect(d, e1a, tp);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1b, EFFECT_CANNOT_SPECIAL_SUMMON);
    // Duel.RegisterEffect(e1b,tp)
    duel::register_effect(d, e1b, tp);
}
