//! Victory Viper XX03
//! ビクトリー・バイパーXX03
use super::prelude::*;

pub const ID: u32 = 93130021;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_OPTION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsRelateToBattle() and c:GetBattleTarget():IsMonster()
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    let bt = card::get_battle_target(d, c).nil();
    card::is_monster(d, bt)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSpellTrap()
    card::is_faceup(d, c) && card::is_spell_trap(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local t1=Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let t1 = duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    // local t2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,c:GetAttack(),c:GetDefense(),c:GetLevel(),c:GetRace(),c:GetAttribute())
    let t2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && {
        let md = duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(card::get_attack(d, c)),
            defense: Some(card::get_defense(d, c)),
            level: Some(card::get_level(d, c)),
            race: Some(card::get_race(d, c)),
            attribute: Some(card::get_attribute(d, c)),
        };
        duel::is_player_can_special_summon_monster(d, tp, ID + 1, md, None, None, 0)
    };
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    // if t1 and t2 then
    //     op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2),aux.Stringid(id,3))
    // elseif t1 then
    //     op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
    // elseif t2 then
    //     op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,3))
    //     if op==1 then op=2 end
    // else op=Duel.SelectOption(tp,aux.Stringid(id,1)) end
    let op = if t1 && t2 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2), aux::stringid(ID, 3)])
    } else if t1 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
    } else if t2 {
        let o = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 3)]);
        if o == 1 { 2 } else { o }
    } else {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1)])
    };
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // else
    } else {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
    }
    true
}
/// `s.tokenatk(e,c)` etc.: the stats of the token copy the Victory Viper
fn tokenatk(d: &mut Duel, e: EffectId) -> i64 {
    // return e:GetOwner():GetAttack()
    let owner = effect::get_owner(d, e).nil();
    card::get_attack(d, owner) as i64
}
fn tokendef(d: &mut Duel, e: EffectId) -> i64 {
    let owner = effect::get_owner(d, e).nil();
    card::get_defense(d, owner) as i64
}
fn tokenlv(d: &mut Duel, e: EffectId) -> i64 {
    let owner = effect::get_owner(d, e).nil();
    card::get_level(d, owner) as i64
}
fn tokenrace(d: &mut Duel, e: EffectId) -> i64 {
    let owner = effect::get_owner(d, e).nil();
    card::get_race(d, owner) as i64
}
fn tokenatt(d: &mut Duel, e: EffectId) -> i64 {
    let owner = effect::get_owner(d, e).nil();
    card::get_attribute(d, owner) as i64
}
/// `s.tokendes(e)`: `return not e:GetOwner():IsRelateToCard(e:GetHandler())`
fn tokendes(d: &mut Duel, a: &EffArgs) -> bool {
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    !card::is_relate_to_card(d, owner, h)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetLabel()==1 then
    let label = effect::get_label(d, a.e);
    if label == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    // elseif e:GetLabel()==2 then
    } else if label == 2 {
        // local atk=c:GetAttack()
        let atk = card::get_attack(d, c);
        // local def=c:GetDefense()
        let def = card::get_defense(d, c);
        // local lv=c:GetLevel()
        let lv = card::get_level(d, c);
        // local race=c:GetRace()
        let race = card::get_race(d, c);
        // local att=c:GetAttribute()
        let att = card::get_attribute(d, c);
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 or not c:IsRelateToEffect(e) or c:IsFacedown()
        //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,atk,def,lv,race,att) then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
            || !card::is_relate_to_effect(d, c, a.e)
            || card::is_facedown(d, c)
        {
            return;
        }
        let md = duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(atk),
            defense: Some(def),
            level: Some(lv),
            race: Some(race),
            attribute: Some(att),
        };
        if !duel::is_player_can_special_summon_monster(d, tp, ID + 1, md, None, None, 0) {
            return;
        }
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // c:CreateRelation(token,RESET_EVENT|RESETS_STANDARD)
        card::create_relation(d, c, token, RESET_EVENT | RESETS_STANDARD);
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetValue(s.tokenatk)
        effect::set_value_fn(d, e1, effect_value(tokenatk));
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // token:RegisterEffect(e1,true)
        card::register_effect(d, token, e1, true);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // e2:SetValue(s.tokendef)
        effect::set_value_fn(d, e2, effect_value(tokendef));
        // token:RegisterEffect(e2,true)
        card::register_effect(d, token, e2, true);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e3, EFFECT_CHANGE_LEVEL);
        // e3:SetValue(s.tokenlv)
        effect::set_value_fn(d, e3, effect_value(tokenlv));
        // e3:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e3, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // token:RegisterEffect(e3,true)
        card::register_effect(d, token, e3, true);
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        // e4:SetCode(EFFECT_CHANGE_RACE)
        effect::set_code(d, e4, EFFECT_CHANGE_RACE);
        // e4:SetValue(s.tokenrace)
        effect::set_value_fn(d, e4, effect_value(tokenrace));
        // e4:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e4, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // token:RegisterEffect(e4,true)
        card::register_effect(d, token, e4, true);
        // local e5=Effect.CreateEffect(c)
        let e5 = effect::create_effect(d, c);
        // e5:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
        // e5:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e5, EFFECT_CHANGE_ATTRIBUTE);
        // e5:SetValue(s.tokenatt)
        effect::set_value_fn(d, e5, effect_value(tokenatt));
        // e5:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e5, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // token:RegisterEffect(e5,true)
        card::register_effect(d, token, e5, true);
        // local e6=Effect.CreateEffect(c)
        let e6 = effect::create_effect(d, c);
        // e6:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
        // e6:SetCode(EFFECT_SELF_DESTROY)
        effect::set_code(d, e6, EFFECT_SELF_DESTROY);
        // e6:SetCondition(s.tokendes)
        effect::set_condition(d, e6, Some(condition(tokendes)));
        // e6:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e6, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // token:RegisterEffect(e6,true)
        card::register_effect(d, token, e6, true);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    } else {
        // if c:IsRelateToEffect(e) and c:IsFaceup() then
        if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(400)
            effect::set_value(d, e1, 400);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
