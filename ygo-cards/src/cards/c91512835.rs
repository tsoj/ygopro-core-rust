//! Insect Queen
use super::prelude::*;

pub const ID: u32 = 91512835;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_INSECT_MONSTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must Tribute 1 monster to declare an attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_ATTACK_COST)
    effect::set_code(d, e1, EFFECT_ATTACK_COST);
    // e1:SetCost(s.atcost)
    effect::set_cost(d, e1, Some(lua_fn(atcost)));
    // e1:SetOperation(s.atop)
    effect::set_operation(d, e1, Some(operation(atop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This card gains 200 ATK for each Insect monster on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(s.value)
    effect::set_value_fn(d, e2, effect_card_value(value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register flag if this card destroys a monster by battle
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon 1 "Insect Monster Token" during End Phase
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e4, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::has_flag_effect(d, h, ID, None)
    })));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.atfilter(c,tp)`
fn atfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) or (Duel.IsPlayerAffectedByEffect(tp,300307012) and c:IsLocation(LOCATION_HAND) and c:IsRace(RACE_INSECT))
    card::is_location(d, c, LOCATION_MZONE)
        || (duel::is_player_affected_by_effect(d, tp, 300307012).is_some() && card::is_location(d, c, LOCATION_HAND) && card::is_race(d, c, RACE_INSECT))
}
/// `s.atcost(e,c,tp)`
fn atcost(d: &mut Duel, a: Args) -> bool {
    // return Duel.CheckReleaseGroupCost(tp,s.atfilter,1,true,nil,e:GetHandler(),tp)
    let tp = a.p(2);
    let h = effect::get_handler(d, a.e(0)).nil();
    duel::check_release_group_cost(d, tp, Some(&|d, c| atfilter(d, c, tp)), 1, 1, true, None, h.into())
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.IsAttackCostPaid()~=2 and c:IsLocation(LOCATION_MZONE) then
    if duel::is_attack_cost_paid(d) != 2 && card::is_location(d, c, LOCATION_MZONE) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
        // local tc=Duel.GetReleaseGroup(tp,true,false,REASON_COST):Filter(s.atfilter,c,tp):SelectUnselect(Group.CreateGroup(),tp,Duel.IsAttackCostPaid()==0,Duel.IsAttackCostPaid()==0)
        let rg = duel::get_release_group(d, tp, true, false, Some(REASON_COST));
        let rg = group::filter(d, &rg, &|d, x| atfilter(d, x, tp), c.into());
        let paid0 = duel::is_attack_cost_paid(d) == 0;
        let tc = group::select_unselect(d, &rg, Some(&CardSet::new()), tp, paid0, paid0, None, None);
        // if tc then
        if let Some(tc) = tc {
            // Duel.Release(tc,REASON_COST)
            duel::release(d, CardSet::from_card(tc), REASON_COST, None);
            // Duel.AttackCostPaid()
            duel::attack_cost_paid(d, None);
        } else {
            // Duel.AttackCostPaid(2)
            duel::attack_cost_paid(d, Some(2));
        }
    }
}
/// `s.value(e,c)`
fn value(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_INSECT),0,LOCATION_MZONE,LOCATION_MZONE,nil)*200
    duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_INSECT))), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i64 * 200
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    let c = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
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
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_INSECT_MONSTER,0,TYPES_TOKEN,100,100,1,RACE_INSECT,ATTRIBUTE_EARTH) then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, TOKEN_INSECT_MONSTER, insect_token_data(), None, None, 0)
    {
        // local token=Duel.CreateToken(tp,TOKEN_INSECT_MONSTER)
        let token = duel::create_token(d, tp, TOKEN_INSECT_MONSTER).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP_ATTACK)
        duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP_ATTACK as u8, None);
    }
}
/// The token of `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_INSECT_MONSTER,0,TYPES_TOKEN,100,100,1,RACE_INSECT,ATTRIBUTE_EARTH)`
fn insect_token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(100),
        defense: Some(100),
        level: Some(1),
        race: Some(RACE_INSECT),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
