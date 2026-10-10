//! Divine Dragon - Excelion
use super::prelude::*;

pub const ID: u32 = 10032958;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to defense
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=Duel.GetMatchingGroupCount(Card.IsCode,tp,LOCATION_GRAVE,0,nil,id)
    let mut ct = duel::get_matching_group_count(d, Some(&|d, x| card::is_code(d, x, &[ID])), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64;
    // if ct>3 then ct=3 end
    if ct > 3 {
        ct = 3;
    }
    // if ct>0 and c:IsFaceup() and c:IsRelateToEffect(e) then
    if ct > 0 && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local opt1=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2),aux.Stringid(id,3))
        let opt1 = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2), aux::stringid(ID, 3)]);
        // local opt2=0 (only read after it is assigned below)
        let mut opt2: i32;
        // local opt3=0 (only read after it is assigned below)
        let opt3: i32;
        // s.reg(c,opt1)
        reg(d, c, opt1);
        // if ct<2 then return end
        if ct < 2 {
            return;
        }
        // if opt1==0 then opt2=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))+1
        if opt1 == 0 {
            opt2 = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]) + 1;
        }
        // elseif opt1==2 then opt2=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        else if opt1 == 2 {
            opt2 = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
        }
        // else
        else {
            // opt2=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,3))
            opt2 = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 3)]);
            // if opt2==1 then opt2=2 end
            if opt2 == 1 {
                opt2 = 2;
            }
        }
        // s.reg(c,opt2)
        reg(d, c, opt2);
        // if ct<3 then return end
        if ct < 3 {
            return;
        }
        // if opt1~=0 and opt2~=0 then opt3=Duel.SelectOption(tp,aux.Stringid(id,1))
        if opt1 != 0 && opt2 != 0 {
            opt3 = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]);
        }
        // elseif opt1~=1 and opt2~=1 then opt3=Duel.SelectOption(tp,aux.Stringid(id,2))+1
        else if opt1 != 1 && opt2 != 1 {
            opt3 = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]) + 1;
        }
        // else opt3=Duel.SelectOption(tp,aux.Stringid(id,3))+2 end
        else {
            opt3 = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 3)]) + 2;
        }
        // s.reg(c,opt3)
        reg(d, c, opt3);
    }
}
/// `s.reg(c,opt)`
fn reg(d: &mut Duel, c: CardId, opt: i32) {
    if opt == 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    } else if opt == 1 {
        // chain attack
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
        // e1:SetCode(EVENT_BATTLE_DESTROYING)
        effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
        // e1:SetCondition(s.atcon)
        effect::set_condition(d, e1, Some(condition(atcon)));
        // e1:SetOperation(s.atop)
        effect::set_operation(d, e1, Some(operation(atop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    } else {
        // damage
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e1, aux::stringid(ID, 3));
        // e1:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, e1, CATEGORY_DAMAGE);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EVENT_BATTLE_DESTROYING)
        effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
        // e1:SetCondition(aux.bdgcon)
        effect::set_condition(d, e1, Some(aux::bdgcon()));
        // e1:SetTarget(s.damtg)
        effect::set_target(d, e1, Some(cost_target(damtg)));
        // e1:SetOperation(s.damop)
        effect::set_operation(d, e1, Some(operation(damop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.atcon(e,tp,eg,ep,ev,re,r,rp)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return aux.bdocon(e,tp,eg,ep,ev,re,r,rp) and e:GetHandler():CanChainAttack()
    let args = [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if !aux::bdocon().call(d, &args).truthy() {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::can_chain_attack(d, h, Some(2), false)
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, _a: &EffArgs) {
    // Duel.ChainAttack()
    duel::chain_attack(d, None);
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c).nil();
    // local dam=bc:GetAttack()
    let mut dam = card::get_attack(d, bc) as i64;
    // if dam<0 then dam=0 end
    if dam < 0 {
        dam = 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
