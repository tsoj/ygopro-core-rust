//! Early Palm Gets the Win
use super::prelude::*;

pub const ID: u32 = 58995660;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Roll a six-sided die and apply this effect to that monster you control based on the result until the end of this turn, or, if you activated this effect when an attack was declared involving 2 monsters in the same column, you choose the effect instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DICE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DICE + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.GetBattleMonster(tp) end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp) return Duel.GetBattleMonster(tp) end`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_battle_monster(d, a.tp).0.is_some()
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local bc1=Duel.GetBattleMonster(tp)
    let bc1 = duel::get_battle_monster(d, tp).0.nil();
    // local bc2=bc1:GetBattleTarget()
    let bc2 = card::get_battle_target(d, bc1);
    // local cd=e:GetChainData()
    // cd.in_same_column=bc2 and bc1:GetColumnGroup():IsContains(bc2)
    let in_same_column = match bc2 {
        Some(bc2) => {
            let col = card::get_column_group(d, bc1, 0, 0);
            Value::Bool(group::is_contains(&col, bc2))
        }
        None => Value::Nil,
    };
    chain::set_effect_data(d, a.e, None, chain::field::CARD, in_same_column);
    // cd.own_monster=bc1
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 1, Value::Card(bc1));
    // bc1:CreateEffectRelation(e)
    card::create_effect_relation(d, bc1, a.e);
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, tp, 1);
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local cd=e:GetChainData()
    // local bc=cd.own_monster
    let bc = chain::effect_data(d, a.e, None, chain::field::CARD + 1).as_card().nil();
    // if not bc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, bc, a.e) {
        return;
    }
    // local op=nil
    let op: Option<i64>;
    // if cd.in_same_column then
    if chain::effect_data(d, a.e, None, chain::field::CARD).truthy() {
        // local is_faceup=bc:IsFaceup()
        let is_faceup = card::is_faceup(d, bc);
        // op=Duel.SelectEffect(tp,{true,aux.Stringid(id,1)},{is_faceup,aux.Stringid(id,2)},{is_faceup,aux.Stringid(id,3)})
        op = duel::select_effect(d, tp, &[(true, aux::stringid(ID, 1)), (is_faceup, aux::stringid(ID, 2)), (is_faceup, aux::stringid(ID, 3))]).map(|x| x as i64);
    } else {
        // op=Duel.TossDice(tp,1)
        op = Some(duel::toss_dice(d, tp, 1, 0)[0] as i64);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if op==1 or op==4 then
    if op == Some(1) || op == Some(4) {
        // ● 1 or 4: The first time it would be destroyed by battle or card effect, it is not destroyed
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,4))
        effect::set_description(d, e1, aux::stringid(ID, 4));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetValue(function(e,re,r,rp) return r&(REASON_BATTLE|REASON_EFFECT)>0 end)
        effect::set_value_fn(d, e1, lua_fn(indestructable_value));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
    } else if op == Some(2) || op == Some(5) {
        // ● 2 or 5: It loses 500 ATK, also it is unaffected by activated Spell/Trap effects, except "Early Palm Gets the Win"
        // bc:UpdateAttack(-500,RESETS_STANDARD_PHASE_END,c)
        card::update_attack(d, bc, -500, Some(RESETS_STANDARD_PHASE_END), Some(c), None);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(aux.Stringid(id,5))
        effect::set_description(d, e2, aux::stringid(ID, 5));
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetCode(EFFECT_IMMUNE_EFFECT)
        effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
        // e2:SetValue(function(e,te) return te:IsActivated() and te:IsSpellTrapEffect() and not te:IsCardCode(id) end)
        effect::set_value_fn(d, e2, lua_fn(immune_value));
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // bc:RegisterEffect(e2)
        card::register_effect(d, bc, e2, false);
    } else if op == Some(3) || op == Some(6) {
        // ● 3 or 6: It gains 1000 ATK, also it can make up to 2 attacks on monsters during each Battle Phase
        // bc:UpdateAttack(1000,RESETS_STANDARD_PHASE_END,c)
        card::update_attack(d, bc, 1000, Some(RESETS_STANDARD_PHASE_END), Some(c), None);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetDescription(aux.Stringid(id,6))
        effect::set_description(d, e3, aux::stringid(ID, 6));
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e3:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
        effect::set_code(d, e3, EFFECT_EXTRA_ATTACK_MONSTER);
        // e3:SetValue(1)
        effect::set_value(d, e3, 1);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // bc:RegisterEffect(e3)
        card::register_effect(d, bc, e3, false);
    }
}
/// `function(e,re,r,rp) return r&(REASON_BATTLE|REASON_EFFECT)>0 end`
fn indestructable_value(_d: &mut Duel, a: Args) -> bool {
    (a.int(2) & (REASON_BATTLE | REASON_EFFECT) as i64) > 0
}
/// `function(e,te) return te:IsActivated() and te:IsSpellTrapEffect() and not te:IsCardCode(id) end`
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    effect::is_activated(d, te) && effect::is_spell_trap_effect(d, te) && !effect::is_card_code(d, te, &[ID])
}
