//! Sauropod Brachion
use super::prelude::*;

pub const ID: u32 = 41753322;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetRange(LOCATION_DECK)
    effect::set_range(d, e1, LOCATION_DECK);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Summon with 1 tribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    // e2:SetCondition(s.otcon)
    effect::set_condition(d, e2, Some(lua_fn(otcon)));
    // e2:SetTarget(s.ottg)
    effect::set_target(d, e2, Some(lua_fn(ottg)));
    // e2:SetOperation(s.otop)
    effect::set_operation(d, e2, Some(lua_fn(otop)));
    // e2:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e2, SUMMON_TYPE_TRIBUTE as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change itself to face-down
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_POSITION + CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTarget(s.postg)
    effect::set_target(d, e3, Some(cost_target(postg)));
    // e3:SetOperation(s.posop)
    effect::set_operation(d, e3, Some(operation(posop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // position
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e4, CATEGORY_POSITION + CATEGORY_SET);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_FLIP_SUMMON_SUCCESS);
    // e4:SetTarget(s.postg2)
    effect::set_target(d, e4, Some(cost_target(postg2)));
    // e4:SetOperation(s.posop2)
    effect::set_operation(d, e4, Some(operation(posop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Double damage
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e5, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e5:SetCondition(s.dcon)
    effect::set_condition(d, e5, Some(condition(dcon)));
    // e5:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e5, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.otcon(e,c)`
fn otcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.CheckReleaseGroup(c:GetControler(),Card.IsRace,1,false,1,true,c,c:GetControler(),nil,false,nil,RACE_DINOSAUR)
    let tp = card::get_controler(d, c);
    duel::check_release_group(
        d,
        tp,
        Some(&|d, x| card::is_race(d, x, RACE_DINOSAUR as u64)),
        1,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), ..Default::default() },
        NO_EX,
        false,
    )
}
/// `s.ottg(e,tp,eg,ep,ev,re,r,rp,c)`
fn ottg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=Duel.SelectReleaseGroup(tp,Card.IsRace,1,1,false,true,true,c,nil,nil,false,nil,RACE_DINOSAUR)
    // (the core passes the number 1 as c here: it is not a card)
    let g = duel::select_release_group(
        d,
        tp,
        Some(&|d, x| card::is_race(d, x, RACE_DINOSAUR as u64)),
        1,
        1,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, ..Default::default() },
        NO_EX,
        false,
    );
    // if g then
    if let Some(g) = g {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.otop(e,tp,eg,ep,ev,re,r,rp,c)`
fn otop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    if let Value::Group(g) = effect::get_label_object(d, e) {
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
        // g:DeleteGroup()
    }
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanTurnSet() and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::is_can_turn_set(d, c) && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET),0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,c,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) {
        // Duel.ChangePosition(c,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, c, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
/// `s.postg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&card::is_can_turn_set), tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,tp,POS_FACEDOWN_DEFENSE)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop2(e,tp,eg,ep,ev,re,r,rp)`
fn posop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&card::is_can_turn_set), tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
    // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
    duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
}
/// `s.dcon(e)`
fn dcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttackTarget()==e:GetHandler()
    duel::get_attack_target(d) == effect::get_handler(d, a.e)
}
