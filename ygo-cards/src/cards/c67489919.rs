//! Mecha Phantom Beast Blue Impala
//! 幻獣機ブルーインパラス
use super::prelude::*;

pub const ID: u32 = 67489919;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MECHA_PHANTOM_BEAST],
    listed_names: &[TOKEN_MECHA_PHANTOM_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by battle or effects while you control a Token
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetCondition(s.indcon)
    effect::set_condition(d, e1, Some(condition(indcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Mecha Phantom Beast Token"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Can only be used as Synchro material with "Mecha Phantom Beast" monsters
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_SYNCHRO_MAT_RESTRICTION)
    effect::set_code(d, e4, EFFECT_SYNCHRO_MAT_RESTRICTION);
    // e4:SetValue(s.synfilter)
    effect::set_value_fn(d, e4, lua_fn(|d, a: Args| synfilter(d, a.e(0), a.c(1))));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Cannot be used as a Synchro material, except for a Machine monster
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
    effect::set_code(d, e5, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e5:SetValue(s.synlimit)
    effect::set_value_fn(d, e5, lua_fn(|d, a: Args| synlimit(d, a.card(1))));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Can use materials from the hand
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e6, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e6:SetCode(EFFECT_HAND_SYNCHRO)
    effect::set_code(d, e6, EFFECT_HAND_SYNCHRO);
    // e6:SetLabel(id)
    effect::set_label(d, e6, &[ID as i64]);
    // e6:SetValue(s.synval)
    effect::set_value_fn(d, e6, lua_fn(|d, a: Args| synval(d, a)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.synfilter(e,c)`
fn synfilter(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_MECHA_PHANTOM_BEAST) and c:IsControler(e:GetHandlerPlayer())
    //     and (c:IsLocation(LOCATION_HAND) or (c:IsFaceup() and c:IsLocation(LOCATION_MZONE)))
    let p = effect::get_handler_player(d, e);
    card::is_set_card(d, c, &[SET_MECHA_PHANTOM_BEAST as u16])
        && card::is_controler(d, c, p)
        && (card::is_location(d, c, LOCATION_HAND) || (card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE)))
}
/// `s.synval(e,c,sc)`
fn synval(d: &mut Duel, a: Args) -> bool {
    // if c:IsLocation(LOCATION_HAND) then
    let c = a.c(1);
    if card::is_location(d, c, LOCATION_HAND) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e(0)).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_HAND_SYNCHRO+EFFECT_SYNCHRO_CHECK)
        effect::set_code(d, e1, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK);
        // e1:SetLabel(id)
        effect::set_label(d, e1, &[ID as i64]);
        // e1:SetTarget(s.synchktg)
        effect::set_target(d, e1, Some(lua_fn(synchktg)));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // return true
        return true;
    }
    // else return false end
    false
}
/// `s.chk2(c)`
fn chk2(d: &mut Duel, c: CardId) -> bool {
    // if not c:IsHasEffect(EFFECT_HAND_SYNCHRO) or c:IsHasEffect(EFFECT_HAND_SYNCHRO+EFFECT_SYNCHRO_CHECK) then return false end
    if card::is_has_effect(d, c, EFFECT_HAND_SYNCHRO, None).is_empty() || !card::is_has_effect(d, c, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK, None).is_empty() {
        return false;
    }
    // local te={c:GetCardEffect(EFFECT_HAND_SYNCHRO)}
    let te = card::get_card_effect(d, c, EFFECT_HAND_SYNCHRO);
    // for i=1,#te do
    for e in te {
        // local e=te[i]
        // if e:GetLabel()==id then return true end
        if effect::get_label(d, e) == ID as i64 {
            return true;
        }
    }
    // return false
    false
}
/// `s.synchktg(e,c,sg,tg,ntg,tsg,ntsg)`
fn synchktg(d: &mut Duel, a: Args) -> Value {
    // if c then
    if let Some(c) = a.card(1) {
        let sg = a.g(2);
        let tg = a.g(3);
        let ntg = a.g(4);
        // local res=tg:IsExists(s.chk2,1,c) or ntg:IsExists(s.chk2,1,c) or sg:IsExists(s.chk2,1,c)
        let res = group::is_exists(d, &tg, &chk2, 1, c.into()) || group::is_exists(d, &ntg, &chk2, 1, c.into()) || group::is_exists(d, &sg, &chk2, 1, c.into());
        // return res,Group.CreateGroup(),Group.CreateGroup()
        return Value::Multi(vec![Value::Bool(res), Value::Group(CardSet::new()), Value::Group(CardSet::new())]);
    }
    // else return true end
    Value::Bool(true)
}
/// `s.tknfilter(c)`
fn tknfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TOKEN) or c:IsOriginalType(TYPE_TOKEN)
    card::is_type(d, c, TYPE_TOKEN) || card::is_original_type(d, c, TYPE_TOKEN)
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.tknfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&tknfilter), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0 && duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0
}
/// The Token data of `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_MECHA_PHANTOM_BEAST,SET_MECHA_PHANTOM_BEAST,TYPES_TOKEN,0,0,3,RACE_MACHINE,ATTRIBUTE_WIND)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_MECHA_PHANTOM_BEAST as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(3),
        race: Some(RACE_MACHINE as u64),
        attribute: Some(ATTRIBUTE_WIND),
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()  (unused in the script)
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_MECHA_PHANTOM_BEAST,...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_MECHA_PHANTOM_BEAST, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_MECHA_PHANTOM_BEAST,...) then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, TOKEN_MECHA_PHANTOM_BEAST, token_data(), None, None, 0)
    {
        // local token=Duel.CreateToken(tp,TOKEN_MECHA_PHANTOM_BEAST)
        let token = duel::create_token(d, tp, TOKEN_MECHA_PHANTOM_BEAST).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.synlimit(e,c)`
fn synlimit(d: &mut Duel, c: Option<CardId>) -> bool {
    // if not c then return false end
    let Some(c) = c else {
        return false;
    };
    // return not c:IsRace(RACE_MACHINE)
    !card::is_race(d, c, RACE_MACHINE as u64)
}
