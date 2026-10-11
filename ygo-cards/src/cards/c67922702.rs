//! Mecha Phantom Beast Tetherwolf
//! 幻獣機テザーウルフ
use super::prelude::*;

pub const ID: u32 = 67922702;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_MECHA_PHANTOM_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Mecha Phantom Beast Token" (Machine/WIND/Level 3/ATK 0/DEF 0)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // This card's Level is increased by the total Levels of all "Mecha Phantom Beast Tokens" you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e2, EFFECT_UPDATE_LEVEL);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(function(e,c) return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,TOKEN_MECHA_PHANTOM_BEAST),c:GetControler(),LOCATION_MZONE,0,nil):GetSum(Card.GetLevel) end)
    effect::set_value_fn(d, e2, effect_card_value(value_e2));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // While you control a Token, this card cannot be destroyed by battle or card effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.indcon)
    effect::set_condition(d, e3, Some(lua_fn(indcon)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);

    // Make this card gain 800 ATK, until the end of that turn
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e5, CATEGORY_ATKCHANGE);
    // e5:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e5, EFFECT_TYPE_QUICK_O);
    // e5:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e5, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e5:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e5, EVENT_FREE_CHAIN);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e5, TIMING_DAMAGE_STEP, None);
    // e5:SetCondition(s.atkcon)
    effect::set_condition(d, e5, Some(condition(atkcon)));
    // e5:SetCost(s.atkcost)
    effect::set_cost(d, e5, Some(cost_target(atkcost)));
    // e5:SetOperation(s.atkop)
    effect::set_operation(d, e5, Some(operation(atkop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `function(e,c) return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,TOKEN_MECHA_PHANTOM_BEAST),c:GetControler(),LOCATION_MZONE,0,nil):GetSum(Card.GetLevel) end`
fn value_e2(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let tp = card::get_controler(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[TOKEN_MECHA_PHANTOM_BEAST])), tp, LOCATION_MZONE, 0, NO_EX);
    group::get_sum(d, &g, &|d, x| card::get_level(d, x) as i64)
}
/// `s.tknfilter(c)`
fn tknfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TOKEN) or c:IsOriginalType(TYPE_TOKEN)
    card::is_type(d, c, TYPE_TOKEN) || card::is_original_type(d, c, TYPE_TOKEN)
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return Duel.IsExistingMatchingCard(s.tknfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    let p = effect::get_handler_player(d, e);
    duel::is_existing_matching_card(d, Some(&tknfilter), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_MECHA_PHANTOM_BEAST,SET_MECHA_PHANTOM_BEAST,TYPES_TOKEN,0,0,3,RACE_MACHINE,ATTRIBUTE_WIND) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_MECHA_PHANTOM_BEAST, token_data(), None, None, 0)
    {
        // local token=Duel.CreateToken(tp,TOKEN_MECHA_PHANTOM_BEAST_TETHERWOLF)
        let token = duel::create_token(d, a.tp, TOKEN_MECHA_PHANTOM_BEAST_TETHERWOLF).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// The printed data of the Mecha Phantom Beast Token: Machine/WIND/Level 3/ATK 0/DEF 0
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_MECHA_PHANTOM_BEAST as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(3),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_WIND),
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // return Duel.IsPhase(PHASE_DAMAGE) and not Duel.IsDamageCalculated() and bc and bc:IsControler(1-tp)
    duel::is_phase(d, PHASE_DAMAGE)
        && !duel::is_damage_calculated(d)
        && match bc {
            Some(bc) => card::is_controler(d, bc, 1 - a.tp),
            None => false,
        }
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:HasFlagEffect(id) and Duel.CheckReleaseGroupCost(tp,Card.IsType,1,false,nil,nil,TYPE_TOKEN) end
    if a.chk == Some(0) {
        return !card::has_flag_effect(d, c, ID, None)
            && duel::check_release_group_cost(d, a.tp, Some(&|d, x| card::is_type(d, x, TYPE_TOKEN)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsType,1,1,false,nil,nil,TYPE_TOKEN)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, x| card::is_type(d, x, TYPE_TOKEN)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 0, 1, 0, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // c:UpdateAttack(800,RESETS_STANDARD_DISABLE_PHASE_END)
        card::update_attack(d, c, 800, Some(RESETS_STANDARD_DISABLE_PHASE_END), None, None);
    }
}
