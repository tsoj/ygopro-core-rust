//! Dark Magic Mirror Force
use super::prelude::*;

pub const ID: u32 = 59016454;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN, CARD_SHINING_SARCOPHAGUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate (on attack)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate (on effect)
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.GetAttacker():IsControler(1-tp)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.ListsCode,CARD_SHINING_SARCOPHAGUS),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let atk = duel::get_attacker(d).nil();
    card::is_controler(d, atk, 1 - tp)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::lists_code(d, c, &[CARD_SHINING_SARCOPHAGUS])), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
// function s.effcon(e,tp,eg,ep,ev,re,r,rp)
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if not (rp==1-tp and re:IsMonsterEffect()
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.ListsCode,CARD_SHINING_SARCOPHAGUS),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)) then return false end
    if !(a.rp == 1 - tp
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::lists_code(d, c, &[CARD_SHINING_SARCOPHAGUS])), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX))
    {
        return false;
    }
    // local ex,tg,ct=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    // return ex and tg~=nil and ct+tg:FilterCount(aux.AND(Card.IsOnField,Card.IsMonster),nil)-#tg>0
    match duel::get_operation_info(d, a.ev as u32, CATEGORY_DESTROY) {
        None => false,
        Some(op) => match op.op_cards {
            None => false,
            Some(tg) => {
                let monsters = group::filter_count(d, &tg, &|d, c| card::is_on_field(d, c) && card::is_monster(d, c), NO_EX) as i64;
                op.op_count as i64 + monsters - tg.len() as i64 > 0
            }
        },
    }
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAttackPos,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_attack_pos(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*500)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, (g.len() * 500) as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAttackPos,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_attack_pos(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // local ct=0
    let mut ct = 0;
    // if #g>0 then ct=Duel.Destroy(g,REASON_EFFECT) end
    if !g.is_empty() {
        ct = duel::destroy(d, &g, REASON_EFFECT, None, None);
    }
    // if ct>0 and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_DARK_MAGICIAN),tp,LOCATION_MZONE,0,1,nil) then
    if ct > 0 && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_DARK_MAGICIAN])), tp, LOCATION_MZONE, 0, 1, NO_EX) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,ct*500,REASON_EFFECT)
        duel::damage(d, 1 - tp, (ct * 500) as i64, REASON_EFFECT, false, None);
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // --The first time each monster you control that mentions "Shining Sarcophagus" would be destroyerd by battle or card effect, it is not destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.ListsCode,CARD_SHINING_SARCOPHAGUS))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::lists_code(d, c, &[CARD_SHINING_SARCOPHAGUS]))));
    // e1:SetValue(function(e,re,r,rp) return r&(REASON_BATTLE|REASON_EFFECT)>0 and 1 or 0 end)
    effect::set_value_fn(d, e1, lua_fn(indestructable_value));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
}
// function(e,re,r,rp) return r&(REASON_BATTLE|REASON_EFFECT)>0 and 1 or 0 end
fn indestructable_value(_d: &mut Duel, a: Args) -> i64 {
    let r = a.int(2) as u32;
    if r & (REASON_BATTLE | REASON_EFFECT) > 0 {
        1
    } else {
        0
    }
}
