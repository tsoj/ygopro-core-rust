//! World Legacy - "World Lance"
use super::prelude::*;

pub const ID: u32 = 46647144;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_WORLD_LEGACY], listed_series: &[SET_WORLD_LEGACY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkdown
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetValue(s.atktg)
    effect::set_value_fn(d, e2, effect_card_fn(atktg));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // token
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.tkcon)
    effect::set_condition(d, e3, Some(condition(tkcon)));
    // e3:SetTarget(s.tktg)
    effect::set_target(d, e3, Some(cost_target(tktg)));
    // e3:SetOperation(s.tkop)
    effect::set_operation(d, e3, Some(operation(tkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local a=Duel.GetAttacker()
    // local d=Duel.GetAttackTarget()
    // if not (a and d) then return false end
    let (Some(mut at), Some(mut dt)) = (duel::get_attacker(d), duel::get_attack_target(d)) else {
        return false;
    };
    // if a:IsControler(1-tp) then a,d=d,a end
    if card::is_controler(d, at, 1 - a.tp) {
        std::mem::swap(&mut at, &mut dt);
    }
    // e:SetLabelObject(d)
    effect::set_label_object(d, e, Value::Card(dt));
    // local g=Group.FromCards(a,d)
    let mut g = CardSet::from_card(at);
    group::add_card(&mut g, &CardSet::from_card(dt));
    // return a and d and a:IsRelateToBattle() and d:IsRelateToBattle() and g:IsExists(Card.IsType,1,nil,TYPE_LINK)
    card::is_relate_to_battle(d, at) && card::is_relate_to_battle(d, dt) && group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_LINK), 1, NO_EX)
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if tc and tc:IsFaceup() and tc:IsRelateToBattle() then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_battle(d, tc) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(-3000)
            effect::set_value(d, e1, -3000);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
// function s.atktg(e,c)
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_WORLD_LEGACY) and c~=e:GetHandler()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_WORLD_LEGACY as u16]) && Some(c) != effect::get_handler(d, e)
}
// function s.tkcon(e,tp,eg,ep,ev,re,r,rp)
fn tkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonLocation,1,nil,LOCATION_EXTRA)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_location(d, c, LOCATION_EXTRA), 1, NO_EX)
}
// function s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    true
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_WORLD_LEGACY as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
// function s.tkop(e,tp,eg,ep,ev,re,r,rp)
fn tkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        // and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_WORLD_LEGACY,TYPES_TOKEN,0,0,1,RACE_MACHINE,ATTRIBUTE_DARK,POS_FACEUP_DEFENSE)
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP_DEFENSE as u8), None, 0)
        // and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_WORLD_LEGACY,TYPES_TOKEN,0,0,1,RACE_MACHINE,ATTRIBUTE_DARK,POS_FACEUP_DEFENSE,1-tp)
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP_DEFENSE as u8), Some(1 - tp), 0)
    {
        // local t1=Duel.CreateToken(tp,id+1)
        let t1 = duel::create_token(d, tp, ID + 1).nil();
        // local t2=Duel.CreateToken(tp,id+1)
        let t2 = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummonStep(t1,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, t1, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        // Duel.SpecialSummonStep(t2,0,tp,1-tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, t2, 0, tp, 1 - tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
